//! Bancs de test AUTO / EDGE / TORTURE (P2.1, P2.2).
//! Le banc n'exécute QUE les `required_commands` de chaque mission, réellement, dans le bac à
//! sable, depuis la cabine du joueur — il ne prépare jamais lui-même l'état attendu par les
//! checks. Une mission qui échoue ici est un vrai bug de contenu.

use super::{parse, Game};
use crate::shell;

/// Entrée fournie aux commandes pendant les bancs (mots de passe, sessions ssh)
fn bench_stdin(rc: &str) -> Vec<u8> {
    let p = parse::parse(rc);
    if p.effective().iter().any(|(b, _)| b == "ssh") {
        b"exit\n".to_vec()
    } else {
        b"Orion7-Secret!\nOrion7-Secret!\n".to_vec()
    }
}

impl Game {
    fn reset_for_mission(&mut self) {
        self.mission_commands.clear();
        self.pending_code = None;
        self.current_dir = shell::player_home();
        self.previous_dir = self.current_dir.clone();
        self.mark_events();
    }

    /// Joue la solution d'une mission (commandes requises + gsh check + code). Ok = validée.
    pub(super) fn solve_mission(
        &mut self,
        wrong_codes: &[&str],
        issues: &mut Vec<String>,
    ) -> Result<(), String> {
        let m = self.missions[self.current_mission].clone();
        self.reset_for_mission();
        let mut failed_rcs = Vec::new();
        for rc in &m.required_commands {
            let code = self.exec_with(rc, Some(bench_stdin(rc)));
            if code != 0 {
                failed_rcs.push(format!("`{}` → code {}", rc, code));
            }
        }
        let before = self.completed.len();
        let (ok, missing) = self.check_mission_detailed();
        if !ok {
            let why = format!(
                "checks non satisfaits [{}]{}",
                missing.join(", "),
                if failed_rcs.is_empty() {
                    String::new()
                } else {
                    format!(" — commandes en échec: {}", failed_rcs.join(" ; "))
                }
            );
            self.current_mission += 1;
            self.reset_for_mission();
            return Err(why);
        }
        self.cmd_check();
        if self.pending_code.is_some() {
            for w in wrong_codes {
                if self.submit_code(w) {
                    issues.push(format!(
                        "{}: le mauvais code « {} » a validé la mission!",
                        m.id, w
                    ));
                }
            }
            if let Some(code) = self.pending_code.clone() {
                self.submit_code(&code);
            }
        }
        if self.completed.len() > before {
            Ok(())
        } else {
            self.current_mission += 1;
            self.reset_for_mission();
            Err("validation refusée après les checks".into())
        }
    }

    /// Mode auto (GAMESHELL_AUTO=1): joue TOUTES les missions avec leurs seules commandes requises
    pub fn run_auto(&mut self) -> (usize, usize) {
        let total = self.missions.len();
        let mut ok = 0usize;
        let mut fails: Vec<String> = Vec::new();
        let mut issues = Vec::new();
        while self.current_mission < self.missions.len() {
            let id = self.missions[self.current_mission].id.clone();
            match self.solve_mission(&[], &mut issues) {
                Ok(()) => ok += 1,
                Err(e) => {
                    println!("   ⚠️  {}: {}", id, e);
                    fails.push(id);
                }
            }
        }
        println!(
            "🤖 AUTO: {}/{} missions résolues{}",
            ok,
            total,
            if fails.is_empty() {
                String::new()
            } else {
                format!(" — échecs: {}", fails.join(", "))
            }
        );
        (ok, total)
    }

    /// Mode EDGE: cas limites sur chaque mission (check vide, garbage, echo des cibles, ls /armes,
    /// mauvais code, bon code, position et historique remis à zéro après validation)
    pub fn run_edge(&mut self) -> (usize, Vec<String>) {
        let mut ok = 0usize;
        let mut issues: Vec<String> = Vec::new();
        let home = shell::player_home();
        // gsh reset: historique vidé + monde régénéré (C12/D2)
        self.mission_commands.push("ls".into());
        self.current_dir = format!("{}/tmp", shell::world_dir());
        self.cmd_reset();
        if !self.mission_commands.is_empty() {
            issues.push("gsh reset n'a pas vidé l'historique".into());
        }
        if self.current_dir != home {
            issues.push("gsh reset n'a pas remis la position à la cabine".into());
        }
        if !shell::dir_exists(&shell::world_dir()) {
            issues.push("gsh reset a supprimé le monde".into());
        }
        while self.current_mission < self.missions.len() {
            let m = self.missions[self.current_mission].clone();
            self.reset_for_mission();
            if self.cmd_check() {
                issues.push(format!("{}: check à vide a VALIDÉ la mission!", m.id));
            }
            self.pending_code = None;
            let mut traps: Vec<String> = vec!["foobar123".into(), "ls /armes".into()];
            let targets: Vec<String> = m.checks.iter().map(|c| c.target.clone()).collect();
            traps.push(format!(
                "echo {}",
                targets
                    .iter()
                    .map(|t| format!("'{}'", t.replace('\'', "")))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
            for t in &traps {
                self.reset_for_mission();
                self.exec_shell(t);
                if self.check_mission_detailed().0 {
                    issues.push(format!("{}: « {} » a validé la mission!", m.id, t));
                }
            }
            let before = self.completed.len();
            match self.solve_mission(&["MAUVAIS-CODE"], &mut issues) {
                Ok(()) => {
                    ok += 1;
                    if self.current_dir != home {
                        issues.push(format!(
                            "{}: position non remise à la cabine après validation",
                            m.id
                        ));
                    }
                    if !self.mission_commands.is_empty() {
                        issues.push(format!("{}: historique mission non vidé", m.id));
                    }
                    if self.completed.len() != before + 1 {
                        issues.push(format!("{}: compteur de missions incohérent", m.id));
                    }
                }
                Err(e) => issues.push(format!("{}: mission non résolue — {}", m.id, e)),
            }
        }
        (ok, issues)
    }

    /// Mode TORTURE: isolation de l'hôte testée sur l'EXÉCUTION réelle, puis chaque mission
    /// tordue (check vide, moitié des commandes, typo, 2 mauvais codes) avant résolution.
    pub fn run_torture(&mut self) -> (usize, Vec<String>) {
        let mut issues: Vec<String> = Vec::new();
        issues.extend(self.isolation_issues());
        let mut ok = 0usize;
        while self.current_mission < self.missions.len() {
            let m = self.missions[self.current_mission].clone();
            self.reset_for_mission();
            if self.cmd_check() {
                issues.push(format!("{}: check à vide a validé", m.id));
            }
            // moitié des commandes, injectées dans l'historique SANS être exécutées (le monde ne change pas)
            if m.required_commands.len() > 1 {
                let half = m.required_commands.len() / 2;
                self.mission_commands = m.required_commands.iter().take(half).cloned().collect();
                let only_cmd_checks = m
                    .checks
                    .iter()
                    .all(|c| matches!(c.check_type.as_str(), "command_executed" | "file_read"));
                if self.check_mission_detailed().0 && !only_cmd_checks {
                    issues.push(format!("{}: la moitié des commandes (non exécutées) a suffi à valider des checks d'effet", m.id));
                }
            }
            self.reset_for_mission();
            self.mission_commands.push("pwdd".into());
            if self.check_mission_detailed().0 {
                issues.push(format!("{}: typo 'pwdd' a validé", m.id));
            }
            match self.solve_mission(&["FAUX-1", "FAUX-2"], &mut issues) {
                Ok(()) => {
                    ok += 1;
                    if self.current_dir != shell::player_home() {
                        issues.push(format!("{}: position non reset", m.id));
                    }
                    if !self.mission_commands.is_empty() {
                        issues.push(format!("{}: historique non vidé", m.id));
                    }
                }
                Err(e) => issues.push(format!("{}: non résolue en torture — {}", m.id, e)),
            }
        }
        (ok, issues)
    }

    /// Tests d'isolation sur l'exécution réelle (P0.1 / A1 / A16)
    fn isolation_issues(&mut self) -> Vec<String> {
        let mut issues = Vec::new();
        if !shell::sandbox_available() {
            issues.push("ISOLATION: bubblewrap indisponible — isolation de l'hôte NON garantie (installez bubblewrap)".into());
            return issues;
        }
        let real_user = host_user_name();
        let real_home = std::env::var("GSH_REAL_HOME").unwrap_or_default();
        let cap = |g: &mut Game, l: &str| {
            shell::run_line(
                l,
                &g.current_dir,
                &g.previous_dir,
                &shell::RunOpts {
                    capture: true,
                    quiet: false,
                    stdin: Some(Vec::new()),
                },
            )
            .out
        };
        self.reset_for_mission();
        let leaks = |out: &str| {
            (!real_user.is_empty()
                && real_user != shell::world_user()
                && out
                    .split(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
                    .any(|w| w == real_user))
                || (!real_home.is_empty() && real_home.len() > 1 && out.contains(&real_home))
        };
        for l in [
            "cat /../../../../etc/passwd",
            "cat ../../../../../etc/passwd",
            "echo $USER $HOME $LOGNAME",
            "ls -l ~",
            "stat ~",
            "id",
            "whoami",
            "bash -c whoami",
            "uname -a",
            "hostname",
            "pwd; cd /..; cd ../../..; pwd -P",
            "ls -la /",
            "env",
        ] {
            let out = cap(self, l);
            if leaks(&out) {
                issues.push(format!(
                    "FUITE HÔTE: « {} » révèle l'utilisateur ou le dossier réel",
                    l
                ));
            }
        }
        // cd /.. et cd ../../.. restent dans le monde
        self.exec_shell("cd /..");
        self.exec_shell("cd ../../../..");
        if shell::to_world(&self.current_dir).is_none() {
            issues.push(format!(
                "ÉVASION: cd /.. sort du monde ({})",
                self.current_dir
            ));
        }
        self.reset_for_mission();
        let ls_root = cap(self, "ls /");
        for sys in ["bin", "usr", "etc", "proc", "lib"] {
            if ls_root
                .lines()
                .any(|l| l.split_whitespace().any(|w| w == sys))
            {
                issues.push(format!(
                    "ls / montre le dossier système /{} (règle A1)",
                    sys
                ));
            }
        }
        // rm dans le monde ne touche jamais l'hôte: fichier témoin dans le VRAI /tmp
        let witness = format!("/tmp/gsh_temoin_{}", std::process::id());
        let _ = std::fs::write(&witness, "témoin");
        self.exec_shell(&format!("rm -f {}", witness));
        if !std::path::Path::new(&witness).exists() {
            issues.push("FUITE HÔTE: rm dans le jeu a supprimé un fichier du vrai /tmp!".into());
        }
        let _ = std::fs::remove_file(&witness);
        // sudo useradd ne crée jamais de compte sur l'hôte
        self.exec_shell("sudo useradd -m gsh_test_pirate");
        if std::fs::read_to_string("/etc/passwd")
            .map(|s| s.contains("gsh_test_pirate"))
            .unwrap_or(false)
        {
            issues.push("FUITE HÔTE: sudo useradd a créé un compte RÉEL!".into());
        }
        if !crate::sim::users::user_exists(&shell::sim_ctx(), "gsh_test_pirate") {
            issues.push("sudo useradd simulé n'a pas créé le compte dans le jeu".into());
        }
        self.exec_shell("sudo userdel -r gsh_test_pirate");
        // pas de réseau réel
        let net = cap(
            self,
            "bash -c 'echo > /dev/tcp/1.1.1.1/80' 2>&1 && echo CONNECTE",
        );
        if net.contains("CONNECTE") {
            issues.push("FUITE HÔTE: le bac à sable a accès au réseau réel".into());
        }
        self.reset_for_mission();
        issues
    }
}

fn host_user_name() -> String {
    let uid = crate::sys::uid();
    std::fs::read_to_string("/etc/passwd")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                let f: Vec<&str> = l.split(':').collect();
                if f.len() > 2 && f[2].parse::<u32>().ok() == Some(uid) {
                    Some(f[0].to_string())
                } else {
                    None
                }
            })
        })
        .unwrap_or_default()
}

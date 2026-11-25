//! Enveloppes de commandes de fichiers: ls / find masquent les dossiers système du bac à sable,
//! nano exige un terminal, zip/unzip ont un repli intégré si l'outil manque, mount/df thématisés.

use super::{exec_real, real_bin, Ctx};

/// Points de montage techniques du bac à sable (jamais montrés dans `ls /`) — règle A1
pub const SYSTEM_ENTRIES: &[&str] = &[
    "bin", "sbin", "lib", "lib32", "lib64", "libx32", "usr", "etc", "proc", "dev",
];

fn is_root_dir(c: &Ctx, p: &str) -> bool {
    let real = std::fs::canonicalize(p).ok();
    let root = std::fs::canonicalize(&c.root).ok();
    real.is_some() && real == root
}

pub fn ls(c: &Ctx, a: &[String]) -> i32 {
    let operands: Vec<&String> = a
        .iter()
        .filter(|x| !x.starts_with('-') || x.as_str() == "-")
        .collect();
    let lists_root = if operands.is_empty() {
        is_root_dir(c, ".")
    } else {
        operands.iter().any(|o| is_root_dir(c, o))
    };
    let mut args: Vec<String> = Vec::new();
    if lists_root {
        for e in SYSTEM_ENTRIES {
            args.push(format!("--ignore={}", e));
        }
    }
    args.extend(a.iter().cloned());
    exec_real("ls", &args)
}

pub fn find(c: &Ctx, a: &[String]) -> i32 {
    let mut args: Vec<String> = Vec::new();
    let mut in_paths = true;
    for x in a {
        if in_paths && (x.starts_with('-') || x == "(" || x == "!" || x == ",") {
            in_paths = false;
        }
        if in_paths && is_root_dir(c, x) {
            // find / → parcourt les compartiments du monde, pas les dossiers système du bac à sable
            let base = x.trim_end_matches('/');
            let mut entries: Vec<String> = std::fs::read_dir(x)
                .map(|rd| {
                    rd.filter_map(|e| e.ok())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect()
                })
                .unwrap_or_default();
            entries.retain(|e| !SYSTEM_ENTRIES.contains(&e.as_str()));
            entries.sort();
            for e in entries {
                args.push(format!("{}/{}", base, e));
            }
            continue;
        }
        args.push(x.clone());
    }
    exec_real("find", &args)
}

pub fn nano(a: &[String]) -> i32 {
    if !crate::sys::is_tty(0) || !crate::sys::is_tty(1) {
        eprintln!("nano: un terminal interactif est nécessaire");
        return 1;
    }
    exec_real("nano", a)
}

pub fn mount(c: &Ctx) -> i32 {
    let (dev_root, extra) = match c.camp {
        2 => ("/dev/sda1 on / type ext4 (rw,relatime)", "/dev/sdb1 on /greniers type ext4 (rw,relatime)\n/dev/sdc1 on /archives type ext4 (rw,noexec,relatime)"),
        3 => ("/dev/nvme0n1p2 on / type ext4 (rw,relatime)", "/dev/nvme1n1p1 on /logs type xfs (rw,relatime)\n/dev/md0 on /archives type ext4 (rw,noexec,relatime)"),
        _ => ("/dev/vda1 on / type ext4 (rw,relatime)", "/dev/vdb1 on /soutes type ext4 (rw,relatime)\n/dev/vdc1 on /archives type ext4 (rw,noexec,relatime)"),
    };
    println!("{}\nproc on /proc type proc (rw,nosuid,nodev,noexec,relatime)\ntmpfs on /tmp type tmpfs (rw,nosuid,nodev)\n{}", dev_root, extra);
    0
}

pub fn df(c: &Ctx, a: &[String]) -> i32 {
    let human = a.iter().any(|x| x.starts_with('-') && x.contains('h'));
    let rows: Vec<(&str, u64, u64, &str)> = match c.camp {
        2 => vec![
            ("/dev/sda1", 20_480_000, 14_336_000, "/"),
            ("/dev/sdb1", 51_200_000, 46_080_000, "/greniers"),
            ("/dev/sdc1", 10_240_000, 2_048_000, "/archives"),
            ("tmpfs", 2_000_000, 12_000, "/tmp"),
        ],
        3 => vec![
            ("/dev/nvme0n1p2", 102_400_000, 61_440_000, "/"),
            ("/dev/nvme1n1p1", 204_800_000, 188_416_000, "/logs"),
            ("/dev/md0", 409_600_000, 122_880_000, "/archives"),
            ("tmpfs", 4_000_000, 20_000, "/tmp"),
        ],
        _ => vec![
            ("/dev/vda1", 30_720_000, 18_432_000, "/"),
            ("/dev/vdb1", 40_960_000, 36_864_000, "/soutes"),
            ("/dev/vdc1", 10_240_000, 3_072_000, "/archives"),
            ("tmpfs", 2_000_000, 8_000, "/tmp"),
        ],
    };
    let hs = |k: u64| -> String {
        let g = k as f64 / 1_048_576.0;
        if g >= 1.0 {
            format!("{:.1}G", g).replace('.', ",")
        } else {
            format!("{}M", k / 1024)
        }
    };
    if human {
        println!("Sys. de fichiers  Taille Utilisé Dispo Uti% Monté sur");
    } else {
        println!("Sys. de fichiers blocs de 1K   Utilisé    Disponible Uti% Monté sur");
    }
    for (d, size, used, m) in rows {
        let pct = used * 100 / size;
        if human {
            println!(
                "{:<17} {:>6} {:>7} {:>5} {:>3}% {}",
                d,
                hs(size),
                hs(used),
                hs(size - used),
                pct,
                m
            );
        } else {
            println!(
                "{:<16} {:>12} {:>9} {:>13} {:>3}% {}",
                d,
                size,
                used,
                size - used,
                pct,
                m
            );
        }
    }
    0
}

// ─────────────────── zip / unzip: repli intégré (format ZIP « stocké », sans compression) ───────────────────

fn crc32(data: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            c = if c & 1 != 0 {
                0xEDB88320 ^ (c >> 1)
            } else {
                c >> 1
            };
        }
        table[i as usize] = c;
    }
    let mut crc = 0xFFFFFFFFu32;
    for b in data {
        crc = table[((crc ^ *b as u32) & 0xFF) as usize] ^ (crc >> 8);
    }
    !crc
}

fn collect(
    path: &std::path::Path,
    name: &str,
    recursive: bool,
    out: &mut Vec<(String, Option<Vec<u8>>)>,
) {
    if path.is_dir() {
        if !recursive {
            out.push((format!("{}/", name.trim_end_matches('/')), None));
            return;
        }
        out.push((format!("{}/", name.trim_end_matches('/')), None));
        let mut ents: Vec<_> = std::fs::read_dir(path)
            .map(|r| r.filter_map(|e| e.ok()).collect())
            .unwrap_or_default();
        ents.sort_by_key(|e| e.file_name());
        for e in ents {
            collect(
                &e.path(),
                &format!(
                    "{}/{}",
                    name.trim_end_matches('/'),
                    e.file_name().to_string_lossy()
                ),
                true,
                out,
            );
        }
    } else if let Ok(d) = std::fs::read(path) {
        out.push((name.to_string(), Some(d)));
    }
}

pub fn zip(a: &[String]) -> i32 {
    if real_bin("zip").is_some() {
        return exec_real("zip", a);
    }
    let recursive = a.iter().any(|x| x.starts_with('-') && x.contains('r'));
    let quiet = a.iter().any(|x| x.starts_with('-') && x.contains('q'));
    let ops: Vec<&String> = a.iter().filter(|x| !x.starts_with('-')).collect();
    if ops.len() < 2 {
        eprintln!("zip error: Nothing to do! (usage: zip [-r] archive.zip fichiers...)");
        return 12;
    }
    let mut archive = ops[0].clone();
    if !archive.ends_with(".zip") {
        archive.push_str(".zip");
    }
    let mut entries = Vec::new();
    for src in &ops[1..] {
        let p = std::path::Path::new(src.as_str());
        if !p.exists() {
            eprintln!("\tzip warning: name not matched: {}", src);
            continue;
        }
        collect(p, src.trim_start_matches('/'), recursive, &mut entries);
    }
    if entries.is_empty() {
        eprintln!("zip error: Nothing to do! ({})", archive);
        return 12;
    }
    let mut out: Vec<u8> = Vec::new();
    let mut central: Vec<u8> = Vec::new();
    let le16 = |v: &mut Vec<u8>, x: u16| v.extend_from_slice(&x.to_le_bytes());
    let le32 = |v: &mut Vec<u8>, x: u32| v.extend_from_slice(&x.to_le_bytes());
    for (name, data) in &entries {
        let d = data.clone().unwrap_or_default();
        let crc = crc32(&d);
        let off = out.len() as u32;
        le32(&mut out, 0x04034b50);
        le16(&mut out, 10);
        le16(&mut out, 0x0800);
        le16(&mut out, 0);
        le16(&mut out, 0);
        le16(&mut out, 0x5821);
        le32(&mut out, crc);
        le32(&mut out, d.len() as u32);
        le32(&mut out, d.len() as u32);
        le16(&mut out, name.len() as u16);
        le16(&mut out, 0);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&d);
        le32(&mut central, 0x02014b50);
        le16(&mut central, 0x031e);
        le16(&mut central, 10);
        le16(&mut central, 0x0800);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le16(&mut central, 0x5821);
        le32(&mut central, crc);
        le32(&mut central, d.len() as u32);
        le32(&mut central, d.len() as u32);
        le16(&mut central, name.len() as u16);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le16(&mut central, 0);
        le32(
            &mut central,
            if data.is_none() {
                0o40755 << 16 | 0x10
            } else {
                0o100644 << 16
            },
        );
        le32(&mut central, off);
        central.extend_from_slice(name.as_bytes());
        if !quiet {
            println!("  adding: {} (stored 0%)", name);
        }
    }
    let cd_off = out.len() as u32;
    let cd_len = central.len() as u32;
    out.extend_from_slice(&central);
    le32(&mut out, 0x06054b50);
    le16(&mut out, 0);
    le16(&mut out, 0);
    le16(&mut out, entries.len() as u16);
    le16(&mut out, entries.len() as u16);
    le32(&mut out, cd_len);
    le32(&mut out, cd_off);
    le16(&mut out, 0);
    if std::fs::write(&archive, out).is_err() {
        eprintln!("zip I/O error: impossible d'écrire {}", archive);
        return 15;
    }
    0
}

pub fn unzip(a: &[String]) -> i32 {
    if real_bin("unzip").is_some() {
        return exec_real("unzip", a);
    }
    unzip_embedded(a)
}

fn unzip_embedded(a: &[String]) -> i32 {
    let list = a.iter().any(|x| x == "-l");
    let dest = a
        .iter()
        .position(|x| x == "-d")
        .and_then(|i| a.get(i + 1))
        .cloned();
    let archive = match a
        .iter()
        .enumerate()
        .find(|(i, x)| !x.starts_with('-') && (*i == 0 || a[i - 1] != "-d"))
    {
        Some((_, x)) => x.clone(),
        None => {
            eprintln!("UnZip 6.00 — usage: unzip [-l] archive.zip [-d dossier]");
            return 10;
        }
    };
    let data = match std::fs::read(&archive).or_else(|_| std::fs::read(format!("{}.zip", archive)))
    {
        Ok(d) => d,
        Err(_) => {
            eprintln!(
                "unzip:  cannot find or open {}, {}.zip or {}.ZIP.",
                archive, archive, archive
            );
            return 9;
        }
    };
    let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
    let mut i = 0usize;
    let u16at = |d: &[u8], k: usize| u16::from_le_bytes([d[k], d[k + 1]]) as usize;
    let u32at =
        |d: &[u8], k: usize| u32::from_le_bytes([d[k], d[k + 1], d[k + 2], d[k + 3]]) as usize;
    while i + 30 <= data.len() && u32at(&data, i) == 0x04034b50 {
        let method = u16at(&data, i + 8);
        let size = u32at(&data, i + 18);
        let nlen = u16at(&data, i + 26);
        let xlen = u16at(&data, i + 28);
        if i + 30 + nlen + xlen + size > data.len() {
            break;
        }
        let name = String::from_utf8_lossy(&data[i + 30..i + 30 + nlen]).to_string();
        if method != 0 {
            eprintln!(
                "unzip: {} compressé (méthode {}): installez le paquet unzip pour l'extraire",
                name, method
            );
            return 81;
        }
        entries.push((
            name,
            data[i + 30 + nlen + xlen..i + 30 + nlen + xlen + size].to_vec(),
        ));
        i += 30 + nlen + xlen + size;
    }
    if list {
        println!(
            "Archive:  {}\n  Length      Date    Time    Name\n---------  ---------- -----   ----",
            archive
        );
        let mut total = 0;
        for (n, d) in &entries {
            total += d.len();
            println!("{:>9}  2024-01-15 08:00   {}", d.len(), n);
        }
        println!(
            "---------                     -------\n{:>9}                     {} files",
            total,
            entries.len()
        );
        return 0;
    }
    println!("Archive:  {}", archive);
    let base = dest.unwrap_or_else(|| ".".into());
    // Traversal refusé: une entrée de ZIP ne peut jamais s'échapper du répertoire de destination
    let mut skipped = 0usize;
    for (n, d) in &entries {
        let p = match safe_join(&base, n) {
            Some(p) => p,
            None => {
                eprintln!("unzip: entrée ignorée (chemin hors du but): {}", n);
                skipped += 1;
                continue;
            }
        };
        if n.ends_with('/') {
            let _ = std::fs::create_dir_all(&p);
            println!("   creating: {}", n);
            continue;
        }
        if let Some(par) = p.parent() {
            let _ = std::fs::create_dir_all(par);
        }
        let _ = std::fs::write(&p, d);
        println!(" extracting: {}", n);
    }
    if skipped > 0 {
        return 1;
    }
    0
}

/// Racine + nom de ZIP normalisés: refuse `..`, racine absolue et préfixes (traversal).
/// `base` reste relatif au répertoire du joueur; seuls des composants normaux sont acceptés.
fn safe_join(base: &str, name: &str) -> Option<std::path::PathBuf> {
    use std::path::{Component, PathBuf};
    let mut p = PathBuf::from(base);
    for comp in std::path::Path::new(name.trim_start_matches('/')).components() {
        match comp {
            Component::Normal(c) => p.push(c),
            Component::CurDir => {}
            _ => return None,
        }
    }
    Some(p)
}

#[cfg(test)]
mod tests {
    use super::{crc32, safe_join, unzip_embedded};

    fn build_stored_zip(entries: &[(&str, &[u8])]) -> Vec<u8> {
        fn le16(v: &mut Vec<u8>, x: u16) {
            v.extend_from_slice(&x.to_le_bytes());
        }
        fn le32(v: &mut Vec<u8>, x: u32) {
            v.extend_from_slice(&x.to_le_bytes());
        }
        let mut out = Vec::new();
        for (name, data) in entries {
            le32(&mut out, 0x04034b50);
            le16(&mut out, 10);
            le16(&mut out, 0x0800);
            le16(&mut out, 0); // méthode 0 = stocké
            le16(&mut out, 0x5821);
            le16(&mut out, 0);
            le32(&mut out, crc32(data));
            le32(&mut out, data.len() as u32);
            le32(&mut out, data.len() as u32);
            le16(&mut out, name.len() as u16);
            le16(&mut out, 0);
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(data);
        }
        out
    }

    #[test]
    fn unzip_embedded_ignore_traversal() {
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp = std::env::temp_dir().join(format!("gsh_unzip_{}_{}", pid, nanos));
        let base = tmp.join("base");
        std::fs::create_dir_all(&base).unwrap();
        let zip_path = tmp.join("hostile.zip");
        let zip = build_stored_zip(&[
            ("ok.txt", b"legitimate"),
            ("../escaped.txt", b"evil"),
            ("sub/deep.txt", b"deep"),
        ]);
        std::fs::write(&zip_path, &zip).unwrap();
        let rc = unzip_embedded(&[
            "-d".to_string(),
            base.to_string_lossy().into_owned(),
            zip_path.to_string_lossy().into_owned(),
        ]);
        assert_eq!(rc, 1, "une entrée traversée doit échouer");
        assert!(base.join("ok.txt").exists());
        assert_eq!(
            std::fs::read_to_string(base.join("ok.txt")).unwrap(),
            "legitimate"
        );
        assert!(base.join("sub").join("deep.txt").exists());
        assert!(
            !tmp.join("escaped.txt").exists(),
            "aucune entrée ne doit échapper du répertoire de destination"
        );
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn unzip_embedded_est_ecrite_dans_la_but() {
        let pid = std::process::id();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let tmp = std::env::temp_dir().join(format!("gsh_unzip_ok_{}_{}", pid, nanos));
        let base = tmp.join("base");
        std::fs::create_dir_all(&base).unwrap();
        let zip_path = tmp.join("legit.zip");
        let zip = build_stored_zip(&[("a/b/c.txt", b"sciemment"), ("root.txt", b"r")]);
        std::fs::write(&zip_path, &zip).unwrap();
        let rc = unzip_embedded(&[
            "-d".to_string(),
            base.to_string_lossy().into_owned(),
            zip_path.to_string_lossy().into_owned(),
        ]);
        assert_eq!(rc, 0);
        assert_eq!(
            std::fs::read_to_string(base.join("a").join("b").join("c.txt")).unwrap(),
            "sciemment"
        );
        assert!(base.join("root.txt").exists());
        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn safe_join_refuse_traversal() {
        assert!(safe_join(".", "../evasion.txt").is_none());
        assert!(safe_join(".", "foo/../../etc/passwd").is_none());
        assert!(safe_join("/abs", "/etc/passwd").is_some()); // racine trimée = relative sous /abs
        assert!(safe_join(".", "..").is_none());
    }

    #[test]
    fn safe_join_accepte_chemins_normaux() {
        let r = safe_join(".", "sub/dir/fichier.txt").unwrap();
        assert_eq!(
            r,
            std::path::PathBuf::from(".")
                .join("sub")
                .join("dir")
                .join("fichier.txt")
        );
        assert!(safe_join(".", "sub/./x").is_some()); // « . » normal
        assert!(safe_join("/dest", "/sub/x").is_some()); // racine trimée = chemin relatif
    }
}

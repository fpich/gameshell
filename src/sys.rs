//! Appels système bas niveau (FFI libc déclarée à la main: aucune crate externe,
//! le build doit rester 100% offline). Linux / glibc uniquement.

#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_ulong};
use std::sync::atomic::{AtomicBool, Ordering};

pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGTSTP: c_int = 20;
const SIG_IGN: usize = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

#[repr(C)]
#[derive(Default)]
struct Winsize {
    ws_row: u16,
    ws_col: u16,
    ws_xpixel: u16,
    ws_ypixel: u16,
}

extern "C" {
    fn signal(sig: c_int, handler: usize) -> usize;
    fn isatty(fd: c_int) -> c_int;
    fn tcgetattr(fd: c_int, t: *mut Termios) -> c_int;
    fn tcsetattr(fd: c_int, act: c_int, t: *const Termios) -> c_int;
    fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
    fn dup2(old: c_int, new: c_int) -> c_int;
    fn getuid() -> u32;
}

// Drapeaux termios (Linux)
const ISIG: u32 = 0o1;
const ICANON: u32 = 0o2;
const ECHO: u32 = 0o10;
const IEXTEN: u32 = 0o100000;
const ICRNL: u32 = 0o400;
const IXON: u32 = 0o2000;
const VTIME: usize = 5;
const VMIN: usize = 6;
const TIOCGWINSZ: c_ulong = 0x5413;

static SIGNAL_SEEN: AtomicBool = AtomicBool::new(false);

extern "C" fn on_signal(_sig: c_int) {
    // Gestionnaire vide: le parent survit au Ctrl+C destiné à l'enfant.
    // Un gestionnaire (contrairement à SIG_IGN) est remis à zéro par exec():
    // l'enfant garde donc le comportement par défaut et SEUL lui est interrompu (P1.3).
    SIGNAL_SEEN.store(true, Ordering::Relaxed);
}

/// Le jeu survit à Ctrl+C / Ctrl+\ envoyés pendant une commande (P1.3) et
/// ignore Ctrl+Z (pas de contrôle de tâches: un enfant stoppé bloquerait le jeu).
pub fn protect_parent_from_signals() {
    unsafe {
        signal(SIGINT, on_signal as *const () as usize);
        signal(SIGQUIT, on_signal as *const () as usize);
        signal(SIGTSTP, SIG_IGN);
    }
}

/// Dans l'enfant (pre_exec): bwrap et son init ignorent Ctrl+C; le relais `gsh-sim __exec`
/// rétablit ensuite le comportement par défaut pour la commande du joueur (async-signal-safe).
pub fn ignore_int_quit() {
    unsafe {
        signal(SIGINT, SIG_IGN);
        signal(SIGQUIT, SIG_IGN);
    }
}

/// Rétablit SIGINT/SIGQUIT par défaut (relais avant bash)
pub fn default_int_quit() {
    unsafe {
        signal(SIGINT, 0);
        signal(SIGQUIT, 0);
    }
}

/// Pour les simulateurs (ping, top, journalctl -f): Ctrl+C termine proprement.
pub fn catch_sigint() {
    SIGNAL_SEEN.store(false, Ordering::Relaxed);
    unsafe {
        signal(SIGINT, on_signal as *const () as usize);
    }
}

pub fn sigint_seen() -> bool {
    SIGNAL_SEEN.load(Ordering::Relaxed)
}

pub fn is_tty(fd: i32) -> bool {
    unsafe { isatty(fd) == 1 }
}

pub fn get_termios(fd: i32) -> Option<Termios> {
    let mut t = Termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    if unsafe { tcgetattr(fd, &mut t) } == 0 {
        Some(t)
    } else {
        None
    }
}

pub fn set_termios(fd: i32, t: &Termios) -> bool {
    unsafe { tcsetattr(fd, 0, t) == 0 }
}

/// Mode « raw » pour l'éditeur de ligne. OPOST reste actif (\n → \r\n).
/// `timeout_ds` = délai de lecture en dixièmes de seconde (0 = bloquant).
pub fn raw_from(orig: &Termios, timeout_ds: u8) -> Termios {
    let mut t = *orig;
    t.c_lflag &= !(ICANON | ECHO | ISIG | IEXTEN);
    t.c_iflag &= !(ICRNL | IXON);
    if timeout_ds == 0 {
        t.c_cc[VMIN] = 1;
        t.c_cc[VTIME] = 0;
    } else {
        t.c_cc[VMIN] = 0;
        t.c_cc[VTIME] = timeout_ds;
    }
    t
}

/// Termios sans écho (saisie de mot de passe)
pub fn noecho_from(orig: &Termios) -> Termios {
    let mut t = *orig;
    t.c_lflag &= !ECHO;
    t
}

/// Largeur du terminal (80 par défaut)
pub fn term_cols() -> usize {
    let mut ws = Winsize::default();
    let r = unsafe { ioctl(1, TIOCGWINSZ, &mut ws as *mut Winsize) };
    if r == 0 && ws.ws_col > 0 {
        ws.ws_col as usize
    } else {
        80
    }
}

pub fn dup_fd(old: i32, new: i32) -> bool {
    unsafe { dup2(old, new) >= 0 }
}

pub fn uid() -> u32 {
    unsafe { getuid() }
}

/// Largeur d'affichage approximative d'un caractère (wcwidth simplifié):
/// combinants / sélecteurs de variante / ZWJ = 0, emoji et pleine chasse = 2.
pub fn char_width(c: char) -> usize {
    let u = c as u32;
    if u == 0 {
        return 0;
    }
    if u < 0x20 || (0x7f..0xa0).contains(&u) {
        return 0;
    }
    if (0x300..=0x36f).contains(&u)
        || u == 0x200d
        || (0xfe00..=0xfe0f).contains(&u)
        || (0x200b..=0x200f).contains(&u)
        || (0xe0020..=0xe007f).contains(&u)
    {
        return 0;
    }
    let wide = (0x1100..=0x115f).contains(&u)
        || (0x2e80..=0xa4cf).contains(&u)
        || (0xac00..=0xd7a3).contains(&u)
        || (0xf900..=0xfaff).contains(&u)
        || (0xfe30..=0xfe4f).contains(&u)
        || (0xff00..=0xff60).contains(&u)
        || (0xffe0..=0xffe6).contains(&u)
        || (0x1f300..=0x1f64f).contains(&u)
        || (0x1f680..=0x1f6ff).contains(&u)
        || (0x1f900..=0x1f9ff).contains(&u)
        || (0x1fa70..=0x1faff).contains(&u)
        || (0x1f1e6..=0x1f1ff).contains(&u)
        || (0x1f3fb..=0x1f3ff).contains(&u)
        || matches!(u, 0x231a | 0x231b | 0x23e9..=0x23ec | 0x23f0 | 0x23f3 | 0x25fd | 0x25fe | 0x2614 | 0x2615
            | 0x2648..=0x2653 | 0x267f | 0x2693 | 0x26a1 | 0x26aa | 0x26ab | 0x26bd | 0x26be | 0x26c4 | 0x26c5
            | 0x26ce | 0x26d4 | 0x26ea | 0x26f2 | 0x26f3 | 0x26f5 | 0x26fa | 0x26fd | 0x2705 | 0x270a | 0x270b
            | 0x2728 | 0x274c | 0x274e | 0x2753..=0x2755 | 0x2757 | 0x2795..=0x2797 | 0x27b0 | 0x27bf | 0x2b1b
            | 0x2b1c | 0x2b50 | 0x2b55 | 0x1f004 | 0x1f0cf | 0x1f18e | 0x1f191..=0x1f19a | 0x1f201 | 0x1f21a
            | 0x1f22f | 0x1f232..=0x1f236 | 0x1f238..=0x1f23a | 0x1f250 | 0x1f251);
    if wide {
        2
    } else {
        1
    }
}

/// Largeur d'affichage d'une chaîne (séquences ANSI ignorées)
pub fn str_width(s: &str) -> usize {
    let mut w = 0;
    let mut after_zwj = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for c2 in chars.by_ref() {
                    if ('\x40'..='\x7e').contains(&c2) {
                        break;
                    }
                }
            } else {
                chars.next();
            }
            continue;
        }
        if c == '\u{200d}' {
            after_zwj = true;
            continue;
        }
        if after_zwj {
            after_zwj = false;
            // séquence emoji: le glyphe joint (emoji) ne compte pas, mais un glyphe
            // simple après un ZWJ (texte ordinaire) reste compté normalement
            if char_width(c) >= 2 {
                continue;
            }
        }
        let mut cw = char_width(c);
        if cw == 1 && chars.peek() == Some(&'\u{fe0f}') {
            cw = 2;
        } // présentation emoji
        w += cw;
    }
    w
}

#[cfg(test)]
mod termios_tests {
    use super::{
        noecho_from, raw_from, Termios, ECHO, ICANON, ICRNL, IEXTEN, ISIG, IXON, VMIN, VTIME,
    };

    fn base() -> Termios {
        Termios {
            c_iflag: ICRNL | IXON,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: ICANON | ECHO | ISIG | IEXTEN,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        }
    }

    #[test]
    fn raw_mode_clears_all_flags_and_preserves_oflag() {
        let t = raw_from(&base(), 0);
        assert_eq!(t.c_lflag & (ICANON | ECHO | ISIG | IEXTEN), 0);
        assert_eq!(t.c_iflag & (ICRNL | IXON), 0);
        // OPOST reste actif

        assert_eq!(t.c_oflag, base().c_oflag);
        // timeout 0: lecture bloquante
        assert_eq!(t.c_cc[VMIN], 1);
        assert_eq!(t.c_cc[VTIME], 0);
    }

    #[test]
    fn raw_mode_timeout_mode() {
        let t = raw_from(&base(), 3);
        assert_eq!(t.c_cc[VMIN], 0);
        assert_eq!(t.c_cc[VTIME], 3);
    }

    #[test]
    fn noecho_clears_only_echo_and_keeps_icanc_flags() {
        let t = noecho_from(&base());
        assert_eq!(t.c_lflag & ECHO, 0);
        assert_eq!(t.c_lflag & ICANON, ICANON);
        assert_eq!(t.c_lflag & ISIG, ISIG);
        // c_iflag inchangé
        assert_eq!(t.c_iflag, base().c_iflag);
    }
}

#[cfg(test)]
mod str_width_tests {
    use super::str_width;

    #[test]
    fn ascii_width() {
        assert_eq!(str_width(""), 0);
        assert_eq!(str_width("abc"), 3);
        assert_eq!(str_width("abc def"), 7);
    }

    #[test]
    fn emoji_compte_deux_colonnes_et_zwj_neutre() {
        // emoji simple = 2 colonnes
        assert_eq!(str_width("🏆"), 2);
        // séquence ZWJ (famille = emoji + join + emoji) : ne compte que les 2 = 4
        // ménage simple avecZWJ: on teste simplement que ZWJ+emoji ne compte pas le join
        assert_eq!(str_width("a\u{200d}b"), 2); // a + b
                                                // emoji avec variation-selector fe0f
        assert_eq!(str_width("⭐️"), 2);
    }

    #[test]
    fn controle_et_séq_ansi() {
        // caractères de contrôle (code < 0x20) = 0 colonnes
        assert_eq!(str_width("\x1b[1m"), 0);
        assert_eq!(str_width("a\x00b"), 2);
    }
}

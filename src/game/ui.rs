//! Cadres et retours à la ligne calculés sur la LARGEUR D'AFFICHAGE (emoji = 2 colonnes),
//! pour des bordures droites alignées (règles B5/B9, audit P4).

use crate::sys::str_width;

/// Complète `s` avec des espaces jusqu'à `w` colonnes affichées (tronque si trop long)
pub fn pad(s: &str, w: usize) -> String {
    let cur = str_width(s);
    if cur < w {
        return format!("{}{}", s, " ".repeat(w - cur));
    }
    if cur == w {
        return s.to_string();
    }
    // Troncature: les séquences ANSI (ESC [ ... ) passent telles quelles sans
    // être comptées, seule la partie visible est tronquée (jamais de séquence
    // coupée en plein milieu).
    let mut out = String::new();
    let mut acc = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // séquence ANSI terminée par un caractère dans la plage @-~
            out.push(c);
            if chars.peek() == Some(&'[') {
                chars.next();
                out.push('[');
                for c2 in chars.by_ref() {
                    out.push(c2);
                    if ('\x40'..='\x7e').contains(&c2) {
                        break;
                    }
                }
            } else {
                if let Some(c2) = chars.next() {
                    out.push(c2); // séquence ESC + 1 caractère (ex: \x1bM)
                }
            }
            continue;
        }
        let cw = crate::sys::char_width(c);
        if acc + cw > w.saturating_sub(1) {
            break;
        }
        out.push(c);
        acc += cw;
    }
    out.push('…');
    acc += 1;
    format!("{}{}", out, " ".repeat(w.saturating_sub(acc)))
}

/// Retour à la ligne par mots sur `width` colonnes affichées
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    let mut cur_w = 0;
    for word in text.split_whitespace() {
        let ww = str_width(word);
        if cur_w + ww + usize::from(!cur.is_empty()) > width && !cur.is_empty() {
            lines.push(std::mem::take(&mut cur));
            cur_w = 0;
        }
        if !cur.is_empty() {
            cur.push(' ');
            cur_w += 1;
        }
        cur.push_str(word);
        cur_w += ww;
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Boîte simple (╔═╗) de largeur intérieure `w`, lignes déjà prêtes
pub fn simple_box(lines: &[String], w: usize) -> String {
    let mut s = format!("  ╔{}╗\n", "═".repeat(w + 2));
    for l in lines {
        for part in wrap_keep(l, w) {
            s.push_str(&format!("  ║ {} ║\n", pad(&part, w)));
        }
    }
    s.push_str(&format!("  ╚{}╝", "═".repeat(w + 2)));
    s
}

/// Comme wrap, mais conserve une ligne vide
fn wrap_keep(l: &str, w: usize) -> Vec<String> {
    if l.trim().is_empty() {
        vec![String::new()]
    } else if str_width(l) <= w {
        vec![l.to_string()]
    } else {
        wrap(l, w)
    }
}

/// Boîte légère (┌─┐) avec titre
pub fn light_box(title: &str, lines: &[String], w: usize) -> String {
    let t = format!("─ {} ", title);
    let mut s = format!(
        "  ┌{}{}┐\n",
        t,
        "─".repeat((w + 2).saturating_sub(str_width(&t)))
    );
    for l in lines {
        for part in wrap_keep(l, w) {
            s.push_str(&format!("  │ {} │\n", pad(&part, w)));
        }
    }
    s.push_str(&format!("  └{}┘", "─".repeat(w + 2)));
    s
}

/// Boîte épaisse (┏━┓) avec titre, séparateurs « ---- » (ligne ┣━┫)
pub fn heavy_box(title: &str, lines: &[String], w: usize) -> String {
    let t = format!("━ {} ", title);
    let mut s = format!(
        "  ┏{}{}┓\n",
        t,
        "━".repeat((w + 2).saturating_sub(str_width(&t)))
    );
    for l in lines {
        if l == "----" {
            s.push_str(&format!("  ┣{}┫\n", "━".repeat(w + 2)));
            continue;
        }
        for part in wrap_keep(l, w) {
            s.push_str(&format!("  ┃ {} ┃\n", pad(&part, w)));
        }
    }
    s.push_str(&format!("  ┗{}┛", "━".repeat(w + 2)));
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_tronque_ou_remplit() {
        assert_eq!(pad("abc", 5), "abc  ");
        assert_eq!(pad("a", 3), "a  ");
        // emoji (2 colonnes) compté correctement
        let p = pad("🏆", 4);
        assert_eq!(str_width(&p), 4);
        // troncation sans couper une séquence ANSI (du texte brut uniquement)
        let p = pad("abcdefgh", 4);
        assert_eq!(str_width(&p), 4);
        // emoji + texte : au moins un caractère des 2 colonnes compté
        assert_eq!(str_width(&pad("🏆★", 5)), 5);
    }

    #[test]
    fn wrap_casse_sur_mots() {
        assert_eq!(wrap("un deux trois", 10), vec!["un deux", "trois"]);
        // mot dépasse la largeur : il est isolé
        assert_eq!(wrap("abcdefgh", 4), vec!["abcdefgh"]);
        // ligne vide conservée
        assert_eq!(wrap("", 10), vec![String::new()]);
        // horizontalement de 2 emoji
        assert_eq!(str_width(&wrap("ab", 2)[0]), 2);
    }

    #[test]
    fn wrap_keep_preserve_vides() {
        assert_eq!(wrap_keep("", 10), vec![String::new()]);
        assert_eq!(wrap_keep("abc", 10), vec!["abc".to_string()]);
        // mot plus large que la boîte : isolé (pas de coupure destructrice)
        let parts = wrap_keep("aaa", 2);
        assert_eq!(parts, vec!["aaa".to_string()]);
    }
    #[test]
    fn bordures_alignees() {
        let b = heavy_box(
            "🏆 FÉLICITATIONS",
            &[
                "Missions: 100/100".into(),
                "----".into(),
                "🏅 Badge 🗂️ Explorateur".into(),
            ],
            40,
        );
        let widths: Vec<usize> = b.lines().map(str_width).collect();
        assert!(
            widths.iter().all(|w| *w == widths[0]),
            "largeurs inégales: {:?}\n{}",
            widths,
            b
        );
    }
    #[test]
    fn pad_tronque_le_texte_colore_sans_compte_les_echappements() {
        // ANSI colorée de longueur nominale > w: seule la partie visible compte
        let s = "\x1b[31mabcdefgh\x1b[0m"; // "abcdefgh" = 8 colonnes, w = 5
        let p = pad(s, 5);
        assert_eq!(str_width(&p), 5);
        assert!(p.contains("abc"), "contenu conservé: {p:?}");
    }
}

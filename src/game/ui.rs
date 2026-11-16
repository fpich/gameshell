//! Cadres et retours à la ligne calculés sur la LARGEUR D'AFFICHAGE (emoji = 2 colonnes),
//! pour des bordures droites alignées (règles B5/B9, audit P4).

use crate::sys::str_width;

/// Complète `s` avec des espaces jusqu'à `w` colonnes affichées (tronque si trop long)
pub fn pad(s: &str, w: usize) -> String {
    let cur = str_width(s);
    if cur >= w {
        if cur == w { return s.to_string(); }
        // tronque proprement (sans couper une séquence ANSI: on ne tronque que du texte brut)
        let mut out = String::new();
        let mut acc = 0;
        for ch in s.chars() {
            let cw = crate::sys::str_width(&ch.to_string());
            if acc + cw > w.saturating_sub(1) { break; }
            out.push(ch);
            acc += cw;
        }
        out.push('…');
        acc += 1;
        return format!("{}{}", out, " ".repeat(w.saturating_sub(acc)));
    }
    format!("{}{}", s, " ".repeat(w - cur))
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
        if !cur.is_empty() { cur.push(' '); cur_w += 1; }
        cur.push_str(word);
        cur_w += ww;
    }
    if !cur.is_empty() { lines.push(cur); }
    if lines.is_empty() { lines.push(String::new()); }
    lines
}

/// Boîte simple (╔═╗) de largeur intérieure `w`, lignes déjà prêtes
pub fn simple_box(lines: &[String], w: usize) -> String {
    let mut s = format!("  ╔{}╗\n", "═".repeat(w + 2));
    for l in lines {
        for part in wrap_keep(l, w) { s.push_str(&format!("  ║ {} ║\n", pad(&part, w))); }
    }
    s.push_str(&format!("  ╚{}╝", "═".repeat(w + 2)));
    s
}

/// Comme wrap, mais conserve une ligne vide
fn wrap_keep(l: &str, w: usize) -> Vec<String> {
    if l.trim().is_empty() { vec![String::new()] } else if str_width(l) <= w { vec![l.to_string()] } else { wrap(l, w) }
}

/// Boîte légère (┌─┐) avec titre
pub fn light_box(title: &str, lines: &[String], w: usize) -> String {
    let t = format!("─ {} ", title);
    let mut s = format!("  ┌{}{}┐\n", t, "─".repeat((w + 2).saturating_sub(str_width(&t))));
    for l in lines {
        for part in wrap_keep(l, w) { s.push_str(&format!("  │ {} │\n", pad(&part, w))); }
    }
    s.push_str(&format!("  └{}┘", "─".repeat(w + 2)));
    s
}

/// Boîte épaisse (┏━┓) avec titre, séparateurs « ---- » (ligne ┣━┫)
pub fn heavy_box(title: &str, lines: &[String], w: usize) -> String {
    let t = format!("━ {} ", title);
    let mut s = format!("  ┏{}{}┓\n", t, "━".repeat((w + 2).saturating_sub(str_width(&t))));
    for l in lines {
        if l == "----" { s.push_str(&format!("  ┣{}┫\n", "━".repeat(w + 2))); continue; }
        for part in wrap_keep(l, w) { s.push_str(&format!("  ┃ {} ┃\n", pad(&part, w))); }
    }
    s.push_str(&format!("  ┗{}┛", "━".repeat(w + 2)));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bordures_alignees() {
        let b = heavy_box("🏆 FÉLICITATIONS", &["Missions: 100/100".into(), "----".into(), "🏅 Badge 🗂️ Explorateur".into()], 40);
        let widths: Vec<usize> = b.lines().map(|l| str_width(l)).collect();
        assert!(widths.iter().all(|w| *w == widths[0]), "largeurs inégales: {:?}\n{}", widths, b);
    }
}

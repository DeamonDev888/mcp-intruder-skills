// frontmatter.rs — extraction de la description YAML d'un SKILL.md
// (port du parseur awk : la description peut s'étendre sur des lignes indentées)

/// Détecte une ligne YAML "clé:" (fin de description multi-ligne).
fn is_yaml_key(trimmed: &str) -> bool {
    match trimmed.find(':') {
        Some(i) => {
            let key = &trimmed[..i];
            !key.is_empty()
                && key
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        }
        None => false,
    }
}

/// Extrait la valeur du champ `description:` du frontmatter (entre les deux `---`).
pub fn parse_description(text: &str) -> String {
    let mut fm_lines: Vec<&str> = Vec::new();
    let mut state = 0; // 0 avant ---, 1 dans le frontmatter, 2 après
    for line in text.lines() {
        match line.trim() {
            "---" => {
                state += 1;
                if state >= 2 {
                    break;
                }
                continue;
            }
            _ if state == 1 => fm_lines.push(line),
            _ if state >= 2 => break,
            _ => {}
        }
    }
    if fm_lines.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    let mut in_desc = false;
    for line in &fm_lines {
        if !in_desc {
            if let Some(rest) = strip_key(line, "description") {
                out.push_str(rest.trim());
                in_desc = true;
            }
        } else {
            let trimmed = line.trim();
            if (line.starts_with(' ') || line.starts_with('\t')) && !is_yaml_key(trimmed) {
                out.push(' ');
                out.push_str(trimmed);
            } else {
                break;
            }
        }
    }

    // rabat les espaces multiples et retire les guillemets englobants
    let mut collapsed = String::new();
    let mut prev_space = false;
    for c in out.chars() {
        let sp = c == ' ' || c == '\t';
        if !(sp && prev_space) {
            collapsed.push(if sp { ' ' } else { c });
        }
        prev_space = sp;
    }
    let collapsed = collapsed.trim();
    let collapsed = collapsed
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .unwrap_or(collapsed);
    collapsed.to_string()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let t = line.trim_start();
    let rest = t.strip_prefix(key)?;
    let rest = rest.strip_prefix(':')?;
    Some(rest.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let s = "---\nname: x\ndescription: Hello world\n---\nbody";
        assert_eq!(parse_description(s), "Hello world");
    }

    #[test]
    fn multiline() {
        let s = "---\nname: x\ndescription: Line one,\n  continued here\nname2: y\n---\n";
        assert_eq!(parse_description(s), "Line one, continued here");
    }

    #[test]
    fn quoted() {
        let s = "---\ndescription: \"Use when hunting xss\"\n---\n";
        assert_eq!(parse_description(s), "Use when hunting xss");
    }

    #[test]
    fn absent() {
        assert_eq!(parse_description("no frontmatter"), "");
    }
}

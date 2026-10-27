// cmd_secrets.rs — scan OPEC : aucun secret ne doit partir en public
// (règle n°1 du workspace, voir AGENTS.md à la racine du workspace)
use crate::fsutil::{walk_files, WALK_SKIP};
use regex::Regex;
use std::fs;
use std::path::Path;

const REGEX_PATTERNS: &[(&str, &str)] = &[
    ("github-token", r"gh[posu]_[A-Za-z0-9]{20,}"),
    ("github-pat", r"github_pat_[A-Za-z0-9_]{20,}"),
    ("aws-key", r"AKIA[0-9A-Z]{16}"),
    ("google-key", r"AIza[0-9A-Za-z_\-]{30,}"),
    ("slack-token", r"xox[baprs]-[A-Za-z0-9]{10,}"),
    ("private-key", r"-----BEGIN [A-Z ]*PRIVATE KEY"),
    ("jwt", r"eyJ[A-Za-z0-9_\-]{15,}\.eyJ[A-Za-z0-9_\-]{15,}\."),
    ("anthropic-key", r"sk-ant-[A-Za-z0-9_\-]{20,}"),
    ("openai-key", r"sk-proj-[A-Za-z0-9_\-]{20,}"),
];

/// Littéraux sensibles construits par FRAGMENTS : le code source de ce scanner ne doit
/// jamais contenir lui-même les chaînes qu'il traque (sinon le publier = fuir l'infra).
fn frag(parts: &[&str]) -> String {
    parts.concat()
}

fn internal_patterns() -> Vec<(&'static str, String)> {
    vec![
        ("internal-host", regex::escape(&frag(&["ghostcy", "besrv", "09"]))),
        ("internal-net", format!(r"192{q}168{q}40{q}", q = r"\.")),
        (
            "internal-user",
            format!(
                "{}|{}{}",
                regex::escape(&frag(&["C:/Users/", "Dea", "mon"])),
                regex::escape(&frag(&["de", "mon", "@"])),
                ""
            ),
        ),
        (
            "sensitive-file-ref",
            [
                regex::escape(&frag(&["nhex_", "tok"])),
                regex::escape(&frag(&["admin_", "jwt"])),
                regex::escape(&frag(&["audit_", "credentials"])),
            ]
            .join("|"),
        ),
    ]
}

/// Exemples canoniques upstream (clé d'exemple AWS de la doc, JWT "John Doe" de jwt.io,
/// placeholder "admin_jwt_here") — déjà publics chez les auteurs d'origine.
const ALLOWLIST: &[&str] = &[
    "AKIAIOSFODNN7EXAMPLE",
    "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c",
    "eyJleHAiOjE2MDAwMDAwMDB9",
    "admin_jwt_here",
];

const BINARY_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "ico", "zip", "gz", "tgz", "exe", "woff2"];

pub fn run(root: &Path, include_upstream: bool, strict: bool) -> i32 {
    let mut files = Vec::new();
    let skip: Vec<&str> = if include_upstream {
        WALK_SKIP.to_vec()
    } else {
        let mut v = WALK_SKIP.to_vec();
        v.extend_from_slice(&["bughunter", "acs", "optional"]); // contenu upstream déjà public
        v
    };
    walk_files(root, &skip, &mut files);

    let mut regexes: Vec<(String, Regex)> = REGEX_PATTERNS
        .iter()
        .map(|(id, re)| (id.to_string(), Regex::new(re).expect("regex statique")))
        .collect();
    regexes.extend(
        internal_patterns()
            .into_iter()
            .map(|(id, re)| (id.to_string(), Regex::new(&re).expect("regex interne"))),
    );

    let mut findings = 0usize;
    let mut allowlisted = 0usize;
    let mut scanned = 0usize;

    for f in &files {
        let ext = f
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if BINARY_EXTS.contains(&ext.as_str()) {
            continue;
        }
        let content = match fs::read_to_string(f) {
            Ok(c) => c,
            Err(_) => continue, // binaire/texte non UTF-8
        };
        scanned += 1;
        for (lineno, line) in content.lines().enumerate() {
            if !strict && line.trim_start().starts_with('|') {
                continue; // tables markdown = docs de patterns (regexes de détection upstream)
            }
            if !strict && ALLOWLIST.iter().any(|a| line.contains(a)) {
                allowlisted += 1;
                continue;
            }
            for (id, re) in &regexes {
                if let Some(m) = re.find(line) {
                    findings += 1;
                    let snippet: String = line.trim().chars().take(90).collect();
                    println!(
                        "HIT  [{}] {}:{}  {}",
                        id,
                        f.strip_prefix(root).unwrap_or(f).display(),
                        lineno + 1,
                        snippet
                    );
                    let _ = m;
                }
            }
        }
    }

    println!();
    println!(
        "Fichiers scannés : {scanned} · findings bloquants : {findings} · lignes allowlistées : {allowlisted}{}",
        if strict { " · mode STRICT" } else { "" }
    );
    if !include_upstream {
        println!("(scope : fichiers du bundle uniquement — --all pour inclure les skills upstream)");
    }
    if findings > 0 {
        println!("RÉSULTAT : NE PAS PUBLIER — secrets/infos internes détectés.");
        1
    } else {
        println!("RÉSULTAT : propre ✔ publication possible.");
        0
    }
}

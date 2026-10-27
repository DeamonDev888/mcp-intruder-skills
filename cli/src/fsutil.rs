// fsutil.rs — copie récursive + localisation de la racine du bundle
use std::fs;
use std::path::{Path, PathBuf};

/// Copie récursive src → dst. Retourne le nombre de fichiers copiés.
pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<usize> {
    fn rec(s: &Path, d: &Path, c: &mut usize) -> std::io::Result<()> {
        fs::create_dir_all(d)?;
        for e in fs::read_dir(s)? {
            let e = e?;
            let p = e.path();
            let t = d.join(e.file_name());
            if p.is_dir() {
                rec(&p, &t, c)?;
            } else {
                fs::copy(&p, &t)?;
                *c += 1;
            }
        }
        Ok(())
    }
    let mut count = 0;
    rec(src, dst, &mut count)?;
    Ok(count)
}

/// Retrouve la racine du bundle (contient cli/Cargo.toml) en remontant depuis le cwd.
pub fn find_bundle_root() -> Option<PathBuf> {
    let mut d = std::env::current_dir().ok()?;
    loop {
        if d.join("cli").join("Cargo.toml").is_file() {
            return Some(d);
        }
        d = d.parent()?.to_path_buf();
    }
}

/// Liste les sous-dossiers de `dir` contenant un SKILL.md (triés par nom).
pub fn skill_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() && p.join("SKILL.md").is_file() {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

/// Parcours récursif de tous les fichiers, en sautant les dossiers exclus.
pub fn walk_files(root: &Path, skip_dirs: &[&str], out: &mut Vec<PathBuf>) {
    if let Ok(rd) = fs::read_dir(root) {
        for e in rd.flatten() {
            let p = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            if p.is_dir() {
                if skip_dirs.contains(&name.as_str()) {
                    continue;
                }
                walk_files(&p, skip_dirs, out);
            } else {
                out.push(p);
            }
        }
    }
}

const BIN_EXTS: &[&str] = &["", ".exe", ".bat", ".cmd", ".com"];

/// Dossiers jamais parcourus (git, build, archives upstream).
pub const WALK_SKIP: &[&str] = &[".git", "target", ".upstream", ".gen_tmp", "node_modules"];

/// `command -v` portable : cherche le nom dans PATH (extensions Windows incluses).
pub fn in_path(name: &str) -> bool {
    let path = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };
    for dir in path.split(sep) {
        if dir.is_empty() {
            continue;
        }
        for ext in BIN_EXTS {
            let cand = Path::new(dir).join(format!("{name}{ext}"));
            if cand.is_file() {
                return true;
            }
        }
    }
    false
}

/// Un des noms alternatifs ("a|b") est présent dans PATH.
pub fn any_in_path(alternatives: &str) -> bool {
    alternatives.split('|').any(in_path)
}

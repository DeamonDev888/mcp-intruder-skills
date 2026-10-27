// cmd_install.rs — déploie les skills vers un répertoire de skills d'agent
use crate::fsutil::{copy_dir_all, skill_dirs};
use crate::lists::ACS_PACKS;
use std::fs;
use std::path::{Path, PathBuf};

pub struct InstallArgs {
    pub target: Option<String>,
    pub all: bool,
    pub optional_groups: Vec<String>,
    pub dry_run: bool,
}

fn default_target() -> PathBuf {
    let home = std::env::var(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
        .unwrap_or_else(|_| ".".into());
    Path::new(&home).join(".claude").join("skills")
}

fn copy_skill(src_parent: &Path, name: &str, target: &Path, dry: bool) -> std::io::Result<usize> {
    let src = src_parent.join(name);
    if !src.join("SKILL.md").is_file() {
        eprintln!("SKIP (pas de SKILL.md) {name}");
        return Ok(0);
    }
    if dry {
        println!("WOULD  {}/{}", target.display(), name);
        return Ok(1);
    }
    let dst = target.join(name);
    if dst.exists() {
        fs::remove_dir_all(&dst)?;
    }
    copy_dir_all(&src, &dst)?;
    Ok(1)
}

pub fn run(root: &Path, args: InstallArgs) -> i32 {
    let target = args
        .target
        .map(PathBuf::from)
        .unwrap_or_else(default_target);
    let dry = args.dry_run;

    println!(
        "=== mcp-intruder-skills → {} {} ===",
        target.display(),
        if dry { "(dry-run)" } else { "" }
    );

    let mut installed = 0usize;
    // Tier 1 : bughunter/ + acs/
    for dir in [root.join("bughunter"), root.join("acs")] {
        for d in skill_dirs(&dir) {
            let name = d.file_name().unwrap().to_string_lossy().to_string();
            installed += copy_skill(&dir, &name, &target, dry).unwrap_or(0);
        }
    }

    // Packs optionnels
    let groups: Vec<String> = if args.all {
        ACS_PACKS.iter().map(|(g, _)| g.to_string()).chain(["bughunter".to_string()]).collect()
    } else {
        args.optional_groups.clone()
    };
    for g in &groups {
        let dir = root.join("optional").join(g);
        if !dir.is_dir() {
            eprintln!("pack inconnu : {g} (attendus : bughunter, acs-ot-ics, acs-red-team, acs-cloud, acs-container)");
            continue;
        }
        for d in skill_dirs(&dir) {
            let name = d.file_name().unwrap().to_string_lossy().to_string();
            installed += copy_skill(&dir, &name, &target, dry).unwrap_or(0);
        }
    }

    // Commands slash BugHunter → layout Claude Code (~/.claude/commands)
    let cmd_src = root.join("bughunter").join("commands");
    if cmd_src.is_dir() {
        let cmd_dst = target
            .parent()
            .map(|p| p.join("commands"))
            .unwrap_or_else(|| target.join("_commands-bughunter"));
        if dry {
            println!("WOULD  {} (commands slash)", cmd_dst.display());
        } else {
            match copy_dir_all(&cmd_src, &cmd_dst) {
                Ok(n) => println!("commands slash : {n} fichiers → {}", cmd_dst.display()),
                Err(e) => eprintln!("commands : échec copie ({e})"),
            }
        }
    }

    if dry {
        println!("dry-run terminé ({} skills visés).", installed);
    } else {
        let n = skill_dirs(&target).len();
        println!("Installé : {n} skills dans {} (visés cette exécution : {installed})", target.display());
        println!(
            "Suivant : cargo run --release --manifest-path cli/Cargo.toml -- check-prereqs  — vérifier les exigences outils"
        );
    }
    0
}

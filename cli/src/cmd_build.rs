// cmd_build.rs — reconstruit le bundle depuis les tarballs upstream extraits
// (port Rust de l'ancien build.sh ; les listes vivent dans lists.rs)
use crate::fsutil::{copy_dir_all, skill_dirs};
use crate::lists::{ACS_PACKS, BH_T1};
use std::fs;
use std::path::Path;

fn copy_skill(src_skills: &Path, dest_parent: &Path, name: &str, report: &mut String) -> bool {
    let src = src_skills.join(name);
    if src.join("SKILL.md").is_file() {
        let _ = fs::remove_dir_all(dest_parent.join(name));
        match copy_dir_all(&src, &dest_parent.join(name)) {
            Ok(_) => {
                report.push_str(&format!("OK      {name}\n"));
                true
            }
            Err(e) => {
                report.push_str(&format!("ERREUR  {name} ({e})\n"));
                false
            }
        }
    } else {
        report.push_str(&format!("MISSING {name}\n"));
        false
    }
}

pub fn run(root: &Path, upstream: &Path) -> i32 {
    let bh = upstream.join("Claude-BugHunter-main");
    let acs = upstream.join("Anthropic-Cybersecurity-Skills-main");
    if !bh.is_dir() || !acs.is_dir() {
        eprintln!("upstream invalide : il faut {}/Claude-BugHunter-main et {}/Anthropic-Cybersecurity-Skills-main", upstream.display(), upstream.display());
        eprintln!("(télécharger : curl -fsSL -o <dir>/bh.tar.gz https://codeload.github.com/elementalsouls/Claude-BugHunter/tar.gz/refs/heads/main, idem mukul975/Anthropic-Cybersecurity-Skills, puis tar xzf)");
        return 2;
    }

    let mut report = String::new();
    for d in [root.join("bughunter"), root.join("acs"), root.join("optional")] {
        let _ = fs::create_dir_all(&d);
    }
    for (pack_name, _) in ACS_PACKS {
        let _ = fs::create_dir_all(root.join("optional").join(pack_name));
    }

    // BugHunter Tier 1
    for name in BH_T1 {
        copy_skill(&bh.join("skills"), &root.join("bughunter"), name, &mut report);
    }
    // BugHunter restant → optional/bughunter
    let mut bh_rest = 0;
    for d in skill_dirs(&bh.join("skills")) {
        let name = d.file_name().unwrap().to_string_lossy().to_string();
        if !BH_T1.contains(&name.as_str()) {
            copy_skill(&bh.join("skills"), &root.join("optional").join("bughunter"), &name, &mut report);
            bh_rest += 1;
        }
    }
    // commands + docs + licences (pas de cbh : Python exclu, tooling Rust-only)
    let _ = copy_dir_all(&bh.join("commands"), &root.join("bughunter").join("commands"));
    let _ = fs::create_dir_all(root.join("bughunter").join("docs"));
    for f in ["docs/skills.md", "LICENSE", "LICENSE-CONTENT", "NOTICE"] {
        let src = bh.join(f);
        if src.is_file() {
            let _ = fs::copy(&src, root.join("bughunter").join(f.rsplit('/').next().unwrap()));
        }
    }

    // ACS Tier 1
    let tier1 = [crate::lists::ACS_WEB, crate::lists::ACS_API, crate::lists::ACS_AI, crate::lists::ACS_CURATED].concat();
    for name in &tier1 {
        copy_skill(&acs.join("skills"), &root.join("acs"), name, &mut report);
    }
    // ACS packs optionnels
    for (pack, list) in ACS_PACKS {
        for name in *list {
            copy_skill(&acs.join("skills"), &root.join("optional").join(pack), name, &mut report);
        }
    }
    // mappings, couverture, licence
    if acs.join("mappings").is_dir() {
        let _ = fs::remove_dir_all(root.join("acs").join("mappings"));
        let _ = copy_dir_all(&acs.join("mappings"), &root.join("acs").join("mappings"));
    }
    for f in ["ATTACK_COVERAGE.md", "LICENSE"] {
        let src = acs.join(f);
        if src.is_file() {
            let _ = fs::copy(&src, root.join("acs").join(f));
        }
    }

    // résumé
    report.push_str("\n=== RÉSUMÉ ===\n");
    for (label, dir) in [
        ("bughunter", "bughunter"),
        ("optional/bughunter", "optional/bughunter"),
        ("acs", "acs"),
        ("optional/acs-ot-ics", "optional/acs-ot-ics"),
        ("optional/acs-red-team", "optional/acs-red-team"),
        ("optional/acs-cloud", "optional/acs-cloud"),
        ("optional/acs-container", "optional/acs-container"),
    ] {
        let c = skill_dirs(&root.join(dir)).len();
        report.push_str(&format!("{label} : {c} skills\n"));
    }
    let missing = report.lines().filter(|l| l.starts_with("MISSING")).count();
    report.push_str(&format!("MISSING total : {missing}\n"));
    fs::write(root.join("BUILD-REPORT.md"), &report).expect("écriture BUILD-REPORT.md");

    let commands = fs::read_dir(root.join("bughunter").join("commands"))
        .map(|rd| rd.filter(|e| e.is_ok()).count())
        .unwrap_or(0);
    println!("commands copiées : {commands}");
    println!("BugHunter restant (optional) : {bh_rest}");
    println!("MISSING total : {missing}");
    let _ = report.lines().last();
    if missing > 0 {
        1
    } else {
        0
    }
}

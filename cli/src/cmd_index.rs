// cmd_index.rs — (re)génère INDEX.md + skills-manifest.json depuis les SKILL.md du bundle
use crate::frontmatter::parse_description;
use crate::lists::{acs_category, bh_category, ACS_PACKS};
use serde_json::json;
use std::fs;
use std::path::Path;

struct Entry {
    id: String,
    source: &'static str,
    category: String,
    tier: &'static str,
    path: String,
    description: String,
}

fn collect(
    root: &Path,
    dir: &Path,
    source: &'static str,
    tier: &'static str,
    cat: &dyn Fn(&str) -> &'static str,
) -> Vec<Entry> {
    let mut out = Vec::new();
    for d in crate::fsutil::skill_dirs(dir) {
        let name = d.file_name().unwrap().to_string_lossy().to_string();
        let desc = fs::read_to_string(d.join("SKILL.md"))
            .map(|t| parse_description(&t))
            .unwrap_or_default();
        out.push(Entry {
            id: name.clone(),
            source,
            category: cat(&name).to_string(),
            tier,
            path: format!("{}/{}", dir.strip_prefix(root).unwrap_or(dir).display(), name),
            description: desc,
        });
    }
    out
}

fn md_escape(s: &str) -> String {
    let t: String = s.chars().filter(|c| !c.is_control()).collect();
    let t = if t.chars().count() > 240 {
        t.chars().take(240).collect::<String>()
    } else {
        t
    };
    t.replace('|', "\\|")
}

pub fn run(root: &Path) -> i32 {
    let mut entries: Vec<Entry> = Vec::new();

    // Tier 1
    for e in collect(root, &root.join("bughunter"), "bughunter", "tier1", &bh_category) {
        entries.push(e);
    }
    for e in collect(root, &root.join("acs"), "acs", "tier1", &acs_category) {
        entries.push(e);
    }
    // Optionnels
    for e in collect(root, &root.join("optional").join("bughunter"), "bughunter", "optional", &|n| {
        if n.starts_with("hunt-") { "hunt-classe" } else { "autre" }
    }) {
        entries.push(e);
    }
    for (pack, _) in ACS_PACKS {
        let cat = match *pack {
            "acs-ot-ics" => "ot-ics",
            "acs-red-team" => "red-team",
            "acs-cloud" => "cloud",
            "acs-container" => "container",
            _ => "autre",
        };
        for e in collect(root, &root.join("optional").join(pack), "acs", "optional", &|_| cat) {
            entries.push(e);
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));

    // ---- skills-manifest.json
    let skills: Vec<serde_json::Value> = entries
        .iter()
        .map(|e| {
            json!({
                "id": e.id, "source": e.source, "category": e.category,
                "tier": e.tier, "path": e.path, "description": e.description,
            })
        })
        .collect();
    let manifest = json!({
        "generated": "2026-10-26",
        "audit_source": "AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md §5.2",
        "tooling": "mcp-intruder-cli (Rust) — commande index",
        "sources": {
            "bughunter": {
                "repo": "https://github.com/elementalsouls/Claude-BugHunter",
                "commit": "6a153f7cf2c46ab9a81c9e09ff270ec82c77318b",
                "license": "MIT (code) + CC BY 4.0 (contenu)"
            },
            "acs": {
                "repo": "https://github.com/mukul975/Anthropic-Cybersecurity-Skills",
                "commit": "54a798831d2266a3ca61ce68a7acb80b81160d57",
                "license": "Apache-2.0"
            }
        },
        "skills": skills,
    });
    let man_path = root.join("skills-manifest.json");
    fs::write(&man_path, serde_json::to_string_pretty(&manifest).unwrap() + "\n").expect("écriture manifest");

    // ---- INDEX.md
    let count = |src: &str, tier: &str| entries.iter().filter(|e| e.source == src && e.tier == tier).count();
    let bh1 = count("bughunter", "tier1");
    let bh2 = count("bughunter", "optional");
    let acs1 = count("acs", "tier1");
    let acs2 = count("acs", "optional");
    let opt_count = |cat: &str| entries.iter().filter(|e| e.tier == "optional" && e.category == cat).count();

    let mut idx = String::new();
    idx.push_str("# MCP-INTRUDER SKILLS — Bundle d'adoption (catalogue schématisé)\n\n");
    idx.push_str("**Généré par `mcp-intruder-cli index`** — sélections issues de l'audit local `AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md` §5.2 (non publié).\n\n");
    idx.push_str("| Source | Tier 1 | Optionnel | Licence |\n|---|---|---|---|\n");
    idx.push_str(&format!(
        "| [Claude-BugHunter](https://github.com/elementalsouls/Claude-BugHunter) @ `6a153f7` | {bh1} | {bh2} | MIT + CC BY 4.0 |\n"
    ));
    idx.push_str(&format!(
        "| [Anthropic-Cybersecurity-Skills](https://github.com/mukul975/Anthropic-Cybersecurity-Skills) @ `54a7988` | {acs1} | {acs2} | Apache-2.0 |\n"
    ));
    idx.push_str(&format!("| **Total** | **{}** | **{}** | + 15 commands slash + mappings MITRE/NIST/OWASP |\n\n", bh1 + acs1, bh2 + acs2));

    idx.push_str("## Architecture du bundle\n```\nmcp-intruder-skills/\n");
    idx.push_str("├── cli/                  Outil Rust unique (install · check-prereqs · scan-secrets · index · build)\n");
    idx.push_str(&format!("├── bughunter/            Tier 1 ({bh1} skills) + commands/ (15) + licences MIT/CC-BY\n"));
    idx.push_str(&format!("├── acs/                  Tier 1 ({acs1} skills) + mappings/ (ATT&CK Navigator, NIST CSF, OWASP)\n"));
    idx.push_str("├── optional/             Packs opt-in (bughunter · acs-ot-ics · acs-red-team · acs-cloud · acs-container)\n");
    idx.push_str("├── assets/               banner-mcp-intruder.svg\n");
    idx.push_str("├── skills-manifest.json  Catalogue machine (id, source, catégorie, tier, description)\n");
    idx.push_str("└── INDEX.md              Ce catalogue\n```\n\n");

    idx.push_str("## Mécanisme de déclenchement\n");
    idx.push_str("Chaque `SKILL.md` porte un frontmatter YAML ; le champ `description` sert de déclencheur sémantique (auto-load quand la tâche correspond). Le tableau ci-dessous reprend **le frontmatter réel** de chaque skill copié.\n\n---\n\n");

    let mut table = |idx: &mut String, title: &str, filter: &dyn Fn(&Entry) -> bool| {
        idx.push_str(title);
        idx.push_str("\n\n| Skill | Déclencheur (frontmatter) |\n|---|---|\n");
        for e in entries.iter().filter(|e| filter(e)) {
            idx.push_str(&format!("| `{}` | {} |\n", e.id, md_escape(&e.description)));
        }
        idx.push('\n');
    };

    let (b, t1) = ("bughunter", "tier1");
    table(&mut idx, "## BUGHUNTER · Process, reporting & méthodologie (8)", &|e| {
        e.source == b && e.tier == t1 && (e.category == "process-reporting" || e.category == "methodologie")
    });
    table(&mut idx, "## BUGHUNTER · Recon & OSINT (5)", &|e| {
        e.source == b && e.tier == t1 && e.category == "recon-osint"
    });
    let n_hunt = entries.iter().filter(|e| e.source == b && e.tier == "tier1" && e.category == "hunt-classe").count();
    table(&mut idx, &format!("## BUGHUNTER · hunt-* — classes de vulnérabilités ({n_hunt})"), &|e| {
        e.source == b && e.tier == "tier1" && e.category == "hunt-classe"
    });

    idx.push_str("## BUGHUNTER · Commands slash (15)\n\n| Commande | Rôle |\n|---|---|\n");
    for (c, r) in [
        ("/hunt <target>", "Méta-routeur Red Team vs WAPT, charge le bon set de skills"),
        ("/recon <target>", "Recon déterministe (subfinder/crt.sh/probes HTTP)"),
        ("/surface <target>", "Surface d'attaque classée P1/P2/Kill"),
        ("/scope <asset>", "Gate de scope déterministe avant tout toucher HTTP"),
        ("/triage · /validate", "7-Question Gate : PASS/KILL/DOWNGRADE/CHAIN-REQUIRED"),
        ("/report", "Rapport H1/Bugcrowd/Intigriti/Immunefi + CVSS"),
        ("/chain", "Chaînes d'exploit A→B (17+ patterns), timeboxing 20 min"),
        ("/intel <target>", "CVE NVD + rapports divulgués Hacktivity"),
        ("/autopilot", "Modes mémoire : --paranoid/--normal (off) · --quick/--yolo (on)"),
        ("/memory-gc", "Gestion du store mémoire par engagement"),
        ("/pickup <host>", "Historique de chasse d'une cible"),
        ("/remember", "Note manuelle dans le ledger"),
        ("/token-scan", "Scan rug-pull EVM/Solana (8 classes)"),
        ("/web3-audit", "Audit smart contract 10 classes + PoC Foundry"),
    ] {
        idx.push_str(&format!("| `{c}` | {r} |\n"));
    }
    idx.push_str("\nLes commands sont des prompts autonomes : elles fonctionnent sans CLI externe (le tooling du bundle est Rust : `mcp-intruder-cli`).\n\n---\n\n");

    let (a, t1) = ("acs", "tier1");
    table(&mut idx, "## ACS · Web Application Security (46)", &|e| {
        e.source == a && e.tier == t1 && e.category == "web-appsec"
    });
    table(&mut idx, "## ACS · API Security (28)", &|e| {
        e.source == a && e.tier == t1 && e.category == "api-security"
    });
    table(&mut idx, "## ACS · AI Security (14) — incl. audit de serveurs MCP", &|e| {
        e.source == a && e.tier == t1 && e.category == "ai-security"
    });
    table(&mut idx, "## ACS · Transversal curaté (6)", &|e| {
        e.source == a && e.tier == t1 && e.category == "transversal"
    });

    idx.push_str("## ACS · Mappings embarqués (acs/mappings/)\n\n- `attack-navigator-layer.json` — couche ATT&CK Navigator (218 techniques)\n- `mitre-attack/`, `nist-csf/`, `owasp/` — mappings par framework\n- `ATTACK_COVERAGE.md` — résumé de couverture (290 techniques distinctes sur 805 skills)\n\n---\n\n");

    idx.push_str("## Packs optionnels (activables via `mcp-intruder-cli install --all` ou `--optional <groupe>`)\n\n| Groupe | Skills | Intérêt |\n|---|---|---|\n");
    idx.push_str(&format!("| `optional/bughunter` | {} | Le reste des hunt-* : frameworks (Laravel/Next.js/Spring), k8s, captcha, brute-force… |\n", opt_count("hunt-classe") + opt_count("autre")));
    idx.push_str(&format!("| `optional/acs-ot-ics` | {} | Modbus/DNP3/S7comm — OT/ICS |\n", opt_count("ot-ics")));
    idx.push_str(&format!("| `optional/acs-red-team` | {} | AD/Kerberos/BloodHound/ADCS/C2 |\n", opt_count("red-team")));
    idx.push_str(&format!("| `optional/acs-cloud` | {} | AWS/Azure/GCP IAM, S3, Pacu/ScoutSuite/CloudFox |\n", opt_count("cloud")));
    idx.push_str(&format!("| `optional/acs-container` | {} | Docker/K8s escape, Falco, Trivy, RBAC |\n", opt_count("container")));
    idx.push_str("\n---\n\n");

    idx.push_str("## Tooling Rust (cli/)\n\n`cargo run --release --manifest-path cli/Cargo.toml -- <commande>`\n\n| Commande | Rôle |\n|---|---|\n| `install [--target DIR] [--all] [--optional G] [--dry-run]` | Déploie les skills (défaut `~/.claude/skills`) + commands slash |\n| `check-prereqs` | Exigences outils (exit 1 si un requis manque) |\n| `scan-secrets [--all] [--strict]` | Scan OPSEC — **à passer avant tout push** (règle AGENTS.md) |\n| `index` | Régénère ce fichier + skills-manifest.json |\n| `build --upstream DIR` | Reconstruit le bundle depuis les tarballs extraits |\n\n---\n\n");
    idx.push_str("## Prérequis outils\n\nVoir [tools-requirements.md](tools-requirements.md) et vérifier avec `mcp-intruder-cli check-prereqs` (à lancer sur la cible d'installation).\n");

    let idx_path = root.join("INDEX.md");
    fs::write(&idx_path, idx).expect("écriture INDEX.md");

    println!("entries: {}", entries.len());
    println!("INDEX.md : {} lignes", fs::read_to_string(&idx_path).unwrap().lines().count());
    println!("skills-manifest.json : {} skills", entries.len());
    0
}

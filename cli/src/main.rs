// mcp-intruder-cli — outil Rust unique du bundle mcp-intruder-skills
// (remplace install.sh, check_prereqs.sh, build.sh, gen_index.sh ; le Python cbh upstream est exclu)
mod cmd_build;
mod cmd_index;
mod cmd_install;
mod cmd_prereqs;
mod cmd_secrets;
mod fsutil;
mod frontmatter;
mod lists;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "mcp-intruder-cli",
    version = "1.0.0",
    about = "Outil Rust unique du bundle mcp-intruder-skills : install · check-prereqs · scan-secrets · index · build"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Déploie les skills vers un répertoire de skills d'agent (défaut : ~/.claude/skills)
    Install {
        #[arg(long)]
        target: Option<String>,
        #[arg(long, help = "inclure tous les packs optionnels")]
        all: bool,
        #[arg(long = "optional", help = "ajouter un pack : bughunter|acs-ot-ics|acs-red-team|acs-cloud|acs-container")]
        optional_groups: Vec<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Vérifie les exigences outils (exit 1 si un requis manque)
    CheckPrereqs,
    /// Scan OPSEC de secrets/infos internes — À PASSER AVANT TOUT PUSH (règle n°1, AGENTS.md)
    ScanSecrets {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long, help = "inclure aussi les skills upstream (bughunter/ acs/ optional/)")]
        all: bool,
        #[arg(long, help = "sans allowlist ni skip des tables markdown")]
        strict: bool,
    },
    /// (Re)génère INDEX.md + skills-manifest.json à la racine du bundle
    Index,
    /// Reconstruit le bundle depuis les tarballs upstream extraits (--upstream DIR)
    Build {
        #[arg(long, help = "dossier contenant Claude-BugHunter-main/ et Anthropic-Cybersecurity-Skills-main/")]
        upstream: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let code = match cli.cmd {
        Cmd::Install { target, all, optional_groups, dry_run } => {
            let root = fsutil::find_bundle_root().unwrap_or_else(|| PathBuf::from("."));
            cmd_install::run(&root, cmd_install::InstallArgs { target, all, optional_groups, dry_run })
        }
        Cmd::CheckPrereqs => cmd_prereqs::run(),
        Cmd::ScanSecrets { path, all, strict } => {
            let root = PathBuf::from(&path);
            if !root.is_dir() {
                eprintln!("chemin introuvable : {path}");
                2
            } else {
                cmd_secrets::run(&root, all, strict)
            }
        }
        Cmd::Index => {
            let root = fsutil::find_bundle_root().unwrap_or_else(|| PathBuf::from("."));
            cmd_index::run(&root)
        }
        Cmd::Build { upstream } => {
            let root = fsutil::find_bundle_root().unwrap_or_else(|| PathBuf::from("."));
            cmd_build::run(&root, &PathBuf::from(upstream))
        }
    };
    std::process::exit(code);
}

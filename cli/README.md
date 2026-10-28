# mcp-intruder-cli

Outil **Rust unique** du bundle [mcp-intruder-skills](https://github.com/DeamonDev888/mcp-intruder-skills).
Remplace l'intégralité du tooling (ex-`install.sh`, `check_prereqs.sh`, `build.sh`, `gen_index.sh`)
— zéro script shell, zéro Python. Dépendances : `clap`, `serde_json`, `regex` (3 crates).

## Build

```bash
cargo build --release          # depuis cli/
cargo test --release           # 4 tests unitaires (parseur frontmatter)
```

## Commandes (tools)

| Commande | Rôle | Exit |
|---|---|---|
| `install [--target DIR] [--all] [--optional G]… [--dry-run]` | Déploie les skills Tier 1 + packs vers `DIR` (défaut `~/.claude/skills`) et copie les 15 commands slash vers `DIR/../commands` | 0 |
| `check-prereqs` | Vérifie les exigences outils des skills adoptés (21 requis + 17 optionnels, avec commande d'installation pour chaque manquant) | 1 si un requis manque |
| `scan-secrets [PATH] [--all] [--strict]` | **Scan OPSEC avant tout push** : patterns de secrets (GitHub/AWS/Google/Slack/JWT/clés privées…) + infos internes (hostname, réseau, chemins utilisateurs, noms de fichiers sensibles) | 1 si un hit bloquant |
| `index` | (Re)génère `INDEX.md` + `skills-manifest.json` (340 entrées : id, source, catégorie, tier, déclencheur réel extrait du frontmatter) | 0 |
| `build --upstream DIR` | Reconstruit le bundle depuis les tarballs upstream extraits (`Claude-BugHunter-main/` + `Anthropic-Cybersecurity-Skills-main/`) ; les sélections vivent dans `src/lists.rs` | 1 si un skill manquant |

Exemples :

```bash
mcp-intruder-cli install --target ~/.claude/skills --optional acs-ot-ics
mcp-intruder-cli install --dry-run
mcp-intruder-cli check-prereqs
mcp-intruder-cli scan-secrets                     # scope : fichiers du bundle (authored)
mcp-intruder-cli scan-secrets --all --strict      # + skills upstream, sans allowlist
mcp-intruder-cli index
mcp-intruder-cli build --upstream .upstream
```

## OPSEC (règle n°1 du workspace)

- `scan-secrets` est **le gate obligatoire avant tout push public** (voir `AGENTS.md` à la racine du workspace).
- Les patterns sensibles (hostname interne, réseau interne, chemins utilisateur, noms de fichiers
  sensibles) sont construits **par fragments à l'exécution** : ce code source ne contient jamais
  les chaînes qu'il traque — publier le scanner ne fuit donc rien.
- Mode par défaut tolérant : allowlist des exemples canoniques upstream (clé d'exemple AWS de la doc,
  JWT "John Doe" de jwt.io, placeholders) + skip des tables markdown (docs de regexes de détection).
  `--strict` désactive les deux ; `--all` inclut les dossiers upstream `bughunter/ acs/ optional/`.

## MCP — état et feuille de route

Ce bundle est **la couche skills/connaissances** : il n'expose pas de serveur MCP aujourd'hui.
Le serveur MCP (protocole Streamable HTTP via `litemcp`) vit dans le dépôt **`mcp_intruder`**
(refactor en cours dans une session séparée — voir l'audit interne).

La feuille de route MCP complète (~25 outils planifiés : `dns_scan`, `port_scan`, `tls_audit`,
`nuclei_scan`, `chainx_run`, …) est publiée ici : [MCP-ROADMAP.md](../MCP-ROADMAP.md).

Ce que ce CLI apporte déjà à la chaîne MCP côté OPSEC :
- `scan-secrets` = le gate de publication de tout artefact (repo, bundle, futur serveur MCP) ;
- `check-prereqs` = la précondition d'environnement avant d'exposer des outils de scan.

## Structure du crate

```
cli/
├── Cargo.toml
└── src/
    ├── main.rs          — clap (dispatch des 5 sous-commandes)
    ├── lists.rs         — sélections curatées (45 BH + 94 ACS + packs, catégories)
    ├── fsutil.rs        — copie récursive, localisation racine, recherche PATH portable
    ├── frontmatter.rs   — parseur description YAML (multilignes) + tests unitaires
    ├── cmd_install.rs   — déploiement
    ├── cmd_prereqs.rs   — exigences outils
    ├── cmd_secrets.rs   — scan OPSEC
    ├── cmd_index.rs     — génération INDEX.md + manifest JSON
    └── cmd_build.rs     — reconstruction depuis upstream
```

## Licences

Code MIT (bundle DeamonDev888). Contenu des skills adoptés : MIT + CC BY 4.0 (BugHunter),
Apache-2.0 (ACS) — licences conservées dans `bughunter/` et `acs/`. Voir LICENSE et NOTICE
à la racine du dépôt.

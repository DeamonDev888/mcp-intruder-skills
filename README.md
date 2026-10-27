![MCP-Intruder Skills](assets/banner-mcp-intruder.png)

# mcp-intruder-skills — Bundle d'adoption BugHunter + Anthropic-Cybersecurity-Skills

Bundle prêt à déployer construit le **2026-10-26** à partir de l'audit local
`AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md` (§5.2 « couche cerveau », non publié).
Il réunit la sélection curatée de deux référentiels, organisée en outils schématisés,
avec un **tooling 100 % Rust** (`cli/` — aucune dépendance bash/Python).

> **Ce bundle ne touche pas au code de `mcp_intruder`** — c'est la couche connaissances/skills,
> indépendante de la refactor Rust en cours.

## Contenu

| Élément | Quantité | Détail |
|---|---|---|
| Skills Tier 1 | **139** | 45 BugHunter + 94 ACS — installés par défaut |
| Skills optionnels | 201 | 38 BugHunter + packs ACS OT/ICS (29), Red Team (35), Cloud (66), Container (33) |
| Commands slash | 15 | `/hunt /recon /surface /scope /triage /validate /report /chain /intel …` |
| CLI Rust `mcp-intruder-cli` | 5 commandes | install · check-prereqs · scan-secrets · index · build |
| Feuille de route MCP | — | [MCP-ROADMAP.md](MCP-ROADMAP.md) — les ~25 outils MCP planifiés pour `mcp_intruder` (P0→P3) |
| Mappings | 3 | ATT&CK Navigator layer, NIST CSF, OWASP Top 10 (ACS `mappings/`) |

- **Catalogue lisible :** [INDEX.md](INDEX.md) — chaque skill avec son déclencheur (frontmatter réel).
- **Catalogue machine :** [skills-manifest.json](skills-manifest.json) — 340 entrées typées (source/catégorie/tier).
- **Exigences outils :** [tools-requirements.md](tools-requirements.md) + `mcp-intruder-cli check-prereqs`.
- **OPSEC :** `mcp-intruder-cli scan-secrets` — à passer avant tout push (zéro secret en public, règle n°1).

## Tooling Rust (cli/)

Un seul binaire, quatre dépendances Rust (clap, serde_json, regex) — zéro script shell, zéro Python :

```bash
cargo build --release --manifest-path cli/Cargo.toml
CLI=cli/target/release/mcp-intruder-cli

$CLI install --target ~/.claude/skills                    # Tier 1 → ~/.claude/skills
$CLI install --optional acs-ot-ics                        # + pack Modbus/ICS
$CLI install --all                                        # tout (139 + 201)
$CLI install --dry-run                                    # simulation
$CLI check-prereqs                                        # exigences outils (exit 1 si requis manquant)
$CLI scan-secrets                                         # scan OPSEC (fichiers du bundle)
$CLI scan-secrets --all --strict                          # scan étendu upstream, sans allowlist
$CLI index                                                # régénère INDEX.md + skills-manifest.json
$CLI build --upstream .upstream                           # reconstruit depuis les tarballs extraits
```

Les patterns sensibles du scanner (hostnames, chemins utilisateurs) sont construits par fragments
à l'exécution : le code source du scanner ne contient jamais les chaînes qu'il traque.

## Déploiement sur le serveur Kali

```bash
# 1. copier le dossier mcp-intruder-skills/ sur votre serveur (scp/rsync), puis :
cd mcp-intruder-skills
cargo build --release --manifest-path cli/Cargo.toml
cli/target/release/mcp-intruder-cli check-prereqs         # exigences outils
cli/target/release/mcp-intruder-cli install --optional acs-ot-ics
```

Les skills deviennent auto-déclenchés : le frontmatter `description` sert de déclencheur sémantique.
Les commands BugHunter vont dans `~/.claude/commands/` (elles forcent le chargement des skills).

## Déploiement Windows (optionnel)

```bash
$CLI install --target "C:/Users/<user>/.agents/skills"
```

⚠️ Chaque skill installé localement ajoute sa description au contexte de **toutes** les futures
sessions de l'agent (139 skills ≈ charge notable). Préférer le déploiement sur Kali, ou installer
localement un sous-ensemble via `--optional` après avoir déplacé les skills voulus dans un pack.

## Utilisation type (chaîne BugHunter sur une cible autorisée)

Les 15 commands slash sont des prompts autonomes — elles fonctionnent sans CLI externe :

```
/scope <asset>      → gate de scope avant tout test
/hunt <target>      → route vers les hunt-* pertinents
/triage             → 7-Question Gate sur chaque finding
/report             → brouillon H1/Bugcrowd (CVSS 3.1)
```

Règle d'or reprise de BugHunter : **la 7-Question Gate (`triage-validation`) avant toute
déclaration de vulnérabilité** ; `evidence-hygiene` pour les preuves (masquage cookies/PII).

## Mise à jour du bundle

`mcp-intruder-cli build` + `mcp-intruder-cli index` reconstruisent tout depuis les tarballs :

```bash
mkdir -p .upstream && cd .upstream
curl -fsSL -o bh.tar.gz  https://codeload.github.com/elementalsouls/Claude-BugHunter/tar.gz/refs/heads/main
curl -fsSL -o acs.tar.gz https://codeload.github.com/mukul975/Anthropic-Cybersecurity-Skills/tar.gz/refs/heads/main
tar xzf bh.tar.gz && tar xzf acs.tar.gz && cd ..
cargo run --release --manifest-path cli/Cargo.toml -- build --upstream .upstream
cargo run --release --manifest-path cli/Cargo.toml -- index
rm -rf .upstream
```

Note : les `scripts/*.py` présents **à l'intérieur** des skills (helpers ACS/BugHunter) font partie
du contenu upstream des skills — ce ne sont pas du tooling du bundle. Le tooling proprement dit
(installation, vérification, scan, indexation, build) est intégralement Rust.

## Licences (respectées par la copie)

- **BugHunter** : code MIT, contenu (skills) CC BY 4.0 → `bughunter/LICENSE`, `LICENSE-CONTENT`, `NOTICE` conservés (attribution obligatoire).
- **ACS** : Apache-2.0 → `acs/LICENSE` conservé, licence par dossier skill conservée.

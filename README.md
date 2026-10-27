![MCP-Intruder Skills](assets/banner-mcp-intruder.png)

# mcp-intruder-skills — Bundle d'adoption BugHunter + Anthropic-Cybersecurity-Skills

Bundle prêt à déployer construit le **2026-10-26** à partir de l'audit local
`AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md` (§5.2 « couche cerveau », non publié).
Il réunit la sélection curatée de deux référentiels, organisée en outils schématisés et déployable
sur le serveur Kali (`votre-serveur-kali`) ou toute machine d'agent.

> **Ce bundle ne touche pas au code de `mcp_intruder`** — c'est la couche connaissances/skills,
> indépendante de la refactor Rust en cours.

## Contenu

| Élément | Quantité | Détail |
|---|---|---|
| Skills Tier 1 | **139** | 45 BugHunter + 94 ACS — installés par défaut |
| Skills optionnels | 201 | 38 BugHunter + packs ACS OT/ICS (29), Red Team (35), Cloud (66), Container (33) |
| Commands slash | 15 | `/hunt /recon /surface /scope /triage /validate /report /chain /intel …` |
| CLI `cbh` | 1 | recon/surface/classify/triage/report (python3 ≥ 3.9, stdlib uniquement) |
| Mappings | 3 | ATT&CK Navigator layer, NIST CSF, OWASP Top 10 (ACS `mappings/`) |

- **Catalogue lisible :** [INDEX.md](INDEX.md) — chaque skill avec son déclencheur (frontmatter réel).
- **Catalogue machine :** [skills-manifest.json](skills-manifest.json) — 340 entrées typées (source/catégorie/tier).
- **Exigences outils :** [tools-requirements.md](tools-requirements.md) + `check_prereqs.sh`.
- **Journal de construction :** [BUILD-REPORT.md](BUILD-REPORT.md).

## Déploiement sur le serveur Kali

```bash
# 1. copier le dossier mcp-intruder-skills/ sur votre-serveur-kali (scp/rsync), puis :
cd mcp-intruder-skills
bash check_prereqs.sh                  # exigences outils (exit 1 si un requis manque)
./install.sh                           # Tier 1 → ~/.claude/skills
./install.sh --optional acs-ot-ics     # + pack Modbus/ICS (cible Modbus 502 déjà vue)
./install.sh --all                     # ou tout d'un coup
```

Les skills deviennent auto-déclenchés : le frontmatter `description` sert de déclencheur sémantique.
Les commands BugHunter vont dans `~/.claude/commands/` (elles forcent le chargement des skills).

## Déploiement Windows (optionnel)

```bash
./install.sh --target "C:/Users/<user>/.agents/skills"
```

⚠️ Chaque skill installé localement ajoute sa description au contexte de **toutes** les futures
sessions de l'agent (139 skills ≈ charge notable). Préférer le déploiement sur Kali, ou installer
localement un sous-ensemble via `--optional` après avoir déplacé les skills voulus dans un pack.

## Utilisation type (chaîne BugHunter sur une cible autorisée)

```bash
python3 ~/.claude/skills/_cbh-cli/cbh.py recon target.com   # recon déterministe → recon/target.com/
# puis dans l'agent :
#   /scope <asset>      → gate de scope avant tout test
#   /hunt <target>      → route vers les hunt-* pertinents
#   /triage             → 7-Question Gate sur chaque finding
#   /report             → brouillon H1/Bugcrowd (CVSS 3.1)
```

Règle d'or reprise de BugHunter : **la 7-Question Gate (`triage-validation`) avant toute
déclaration de vulnérabilité** ; `evidence-hygiene` pour les preuves (masquage cookies/PII).

## Mise à jour du bundle

`build.sh` + `gen_index.sh` reconstruisent tout depuis les tarballs `.upstream/` :

```bash
cd mcp-intruder-skills/.upstream
curl -fsSL -o bh.tar.gz  https://codeload.github.com/elementalsouls/Claude-BugHunter/tar.gz/refs/heads/main
curl -fsSL -o acs.tar.gz https://codeload.github.com/mukul975/Anthropic-Cybersecurity-Skills/tar.gz/refs/heads/main
tar xzf bh.tar.gz && tar xzf acs.tar.gz
cd .. && bash build.sh && bash gen_index.sh && rm -rf .upstream
```

## Licences (respectées par la copie)

- **BugHunter** : code MIT, contenu (skills) CC BY 4.0 → `bughunter/LICENSE`, `LICENSE-CONTENT`, `NOTICE` conservés (attribution obligatoire).
- **ACS** : Apache-2.0 → `acs/LICENSE` conservé, licence par dossier skill conservée.

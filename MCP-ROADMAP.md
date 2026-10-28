# MCP-ROADMAP — feuille de route MCP du projet MCP-Intruder

**Statut (2026-10-26) :** le bundle `mcp-intruder-skills` fournit la couche skills/connaissances.
Le **serveur MCP** appartient au dépôt `mcp_intruder` (moteur Rust de scan, refactor en cours dans
une session dédiée). Cette page publie le plan, extraite de l'audit interne
`AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md` (§5-7) — document conservé en local.

## État actuel du serveur MCP (`mcp_intruder`) — ✅ P0 RÉALISÉ

- Crate `litemcp` **0.9.0** en place (Streamable HTTP, JSON-RPC géré par la crate) — propulsée par
  [LiteMCP](https://github.com/DeamonDev888/litemcp), la lib MCP la plus légère et la plus rapide.
- `src/bin/mcp_server.rs` **écrit et publié** : transport Streamable HTTP (`POST /mcp`, bind
  `127.0.0.1:7808`) + endpoint santé `GET /health`.
- **21 outils exposés** : `ping`, `scan_target`, `scan_history`, `tools_diag`, `dns_audit`,
  `tls_audit`, `http_audit`, `whois_audit`, `geo_locate`, `email_sec_audit`, `subdomains_audit`,
  `crt_sh_lookup`, `waf_detect`, `tech_stack_audit`, `dnsrecon_audit`, `dns_hardening_audit`,
  + plan de contrôle jobs asynchrones : `job_start`, `job_status`, `job_result`, `job_stop`, `job_list`.

## P0 — rendre le serveur MCP fonctionnel : ✅ FAIT

- ✅ `mcp_server.rs` écrit (bind + serve sur `build_server()`).
- ✅ Modules de scan exposés comme outils MCP (lecture-seule) — 21 outils (voir ci-dessus).
- ✅ Plan de contrôle jobs : exécutions longues en arrière-plan (scan complet, module lourd,
  binaire d'impact en liste blanche) rendant un `job_id` immédiat.
- ⬜ Transport stdio complémentaire (pour usage local type Claude Code hors HTTP) — à arbitrer.
- ✅ Tests MCP dédiés : initialize / tools/list / tools/call + erreurs — dispatch en mémoire
  (`ServerCore`, sans socket ni PostgreSQL), suite verte (152 tests lib, clippy `-D warnings`) ;
  régressions dédiées : cible malformée refusée en `invalid_input` avant tout réseau,
  `ping` strict (champ inconnu → -32602). Reste la CI comme exécuteur automatique.

## P1 — élargir la surface MCP

- ✅ Wrappers Kali couverts par les 21 outils : `waf_detect`, `dnsrecon_audit`, `tech_stack_audit`,
  `subdomains_audit`, `whois_audit`, `crt_sh_lookup`, `geo_locate`, `email_sec_audit`…
- ✅ Bins d'impact : exécutables via `job_start` (`kind: bin`, liste blanche + `args`).
- ⬜ `nuclei_scan`, `nikto_scan`, `nmap_deep_scan`, `ffuf_scan`, `http_probe` en outils dédiés
  (aujourd'hui couverts via `scan_target`/`job_start` selon profil).
- ⬜ Mapping MITRE ATT&CK des catégories de findings + croisement CISA KEV.
- ⬜ CI minimale (clippy `-D warnings`, tests, tests MCP).

## P2 — couche cerveau (RÉALISÉ côté skills)

- ✅ Adoption des skills : ce dépôt (`139 Tier 1 + 201 opt-in`, voir [INDEX.md](INDEX.md)).
- ✅ `triage-validation` (7-Question Gate), `evidence-hygiene`, `report-writing` (CVSS 3.1).
- ⬜ Brancher `performing-cve-prioritization-with-kev-catalog` sur les findings CVE du moteur.
- ⬜ Appliquer `auditing-mcp-servers-for-tool-poisoning` **au futur serveur MCP lui-même**
  (auto-audit tool poisoning/shadowing avant exposition).

## P3 — extensions moteur (arbitrer avec les cibles réelles)

- Probes web read-only manquantes : open redirect, host header, CRLF (XSS/SSRF restent du ressort
  des skills côté agent).
- Bins issus des pratiques d'engagement : `jwtforge`, `nrbfx`, `modbusx`.
- Domaines cloud/k8s en opt-in (packs `optional/` de ce dépôt déjà prêts).

## Règle transverse

Tout artefact lié à MCP passe le gate `mcp-intruder-cli scan-secrets` avant publication
(règle n°1, zéro secret en public) — voir [cli/README.md](cli/README.md#opsec-règle-n1-du-workspace).

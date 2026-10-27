# MCP-ROADMAP — feuille de route MCP du projet MCP-Intruder

**Statut (2026-10-26) :** le bundle `mcp-intruder-skills` fournit la couche skills/connaissances.
Le **serveur MCP** appartient au dépôt `mcp_intruder` (moteur Rust de scan, refactor en cours dans
une session dédiée). Cette page publie le plan, extraite de l'audit interne
`AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md` (§5-7) — document conservé en local.

## État actuel du serveur MCP (`mcp_intruder`)

- Crate `litemcp 0.7.1` en place (Streamable HTTP, JSON-RPC géré par la crate) ; point d'entrée `mcp::build_server()`.
- **1 seul outil exposé : `ping`.**
- Bloquant P0 : le binaire `src/bin/mcp_server.rs` est référencé mais pas encore écrit ; aucun
  module de scan branché sur `.tool(...)` ; zéro test MCP ; CHANGELOG sans entrée 0.6.0.

## P0 — rendre le serveur MCP fonctionnel

1. Écrire `src/bin/mcp_server.rs` (addr/port en CLI, `.bind(addr).serve()` sur `build_server()`).
2. Exposer les premiers outils MCP (chacun = 1 module existant, annotations `read_only`) :

| Outil MCP | Source | Fonction |
|---|---|---|
| `ping` | `mcp/mod.rs` | sonde de disponibilité (existant) |
| `dns_scan` | module `dns` | records A/AAAA/MX/NS/TXT/CAA/SOA/DS, SPF, DNSSEC |
| `dns_hardening_scan` | module `dns_hardening` | open resolver, récursion, AXFR |
| `port_scan` | module `ports` | 75 ports critiques + bannières |
| `full_portscan` | module `full_portscan` | 65535 ports async natif |
| `tls_audit` | module `tls` | chaîne X.509, ciphers, TLS 1.0-1.3 |
| `http_headers_scan` | module `http` | headers sécurité + cookies |
| `email_sec_scan` | module `email_sec` | SPF/DKIM/DMARC/MTA-STS/TLS-RPT/BIMI |
| `subdomain_enum` | module `subdomains` | subfinder + fallback statique |
| `crtsh_enum` | module `crt_sh` | Certificate Transparency |
| `get_report` | `report/` | scan_id → JSON du rapport complet |
| `history` | CLI `history` | historique des scans en DB |

3. Transport : Streamable HTTP (par défaut) + stdio pour usage local type Claude Code.
4. Tests MCP (initialize / tools/list / tools/call + erreurs).

## P1 — élargir la surface MCP

- Exposer les wrappers Kali : `waf_detect`, `nuclei_scan`, `nikto_scan`, `nmap_deep_scan`,
  `ffuf_scan`, `http_probe`, `tech_stack_scan`, `web_endpoints_scan`, `vuln_audit`,
  `diag_exposure_scan`, `sslscan_audit`, `brand_scan` (dnstwist), `whois_scan`, `dnsrecon_scan`,
  `theharvester_scan`, `smb_audit`.
- Exposer les bins d'impact : `chainx_run`, `cnametake_check`, `gkeyx_scan`, `spoofcheck`,
  `subalive`, `surfx`, `envx`, `gitdump`, `lootx` (tous read-only, gates horaires conservés).
- Mapping MITRE ATT&CK des catégories de findings + croisement CISA KEV.
- CI minimale (clippy `-D warnings`, tests, tests MCP).

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

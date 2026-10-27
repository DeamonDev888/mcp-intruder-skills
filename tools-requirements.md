# Exigences outils des skills adoptés — matrice & écart vs serveur Kali

**Machine de référence :** `votre-serveur-kali` (Kali Rolling, <ip-serveur>) — source `MEMO-SERVEUR-KALI.md`.
**À exécuter sur la cible :** `mcp-intruder-cli check-prereqs` (exit 0 = requis OK).

## 1. Outils REQUIS (Tier 1 non fonctionnel sans eux)

| Outil | Skills concernés | Sur Kali (2026-09-18) | Écart | Installation si absent |
|---|---|---|---|---|
| curl | hunt-* (test primaire), ACS web | ✅ | — | préinstallé |
| jq | evidence-hygiene, parsing JSON | ✅ (scripts serveur) | — | `apt install jq` |
| python3 ≥ 3.9 | helpers ACS embarqués dans les skills | ✅ | — | préinstallé |
| git | clonage jwt_tool, MAJ bundle | ✅ | — | préinstallé |
| dig / dnsutils | web2-recon, dns-enum-zone-transfer, MCP-Intruder dns | ✅ | — | préinstallé |
| whois | OSINT, MCP-Intruder whois_audit | ✅ | — | préinstallé |
| openssl | ssl-tls-assessment, MCP-Intruder tls/scheme_detect | ✅ | — | préinstallé |
| nmap | nmap-advanced, MCP-Intruder nmap_deep | ✅ | — | préinstallé |
| nuclei | ACS web, BugHunter recon, MCP-Intruder nuclei_deep | ✅ | — | préinstallé |
| ffuf | fuzzing routes, MCP-Intruder ffuf_audit (+ SecLists quickhits) | ✅ | — | préinstallé |
| subfinder | web2-recon, MCP-Intruder subdomains | ✅ | — | préinstallé |
| sqlmap | sqli (ACS), MCP-Intruder sqli_audit | ✅ | — | préinstallé |
| nikto | web-app-scanning-nikto, MCP-Intruder nikto_deep | ✅ | — | préinstallé |
| httpx (pd) | probe HTTP ACS/BugHunter, MCP-Intruder http_probe | ✅ pdhttpx | — | `apt install httpx` sinon go install PD |
| whatweb | tech stack, MCP-Intruder tech_stack | ✅ | — | préinstallé |
| wafw00f | waf-bypass, MCP-Intruder waf | ✅ | — | préinstallé |
| sslscan | ssl-tls-assessment, MCP-Intruder sslscan_audit | ✅ | — | préinstallé |
| dnstwist | brand/typosquatting, MCP-Intruder brand_sec | ✅ | — | préinstallé |
| theHarvester | OSINT, MCP-Intruder theharvester_audit | ✅ | — | préinstallé |
| dnsrecon | zone transfer, MCP-Intruder dnsrecon_audit | ✅ | — | préinstallé |
| SecLists (/usr/share/seclists) | ffuf quickhits, security-arsenal | ✅ | — | `apt install seclists` |

**Bilan requis : a priori 0 écart sur votre-serveur-kali** — le socle Kali déjà installé couvre
100 % des exigences REQUISES du Tier 1. Vérifier à l'exécution (les paquets peuvent avoir bougé).

## 2. Outils OPTIONNELS (pour exploiter tout le Tier 1 à 100 %)

| Outil | Débloque | État Kali | Écart | Installation |
|---|---|---|---|---|
| waybackurls / gau | hunt-source-leak, spa-api (URLs archivées) | ❌ | **écart 1** | `go install github.com/tomnomnom/waybackurls@latest` (nécessite `apt install golang`) |
| katana | crawl JS/API (spa-api, shadow-api) | ❌ | **écart 2** | `go install github.com/projectdiscovery/katana/cmd/katana@latest` |
| dnsx | résolution de masse (web2-recon) | ❌ | **écart 3** | `go install github.com/projectdiscovery/dnsx/cmd/dnsx@latest` |
| jwt_tool | hunt-jwt-crypto, JWT ACS (none/alg confusion) | ❌ (forge JWT faite à la main) | **écart 4** | `git clone https://github.com/ticarpi/jwt_tool ~/tools/jwt_tool` |
| Arjun | shadow-api, discovery de paramètres | ❌ | **écart 5** | `pipx install arjun` |
| graphql-cop / inql / graphw00f | graphql assessment (introspection/depth) | ❌ | **écart 6** | `pipx install graphql-cop` |
| Burp Suite (+ extension MCP) | xss-burp, OOB/blind, MCP BugHunter | ❌ (0 mention serveur) | **écart 7** | `apt install burpsuite` + extension Burp MCP (java) |
| mitmproxy | proxy d'interception alternatif | ❌ | **écart 8** | `apt install mitmproxy` |
| amass | énum sous-domaines profondeur | ❌ | **écart 9** | `apt install amass` |
| semgrep | devsecops-security-scanning | ❌ | écart 10 | `pipx install semgrep` |
| golang | prérequis des installs `go install` ci-dessus | ❌ | **blocage des écarts 1-3** | `apt install golang` |
| trivy / docker / semgrep… | packs optionnels (container/cloud/devsecops) | ❌ (Docker explicitement non installé) | écart 11 | seulement si packs activés |
| promptfoo / garak / pyrit | AI Security (LLM red teaming) | ❌ | écart 12 | `npm i -g promptfoo` · `pipx install garak` · `pip install pyrit` |

**Ordre d'installation recommandé (retour immédiat, ~15 min) :**

```bash
sudo apt install -y golang seclists amass mitmproxy burpsuite
export PATH=$PATH:$(go env GOPATH 2>/dev/null)/bin
go install github.com/tomnomnom/waybackurls@latest
go install github.com/projectdiscovery/katana/cmd/katana@latest
go install github.com/projectdiscovery/dnsx/cmd/dnsx@latest
pipx install arjun && pipx install graphql-cop
git clone https://github.com/ticarpi/jwt_tool ~/tools/jwt_tool && chmod +x ~/tools/jwt_tool/jwt_tool.py
```

Les skills dégradent proprement sans les optionnels (curl-first chez BugHunter : la chasse
fonctionne, la couverture discovery est simplement moindre). **Les 7 premiers écarts couvrent
les classes les plus chassées (source-leak, JWT, GraphQL, shadow-API, OOB Burp).**

## 3. Correspondance skills ↔ mcp_intruder (déjà couvert par le moteur)

Les skills ci-dessous recoupent des modules MCP-Intruder existants — à utiliser comme **méthodologie**
autour du moteur, pas comme doublon :

| Skill | Module MCP-Intruder |
|---|---|
| performing-security-headers-audit | `http` + findings HTTP |
| performing-ssl-tls-security-assessment | `tls` + `sslscan_audit` |
| performing-subdomain-enumeration-with-subfinder | `subdomains` + `crt_sh` |
| performing-dns-enumeration-and-zone-transfer | `dns` + `dnsrecon_audit` + `dns_hardening` |
| exploiting-sql-injection-with-sqlmap | `sqli_audit` (opt-in) |
| performing-web-application-scanning-with-nikto | `nikto_deep` |
| performing-web-application-firewall-bypass | `waf` (edge-guard) |
| auditing-mcp-servers-for-tool-poisoning | à appliquer **au futur serveur MCP MCP-Intruder** (P0 roadmap) |
| performing-cve-prioritization-with-kev-catalog | à brancher sur `audit_tech`/findings CVE (P1 roadmap) |

## 4. Ce que ces skills apportent et que MCP-Intruder n'a PAS (rappel du diff §4 de l'audit)

XSS, SSRF, SSTI, désérialisation, JWT/OAuth/SAML, GraphQL, smuggling, cache poisoning,
host header, race conditions, upload, business logic, IDOR/ATO/MFA — **139 skills Tier 1
comblent précisément ces ~20 familles absentes du moteur**, côté méthodologie et chasse
curl-first. L'industrialisation dans le moteur Rust reste la P3 de la roadmap.

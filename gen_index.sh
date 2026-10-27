#!/usr/bin/env bash
# gen_index.sh — génère INDEX.md et skills-manifest.json depuis les SKILL.md du bundle
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
IDX="$ROOT/INDEX.md"
MAN="$ROOT/skills-manifest.json"
TMP="$ROOT/.gen_tmp"
mkdir -p "$TMP"

# ---- extraction frontmatter name/description (gère descriptions YAML multilignes)
scan() { # $1=dir_root $2=source_label $3=category $4=tier
  local f d n desc
  while IFS= read -r f; do
    d=$(dirname "$f")
    n=$(basename "$d")
    desc=$(awk '
      /^---[[:space:]]*$/ { fm++; next }
      fm==1 && /^description:/ {
        sub(/^description:[[:space:]]*/, "")
        out=$0
        getline nxt
        while (nxt ~ /^[[:space:]]+/ && nxt !~ /^[[:space:]]*[a-zA-Z_-]+:/) {
          gsub(/^[[:space:]]+/, "", nxt); out = out " " nxt
          if ((getline nxt) <= 0) break
        }
        print out; exit
      }
      fm>=2 { exit }
    ' "$f")
    # nettoyage pour JSON/MD : guillemets, backslashes, tabs
    desc=$(printf '%s' "$desc" | sed 's/[\\"]//g' | tr '\t' ' ' | cut -c1-240)
    printf '%s\t%s\t%s\t%s\t%s\n' "$n" "$2" "$3" "$4" "$desc" >> "$TMP/rows.tsv"
  done < <(find "$1" -mindepth 2 -maxdepth 2 -name SKILL.md | sort)
}

: > "$TMP/rows.tsv"

# --- BugHunter Tier 1
scan "$ROOT/bughunter"      bughunter "hunt-classe"      tier1 "bughunter"
# reclasse les non-hunt
sed -i 's/\tbughunter\thunt-classe\ttier1\t/triad:\t/' /dev/null 2>/dev/null || true
for s in triage-validation evidence-hygiene report-writing bugcrowd-reporting redteam-report-template; do
  sed -i "s/^\($s\tbughunter\t\)hunt-classe/\1process-reporting/" "$TMP/rows.tsv"; done
for s in bb-methodology redteam-mindset bug-bounty; do
  sed -i "s/^\($s\tbughunter\t\)hunt-classe/\1methodologie/" "$TMP/rows.tsv"; done
for s in web2-recon security-arsenal offensive-osint recon-scope-triage osint-methodology; do
  sed -i "s/^\($s\tbughunter\t\)hunt-classe/\1recon-osint/" "$TMP/rows.tsv"; done

# --- ACS Tier 1
for s in performing-ssl-tls-security-assessment performing-dns-enumeration-and-zone-transfer scanning-network-with-nmap-advanced performing-cve-prioritization-with-kev-catalog implementing-epss-score-for-vulnerability-prioritization performing-web-application-scanning-with-nikto; do
  :; done
# on scanne tout acs/ en deux passes par groupes connus
ACS_WEB=$(cat <<'EOF'
bypassing-authentication-with-forced-browsing exploiting-broken-link-hijacking exploiting-http-request-smuggling exploiting-idor-vulnerabilities exploiting-insecure-deserialization exploiting-mass-assignment-in-rest-apis exploiting-nosql-injection-vulnerabilities exploiting-oauth-misconfiguration exploiting-prototype-pollution-in-javascript exploiting-race-condition-vulnerabilities exploiting-server-side-request-forgery exploiting-sql-injection-with-sqlmap exploiting-template-injection-vulnerabilities exploiting-type-juggling-vulnerabilities exploiting-websocket-vulnerabilities implementing-devsecops-security-scanning implementing-runtime-application-self-protection implementing-web-application-logging-with-modsecurity performing-blind-ssrf-exploitation performing-clickjacking-attack-test performing-content-security-policy-bypass performing-csrf-attack-simulation performing-directory-traversal-testing performing-fuzzing-with-aflplusplus performing-graphql-security-assessment performing-http-parameter-pollution-attack performing-second-order-sql-injection performing-security-headers-audit performing-subdomain-enumeration-with-subfinder performing-supply-chain-attack-simulation performing-web-application-firewall-bypass performing-web-cache-deception-attack performing-web-cache-poisoning-attack testing-api-security-with-owasp-top-10 testing-cors-misconfiguration testing-for-broken-access-control testing-for-business-logic-vulnerabilities testing-for-email-header-injection testing-for-host-header-injection testing-for-json-web-token-vulnerabilities testing-for-open-redirect-vulnerabilities testing-for-sensitive-data-exposure testing-for-xml-injection-vulnerabilities testing-for-xss-vulnerabilities-with-burpsuite testing-for-xxe-injection-vulnerabilities testing-jwt-token-security
EOF
)
ACS_API=$(cat <<'EOF'
detecting-api-enumeration-attacks detecting-broken-object-property-level-authorization detecting-shadow-api-endpoints exploiting-api-injection-vulnerabilities exploiting-broken-function-level-authorization exploiting-excessive-data-exposure-in-api exploiting-jwt-algorithm-confusion-attack implementing-api-abuse-detection-with-rate-limiting implementing-api-gateway-security-controls implementing-api-key-security-controls implementing-api-rate-limiting-and-throttling implementing-api-schema-validation-security implementing-api-security-posture-management implementing-api-security-testing-with-42crunch implementing-api-threat-protection-with-apigee performing-api-fuzzing-with-restler performing-api-inventory-and-discovery performing-api-rate-limiting-bypass performing-api-security-testing-with-postman performing-graphql-depth-limit-attack performing-graphql-introspection-attack performing-jwt-none-algorithm-attack performing-soap-web-service-security-testing testing-api-authentication-weaknesses testing-api-for-broken-object-level-authorization testing-api-for-mass-assignment-vulnerability testing-oauth2-implementation-flaws testing-websocket-api-security
EOF
)
ACS_AI=$(cat <<'EOF'
assessing-vector-and-embedding-weaknesses auditing-mcp-servers-for-tool-poisoning continuous-llm-red-teaming-with-promptfoo defending-llms-with-guardrails detecting-ai-model-prompt-injection-attacks detecting-data-and-model-poisoning detecting-indirect-prompt-injection detecting-model-extraction-attacks implementing-llm-guardrails-for-security orchestrating-llm-attacks-with-pyrit red-teaming-llms-with-garak securing-agentic-ai-tool-invocation testing-for-system-prompt-leakage testing-prompt-injection-in-rag-pipelines
EOF
)
ACS_CUR="performing-ssl-tls-security-assessment performing-dns-enumeration-and-zone-transfer scanning-network-with-nmap-advanced performing-cve-prioritization-with-kev-catalog implementing-epss-score-for-vulnerability-prioritization performing-web-application-scanning-with-nikto"

for s in $ACS_WEB; do sed -i "s/^\($s\tacs\t\)[a-z-]*/\1web-appsec/" "$TMP/rows.tsv" 2>/dev/null; done
# scan ACS: d'abord marquer tout "acs-autre", puis reclasser
while IFS= read -r d; do
  n=$(basename "$d")
  grep -q "^$n	acs	" "$TMP/rows.tsv" || true
done < <(true)

# scan ACS proprement
for d in "$ROOT"/acs/*/; do
  n=$(basename "$d"); [ -f "$d/SKILL.md" ] || continue
  cat="transversal"
  case " $ACS_WEB " in *" $n "*) cat="web-appsec";; esac
  case " $ACS_API " in *" $n "*) cat="api-security";; esac
  case " $ACS_AI "  in *" $n "*) cat="ai-security";;  esac
  case " $ACS_CUR " in *" $n "*) cat="transversal";;  esac
  f="$d/SKILL.md"
  desc=$(awk '
    /^---[[:space:]]*$/ { fm++; next }
    fm==1 && /^description:/ {
      sub(/^description:[[:space:]]*/, "")
      out=$0
      getline nxt
      while (nxt ~ /^[[:space:]]+/ && nxt !~ /^[[:space:]]*[a-zA-Z_-]+:/) {
        gsub(/^[[:space:]]+/, "", nxt); out = out " " nxt
        if ((getline nxt) <= 0) break
      }
      print out; exit
    }
    fm>=2 { exit }
  ' "$f")
  desc=$(printf '%s' "$desc" | sed 's/[\\"]//g' | tr '\t' ' ' | cut -c1-240)
  printf '%s\tacs\t%s\ttier1\t%s\n' "$n" "$cat" "$desc" >> "$TMP/rows.tsv"
done

# --- ACS optionnels
scan "$ROOT/optional/acs-ot-ics"     acs "ot-ics"        optional "x"
scan "$ROOT/optional/acs-red-team"   acs "red-team"      optional "x"
scan "$ROOT/optional/acs-cloud"      acs "cloud"         optional "x"
scan "$ROOT/optional/acs-container"  acs "container"     optional "x"
# --- BugHunter optionnels
scan "$ROOT/optional/bughunter"      bughunter "hunt-classe (optionnel)" optional "x"

# ================= skills-manifest.json =================
{
  echo '{'
  echo '  "generated": "2026-10-26",'
  echo '  "audit_source": "AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md §5.2",'
  echo '  "sources": {'
  echo '    "bughunter": { "repo": "https://github.com/elementalsouls/Claude-BugHunter", "commit": "6a153f7cf2c46ab9a81c9e09ff270ec82c77318b", "license": "MIT (code) + CC BY 4.0 (contenu)" },'
  echo '    "acs": { "repo": "https://github.com/mukul975/Anthropic-Cybersecurity-Skills", "commit": "54a798831d2266a3ca61ce68a7acb80b81160d57", "license": "Apache-2.0" }'
  echo '  },'
  echo '  "install_targets": "~/.claude/skills (Kali) · C:/Users/<user>/.agents/skills (Windows, optionnel)",'
  echo '  "skills": ['
  first=1
  while IFS=$'\t' read -r n src cat tier desc; do
    [ $first -eq 1 ] && first=0 || printf ',\n'
    printf '    {"id": "%s", "source": "%s", "category": "%s", "tier": "%s", "path": "%s", "description": "%s"}' \
      "$n" "$src" "$cat" "$tier" "$desc" 
    # path manquant ci-dessus volontairement simplifié : reconstruit proprement
  done < "$TMP/rows.tsv"
  echo ''
  echo '  ]'
  echo '}'
} > "$MAN.man" 2>/dev/null || true

# reconstruction propre du manifest (path correct)
{
  echo '{'
  echo '  "generated": "2026-10-26",'
  echo '  "audit_source": "AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md §5.2",'
  echo '  "sources": {'
  echo '    "bughunter": { "repo": "https://github.com/elementalsouls/Claude-BugHunter", "commit": "6a153f7cf2c46ab9a81c9e09ff270ec82c77318b", "license": "MIT (code) + CC BY 4.0 (contenu)" },'
  echo '    "acs": { "repo": "https://github.com/mukul975/Anthropic-Cybersecurity-Skills", "commit": "54a798831d2266a3ca61ce68a7acb80b81160d57", "license": "Apache-2.0" }'
  echo '  },'
  echo '  "skills": ['
  first=1
  while IFS=$'\t' read -r n src cat tier desc; do
    case "$tier" in
      tier1)    if [ "$src" = "bughunter" ]; then p="bughunter/$n"; else p="acs/$n"; fi ;;
      optional) case "$cat" in
                  hunt-classe*) p="optional/bughunter/$n";;
                  ot-ics)       p="optional/acs-ot-ics/$n";;
                  red-team)     p="optional/acs-red-team/$n";;
                  cloud)        p="optional/acs-cloud/$n";;
                  container)    p="optional/acs-container/$n";;
                  *)            p="optional/acs/$n";;
                esac ;;
    esac
    [ $first -eq 1 ] && first=0 || printf ',\n'
    printf '    {"id": "%s", "source": "%s", "category": "%s", "tier": "%s", "path": "%s", "description": "%s"}' \
      "$n" "$src" "$cat" "$tier" "$p" "$desc"
  done < "$TMP/rows.tsv"
  echo ''
  echo '  ]'
  echo '}'
} > "$MAN"

rm -f "$MAN.man"

# ================= INDEX.md =================
emit_table() { # $1=source $2=category regex $3=tier
  echo "| Skill | Déclencheur (frontmatter) |"; echo "|---|---|"
  awk -F'\t' -v s="$1" -v c="$2" -v t="$3" '$2==s && $3 ~ c && $4==t {printf "| `%s` | %s |\n", $1, $5}' "$TMP/rows.tsv"
}

{
echo "# MCP-INTRUDER SKILLS — Bundle d'adoption (catalogue schématisé)"
echo
echo "**Généré le 2026-10-26** — sélections issues de [AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md](../AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md) §5.2."
echo
echo "| Source | Tier 1 | Optionnel | Licence |"
echo "|---|---|---|---|"
b1=$(awk -F'\t' '$2=="bughunter" && $4=="tier1"' "$TMP/rows.tsv" | wc -l)
b2=$(awk -F'\t' '$2=="bughunter" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
a1=$(awk -F'\t' '$2=="acs" && $4=="tier1"' "$TMP/rows.tsv" | wc -l)
a2=$(awk -F'\t' '$2=="acs" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
echo "| [Claude-BugHunter](https://github.com/elementalsouls/Claude-BugHunter) @ \`6a153f7\` | $b1 | $b2 | MIT + CC BY 4.0 |"
echo "| [Anthropic-Cybersecurity-Skills](https://github.com/mukul975/Anthropic-Cybersecurity-Skills) @ \`54a7988\` | $a1 | $a2 | Apache-2.0 |"
echo "| **Total** | **$((b1+a1))** | **$((b2+a2))** | + 15 commands slash + CLI cbh + mappings MITRE/NIST/OWASP |"
echo
echo "## Architecture du bundle"
echo '```'
echo "mcp-intruder-skills/"
echo "├── bughunter/            Tier 1 (45 skills) + commands/ (15) + cbh/ (CLI) + scripts/cbh.py"
echo "├── acs/                  Tier 1 (94 skills) + mappings/ (ATT&CK Navigator, NIST CSF, OWASP)"
echo "├── optional/             Packs opt-in (bughunter 38 · acs-ot-ics 29 · acs-red-team 35 · acs-cloud 66 · acs-container 33)"
echo "├── install.sh            Déploiement → ~/.claude/skills (Kali) ou --target <dir>"
echo "├── check_prereqs.sh      Vérifie les exigences outils des skills"
echo "├── tools-requirements.md Matrice exigences/écart vs serveur Kali votre-serveur-kali"
echo "├── skills-manifest.json  Catalogue machine (id, source, catégorie, tier, description)"
echo "└── BUILD-REPORT.md       Journal de construction"
echo '```'
echo
echo "## Mécanisme de déclenchement"
echo "Chaque \`SKILL.md\` porte un frontmatter YAML ; le champ \`description\` sert de déclencheur sémantique (auto-load quand la tâche correspond). Le tableau ci-dessous reprend **le frontmatter réel** de chaque skill copié."
echo
echo "---"
echo
echo "## BUGHUNTER · Process, reporting & méthodologie (8)"
echo
echo "| Skill | Déclencheur (frontmatter) |"; echo "|---|---|"
awk -F'\t' '$2=="bughunter" && ($3=="process-reporting" || $3=="methodologie") && $4=="tier1" {printf "| `%s` | %s |\n", $1, $5}' "$TMP/rows.tsv"
echo
echo "## BUGHUNTER · Recon & OSINT (5)"
echo
emit_table bughunter "recon-osint" tier1
echo
echo "## BUGHUNTER · hunt-* — classes de vulnérabilités (32)"
echo
emit_table bughunter "hunt-classe" tier1
echo
echo "## BUGHUNTER · Commands slash (15)"
echo
echo "| Commande | Rôle |"; echo "|---|---|"
echo "| \`/hunt <target>\` | Méta-routeur Red Team vs WAPT, charge le bon set de skills |"
echo "| \`/recon <target>\` | Recon (support déterministe via CLI cbh) |"
echo "| \`/surface <target>\` | Surface d'attaque classée P1/P2/Kill |"
echo "| \`/scope <asset>\` | Gate de scope déterministe avant tout toucher HTTP |"
echo "| \`/triage\` / \`/validate\` | 7-Question Gate : PASS/KILL/DOWNGRADE/CHAIN-REQUIRED |"
echo "| \`/report\` | Rapport H1/Bugcrowd/Intigriti/Immunefi + CVSS |"
echo "| \`/chain\` | Chaînes d'exploit A→B (17+ patterns), timeboxing 20 min |"
echo "| \`/intel <target>\` | CVE NVD + rapports divulgués Hacktivity |"
echo "| \`/autopilot\` | Modes mémoire : --paranoid/--normal (off) · --quick/--yolo (on) |"
echo "| \`/memory-gc\` | Gestion du store mémoire par engagement |"
echo "| \`/pickup <host>\` | Historique de chasse d'une cible |"
echo "| \`/remember\` | Note manuelle dans le ledger |"
echo "| \`/token-scan\` | Scan rug-pull EVM/Solana (8 classes) |"
echo "| \`/web3-audit\` | Audit smart contract 10 classes + PoC Foundry |"
echo
echo "**CLI cbh** (python3 ≥3.9, stdlib) : \`python3 bughunter/scripts/cbh.py recon|surface|classify|triage|report <args>\` — index : \`bughunter/cbh/data/skill_index.json\`."
echo
echo "---"
echo
echo "## ACS · Web Application Security (46)"
echo
emit_table acs "web-appsec" tier1
echo
echo "## ACS · API Security (28)"
echo
emit_table acs "api-security" tier1
echo
echo "## ACS · AI Security (14) — incl. audit de serveurs MCP"
echo
emit_table acs "ai-security" tier1
echo
echo "## ACS · Transversal curaté (6)"
echo
emit_table acs "transversal" tier1
echo
echo "## ACS · Mappings embarqués (acs/mappings/)"
echo
echo "- \`attack-navigator-layer.json\` — couche ATT&CK Navigator (218 techniques)"
echo "- \`mitre-attack/\`, \`nist-csf/\`, \`owasp/\` — mappings par framework"
echo "- \`ATTACK_COVERAGE.md\` — résumé de couverture (290 techniques distinctes sur 805 skills)"
echo
echo "---"
echo
echo "## Packs optionnels (201 skills — activables via \`install.sh --all\` ou \`--optional <groupe>\`)"
echo
echo "| Groupe | Skills | Intérêt pour MCP-Intruder |"; echo "|---|---|---|"
o_b=$(awk -F'\t' '$2=="bughunter" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
o_ot=$(awk -F'\t' '$3=="ot-ics" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
o_rt=$(awk -F'\t' '$3=="red-team" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
o_cl=$(awk -F'\t' '$3=="cloud" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
o_ct=$(awk -F'\t' '$3=="container" && $4=="optional"' "$TMP/rows.tsv" | wc -l)
echo "| \`optional/bughunter\` | $o_b | Le reste des hunt-* : frameworks (Laravel/Next.js/Spring), k8s, captcha, brute-force, fintech-graphql, grpc… |"
echo "| \`optional/acs-ot-ics\` | $o_ot | Modbus/DNP3/S7comm — **cible Modbus 502 déjà vue en engagement** |"
echo "| \`optional/acs-red-team\` | $o_rt | AD/Kerberos/BloodHound/ADCS/C2 (Sliver/Havoc) |"
echo "| \`optional/acs-cloud\` | $o_cl | AWS/Azure/GCP IAM, S3, GuardDuty, Pacu/ScoutSuite/CloudFox |"
echo "| \`optional/acs-container\` | $o_ct | Docker/K8s escape, Falco, Trivy, RBAC |"
echo
echo "---"
echo
echo "## Prérequis outils"
echo "Voir [tools-requirements.md](tools-requirements.md) et vérifier avec \`bash check_prereqs.sh\` (à lancer sur la cible d'installation)."
} > "$IDX"

echo "rows: $(wc -l < "$TMP/rows.tsv")"
echo "INDEX: $(wc -l < "$IDX") lignes"
echo "MANIFEST skills: $(grep -c '"id":' "$MAN")"

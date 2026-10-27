#!/usr/bin/env bash
# check_prereqs.sh — vérifie que les exigences outils des skills adoptés sont satisfaites.
# À lancer sur la machine cible (Kali votre-serveur-kali en priorité).
# Codes retour : 0 = requis OK ; 1 = au moins un outil REQUIS manquant.
set -uo pipefail

MISSING_REQ=0
declare -a REQTools=(
  "curl:REQUIS:hunt-* / ACS web : client HTTP de test primaire:apt install curl (préinstallé Kali)"
  "jq:REQUIS:evidence-hygiene (sanitization HAR), parsing JSON:apt install jq"
  "python3:REQUIS:CLI cbh (BugHunter), scripts helpers ACS:apt install python3 (>=3.9)"
  "git:REQUIS:clonage jwt_tool / mise à jour bundle:apt install git"
  "dig:REQUIS:recon DNS (web2-recon, dns-enum):apt install dnsutils"
  "whois:REQUIS:OSINT domaines:apt install whois"
  "openssl:REQUIS:TLS (ssl-tls-assessment, scheme detect):préinstallé"
  "nmap:REQUIS:nmap-advanced, scans réseau, MCP-Intruder nmap_deep:apt install nmap"
  "nuclei:REQUIS:ACS web + BugHunter (templates CVE):apt install nuclei"
  "ffuf:REQUIS:fuzzing routes (quickhits SecLists):apt install ffuf"
  "subfinder:REQUIS:énumération sous-domaines (web2-recon, cbh recon):apt install subfinder"
  "sqlmap:REQUIS:SQLi (2nd-order, exploiting-sql-injection):apt install sqlmap"
  "nikto:REQUIS:web-app-scanning-nikto, MCP-Intruder nikto_deep:apt install nikto"
  "httpx|pdhttpx:REQUIS:probe HTTP (ACS/BugHunter/MCP-Intruder):apt install httpx (Kali: pdhttpx sinon go install github.com/projectdiscovery/httpx/cmd/httpx@latest)"
  "whatweb:REQUIS:fingerprint tech stack:apt install whatweb"
  "wafw00f:REQUIS:détection WAF (waf-bypass):apt install wafw00f"
  "sslscan:REQUIS:ssl-tls-security-assessment:apt install sslscan"
  "dnstwist:REQUIS:typosquatting (brand):apt install dnstwist"
  "theharvester:REQUIS:OSINT emails/hôtes:apt install theharvester"
  "dnsrecon:REQUIS:zone transfer / SRV:apt install dnsrecon"
)

declare -a OPTTools=(
  "waybackurls:OPTIONNEL:source-leak, SPA API discovery:go install github.com/tomnomnom/waybackurls@latest (ou apt install golang)"
  "gau:OPTIONNEL:source-leak (URLs archivées):go install github.com/lc/gau/v2/cmd/gau@latest"
  "katana:OPTIONNEL:crawl JS/API discovery (spa-api):go install github.com/projectdiscovery/katana/cmd/katana@latest"
  "dnsx:OPTIONNEL:résolution masse (web2-recon):go install github.com/projectdiscovery/dnsx/cmd/dnsx@latest"
  "jwt_tool:OPTIONNEL:skills JWT (none/alg confusion):git clone https://github.com/ticarpi/jwt_tool ~/tools/jwt_tool"
  "arjun:OPTIONNEL:shadow-api / discovery de paramètres:pipx install arjun"
  "graphql-cop:OPTIONNEL:graphql security assessment:pipx install graphql-cop"
  "burpsuite|burpsuite_pro:OPTIONNEL:xss-burp, tests interactifs, MCP Burp (OOB):apt install burpsuite (community) + extension MCP"
  "mitmproxy:OPTIONNEL:proxy alternatif:apt install mitmproxy"
  "amass:OPTIONNEL:énum profondeur (complément subfinder):apt install amass"
  "semgrep:OPTIONNEL:devsecops-security-scanning:pipx install semgrep"
  "trivy:OPTIONNEL:container/SCA (packs optionnels):apt install trivy"
  "docker:OPTIONNEL:sandboxes, eval labs, packs container/cloud:apt install docker.io"
  "promptfoo:OPTIONNEL:LLM red teaming CI (AI security):npm install -g promptfoo"
  "garak:OPTIONNEL:LLM probes (red-teaming-llms):pipx install garak"
  "pyrit:OPTIONNEL:orchestration attaques LLM:pip install pyrit"
  "gf:OPTIONNEL:security-arsenal (patterns grep):go install github.com/tomnomnom/gf@latest"
)

status_of() { # $1 = "a|b" alternatives séparées par |
  local alt IFS='|'
  for alt in $1; do
    if command -v "$alt" >/dev/null 2>&1; then return 0; fi
  done
  return 1
}

sec=$(find /usr/share/seclists /usr/share wordlists -maxdepth 0 -type d 2>/dev/null | head -1)
py_ok=0
command -v python3 >/dev/null 2>&1 && python3 -c 'import sys; sys.exit(0 if sys.version_info>=(3,9) else 1)' && py_ok=1

echo "=== EXIGENCES REQUISES ==="
printf "%-18s %-6s %s\n" "OUTIL" "ÉTAT" "NOTE"
for row in "${REQTools[@]}"; do
  IFS=':' read -r name role why install <<< "$row"
  if [ "$name" = "python3" ]; then
    if [ $py_ok -eq 1 ]; then st="OK"; else st="MANQUANT"; MISSING_REQ=1; fi
  elif status_of "$name"; then st="OK"; else st="MANQUANT"; MISSING_REQ=1; fi
  printf "%-18s %-6s %s\n" "$name" "$st" "$why"
  [ "$st" = "MANQUANT" ] && printf "    → %s\n" "$install"
done

echo
echo "=== EXIGENCES OPTIONNELLES (packs avancés) ==="
printf "%-18s %-6s %s\n" "OUTIL" "ÉTAT" "NOTE"
for row in "${OPTTools[@]}"; do
  IFS=':' read -r name role why install <<< "$row"
  if status_of "$name"; then st="OK"; else st="absent"; fi
  printf "%-18s %-6s %s\n" "$name" "$st" "$why"
  [ "$st" = "absent" ] && printf "    → %s\n" "$install"
done

echo
echo "=== DIVERS ==="
if [ -n "${sec:-}" ]; then echo "SecLists : OK ($sec)"; else echo "SecLists : absent → apt install seclists (requis par ffuf_audit / quickhits)"; fi

echo
if [ $MISSING_REQ -eq 0 ]; then
  echo "RÉSULTAT : toutes les exigences REQUISES sont satisfaites ✔"
else
  echo "RÉSULTAT : des outils REQUIS manquent — installer les lignes MANQUANT ci-dessus."
fi
exit $MISSING_REQ

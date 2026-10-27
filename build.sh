#!/usr/bin/env bash
# build.sh — construit le bundle mcp-intruder-skills depuis les tarballs extraits dans .upstream/
# Source de vérité des sélections : AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md §5.2
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
UP="$ROOT/.upstream"
BH="$UP/Claude-BugHunter-main"
ACS="$UP/Anthropic-Cybersecurity-Skills-main"
RPT="$ROOT/BUILD-REPORT.md"
: > "$RPT"

copy_skill() { # $1=repo_skills_dir $2=dest_parent $3=name
  if [ -f "$1/$3/SKILL.md" ]; then
    cp -r "$1/$3" "$2/$3"
    echo "OK      $3" >> "$RPT"
  else
    echo "MISSING $3" >> "$RPT"
  fi
}

mkdir -p "$ROOT/bughunter" "$ROOT/acs" \
         "$ROOT/optional/bughunter" "$ROOT/optional/acs-ot-ics" \
         "$ROOT/optional/acs-red-team" "$ROOT/optional/acs-cloud" \
         "$ROOT/optional/acs-container"

# ================= BUGHUNTER — Tier 1 (45) =================
BH_T1=(
  # hunt-* (32) — classes de vulnérabilités prioritaires
  hunt-xss hunt-ssrf hunt-sqli hunt-idor hunt-auth-bypass hunt-jwt-crypto
  hunt-oauth hunt-saml hunt-rce hunt-graphql hunt-http-smuggling hunt-cache-poison
  hunt-race-condition hunt-file-upload hunt-lfi hunt-xxe hunt-ssti hunt-deserialization
  hunt-host-header hunt-open-redirect hunt-business-logic hunt-ato hunt-mfa-bypass
  hunt-session hunt-cors hunt-websocket hunt-source-leak hunt-shadow-api hunt-spa-api
  hunt-subdomain hunt-tls-network hunt-llm-ai
  # process / reporting (5)
  triage-validation evidence-hygiene report-writing bugcrowd-reporting redteam-report-template
  # méthodologie (3)
  bb-methodology redteam-mindset bug-bounty
  # recon & OSINT (5)
  web2-recon security-arsenal offensive-osint recon-scope-triage osint-methodology
)
for s in "${BH_T1[@]}"; do copy_skill "$BH/skills" "$ROOT/bughunter" "$s"; done

# BugHunter — le reste → optional/bughunter
for d in "$BH"/skills/*/; do
  n=$(basename "$d"); skip=0
  for s in "${BH_T1[@]}"; do [ "$n" = "$s" ] && skip=1 && break; done
  [ $skip -eq 0 ] && copy_skill "$BH/skills" "$ROOT/optional/bughunter" "$n"
done

# BugHunter — commands, CLI cbh, docs, licences
cp -r "$BH/commands" "$ROOT/bughunter/commands"
mkdir -p "$ROOT/bughunter/scripts" "$ROOT/bughunter/docs"
cp "$BH/scripts/cbh.py" "$ROOT/bughunter/scripts/cbh.py"
cp -r "$BH/cbh" "$ROOT/bughunter/cbh"
cp "$BH/docs/skills.md" "$ROOT/bughunter/docs/skills.md" 2>/dev/null
cp "$BH/LICENSE" "$BH/LICENSE-CONTENT" "$BH/NOTICE" "$ROOT/bughunter/" 2>/dev/null

# ================= ACS — Tier 1 (94) =================
# Web AppSec (46)
ACS_WEB=(
  bypassing-authentication-with-forced-browsing exploiting-broken-link-hijacking
  exploiting-http-request-smuggling exploiting-idor-vulnerabilities
  exploiting-insecure-deserialization exploiting-mass-assignment-in-rest-apis
  exploiting-nosql-injection-vulnerabilities exploiting-oauth-misconfiguration
  exploiting-prototype-pollution-in-javascript exploiting-race-condition-vulnerabilities
  exploiting-server-side-request-forgery exploiting-sql-injection-with-sqlmap
  exploiting-template-injection-vulnerabilities exploiting-type-juggling-vulnerabilities
  exploiting-websocket-vulnerabilities implementing-devsecops-security-scanning
  implementing-runtime-application-self-protection
  implementing-web-application-logging-with-modsecurity
  performing-blind-ssrf-exploitation performing-clickjacking-attack-test
  performing-content-security-policy-bypass performing-csrf-attack-simulation
  performing-directory-traversal-testing performing-fuzzing-with-aflplusplus
  performing-graphql-security-assessment performing-http-parameter-pollution-attack
  performing-second-order-sql-injection performing-security-headers-audit
  performing-subdomain-enumeration-with-subfinder performing-supply-chain-attack-simulation
  performing-web-application-firewall-bypass performing-web-cache-deception-attack
  performing-web-cache-poisoning-attack testing-api-security-with-owasp-top-10
  testing-cors-misconfiguration testing-for-broken-access-control
  testing-for-business-logic-vulnerabilities testing-for-email-header-injection
  testing-for-host-header-injection testing-for-json-web-token-vulnerabilities
  testing-for-open-redirect-vulnerabilities testing-for-sensitive-data-exposure
  testing-for-xml-injection-vulnerabilities testing-for-xss-vulnerabilities-with-burpsuite
  testing-for-xxe-injection-vulnerabilities testing-jwt-token-security
)
# API Security (28)
ACS_API=(
  detecting-api-enumeration-attacks detecting-broken-object-property-level-authorization
  detecting-shadow-api-endpoints exploiting-api-injection-vulnerabilities
  exploiting-broken-function-level-authorization exploiting-excessive-data-exposure-in-api
  exploiting-jwt-algorithm-confusion-attack implementing-api-abuse-detection-with-rate-limiting
  implementing-api-gateway-security-controls implementing-api-key-security-controls
  implementing-api-rate-limiting-and-throttling implementing-api-schema-validation-security
  implementing-api-security-posture-management implementing-api-security-testing-with-42crunch
  implementing-api-threat-protection-with-apigee performing-api-fuzzing-with-restler
  performing-api-inventory-and-discovery performing-api-rate-limiting-bypass
  performing-api-security-testing-with-postman performing-graphql-depth-limit-attack
  performing-graphql-introspection-attack performing-jwt-none-algorithm-attack
  performing-soap-web-service-security-testing testing-api-authentication-weaknesses
  testing-api-for-broken-object-level-authorization testing-api-for-mass-assignment-vulnerability
  testing-oauth2-implementation-flaws testing-websocket-api-security
)
# AI Security (14)
ACS_AI=(
  assessing-vector-and-embedding-weaknesses auditing-mcp-servers-for-tool-poisoning
  continuous-llm-red-teaming-with-promptfoo defending-llms-with-guardrails
  detecting-ai-model-prompt-injection-attacks detecting-data-and-model-poisoning
  detecting-indirect-prompt-injection detecting-model-extraction-attacks
  implementing-llm-guardrails-for-security orchestrating-llm-attacks-with-pyrit
  red-teaming-llms-with-garak securing-agentic-ai-tool-invocation
  testing-for-system-prompt-leakage testing-prompt-injection-in-rag-pipelines
)
# Curated transversal (6)
ACS_CUR=(
  performing-ssl-tls-security-assessment performing-dns-enumeration-and-zone-transfer
  scanning-network-with-nmap-advanced performing-cve-prioritization-with-kev-catalog
  implementing-epss-score-for-vulnerability-prioritization
  performing-web-application-scanning-with-nikto
)
for s in "${ACS_WEB[@]}" "${ACS_API[@]}" "${ACS_AI[@]}" "${ACS_CUR[@]}"; do
  copy_skill "$ACS/skills" "$ROOT/acs" "$s"
done

# ================= ACS — Packs optionnels =================
ACS_OT=(
  detecting-anomalies-in-industrial-control-systems detecting-attacks-on-historian-servers
  detecting-attacks-on-scada-systems detecting-dnp3-protocol-anomalies
  detecting-modbus-command-injection-attacks detecting-modbus-protocol-anomalies
  detecting-stuxnet-style-attacks implementing-conduit-security-for-ot-remote-access
  implementing-dragos-platform-for-ot-monitoring implementing-ics-firewall-with-tofino
  implementing-iec-62443-security-zones implementing-nerc-cip-compliance-controls
  implementing-network-segmentation-for-ot implementing-ot-incident-response-playbook
  implementing-ot-network-traffic-analysis-with-nozomi implementing-patch-management-for-ot-systems
  implementing-purdue-model-network-segmentation monitoring-scada-modbus-traffic-anomalies
  performing-ics-asset-discovery-with-claroty performing-oil-gas-cybersecurity-assessment
  performing-ot-network-security-assessment performing-ot-vulnerability-assessment-with-claroty
  performing-ot-vulnerability-scanning-safely performing-plc-firmware-security-analysis
  performing-power-grid-cybersecurity-assessment performing-s7comm-protocol-security-analysis
  performing-scada-hmi-security-assessment securing-historian-server-in-ot-environment
  securing-remote-access-to-ot-environment
)
ACS_RT=(
  abusing-dpapi-for-credential-access abusing-shadow-credentials-for-privesc
  building-c2-infrastructure-with-sliver-framework building-c2-redirector-infrastructure
  building-red-team-c2-infrastructure-with-havoc coercing-authentication-with-coercer-petitpotam
  conducting-domain-persistence-with-dcsync conducting-full-scope-red-team-engagement
  conducting-internal-reconnaissance-with-bloodhound-ce conducting-pass-the-ticket-attack
  conducting-social-engineering-pretext-call conducting-spearphishing-simulation-campaign
  executing-red-team-engagement-planning exploiting-active-directory-certificate-services-esc1
  exploiting-active-directory-with-bloodhound exploiting-adcs-with-certipy
  exploiting-constrained-delegation-abuse exploiting-kerberoasting-with-impacket
  exploiting-ms17-010-eternalblue-vulnerability exploiting-nopac-cve-2021-42278-42287
  exploiting-zerologon-vulnerability-cve-2020-1472 mapping-attack-paths-with-bloodhound-ce
  operating-havoc-c2 operating-sliver-c2 performing-active-directory-bloodhound-analysis
  performing-active-directory-forest-trust-attack performing-credential-access-with-lazagne
  performing-initial-access-with-evilginx3 performing-kerberoasting-attack
  performing-lateral-movement-with-wmiexec performing-open-source-intelligence-gathering
  performing-physical-intrusion-assessment performing-privilege-escalation-on-linux
  performing-red-team-with-covenant relaying-ntlm-for-adcs-esc8
)
ACS_CLD=(
  analyzing-cloud-storage-access-patterns analyzing-office365-audit-logs-for-compromise
  auditing-aws-s3-bucket-permissions auditing-azure-active-directory-configuration
  auditing-cloud-with-cis-benchmarks auditing-gcp-iam-permissions
  auditing-kubernetes-cluster-rbac auditing-terraform-infrastructure-for-security
  building-cloud-siem-with-sentinel conducting-cloud-penetration-testing
  detecting-aws-cloudtrail-anomalies detecting-aws-credential-exposure-with-trufflehog
  detecting-aws-guardduty-findings-automation detecting-aws-iam-privilege-escalation
  detecting-azure-lateral-movement detecting-azure-service-principal-abuse
  detecting-azure-storage-account-misconfigurations detecting-cloud-threats-with-guardduty
  detecting-compromised-cloud-credentials detecting-cryptomining-in-cloud
  detecting-misconfigured-azure-storage detecting-oauth-token-theft
  detecting-s3-data-exfiltration-attempts detecting-serverless-function-injection
  detecting-shadow-it-cloud-usage detecting-suspicious-oauth-application-consent
  emulating-cloud-attacks-with-stratus-red-team enumerating-cloud-with-cloudfox
  exploiting-aws-with-pacu implementing-aws-config-rules-for-compliance
  implementing-aws-macie-for-data-classification implementing-aws-nitro-enclave-security
  implementing-aws-security-hub implementing-aws-security-hub-compliance
  implementing-azure-defender-for-cloud implementing-cloud-dlp-for-data-protection
  implementing-cloud-security-posture-management implementing-cloud-trail-log-analysis
  implementing-cloud-waf-rules implementing-cloud-workload-protection
  implementing-gcp-binary-authorization implementing-gcp-organization-policy-constraints
  implementing-gcp-vpc-firewall-rules implementing-secrets-management-with-vault
  implementing-zero-trust-in-cloud implementing-zero-trust-network-access
  managing-cloud-identity-with-okta performing-aws-account-enumeration-with-scout-suite
  performing-aws-privilege-escalation-assessment performing-cloud-asset-inventory-with-cartography
  performing-cloud-forensics-with-aws-cloudtrail performing-cloud-log-forensics-with-athena
  performing-cloud-native-forensics-with-falco performing-cloud-native-threat-hunting-with-aws-detective
  performing-cloud-penetration-testing-with-pacu performing-gcp-penetration-testing-with-gcpbucketbrute
  performing-gcp-security-assessment-with-forseti performing-serverless-function-security-review
  remediating-s3-bucket-misconfiguration securing-api-gateway-with-aws-waf
  securing-aws-iam-permissions securing-aws-lambda-execution-roles
  securing-azure-with-microsoft-defender securing-container-registry-images
  securing-kubernetes-on-cloud securing-serverless-functions
)
ACS_CNT=(
  analyzing-kubernetes-audit-logs auditing-kubernetes-rbac-privilege-escalation
  benchmarking-kubernetes-with-kube-bench detecting-container-drift-at-runtime
  detecting-container-escape-attempts detecting-container-escape-with-falco-rules
  detecting-container-runtime-threats-with-falco detecting-privilege-escalation-in-kubernetes-pods
  escaping-containers-to-host hardening-docker-containers-for-production
  hardening-docker-daemon-configuration implementing-container-image-minimal-base-with-distroless
  implementing-container-network-policies-with-calico implementing-image-provenance-verification-with-cosign
  implementing-kubernetes-network-policy-with-calico implementing-kubernetes-pod-security-standards
  implementing-network-policies-for-kubernetes implementing-opa-gatekeeper-for-policy-enforcement
  implementing-pod-security-admission-controller implementing-rbac-hardening-for-kubernetes
  implementing-runtime-security-with-tetragon implementing-supply-chain-security-with-in-toto
  performing-container-escape-detection performing-container-security-scanning-with-trivy
  performing-docker-bench-security-assessment performing-kubernetes-cis-benchmark-with-kube-bench
  performing-kubernetes-etcd-security-assessment performing-kubernetes-penetration-testing
  scanning-container-images-with-grype scanning-docker-images-with-trivy
  scanning-kubernetes-manifests-with-kubesec securing-container-registry-with-harbor
  securing-helm-chart-deployments
)
for s in "${ACS_OT[@]}";    do copy_skill "$ACS/skills" "$ROOT/optional/acs-ot-ics" "$s"; done
for s in "${ACS_RT[@]}";    do copy_skill "$ACS/skills" "$ROOT/optional/acs-red-team" "$s"; done
for s in "${ACS_CLD[@]}";   do copy_skill "$ACS/skills" "$ROOT/optional/acs-cloud" "$s"; done
for s in "${ACS_CNT[@]}";   do copy_skill "$ACS/skills" "$ROOT/optional/acs-container" "$s"; done

# ACS — mappings ATT&CK/NIST/OWASP, couverture, licence
cp -r "$ACS/mappings" "$ROOT/acs/mappings"
cp "$ACS/ATTACK_COVERAGE.md" "$ROOT/acs/ATTACK_COVERAGE.md" 2>/dev/null
cp "$ACS/LICENSE" "$ROOT/acs/LICENSE" 2>/dev/null

# ================= Résumé =================
echo "=== RÉSUMÉ ===" >> "$RPT"
for d in bughunter optional/bughunter acs optional/acs-ot-ics optional/acs-red-team optional/acs-cloud optional/acs-container; do
  c=$(find "$ROOT/$d" -mindepth 2 -maxdepth 2 -name SKILL.md 2>/dev/null | wc -l)
  echo "$d : $c skills" >> "$RPT"
done
m=$(grep -c '^MISSING' "$RPT" || true)
echo "MISSING total : $m" >> "$RPT"
echo "commands copiées : $(ls "$ROOT/bughunter/commands" | wc -l)"
tail -12 "$RPT"

// lists.rs — sélections curatées du bundle (source : audit §5.2, reportées depuis build.sh)
#![allow(dead_code)]

pub const BH_T1: &[&str] = &[
    // hunt-* (32) — classes de vulnérabilités prioritaires
    "hunt-xss", "hunt-ssrf", "hunt-sqli", "hunt-idor", "hunt-auth-bypass", "hunt-jwt-crypto",
    "hunt-oauth", "hunt-saml", "hunt-rce", "hunt-graphql", "hunt-http-smuggling", "hunt-cache-poison",
    "hunt-race-condition", "hunt-file-upload", "hunt-lfi", "hunt-xxe", "hunt-ssti", "hunt-deserialization",
    "hunt-host-header", "hunt-open-redirect", "hunt-business-logic", "hunt-ato", "hunt-mfa-bypass",
    "hunt-session", "hunt-cors", "hunt-websocket", "hunt-source-leak", "hunt-shadow-api", "hunt-spa-api",
    "hunt-subdomain", "hunt-tls-network", "hunt-llm-ai",
    // process / reporting (5)
    "triage-validation", "evidence-hygiene", "report-writing", "bugcrowd-reporting", "redteam-report-template",
    // méthodologie (3)
    "bb-methodology", "redteam-mindset", "bug-bounty",
    // recon & OSINT (5)
    "web2-recon", "security-arsenal", "offensive-osint", "recon-scope-triage", "osint-methodology",
];

pub const BH_PROCESS: &[&str] = &[
    "triage-validation", "evidence-hygiene", "report-writing", "bugcrowd-reporting", "redteam-report-template",
];
pub const BH_METHODO: &[&str] = &["bb-methodology", "redteam-mindset", "bug-bounty"];
pub const BH_RECON: &[&str] = &[
    "web2-recon", "security-arsenal", "offensive-osint", "recon-scope-triage", "osint-methodology",
];

pub const ACS_WEB: &[&str] = &[
    "bypassing-authentication-with-forced-browsing", "exploiting-broken-link-hijacking",
    "exploiting-http-request-smuggling", "exploiting-idor-vulnerabilities",
    "exploiting-insecure-deserialization", "exploiting-mass-assignment-in-rest-apis",
    "exploiting-nosql-injection-vulnerabilities", "exploiting-oauth-misconfiguration",
    "exploiting-prototype-pollution-in-javascript", "exploiting-race-condition-vulnerabilities",
    "exploiting-server-side-request-forgery", "exploiting-sql-injection-with-sqlmap",
    "exploiting-template-injection-vulnerabilities", "exploiting-type-juggling-vulnerabilities",
    "exploiting-websocket-vulnerabilities", "implementing-devsecops-security-scanning",
    "implementing-runtime-application-self-protection",
    "implementing-web-application-logging-with-modsecurity",
    "performing-blind-ssrf-exploitation", "performing-clickjacking-attack-test",
    "performing-content-security-policy-bypass", "performing-csrf-attack-simulation",
    "performing-directory-traversal-testing", "performing-fuzzing-with-aflplusplus",
    "performing-graphql-security-assessment", "performing-http-parameter-pollution-attack",
    "performing-second-order-sql-injection", "performing-security-headers-audit",
    "performing-subdomain-enumeration-with-subfinder", "performing-supply-chain-attack-simulation",
    "performing-web-application-firewall-bypass", "performing-web-cache-deception-attack",
    "performing-web-cache-poisoning-attack", "testing-api-security-with-owasp-top-10",
    "testing-cors-misconfiguration", "testing-for-broken-access-control",
    "testing-for-business-logic-vulnerabilities", "testing-for-email-header-injection",
    "testing-for-host-header-injection", "testing-for-json-web-token-vulnerabilities",
    "testing-for-open-redirect-vulnerabilities", "testing-for-sensitive-data-exposure",
    "testing-for-xml-injection-vulnerabilities", "testing-for-xss-vulnerabilities-with-burpsuite",
    "testing-for-xxe-injection-vulnerabilities", "testing-jwt-token-security",
];

pub const ACS_API: &[&str] = &[
    "detecting-api-enumeration-attacks", "detecting-broken-object-property-level-authorization",
    "detecting-shadow-api-endpoints", "exploiting-api-injection-vulnerabilities",
    "exploiting-broken-function-level-authorization", "exploiting-excessive-data-exposure-in-api",
    "exploiting-jwt-algorithm-confusion-attack", "implementing-api-abuse-detection-with-rate-limiting",
    "implementing-api-gateway-security-controls", "implementing-api-key-security-controls",
    "implementing-api-rate-limiting-and-throttling", "implementing-api-schema-validation-security",
    "implementing-api-security-posture-management", "implementing-api-security-testing-with-42crunch",
    "implementing-api-threat-protection-with-apigee", "performing-api-fuzzing-with-restler",
    "performing-api-inventory-and-discovery", "performing-api-rate-limiting-bypass",
    "performing-api-security-testing-with-postman", "performing-graphql-depth-limit-attack",
    "performing-graphql-introspection-attack", "performing-jwt-none-algorithm-attack",
    "performing-soap-web-service-security-testing", "testing-api-authentication-weaknesses",
    "testing-api-for-broken-object-level-authorization", "testing-api-for-mass-assignment-vulnerability",
    "testing-oauth2-implementation-flaws", "testing-websocket-api-security",
];

pub const ACS_AI: &[&str] = &[
    "assessing-vector-and-embedding-weaknesses", "auditing-mcp-servers-for-tool-poisoning",
    "continuous-llm-red-teaming-with-promptfoo", "defending-llms-with-guardrails",
    "detecting-ai-model-prompt-injection-attacks", "detecting-data-and-model-poisoning",
    "detecting-indirect-prompt-injection", "detecting-model-extraction-attacks",
    "implementing-llm-guardrails-for-security", "orchestrating-llm-attacks-with-pyrit",
    "red-teaming-llms-with-garak", "securing-agentic-ai-tool-invocation",
    "testing-for-system-prompt-leakage", "testing-prompt-injection-in-rag-pipelines",
];

pub const ACS_CURATED: &[&str] = &[
    "performing-ssl-tls-security-assessment", "performing-dns-enumeration-and-zone-transfer",
    "scanning-network-with-nmap-advanced", "performing-cve-prioritization-with-kev-catalog",
    "implementing-epss-score-for-vulnerability-prioritization",
    "performing-web-application-scanning-with-nikto",
];

pub const ACS_OT: &[&str] = &[
    "detecting-anomalies-in-industrial-control-systems", "detecting-attacks-on-historian-servers",
    "detecting-attacks-on-scada-systems", "detecting-dnp3-protocol-anomalies",
    "detecting-modbus-command-injection-attacks", "detecting-modbus-protocol-anomalies",
    "detecting-stuxnet-style-attacks", "implementing-conduit-security-for-ot-remote-access",
    "implementing-dragos-platform-for-ot-monitoring", "implementing-ics-firewall-with-tofino",
    "implementing-iec-62443-security-zones", "implementing-nerc-cip-compliance-controls",
    "implementing-network-segmentation-for-ot", "implementing-ot-incident-response-playbook",
    "implementing-ot-network-traffic-analysis-with-nozomi", "implementing-patch-management-for-ot-systems",
    "implementing-purdue-model-network-segmentation", "monitoring-scada-modbus-traffic-anomalies",
    "performing-ics-asset-discovery-with-claroty", "performing-oil-gas-cybersecurity-assessment",
    "performing-ot-network-security-assessment", "performing-ot-vulnerability-assessment-with-claroty",
    "performing-ot-vulnerability-scanning-safely", "performing-plc-firmware-security-analysis",
    "performing-power-grid-cybersecurity-assessment", "performing-s7comm-protocol-security-analysis",
    "performing-scada-hmi-security-assessment", "securing-historian-server-in-ot-environment",
    "securing-remote-access-to-ot-environment",
];

pub const ACS_REDTEAM: &[&str] = &[
    "abusing-dpapi-for-credential-access", "abusing-shadow-credentials-for-privesc",
    "building-c2-infrastructure-with-sliver-framework", "building-c2-redirector-infrastructure",
    "building-red-team-c2-infrastructure-with-havoc", "coercing-authentication-with-coercer-petitpotam",
    "conducting-domain-persistence-with-dcsync", "conducting-full-scope-red-team-engagement",
    "conducting-internal-reconnaissance-with-bloodhound-ce", "conducting-pass-the-ticket-attack",
    "conducting-social-engineering-pretext-call", "conducting-spearphishing-simulation-campaign",
    "executing-red-team-engagement-planning", "exploiting-active-directory-certificate-services-esc1",
    "exploiting-active-directory-with-bloodhound", "exploiting-adcs-with-certipy",
    "exploiting-constrained-delegation-abuse", "exploiting-kerberoasting-with-impacket",
    "exploiting-ms17-010-eternalblue-vulnerability", "exploiting-nopac-cve-2021-42278-42287",
    "exploiting-zerologon-vulnerability-cve-2020-1472", "mapping-attack-paths-with-bloodhound-ce",
    "operating-havoc-c2", "operating-sliver-c2", "performing-active-directory-bloodhound-analysis",
    "performing-active-directory-forest-trust-attack", "performing-credential-access-with-lazagne",
    "performing-initial-access-with-evilginx3", "performing-kerberoasting-attack",
    "performing-lateral-movement-with-wmiexec", "performing-open-source-intelligence-gathering",
    "performing-physical-intrusion-assessment", "performing-privilege-escalation-on-linux",
    "performing-red-team-with-covenant", "relaying-ntlm-for-adcs-esc8",
];

pub const ACS_CLOUD: &[&str] = &[
    "analyzing-cloud-storage-access-patterns", "analyzing-office365-audit-logs-for-compromise",
    "auditing-aws-s3-bucket-permissions", "auditing-azure-active-directory-configuration",
    "auditing-cloud-with-cis-benchmarks", "auditing-gcp-iam-permissions",
    "auditing-kubernetes-cluster-rbac", "auditing-terraform-infrastructure-for-security",
    "building-cloud-siem-with-sentinel", "conducting-cloud-penetration-testing",
    "detecting-aws-cloudtrail-anomalies", "detecting-aws-credential-exposure-with-trufflehog",
    "detecting-aws-guardduty-findings-automation", "detecting-aws-iam-privilege-escalation",
    "detecting-azure-lateral-movement", "detecting-azure-service-principal-abuse",
    "detecting-azure-storage-account-misconfigurations", "detecting-cloud-threats-with-guardduty",
    "detecting-compromised-cloud-credentials", "detecting-cryptomining-in-cloud",
    "detecting-misconfigured-azure-storage", "detecting-oauth-token-theft",
    "detecting-s3-data-exfiltration-attempts", "detecting-serverless-function-injection",
    "detecting-shadow-it-cloud-usage", "detecting-suspicious-oauth-application-consent",
    "emulating-cloud-attacks-with-stratus-red-team", "enumerating-cloud-with-cloudfox",
    "exploiting-aws-with-pacu", "implementing-aws-config-rules-for-compliance",
    "implementing-aws-macie-for-data-classification", "implementing-aws-nitro-enclave-security",
    "implementing-aws-security-hub", "implementing-aws-security-hub-compliance",
    "implementing-azure-defender-for-cloud", "implementing-cloud-dlp-for-data-protection",
    "implementing-cloud-security-posture-management", "implementing-cloud-trail-log-analysis",
    "implementing-cloud-waf-rules", "implementing-cloud-workload-protection",
    "implementing-gcp-binary-authorization", "implementing-gcp-organization-policy-constraints",
    "implementing-gcp-vpc-firewall-rules", "implementing-secrets-management-with-vault",
    "implementing-zero-trust-in-cloud", "implementing-zero-trust-network-access",
    "managing-cloud-identity-with-okta", "performing-aws-account-enumeration-with-scout-suite",
    "performing-aws-privilege-escalation-assessment", "performing-cloud-asset-inventory-with-cartography",
    "performing-cloud-forensics-with-aws-cloudtrail", "performing-cloud-log-forensics-with-athena",
    "performing-cloud-native-forensics-with-falco", "performing-cloud-native-threat-hunting-with-aws-detective",
    "performing-cloud-penetration-testing-with-pacu", "performing-gcp-penetration-testing-with-gcpbucketbrute",
    "performing-gcp-security-assessment-with-forseti", "performing-serverless-function-security-review",
    "remediating-s3-bucket-misconfiguration", "securing-api-gateway-with-aws-waf",
    "securing-aws-iam-permissions", "securing-aws-lambda-execution-roles",
    "securing-azure-with-microsoft-defender", "securing-container-registry-images",
    "securing-kubernetes-on-cloud", "securing-serverless-functions",
];

pub const ACS_CONTAINER: &[&str] = &[
    "analyzing-kubernetes-audit-logs", "auditing-kubernetes-rbac-privilege-escalation",
    "benchmarking-kubernetes-with-kube-bench", "detecting-container-drift-at-runtime",
    "detecting-container-escape-attempts", "detecting-container-escape-with-falco-rules",
    "detecting-container-runtime-threats-with-falco", "detecting-privilege-escalation-in-kubernetes-pods",
    "escaping-containers-to-host", "hardening-docker-containers-for-production",
    "hardening-docker-daemon-configuration", "implementing-container-image-minimal-base-with-distroless",
    "implementing-container-network-policies-with-calico", "implementing-image-provenance-verification-with-cosign",
    "implementing-kubernetes-network-policy-with-calico", "implementing-kubernetes-pod-security-standards",
    "implementing-network-policies-for-kubernetes", "implementing-opa-gatekeeper-for-policy-enforcement",
    "implementing-pod-security-admission-controller", "implementing-rbac-hardening-for-kubernetes",
    "implementing-runtime-security-with-tetragon", "implementing-supply-chain-security-with-in-toto",
    "performing-container-escape-detection", "performing-container-security-scanning-with-trivy",
    "performing-docker-bench-security-assessment", "performing-kubernetes-cis-benchmark-with-kube-bench",
    "performing-kubernetes-etcd-security-assessment", "performing-kubernetes-penetration-testing",
    "scanning-container-images-with-grype", "scanning-docker-images-with-trivy",
    "scanning-kubernetes-manifests-with-kubesec", "securing-container-registry-with-harbor",
    "securing-helm-chart-deployments",
];

/// Packs optionnels ACS : (nom du dossier sous optional/, liste)
pub const ACS_PACKS: &[(&str, &[&str])] = &[
    ("acs-ot-ics", ACS_OT),
    ("acs-red-team", ACS_REDTEAM),
    ("acs-cloud", ACS_CLOUD),
    ("acs-container", ACS_CONTAINER),
];

pub fn bh_category(name: &str) -> &'static str {
    if name.starts_with("hunt-") {
        return "hunt-classe";
    }
    if BH_PROCESS.contains(&name) {
        return "process-reporting";
    }
    if BH_METHODO.contains(&name) {
        return "methodologie";
    }
    if BH_RECON.contains(&name) {
        return "recon-osint";
    }
    "autre"
}

pub fn acs_category(name: &str) -> &'static str {
    if ACS_WEB.contains(&name) {
        return "web-appsec";
    }
    if ACS_API.contains(&name) {
        return "api-security";
    }
    if ACS_AI.contains(&name) {
        return "ai-security";
    }
    if ACS_CURATED.contains(&name) {
        return "transversal";
    }
    "autre"
}

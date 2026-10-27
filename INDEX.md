# MCP-INTRUDER SKILLS — Bundle d'adoption (catalogue schématisé)

**Généré le 2026-10-26** — sélections issues de audit local AUDIT-OUTILS-MCP-SKILLS-2026-10-26.md (non publié) §5.2.

| Source | Tier 1 | Optionnel | Licence |
|---|---|---|---|
| [Claude-BugHunter](https://github.com/elementalsouls/Claude-BugHunter) @ `6a153f7` | 45 | 38 | MIT + CC BY 4.0 |
| [Anthropic-Cybersecurity-Skills](https://github.com/mukul975/Anthropic-Cybersecurity-Skills) @ `54a7988` | 94 | 163 | Apache-2.0 |
| **Total** | **139** | **201** | + 15 commands slash + CLI cbh + mappings MITRE/NIST/OWASP |

## Architecture du bundle
```
mcp-intruder-skills/
├── bughunter/            Tier 1 (45 skills) + commands/ (15) + cbh/ (CLI) + scripts/cbh.py
├── acs/                  Tier 1 (94 skills) + mappings/ (ATT&CK Navigator, NIST CSF, OWASP)
├── optional/             Packs opt-in (bughunter 38 · acs-ot-ics 29 · acs-red-team 35 · acs-cloud 66 · acs-container 33)
├── install.sh            Déploiement → ~/.claude/skills (Kali) ou --target <dir>
├── check_prereqs.sh      Vérifie les exigences outils des skills
├── tools-requirements.md Matrice exigences/écart vs serveur Kali votre-serveur-kali
├── skills-manifest.json  Catalogue machine (id, source, catégorie, tier, description)
└── BUILD-REPORT.md       Journal de construction
```

## Mécanisme de déclenchement
Chaque `SKILL.md` porte un frontmatter YAML ; le champ `description` sert de déclencheur sémantique (auto-load quand la tâche correspond). Le tableau ci-dessous reprend **le frontmatter réel** de chaque skill copié.

---

## BUGHUNTER · Process, reporting & méthodologie (8)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `bb-methodology` | Use at the START of any bug bounty hunting session, when switching targets, or when feeling lost about what to do next. Master orchestrator that combines the 5-phase non-linear hunting workflow with the critical thinking framework (develope |
| `bug-bounty` | Complete bug bounty workflow — recon (subdomain enumeration, asset discovery, fingerprinting, HackerOne scope, source code audit), pre-hunt learning (disclosed reports, tech stack research, mind maps, threat modeling), vulnerability hunti |
| `bugcrowd-reporting` | Bugcrowd-specific reporting tactics complementing report-writing: VRT category search-and-fallback strategy when no exact match exists, manual severity override when VRT defaults underrate impact, severity-request paragraph as first body se |
| `evidence-hygiene` | Evidence-capture and PoC-redaction discipline for bug-bounty submissions: cookie redaction protocol (which fields to mask, Preview annotation / Burp panel hiding / DevTools workflow), PII black-bar discipline (what to mask in other-user dat |
| `redteam-mindset` | Red-team operator discipline — the mindset corrections that separate offensive testing from defensive WAPT. Built from authorized red-team work where conservative defaults caused multiple findings to be missed and one to be incorrectly re |
| `redteam-report-template` | Client-facing red-team deliverable format — codifies the Subject / Observations / Description / Impact / Recommendation / PoC structure used for external red-team engagements (not bug-bounty platform reports). Different audience, differen |
| `report-writing` | Bug bounty report writing for H1/Bugcrowd/Intigriti/Immunefi — report templates, human tone guidelines, impact-first writing, CVSS 3.1 scoring, title formula, impact statement formula, severity decision guide, downgrade counters, pre-subm |
| `triage-validation` | Finding validation before writing any report — 7-Question Gate (all 7 questions), 4 pre-submission gates, always-rejected list, conditionally valid with chain table, CVSS 3.1 quick reference, severity decision guide, report title formula, |

## BUGHUNTER · Recon & OSINT (5)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `offensive-osint` | Operational arsenal for authorized external red-team and bug-bounty recon. Concrete probes, wordlists, regexes, dorks, curl one-liners for: subdomain enum, GraphQL/Swagger/REST discovery, identity fabric (Entra/Okta/ADFS/Google/SAML/M365 de |
| `osint-methodology` | Comprehensive OSINT methodology for external red-team operations and authorized attack-surface assessments. Covers the 5-stage recon pipeline (seed discovery, asset expansion, enrichment, exposure analysis, reporting), asset-graph disciplin |
| `recon-scope-triage` | Triage ASM/recon output for ownership before testing — separate the target's real assets from namespace-collision noise. Automated recon keyword-matches on the brand name, so for any target whose name is a common/dictionary word, the outp |
| `security-arsenal` | Security payloads, bypass tables, wordlists, gf pattern names, always-rejected bug list, and conditionally-valid-with-chain table. Use when you need specific payloads for XSS/SSRF/SQLi/XXE/NoSQLi/command injection/SSTI/IDOR/path-traversal/H |
| `web2-recon` | Web2 recon pipeline — subdomain enumeration (subfinder, Chaos API, assetfinder), live host discovery (dnsx, httpx), URL crawling (katana, waybackurls, gau), directory fuzzing (ffuf), JS analysis (LinkFinder, SecretFinder), continuous moni |

## BUGHUNTER · hunt-* — classes de vulnérabilités (32)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `hunt-ato` | Hunt account takeover taxonomy — 9 distinct paths to ATO, plus chains. Paths: (1) password reset flaws (host-header injection redirects token, predictable/numeric token, Referer leak, no-expiry/reuse), (2) email change without re-auth, (3 |
| `hunt-auth-bypass` | Hunting skill for auth bypass vulnerabilities. Built from 12 public bug bounty reports across SAML XSW / parser-differential (GitHub Enterprise CVE-2025-25291/25292), SAML signature stripping (Uber, Rocket.Chat, samlify CVE-2025-47949), SAM |
| `hunt-business-logic` | Hunting skill for business logic vulnerabilities. Built from 12 public bug bounty reports. Covers coupon-race-stacking (Instacart, Stripe, Reverb), negative-quantity-in-cart price tampering (Upserve, Eternal/Zomato), decimal/fraction price- |
| `hunt-cache-poison` | Hunting skill for cache poison vulnerabilities. Built from 10 public bug bounty reports including X-Forwarded-Host poisoning, X-HTTP-Method-Override / GCS cache, reflected→stored XSS via cache, classic Omer-Gil Web Cache Deception, Cloudf |
| `hunt-cors` | Hunt CORS Misconfiguration — origin-reflection with credentials, null-origin trust, subdomain-regex bypass (unanchored vs unescaped-dot vs prefix-only), pre-flight (OPTIONS) gating bypass, postMessage origin checks. High only when an atta |
| `hunt-deserialization` | Hunt Insecure Deserialization — Java gadget chains (ysoserial), PHP object injection (phpggc), Python pickle RCE, .NET BinaryFormatter, Ruby Marshal.load, JNDI/Log4Shell. RCE via deserialization is almost always Critical. Use when target  |
| `hunt-file-upload` | Hunt file upload bugs — RCE via webshell, XSS via SVG/HTML, SSRF via XXE in DOCX, path traversal via filename. Bypass tables (10 techniques): double extension (shell.php.jpg if server checks last ext only), magic bytes spoofing (PNG heade |
| `hunt-graphql` | Hunting skill for graphql vulnerabilities. Built from 12 public bug bounty reports across IDOR via node() / GID, mutation IDOR including AI/LLM features, cross-tenant IDOR, SSRF via argument, batching-DoS, query-cost-bypass, SQLi via argume |
| `hunt-host-header` | Hunt Host Header Injection — password reset poisoning → ATO, web cache poisoning via unkeyed Host/X-Forwarded-Host, routing-based SSRF (Host picks upstream → cloud metadata/internal services), path-override SSRF/ACL-bypass (X-Original |
| `hunt-http-smuggling` | Hunt HTTP request smuggling (CL.TE, TE.CL, H2.CL, H2.TE). Cause: front-end proxy and back-end server disagree on where one request ends and the next begins (Content-Length vs Transfer-Encoding header parsing inconsistency). CL.TE: front-end |
| `hunt-idor` | Hunting skill for idor vulnerabilities. Built from 26 public bug bounty reports. Use when hunting idor on any target. |
| `hunt-jwt-crypto` | Hunt JWT cryptographic failures — alg:none signature-stripping and RS256→HS256 key-confusion that let an attacker forge a token for any identity (e.g. an admin) without knowing a secret. Use when the app authenticates with a JSON Web To |
| `hunt-lfi` | Hunt Local File Inclusion (LFI), Remote File Inclusion (RFI), and Path Traversal — /etc/passwd read, log poisoning → RCE, PHP filter-chain RCE (no upload needed), php:// / data:// / zip:// / phar:// wrappers, RFI via allow_url_include,  |
| `hunt-llm-ai` | Hunt LLM/AI feature bugs — prompt injection, indirect injection, exfiltration via tool-use/markdown, ASCII smuggling, agentic AI security (OWASP Agentic Apps 2026, ASI01-ASI10). Patterns: direct injection ('ignore previous instructions'), |
| `hunt-mfa-bypass` | Hunt MFA / 2FA bypass — 7 distinct patterns. (1) MFA not enforced on sensitive endpoints (password change, email change accept without MFA challenge), (2) MFA-step skip via direct navigation to post-login URL, (3) MFA-token replay (same c |
| `hunt-oauth` | Hunting skill for oauth vulnerabilities. Built from 19 public bug bounty reports. Use when hunting oauth on any target. |
| `hunt-open-redirect` | Hunt Open Redirect — all types including low-impact, chained to OAuth token theft → ATO, phishing chains. URL parameter manipulation, JavaScript redirect, meta refresh, header injection. Use when hunting redirect bugs or building ATO ch |
| `hunt-race-condition` | Hunting skill for race condition vulnerabilities. Built from 12 public bug bounty reports including modern HTTP/2 single-packet attack cases (James Kettle DEF CON 2023 Smashing the State Machine; RyotaK / Flatt Security 10,000-request first |
| `hunt-rce` | Hunting skill for rce vulnerabilities. Built from 67 public bug bounty reports. Use when hunting rce on any target. |
| `hunt-saml` | Hunt SAML / SSO attacks. Patterns: XML Signature Wrapping (XSW) — modify Assertion while keeping Signature valid by relocating signed element, comment injection in NameID (admin@target.com<!--evil-->@attacker.com → some parsers see admi |
| `hunt-session` | Hunt Session Management vulnerabilities — session fixation (no regeneration on login), insufficient invalidation on logout / password-change / email-change, predictable or low-entropy session IDs, JWT-as-session with no exp/revocation, re |
| `hunt-shadow-api` | Hunt shadow / zombie / undocumented API surface (OWASP API9 Improper Inventory Management) — enumerate the full API version history (v1/v2/beta/legacy paths, header- and subdomain-based versioning), pull and diff every reachable OpenAPI/S |
| `hunt-source-leak` | Hunt source code and build artifact leakage — JavaScript source maps (.js.map) reconstructing TypeScript/ES6 source, Swagger/OpenAPI JSON endpoint discovery, .env/.git exposure, webpack chunks with hardcoded secrets, robots.txt/security.t |
| `hunt-spa-api` | Discover a single-page-app's hidden backend API from its public JS bundle, then test that API for broken access control / missing authentication. One of the highest-yield web plays in modern recon — SPAs ship their entire backend route ma |
| `hunt-sqli` | Hunting skill for sqli vulnerabilities. Built from 12 public bug bounty reports including modern NoSQL injection (Rocket.Chat CVE-2021-22911 MongoDB $regex, Mongoose ORM CVE-2024-53900 $where bypass), modern ORM raw-fragment SQLi (Django CV |
| `hunt-ssrf` | Hunting skill for ssrf vulnerabilities. Built from 15 public bug bounty reports including AWS metadata SSRF (HackerOne $25k Analytics PDF, Shopify Exchange $25k, Capital One 106M-record breach, Dropbox/HelloSign $4,913), GCP metadata SSRF ( |
| `hunt-ssti` | Hunt server-side template injection (SSTI) across Jinja2 (Flask/Django), Twig (Symfony), Freemarker (Java), ERB (Rails), Spring, Velocity, Mako, Thymeleaf, Smarty. Detection probes use double-curly and dollar-curly math expressions evaluate |
| `hunt-subdomain` | Hunting skill for subdomain takeover vulnerabilities. Includes modern provider fingerprints — Microsoft Azure DevOps `cloudapp.azure.com` regional-pool re-issue (1-click OAuth ATO via wildcard `reply_to`, Binary Security), Zendesk help-de |
| `hunt-tls-network` | Hunt TLS/SSL and DNS misconfigurations — missing HSTS (downgrade attack), weak cipher suites, expired/invalid certificates, mTLS bypass, missing SPF/DKIM/DMARC (email spoofing), DNS Zone Transfer (AXFR), dangling CNAME subdomain takeover, |
| `hunt-websocket` | Hunt WebSocket vulnerabilities — Cross-Site WebSocket Hijacking (CSWSH), missing/weak Origin validation on the WS handshake, no per-message authentication, message tampering, socket.io namespace/room authorization bypass, and handshake-la |
| `hunt-xss` | Hunting skill for xss vulnerabilities. Built from 174 public bug bounty reports. Use when hunting xss on any target. For markup injection that reflects raw HTML but does NOT execute JavaScript (no `<script>`/event-handler execution), see hu |
| `hunt-xxe` | Hunting skill for xxe vulnerabilities. Built from 10 public bug bounty reports including SVG-upload XXE, Office-doc (PPTX/DOCX) XXE, SOAP XXE, SAML AssertionConsumer XXE, blind OOB XXE via DTD callback, parameter-entity XXE, XXE-to-LFI, XXE |

## BUGHUNTER · Commands slash (15)

| Commande | Rôle |
|---|---|
| `/hunt <target>` | Méta-routeur Red Team vs WAPT, charge le bon set de skills |
| `/recon <target>` | Recon (support déterministe via CLI cbh) |
| `/surface <target>` | Surface d'attaque classée P1/P2/Kill |
| `/scope <asset>` | Gate de scope déterministe avant tout toucher HTTP |
| `/triage` / `/validate` | 7-Question Gate : PASS/KILL/DOWNGRADE/CHAIN-REQUIRED |
| `/report` | Rapport H1/Bugcrowd/Intigriti/Immunefi + CVSS |
| `/chain` | Chaînes d'exploit A→B (17+ patterns), timeboxing 20 min |
| `/intel <target>` | CVE NVD + rapports divulgués Hacktivity |
| `/autopilot` | Modes mémoire : --paranoid/--normal (off) · --quick/--yolo (on) |
| `/memory-gc` | Gestion du store mémoire par engagement |
| `/pickup <host>` | Historique de chasse d'une cible |
| `/remember` | Note manuelle dans le ledger |
| `/token-scan` | Scan rug-pull EVM/Solana (8 classes) |
| `/web3-audit` | Audit smart contract 10 classes + PoC Foundry |

**CLI cbh** (python3 ≥3.9, stdlib) : `python3 bughunter/scripts/cbh.py recon|surface|classify|triage|report <args>` — index : `bughunter/cbh/data/skill_index.json`.

---

## ACS · Web Application Security (46)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `bypassing-authentication-with-forced-browsing` | Discovering and accessing unprotected pages, APIs, and administrative interfaces by enumerating URLs and bypassing authentication controls during authorized security assessments. |
| `exploiting-broken-link-hijacking` | >- Discovers and exploits broken link hijacking by spidering a site (Burp Suite Spider, Scrapy, curl scraping), extracting referenced external scripts/domains, and checking DNS/CNAME records and domain registration status for expired or unc |
| `exploiting-http-request-smuggling` | >- Detects and exploits HTTP request smuggling caused by Content-Length/Transfer-Encoding parsing discrepancies between front-end and back-end servers, using Burp Suite Repeater (auto Content-Length disabled), the HTTP Request Smuggler exte |
| `exploiting-idor-vulnerabilities` | >- Identifies and exploits Insecure Direct Object Reference (IDOR) vulnerabilities by manipulating object identifiers (numeric IDs, UUIDs, slugs) in API requests and URLs, using Burp Suite proxy history, Intruder, and the Authorize extensio |
| `exploiting-insecure-deserialization` | Identifying and exploiting insecure deserialization vulnerabilities in Java, PHP, Python, and .NET applications to achieve remote code execution during authorized penetration tests. |
| `exploiting-mass-assignment-in-rest-apis` | >- Discovers and exploits mass assignment (autobinding) in REST APIs by injecting unexpected or hidden parameters (e.g. role, isAdmin, plan) into create/update requests, using Burp Suite Intruder, Arjun, and param-miner to find bindable fie |
| `exploiting-nosql-injection-vulnerabilities` | Detects and exploits NoSQL injection vulnerabilities in MongoDB, CouchDB, and similar databases to demonstrate authentication bypass, data extraction, and unauthorized access via crafted query operators. Use when pentesting APIs or web appl |
| `exploiting-oauth-misconfiguration` | Identifying and exploiting OAuth 2.0 and OpenID Connect misconfigurations including redirect URI manipulation, token leakage, and authorization code theft during security assessments. |
| `exploiting-prototype-pollution-in-javascript` | Detects and exploits JavaScript prototype pollution vulnerabilities in client-side and server-side (Node.js) applications to achieve XSS, RCE, or authentication bypass through property injection into Object.prototype. Use when assessing a J |
| `exploiting-race-condition-vulnerabilities` | Detects and exploits race condition (TOCTOU) vulnerabilities in web applications using Burp Suite's Turbo Intruder extension and its single-packet attack technique to fire parallel requests that bypass rate limits, duplicate transactions, o |
| `exploiting-server-side-request-forgery` | Identifying and exploiting SSRF vulnerabilities to access internal services, cloud metadata, and restricted network resources during authorized penetration tests. |
| `exploiting-sql-injection-with-sqlmap` | Detecting and exploiting SQL injection vulnerabilities using sqlmap to extract database contents during authorized penetration tests. |
| `exploiting-template-injection-vulnerabilities` | Detects and exploits Server-Side Template Injection (SSTI) vulnerabilities across Jinja2, Twig, Freemarker, and other template engines to achieve remote code execution. Use when pentesting a web application that renders user input through a |
| `exploiting-type-juggling-vulnerabilities` | Exploits PHP type juggling vulnerabilities caused by loose (==) comparison operators to bypass authentication, defeat hash verification via magic hashes, and manipulate application logic through type coercion. Use when pentesting PHP applic |
| `exploiting-websocket-vulnerabilities` | Testing WebSocket implementations for authentication bypass, cross-site hijacking, injection attacks, and insecure message handling during authorized security assessments. |
| `implementing-devsecops-security-scanning` | 'Integrates SAST, DAST, and SCA into CI/CD pipelines using Semgrep for SAST, Trivy for SCA and container scanning, OWASP ZAP for DAST, and Gitleaks for secrets detection. Use when setting up automated security scanning in CI/CD, shifting se |
| `implementing-runtime-application-self-protection` | Deploy Runtime Application Self-Protection (RASP) agents to detect and block attacks from within application runtime, covering OpenRASP integration, attack pattern detection, and security policy configuration for Java and Python web applica |
| `implementing-web-application-logging-with-modsecurity` | Configure ModSecurity WAF with the OWASP Core Rule Set (CRS) for web application audit logging, tuning SecRuleEngine, SecAuditEngine, and CRS paranoia levels to reduce false positives, and writing custom SecRules for application-specific th |
| `performing-blind-ssrf-exploitation` | Detect and exploit blind Server-Side Request Forgery (SSRF) using out-of-band techniques such as Burp Collaborator DNS interactions and timing analysis, to reach internal services and cloud metadata endpoints even when server responses are  |
| `performing-clickjacking-attack-test` | Testing web applications for clickjacking vulnerabilities by assessing frame embedding controls and crafting proof-of-concept overlay attacks during authorized security assessments. |
| `performing-content-security-policy-bypass` | Analyze Content-Security-Policy headers and bypass them to achieve cross-site scripting by exploiting unsafe-inline/unsafe-eval, whitelisted JSONP endpoints, base-uri and form-action gaps, and nonce/hash weaknesses, then exfiltrate data eve |
| `performing-csrf-attack-simulation` | Testing web applications for Cross-Site Request Forgery vulnerabilities by crafting forged requests that exploit authenticated user sessions during authorized security assessments. |
| `performing-directory-traversal-testing` | Test web applications for path traversal and Local/Remote File Inclusion vulnerabilities by manipulating file path parameters, applying encoding and filter-bypass techniques, automating discovery with ffuf and dotdotpwn, and reading high-va |
| `performing-fuzzing-with-aflplusplus` | 'Performs coverage-guided fuzzing of compiled binaries with AFL++, instrumenting targets via afl-cc/afl-clang-fast, minimizing corpora with afl-cmin and afl-tmin, running parallel campaigns with afl-fuzz, and triaging crashes with CASR or G |
| `performing-graphql-security-assessment` | Assessing GraphQL API endpoints for introspection leaks, injection attacks, authorization flaws, and denial-of-service vulnerabilities during authorized security tests. |
| `performing-http-parameter-pollution-attack` | Executes HTTP Parameter Pollution attacks that inject duplicate request parameters to bypass input validation, WAF rules, and other security controls when front-end and back-end systems parse duplicate parameters differently. Use during web |
| `performing-second-order-sql-injection` | Detect and exploit second-order SQL injection vulnerabilities where malicious input is stored in a database and later executed in an unsafe SQL query during a different application operation. |
| `performing-security-headers-audit` | Auditing HTTP security headers including CSP, HSTS, X-Frame-Options, and cookie attributes to identify missing or misconfigured browser-level protections. |
| `performing-subdomain-enumeration-with-subfinder` | Enumerate subdomains of target domains using ProjectDiscovery's Subfinder passive reconnaissance tool to map the attack surface during security assessments. |
| `performing-supply-chain-attack-simulation` | >- Simulates and detects software supply chain attacks: typosquatting detection via Levenshtein distance against popular PyPI package names, dependency confusion testing against private registries, SHA-256 package hash verification, and kno |
| `performing-web-application-firewall-bypass` | Bypasses Web Application Firewall protections using encoding tricks, HTTP method manipulation, parameter pollution, and payload obfuscation to smuggle SQL injection, XSS, and other exploit payloads past WAF detection rules. Use when a confi |
| `performing-web-cache-deception-attack` | Executes web cache deception attacks by exploiting path normalization discrepancies between CDN/reverse-proxy caching layers (Cloudflare, Akamai, Varnish, Nginx) and origin servers to cache and retrieve sensitive authenticated content. Use  |
| `performing-web-cache-poisoning-attack` | Exploiting web cache mechanisms to serve malicious content to other users by poisoning cached responses through unkeyed headers and parameters during authorized security tests. |
| `testing-api-security-with-owasp-top-10` | Systematically assesses REST, GraphQL, and gRPC API endpoints against the OWASP API Security Top 10 (2023) using Burp Suite and Postman for automated and manual testing. Use during authorized API penetration tests, before deploying new endp |
| `testing-cors-misconfiguration` | Identifying and exploiting Cross-Origin Resource Sharing misconfigurations that allow unauthorized cross-domain data access and credential theft during security assessments. |
| `testing-for-broken-access-control` | Systematically tests web applications and APIs for broken access control (OWASP A01:2021), including privilege escalation, missing function-level checks, insecure direct object references, and multi-tenant data leakage, using Burp Suite wit |
| `testing-for-business-logic-vulnerabilities` | Manually identifies flaws in application business logic - price manipulation, multi-step workflow bypass, and privilege escalation - by intercepting and modifying requests with Burp Suite, going beyond what automated vulnerability scanners  |
| `testing-for-email-header-injection` | Tests web application email functionality (contact forms, password reset, newsletter subscriptions) for CRLF/SMTP header injection using Burp Suite and OWASP ZAP, checking whether attackers can inject headers, modify recipients, or abuse fo |
| `testing-for-host-header-injection` | Test web applications for HTTP Host header injection vulnerabilities to identify password reset poisoning, web cache poisoning, SSRF, and virtual host routing manipulation risks. |
| `testing-for-json-web-token-vulnerabilities` | Tests JWT implementations for algorithm confusion, none algorithm bypass, kid/jku parameter injection, and weak secret exploitation using jwt_tool and Burp Suite's JWT Editor extension, aiming to achieve authentication bypass and privilege  |
| `testing-for-open-redirect-vulnerabilities` | Identifies and exploits open redirect vulnerabilities by analyzing URL redirection parameters (next, url, redirect, return, goto), applying bypass techniques, and chaining findings into phishing or token-theft exploits, using Burp Suite/OWA |
| `testing-for-sensitive-data-exposure` | Identifying sensitive data exposure vulnerabilities including API key leakage, PII in responses, insecure storage, and unprotected data transmission during security assessments. |
| `testing-for-xml-injection-vulnerabilities` | Test web applications for XML injection vulnerabilities including XXE, XPath injection, and XML entity attacks to identify data exposure and server-side request forgery risks. |
| `testing-for-xss-vulnerabilities-with-burpsuite` | Identifying and validating cross-site scripting vulnerabilities using Burp Suite's scanner, intruder, and repeater tools during authorized security assessments. |
| `testing-for-xxe-injection-vulnerabilities` | Discovering and exploiting XML External Entity injection vulnerabilities to read server files, perform SSRF, and exfiltrate data during authorized penetration tests. |
| `testing-jwt-token-security` | Assessing JSON Web Token implementations for cryptographic weaknesses, algorithm confusion attacks, and authorization bypass vulnerabilities during security engagements. |

## ACS · API Security (28)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `detecting-api-enumeration-attacks` | Detect API enumeration attacks (BOLA/IDOR, OWASP API1:2023) by writing SIEM detection rules that flag sequential or UUID identifier iteration, parameter tampering, and mixed 200/401/403 response patterns from API gateway and WAF logs. Use w |
| `detecting-broken-object-property-level-authorization` | Detect and test for OWASP API3:2023 Broken Object Property Level Authorization (BOPLA), covering excessive data exposure in API responses and mass assignment via injected request-body properties. Use when reviewing API responses/requests fo |
| `detecting-shadow-api-endpoints` | Discover and inventory shadow API endpoints that operate outside documented OpenAPI/Swagger specs, using traffic analysis against API gateways (Kong, AWS API Gateway, Envoy), cloud configuration scanning, and source code repository mining f |
| `exploiting-api-injection-vulnerabilities` | >- Tests API parameters, headers, and request bodies for injection flaws — SQL injection, NoSQL injection, OS command injection, LDAP injection, and SSRF — by crafting payloads tailored to the target backend to extract data, execute com |
| `exploiting-broken-function-level-authorization` | >- Tests APIs for Broken Function Level Authorization (OWASP API5:2023) by identifying admin and privileged endpoints, then reaching them with regular-user credentials via HTTP method switching, URL path manipulation, and parameter tamperin |
| `exploiting-excessive-data-exposure-in-api` | >- Tests APIs for excessive data exposure (OWASP API3:2023) by intercepting raw API responses and comparing them against what the UI actually renders, looking for leaked PII, internal identifiers, debug data, or business-sensitive fields th |
| `exploiting-jwt-algorithm-confusion-attack` | >- Exploits JWT algorithm confusion where the server's verification library trusts the alg named in the token header, by switching RS256 to HS256 (signing with the RSA public key as HMAC secret), setting alg to none, or injecting kid/jku/x5 |
| `implementing-api-abuse-detection-with-rate-limiting` | Implements API abuse detection using token bucket, sliding window, and fixed window rate-limiting algorithms backed by Redis, including adaptive limits that tighten during detected attacks and relax during normal traffic. Use when defending |
| `implementing-api-gateway-security-controls` | 'Configures API gateways such as Kong, AWS API Gateway, Azure APIM, or Apigee as a centralized security enforcement point, covering authentication enforcement, rate limiting and throttling, request validation, IP allowlisting, TLS terminati |
| `implementing-api-key-security-controls` | 'Implements secure API key generation with sufficient entropy, server-side hashing (SHA-256/bcrypt) instead of plaintext storage, per-key scoping to endpoints/IPs/rate limits, zero-downtime rotation, and automated leak monitoring across Git |
| `implementing-api-rate-limiting-and-throttling` | 'Implements API rate limiting and throttling with token bucket, sliding window, and fixed window algorithms, configuring per-user, per-IP, and per-endpoint limits via Redis-backed counters, API gateway plugins, or middleware, and returning  |
| `implementing-api-schema-validation-security` | Implements API schema validation using OpenAPI Specification and JSON Schema documents, enforced both at the API gateway (runtime) and during development (shift-left), to lock down request/response contracts and reject unknown properties. U |
| `implementing-api-security-posture-management` | Implements API Security Posture Management (API-SPM) to continuously discover, classify, and risk-score APIs -- including internal, external, partner, and shadow endpoints -- while aggregating findings from DAST, SAST, SCA, and runtime moni |
| `implementing-api-security-testing-with-42crunch` | Implements API security testing on the 42Crunch platform, combining API Audit for static analysis of OpenAPI definitions, API Conformance Scan for dynamic vulnerability testing, and API Protect for runtime threat prevention, integrated into |
| `implementing-api-threat-protection-with-apigee` | Implements API threat protection using Google Apigee reverse-proxy policies, including JSON/XML threat protection, OAuth 2.0 enforcement, SpikeArrest rate limiting, regex-based threat detection, and Advanced API Security for detecting malic |
| `performing-api-fuzzing-with-restler` | 'Uses Microsoft RESTler to perform stateful REST API fuzzing: compiles an OpenAPI/Swagger spec into a RESTler grammar, configures authentication, and runs test/fuzz-lean/fuzz modes that generate request sequences exercising producer-consume |
| `performing-api-inventory-and-discovery` | 'Performs API inventory and discovery to identify all API endpoints in an organization''s environment including documented, undocumented, shadow, zombie, and deprecated APIs. The tester uses passive traffic analysis, active scanning, DNS en |
| `performing-api-rate-limiting-bypass` | 'Tests API rate limiting for bypass vulnerabilities using Python (requests/aiohttp) and Burp Suite Turbo Intruder to manipulate headers (e.g. X-Forwarded-For spoofing), IPs, HTTP methods, API versions, and encodings, mapping findings to OWA |
| `performing-api-security-testing-with-postman` | 'Uses Postman to build structured API security test collections covering the OWASP API Security Top 10—authentication bypass, authorization flaws, injection, and data exposure—with multi-role environments, automated test scripts, and OW |
| `performing-graphql-depth-limit-attack` | Execute and test GraphQL depth limit attacks using deeply nested recursive queries to identify denial-of-service vulnerabilities in GraphQL APIs. |
| `performing-graphql-introspection-attack` | 'Performs GraphQL introspection attacks that extract the full API schema (types, queries, mutations, subscriptions, field definitions), map the attack surface, test query depth/complexity limits, and exploit GraphQL-specific weaknesses such |
| `performing-jwt-none-algorithm-attack` | Execute and test the JWT none algorithm attack, crafting tokens with the alg header set to none using PyJWT and an intercepting proxy (Burp Suite/mitmproxy) to bypass signature verification and forge arbitrary claims. Use during authorized  |
| `performing-soap-web-service-security-testing` | Performs security testing of SOAP web services by analyzing WSDL definitions and testing for XML injection, XXE, WS-Security bypass, SOAPAction spoofing, and XPath injection. Use when assessing a SOAP/WSDL-based API endpoint for XML-related |
| `testing-api-authentication-weaknesses` | 'Tests API authentication mechanisms for weaknesses including broken token validation, missing authentication on endpoints, weak password policies, credential stuffing susceptibility, token leakage in URLs or logs, and session management fl |
| `testing-api-for-broken-object-level-authorization` | 'Tests REST and GraphQL APIs for Broken Object Level Authorization (BOLA/IDOR, OWASP API1:2023) by intercepting API calls, identifying object ID parameters (numeric IDs, UUIDs, slugs), and systematically substituting IDs belonging to other  |
| `testing-api-for-mass-assignment-vulnerability` | 'Tests APIs for mass assignment (auto-binding), OWASP API3:2023, by identifying writable endpoints, adding undocumented fields to request bodies (role, isAdmin, price, balance), and checking whether the server binds them to the data model w |
| `testing-oauth2-implementation-flaws` | Tests OAuth 2.0 and OpenID Connect implementations for authorization code interception, redirect URI manipulation, CSRF in OAuth flows, token leakage, scope escalation, and PKCE bypass, using Burp Suite Professional and the EsPReSSO extensi |
| `testing-websocket-api-security` | Tests WebSocket API implementations for missing upgrade-handshake authentication, Cross-Site WebSocket Hijacking (CSWSH), message injection, insufficient input validation, message-flooding DoS, and information leakage, using Burp Suite's We |

## ACS · AI Security (14) — incl. audit de serveurs MCP

| Skill | Déclencheur (frontmatter) |
|---|---|
| `assessing-vector-and-embedding-weaknesses` | Test RAG vector stores (Pinecone, Qdrant, Weaviate, Chroma, pgvector, FAISS) for embedding inversion, cross-tenant data leakage, and data poisoning per OWASP LLM08:2025. Use when performing an authorized security assessment of a RAG pipelin |
| `auditing-mcp-servers-for-tool-poisoning` | Audit MCP servers for tool poisoning, tool shadowing, rug pulls, SSRF, and unauthenticated exposure using Invariant Labs' mcp-scan for static/runtime scanning plus manual SSRF/auth checks and description pinning. Use before adding a new MCP |
| `continuous-llm-red-teaming-with-promptfoo` | Wires Promptfoo and DeepTeam into CI/CD for automated, repeatable red-teaming of LLM apps against OWASP LLM Top 10, OWASP Agentic, and MITRE ATLAS presets, failing the build when jailbreak or injection vulnerabilities regress. Use for conti |
| `defending-llms-with-guardrails` | Deploys Llama Guard 3 safety classification, NeMo Guardrails programmable dialogue rails, and LLM Guard input/output scanner pipelines as complementary runtime defenses that inspect and constrain LLM prompts and responses. Use when adding a |
| `detecting-ai-model-prompt-injection-attacks` | Detects prompt injection using regex signature matching, heuristic scoring for structural anomalies, and DeBERTa-based transformer classification, flagging direct injections (system-prompt overrides, role-play escapes) and indirect injectio |
| `detecting-data-and-model-poisoning` | Identify poisoned training data and backdoored ML models across the pipeline using IBM's Adversarial Robustness Toolbox (activation clustering, spectral signatures, trigger reconstruction), Cleanlab for label-quality issues, and supply-chai |
| `detecting-indirect-prompt-injection` | Detect and defend against indirect prompt injection hidden in web pages, documents, and images consumed by an agent, via content extraction (HTML/PDF/OCR), normalization, and scanning with LLM Guard's PromptInjection scanner or Hugging Face |
| `detecting-model-extraction-attacks` | Detect MITRE ATLAS AML.T0024 attacks (model stealing, inversion, membership inference) performed via inference-API abuse, by monitoring per-principal query volume/distribution, rate-limiting and perturbing outputs, and red-teaming your mode |
| `implementing-llm-guardrails-for-security` | 'Implements input/output validation guardrails for LLM applications using NVIDIA NeMo Guardrails (Colang), custom Python validators for PII detection, and the Guardrails AI framework, intercepting user inputs (prompt injection, PII, off-top |
| `orchestrating-llm-attacks-with-pyrit` | Build automated multi-turn adversarial attacks against conversational LLM targets using Microsoft PyRIT's RedTeamingOrchestrator, CrescendoOrchestrator (gradual escalation), and TreeOfAttacksWithPruningOrchestrator (adaptive branching), wit |
| `red-teaming-llms-with-garak` | Runs NVIDIA garak probe suites (jailbreak, prompt injection, data leakage, toxicity, and more) against an LLM endpoint - Hugging Face models, OpenAI-compatible APIs, or Bedrock - then interprets the resulting hit-rate report for triage. Use |
| `securing-agentic-ai-tool-invocation` | Implements defense-in-depth controls at an AI agent's tool-invocation boundary using tool allowlisting, least-privilege identity binding, NeMo Guardrails policy enforcement, human-in-the-loop approval, and audit logging. Use when hardening  |
| `testing-for-system-prompt-leakage` | Extracts LLM system prompts using direct requests, jailbreak/instruction-override framing, translation/encoding tricks, and few-shot replay, combining manual payloads with automated garak and Promptfoo scanners to surface embedded secrets,  |
| `testing-prompt-injection-in-rag-pipelines` | Probes Retrieval-Augmented Generation pipelines for indirect prompt injection via poisoned retrieved documents and embedding-space manipulation, using NVIDIA garak, Promptfoo red-team plugins, and Microsoft PyRIT against vector stores like  |

## ACS · Transversal curaté (6)

| Skill | Déclencheur (frontmatter) |
|---|---|
| `implementing-epss-score-for-vulnerability-prioritization` | Queries FIRST's Exploit Prediction Scoring System (EPSS) API to fetch exploitation-probability and percentile scores for CVEs, then uses those scores to prioritize vulnerability remediation. Use when triaging or ranking a vulnerability back |
| `performing-cve-prioritization-with-kev-catalog` | Fetch and parse the CISA Known Exploited Vulnerabilities (KEV) catalog, enrich it with EPSS scores and CVSS metrics, and build a multi-factor prioritization engine and report that ranks CVE remediation by real-world exploitation evidence an |
| `performing-dns-enumeration-and-zone-transfer` | 'Enumerates DNS records, attempts zone transfers, brute-forces subdomains, and maps DNS infrastructure during authorized reconnaissance to identify attack surface, misconfigurations, and information disclosure in target domains. |
| `performing-ssl-tls-security-assessment` | >- Assess SSL/TLS server configurations using the sslyze Python scanning library to evaluate supported protocol versions, cipher suite strength, certificate chain validation, HSTS enforcement, OCSP stapling, and known vulnerabilities such a |
| `performing-web-application-scanning-with-nikto` | Runs Nikto, an open-source web server and web application scanner, to test over 7,000 potentially dangerous files/programs, check for outdated versions across 1,250+ servers, and identify XSS, SQL injection, misconfigurations, default crede |
| `scanning-network-with-nmap-advanced` | 'Performs advanced network recon using Nmap''s Scripting Engine (NSE), timing controls, firewall/IDS evasion, and structured output parsing to discover hosts, enumerate service versions, detect vulnerabilities, and fingerprint OSes. Use dur |

## ACS · Mappings embarqués (acs/mappings/)

- `attack-navigator-layer.json` — couche ATT&CK Navigator (218 techniques)
- `mitre-attack/`, `nist-csf/`, `owasp/` — mappings par framework
- `ATTACK_COVERAGE.md` — résumé de couverture (290 techniques distinctes sur 805 skills)

---

## Packs optionnels (201 skills — activables via `install.sh --all` ou `--optional <groupe>`)

| Groupe | Skills | Intérêt pour MCP-Intruder |
|---|---|---|
| `optional/bughunter` | 38 | Le reste des hunt-* : frameworks (Laravel/Next.js/Spring), k8s, captcha, brute-force, fintech-graphql, grpc… |
| `optional/acs-ot-ics` | 29 | Modbus/DNP3/S7comm — **cible Modbus 502 déjà vue en engagement** |
| `optional/acs-red-team` | 35 | AD/Kerberos/BloodHound/ADCS/C2 (Sliver/Havoc) |
| `optional/acs-cloud` | 66 | AWS/Azure/GCP IAM, S3, GuardDuty, Pacu/ScoutSuite/CloudFox |
| `optional/acs-container` | 33 | Docker/K8s escape, Falco, Trivy, RBAC |

---

## Prérequis outils
Voir [tools-requirements.md](tools-requirements.md) et vérifier avec `bash check_prereqs.sh` (à lancer sur la cible d'installation).

// cmd_prereqs.rs — vérifie que les exigences outils des skills adoptés sont satisfaites
use crate::fsutil::any_in_path;
use std::process::Command;

struct Tool {
    name: &'static str,
    required: bool,
    why: &'static str,
    hint: &'static str,
}

const TOOLS: &[Tool] = &[
    // REQUIS
    Tool { name: "curl", required: true, why: "hunt-* / ACS web : client HTTP de test primaire", hint: "apt install curl (préinstallé Kali)" },
    Tool { name: "jq", required: true, why: "evidence-hygiene (sanitization HAR), parsing JSON", hint: "apt install jq" },
    Tool { name: "python3", required: true, why: "scripts helpers ACS", hint: "apt install python3 (>=3.9)" },
    Tool { name: "git", required: true, why: "clonage d'outils / mise à jour bundle", hint: "apt install git" },
    Tool { name: "dig", required: true, why: "recon DNS (web2-recon, dns-enum)", hint: "apt install dnsutils" },
    Tool { name: "whois", required: true, why: "OSINT domaines", hint: "apt install whois" },
    Tool { name: "openssl", required: true, why: "TLS (ssl-tls-assessment)", hint: "préinstallé" },
    Tool { name: "nmap", required: true, why: "nmap-advanced, scans réseau", hint: "apt install nmap" },
    Tool { name: "nuclei", required: true, why: "templates CVE (ACS web)", hint: "apt install nuclei" },
    Tool { name: "ffuf", required: true, why: "fuzzing routes (SecLists quickhits)", hint: "apt install ffuf" },
    Tool { name: "subfinder", required: true, why: "énumération sous-domaines", hint: "apt install subfinder" },
    Tool { name: "sqlmap", required: true, why: "SQLi (2nd-order, exploiting-sql-injection)", hint: "apt install sqlmap" },
    Tool { name: "nikto", required: true, why: "web-app-scanning-nikto", hint: "apt install nikto" },
    Tool { name: "httpx|pdhttpx", required: true, why: "probe HTTP (ACS/BugHunter)", hint: "apt install httpx sinon go install github.com/projectdiscovery/httpx/cmd/httpx@latest" },
    Tool { name: "whatweb", required: true, why: "fingerprint tech stack", hint: "apt install whatweb" },
    Tool { name: "wafw00f", required: true, why: "détection WAF (waf-bypass)", hint: "apt install wafw00f" },
    Tool { name: "sslscan", required: true, why: "ssl-tls-security-assessment", hint: "apt install sslscan" },
    Tool { name: "dnstwist", required: true, why: "typosquatting (brand)", hint: "apt install dnstwist" },
    Tool { name: "theharvester", required: true, why: "OSINT emails/hôtes", hint: "apt install theharvester" },
    Tool { name: "dnsrecon", required: true, why: "zone transfer / SRV", hint: "apt install dnsrecon" },
    // OPTIONNELS
    Tool { name: "waybackurls", required: false, why: "source-leak, SPA API discovery", hint: "go install github.com/tomnomnom/waybackurls@latest" },
    Tool { name: "gau", required: false, why: "source-leak (URLs archivées)", hint: "go install github.com/lc/gau/v2/cmd/gau@latest" },
    Tool { name: "katana", required: false, why: "crawl JS/API discovery (spa-api)", hint: "go install github.com/projectdiscovery/katana/cmd/katana@latest" },
    Tool { name: "dnsx", required: false, why: "résolution masse (web2-recon)", hint: "go install github.com/projectdiscovery/dnsx/cmd/dnsx@latest" },
    Tool { name: "jwt_tool", required: false, why: "skills JWT (none/alg confusion)", hint: "git clone https://github.com/ticarpi/jwt_tool" },
    Tool { name: "arjun", required: false, why: "shadow-api / discovery de paramètres", hint: "pipx install arjun" },
    Tool { name: "graphql-cop", required: false, why: "graphql security assessment", hint: "pipx install graphql-cop" },
    Tool { name: "burpsuite", required: false, why: "xss-burp, OOB/blind, MCP Burp", hint: "apt install burpsuite + extension MCP" },
    Tool { name: "mitmproxy", required: false, why: "proxy d'interception alternatif", hint: "apt install mitmproxy" },
    Tool { name: "amass", required: false, why: "énum sous-domaines profondeur", hint: "apt install amass" },
    Tool { name: "semgrep", required: false, why: "devsecops-security-scanning", hint: "pipx install semgrep" },
    Tool { name: "trivy", required: false, why: "container/SCA (packs optionnels)", hint: "apt install trivy" },
    Tool { name: "docker", required: false, why: "sandboxes, eval labs, packs container/cloud", hint: "apt install docker.io" },
    Tool { name: "promptfoo", required: false, why: "LLM red teaming CI (AI security)", hint: "npm install -g promptfoo" },
    Tool { name: "garak", required: false, why: "LLM probes (red-teaming-llms)", hint: "pipx install garak" },
    Tool { name: "pyrit", required: false, why: "orchestration attaques LLM", hint: "pip install pyrit" },
    Tool { name: "gf", required: false, why: "security-arsenal (patterns grep)", hint: "go install github.com/tomnomnom/gf@latest" },
];

fn python3_ok() -> bool {
    for py in ["python3", "python"] {
        if let Ok(out) = Command::new(py).arg("--version").output() {
            let s = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            if let Some(rest) = s.split("Python ").nth(1) {
                if let Some(major_minor) = rest.split('.').take(2).collect::<Vec<_>>().get(0..2) {
                    if let (Ok(maj), Ok(min)) = (major_minor[0].parse::<u32>(), major_minor[1].parse::<u32>()) {
                        return maj > 3 || (maj == 3 && min >= 9);
                    }
                }
            }
        }
    }
    false
}

pub fn run() -> i32 {
    let mut missing_required = 0usize;

    println!("=== EXIGENCES REQUISES ===");
    println!("{:<18} {:<9} {}", "OUTIL", "ÉTAT", "NOTE");
    for t in TOOLS.iter().filter(|t| t.required) {
        let (st, ok) = if t.name == "python3" {
            (if python3_ok() { "OK" } else { "MANQUANT" }, python3_ok())
        } else {
            let ok = any_in_path(t.name);
            (if ok { "OK" } else { "MANQUANT" }, ok)
        };
        if !ok {
            missing_required += 1;
        }
        println!("{:<18} {:<9} {}", t.name, st, t.why);
        if !ok {
            println!("    → {}", t.hint);
        }
    }

    println!();
    println!("=== EXIGENCES OPTIONNELLES (packs avancés) ===");
    println!("{:<18} {:<9} {}", "OUTIL", "ÉTAT", "NOTE");
    for t in TOOLS.iter().filter(|t| !t.required) {
        let ok = any_in_path(t.name);
        println!("{:<18} {:<9} {}", t.name, if ok { "OK" } else { "absent" }, t.why);
        if !ok {
            println!("    → {}", t.hint);
        }
    }

    println!();
    let seclists = ["/usr/share/seclists", "/usr/share/wordlists/seclists"]
        .iter()
        .any(|p| std::path::Path::new(p).is_dir());
    println!(
        "SecLists : {}",
        if seclists { "OK (/usr/share/seclists)" } else { "absent → apt install seclists (requis par ffuf quickhits)" }
    );

    println!();
    if missing_required == 0 {
        println!("RÉSULTAT : toutes les exigences REQUISES sont satisfaites ✔");
        0
    } else {
        println!("RÉSULTAT : {missing_required} outil(s) REQUIS manquant(s) — voir les lignes MANQUANT ci-dessus.");
        1
    }
}

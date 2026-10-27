#!/usr/bin/env bash
# install.sh — déploie le bundle mcp-intruder-skills vers un répertoire de skills
#
# Usage :
#   ./install.sh                          # → ~/.claude/skills (Tier 1 uniquement)
#   ./install.sh --target <dir>           # → répertoire personnalisé
#   ./install.sh --all                    # Tier 1 + tous les packs optionnels
#   ./install.sh --optional <groupe>      # ajoute un pack : bughunter|acs-ot-ics|acs-red-team|acs-cloud|acs-container
#   ./install.sh --dry-run                # affiche ce qui serait copié
#
# Cibles recommandées :
#   Kali (agent Claude Code / Hermes) : ~/.claude/skills
#   Windows (ZCode)                   : ~/.agents/skills  (attention : ajoute les descriptions au contexte de chaque session)
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="$HOME/.claude/skills"
DRY=0
WITH_ALL=0
OPT_GROUPS=()

while [ $# -gt 0 ]; do
  case "$1" in
    --target)   TARGET="$2"; shift 2 ;;
    --all)      WITH_ALL=1; shift ;;
    --optional) OPT_GROUPS+=("$2"); shift 2 ;;
    --dry-run)  DRY=1; shift ;;
    -h|--help)  grep '^#' "$0" | sed 's/^# \{0,1\}//' ; exit 0 ;;
    *) echo "option inconnue: $1" >&2; exit 2 ;;
  esac
done

copy_tree() { # $1=src_dir $2=dest_parent $3=name
  if [ ! -f "$1/$3/SKILL.md" ]; then echo "SKIP (pas de SKILL.md) $3" >&2; return; fi
  if [ "$DRY" -eq 1 ]; then echo "WOULD  $2/$3"; return; fi
  mkdir -p "$2"
  rm -rf "$2/$3"
  cp -r "$1/$3" "$2/$3"
}

count=0; failed=0
install_dir() { # $1=src (contient des dossiers de skills)
  local d n
  for d in "$1"/*/; do
    [ -f "$d/SKILL.md" ] || continue
    n=$(basename "$d")
    copy_tree "$1" "$TARGET" "$n" && count=$((count+1)) || failed=$((failed+1))
  done
}

echo "=== mcp-intruder-skills → $TARGET ${DRY:+(dry-run)} ==="
install_dir "$ROOT/bughunter"
install_dir "$ROOT/acs"

if [ "$WITH_ALL" -eq 1 ]; then
  for g in bughunter acs-ot-ics acs-red-team acs-cloud acs-container; do
    install_dir "$ROOT/optional/$g"
  done
fi
for g in "${OPT_GROUPS[@]:-}"; do
  [ -z "$g" ] && continue
  if [ -d "$ROOT/optional/$g" ]; then install_dir "$ROOT/optional/$g"; else echo "pack inconnu: $g" >&2; fi
done

# commands + CLI cbh (BugHunter)
if [ "$DRY" -eq 0 ]; then
  mkdir -p "$TARGET/../commands" 2>/dev/null || true
  cp -r "$ROOT/bughunter/commands/." "$TARGET/../commands/" 2>/dev/null || \
    cp -r "$ROOT/bughunter/commands" "$TARGET/_commands-bughunter"
  mkdir -p "$TARGET/_cbh-cli"
  cp -r "$ROOT/bughunter/cbh" "$TARGET/_cbh-cli/cbh"
  cp "$ROOT/bughunter/scripts/cbh.py" "$TARGET/_cbh-cli/cbh.py"
fi

if [ "$DRY" -eq 1 ]; then
  echo "dry-run terminé."
else
  n=$(find "$TARGET" -mindepth 2 -maxdepth 2 -name SKILL.md 2>/dev/null | wc -l)
  echo "Installé : $n skills dans $TARGET (échecs: $failed)"
  echo "Suivant : bash $(dirname "$0")/check_prereqs.sh  — vérifier les exigences outils"
fi

#!/usr/bin/env bash
# Push a CORAL attempt upstream to ecdsa.fail.
#
#   ./scripts/submit_attempt.sh <attempt-hash> --model "Claude Opus 4.8" [--note-file note.md] [--run-dir results/.../<ts>]
#
# Steps:
#   1. locate the run dir (latest ecdsa run unless --run-dir given)
#   2. copy src/point_add/ from the attempt's commit into seed/ (the ecdsafail clone)
#   3. `ecdsafail run` to re-score locally
#   4. `ecdsafail submit` with the note (auto-generated from the attempt if none given)
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
seed="${here}/seed"
hash="" ; model="" ; note_file="" ; run_dir="" ; dry=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --model) model="$2"; shift 2 ;;
    --note-file) note_file="$2"; shift 2 ;;
    --run-dir) run_dir="$2"; shift 2 ;;
    --dry-run) dry=1; shift ;;
    -h|--help) sed -n '2,12p' "$0"; exit 0 ;;
    *) hash="$1"; shift ;;
  esac
done
[[ -n "${hash}" && -n "${model}" ]] || { echo "usage: $0 <hash> --model \"<model>\" [--note-file f] [--run-dir d] [--dry-run]" >&2; exit 2; }
[[ -d "${seed}/.git" ]] || { echo "!! ${seed} missing; run bootstrap.sh" >&2; exit 1; }

if [[ -z "${run_dir}" ]]; then
  run_dir="$(ls -dt "${here}/results/"*ecdsa*/*/ 2>/dev/null | head -1 || true)"
  [[ -n "${run_dir}" ]] || { echo "!! no ecdsa run under results/; pass --run-dir" >&2; exit 1; }
fi
repo="${run_dir%/}/repo"
full="$(git -C "${repo}" rev-parse --verify "${hash}^{commit}")"
echo ">> attempt ${full:0:12} from ${repo}"

if [[ -n "$(git -C "${seed}" status --porcelain -- src/point_add)" ]]; then
  echo "!! seed/src/point_add has uncommitted changes; commit or discard them first" >&2
  exit 1
fi

# 2. transplant only the editable surface
rm -rf "${seed}/src/point_add"
mkdir -p "${seed}/src/point_add"
git -C "${repo}" archive "${full}" src/point_add | tar -x -C "${seed}"

# note: default to the attempt's CORAL eval message + metrics
if [[ -z "${note_file}" ]]; then
  note_file="$(mktemp -t ecdsa-note.XXXXXX).md"
  attempt_json="$(ls "${run_dir%/}"/.coral/public/attempts/"${full}"*.json 2>/dev/null | head -1 || true)"
  {
    echo "# CORAL attempt ${full:0:12}"
    echo
    echo "Produced by an autonomous CORAL agent loop (https://github.com/Human-Agent-Society/CORAL) using ${model}."
    echo
    echo "## Commit message"; echo; git -C "${repo}" log -1 --format=%B "${full}"
    if [[ -n "${attempt_json}" ]]; then
      echo; echo "## CORAL grader output"; echo; echo '```json'; cat "${attempt_json}"; echo '```'
    fi
    echo; echo "## Diff vs seed (src/point_add)"; echo; echo '```'
    git -C "${repo}" diff --stat "main...${full}" -- src/point_add
    echo '```'
    echo; echo "See \`src/point_add/memory/\` in this submission for the agent's running notes."
  } > "${note_file}"
  echo ">> wrote auto note to ${note_file} (ecdsa.fail requires >= 5 KiB; pad with --note-file if rejected)"
fi

cd "${seed}"
git add -A src/point_add
git commit -q -m "coral attempt ${full:0:12}" || true
echo ">> ecdsafail run"
ecdsafail run
if [[ "${dry}" -eq 1 ]]; then echo ">> --dry-run: skipping submit"; exit 0; fi
echo ">> ecdsafail submit"
ecdsafail submit --note-file "${note_file}" --model "${model}"

#!/usr/bin/env bash
# Populate ~/coral_ecdsafail/seed/ with the ecdsa.fail challenge repo.
#
# Prefers `ecdsafail clone` (writes the local git config that `ecdsafail submit`
# needs later, runs setup + one benchmark) and falls back to a plain git clone.
#
#   ./bootstrap.sh            # clone (or refresh) seed/
#   ./bootstrap.sh --sync     # also `ecdsafail sync` to the best promoted submission
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
seed="${here}/seed"
repo_url="https://github.com/Layr-Labs/ecdsafail-challenge"
do_sync=0
[[ "${1:-}" == "--sync" ]] && do_sync=1

if [[ -d "${seed}/.git" ]]; then
  echo ">> seed/ already exists; pulling latest main"
  git -C "${seed}" fetch -q origin
  if [[ -n "$(git -C "${seed}" status --porcelain)" ]]; then
    echo "!! seed/ has uncommitted changes; refusing to reset. Commit/stash them or rm -rf seed/." >&2
    exit 1
  fi
  git -C "${seed}" checkout -q main
  git -C "${seed}" reset -q --hard origin/main
elif command -v ecdsafail >/dev/null 2>&1 && ecdsafail config 2>/dev/null | grep -q "Token: configured"; then
  echo ">> cloning via ecdsafail (logged in) -> ${seed}"
  ecdsafail clone "${seed}"
else
  echo ">> ecdsafail CLI not logged in; plain git clone -> ${seed}"
  git clone "${repo_url}" "${seed}"
  (cd "${seed}" && bash -lc ./setup.sh)
fi

if [[ "${do_sync}" -eq 1 ]]; then
  (cd "${seed}" && ecdsafail sync)
fi

# Sanity: the pinned toolchain must be installed or every eval will fail fast.
channel="$(sed -n 's/^[[:space:]]*channel[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "${seed}/rust-toolchain" | head -1)"
if command -v rustup >/dev/null 2>&1 && ! rustup toolchain list | grep -q "^${channel}"; then
  echo ">> installing pinned Rust toolchain ${channel}"
  rustup toolchain install "${channel}" --profile minimal
fi

echo
echo "seed ready at ${seed} (HEAD $(git -C "${seed}" rev-parse --short HEAD))"
echo "next:  coral validate ~/coral_ecdsafail && coral start -c ~/coral_ecdsafail/task.yaml"

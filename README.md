# ecdsa_fail — CORAL on the ecdsa.fail challenge

Runs CORAL's autonomous agent loop against the
[ecdsa.fail](https://ecdsa.fail) secp256k1 quantum point-addition challenge
(`Layr-Labs/ecdsafail-challenge`). Same loop you run by hand with the
`ecdsafail` CLI — clone → edit `src/point_add/` → `benchmark.sh` → submit —
but with N agents, a grader daemon, shared notes, and heartbeats.

```
~/coral_ecdsafail/
├── task.yaml              # CORAL task config (direction: minimize)
├── bootstrap.sh           # clones the challenge repo into seed/ (git-ignored)
├── grader/                # runs the challenge's own ./benchmark.sh, reads score.json
└── scripts/submit_attempt.sh   # copies a winning attempt into seed/ and `ecdsafail submit`s it
```

## Setup

```bash
ecdsafail login <api-key>                  # optional but recommended (enables submit later)
./bootstrap.sh         # clone + setup.sh + pinned toolchain; --sync to start from the best promoted submission
coral validate ~/coral_ecdsafail         # grader scores the seed (~1-3 min, should print the leaderboard score)
coral start -c ~/coral_ecdsafail/task.yaml agents.count=2
```

`seed/` is a live git clone and is git-ignored here; re-run `bootstrap.sh`
any time to fast-forward it to upstream `main` (it refuses if you have local edits).

## Scoring

The grader is a thin wrapper over the challenge harness:

1. Diff the attempt against `main`; **fail** if anything outside
   `src/point_add/` changed (ecdsa.fail enforces the same `editablePaths`).
2. Symlink `target/` to `.coral/private/ecdsa_fail/cargo_target` so the
   harness crates build once per run.
3. `bash -lc ./benchmark.sh` (build → sandboxed `build_circuit` → trusted
   `eval_circuit` over 9024 shots), logs saved under `eval_logs/<hash>/`.
4. Score = `score.json.score` = toffoli × qubits. Lower is better.

`grader.args.reference_best` is only used for the "beats the leaderboard"
annotation — bump it when the frontier moves (`ecdsafail benchmark`).

## Submitting a CORAL attempt to ecdsa.fail

```bash
coral log                                                        # find the best hash
./scripts/submit_attempt.sh <hash> --model "Claude Opus 4.8" \
    [--note-file my-note.md] [--dry-run]
```

The script transplants `src/point_add/` from that commit into `seed/`,
re-runs `ecdsafail run`, and calls `ecdsafail submit`. Without `--note-file`
it drafts a note from the commit message + grader output; ecdsa.fail
requires notes ≥ 5 KiB, so for a real submission write a proper narrative
(the agent's `src/point_add/memory/` notes and `coral notes` are good raw material).

Remember ecdsa.fail only promotes submissions that beat the current best —
check `ecdsafail submissions --all` and `bootstrap.sh --sync` before a long run
so agents start from the frontier.
# quantum_circuit_autoresearch

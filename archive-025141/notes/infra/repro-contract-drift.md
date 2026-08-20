---
creator: captain-nemo
created: 2026-08-20T17:53:10+00:00
commit: n/a
type: experiment
claim: "The retained repro suite is not currently a green admission gate: its global-codec parser and verifier-ceiling fixture are stale relative to the checked-out source."
status: confirmed
evidence:
  attempt: n/a
  verified: true
touched: [src/point_add/memory/repro, src/point_add/trailmix_ludicrous/schedule.rs]
tags: [infra, reproducibility, contract-drift]
---

# Retained repro suite: source/fixture contract drift

## Context
During orientation I ran the handoff's prescribed retained check:

```sh
python3 -m unittest discover -s src/point_add/memory/repro -p 'test_*.py' -v
```

This was a local diagnostic, not a `coral eval`; there are no attempts yet.

## Result
| Eval | Mode | Outcome |
|---|---|---|
| orientation | retained repro suite | FAILED: 82 tests ran; 1 failure and 3 errors |
| orientation | exact scorer backtest | OK: 7 `results.tsv` rows checked; live frontier `1,291,859.302 × 1,154 = 1,490,805,286` |
| orientation | absolute-path release build | OK: `/home/makimakiver/.cargo/bin/cargo build --release --locked --offline` completed in 0.14 s with 3 pre-existing unused-variable warnings |
| `7f23f19ff3b3` | full 9,024-shot baseline | OK: `1,284,776.069` average executed Toffoli × `1,150` qubits; score `1,477,492,400`; 0 classical, phase, and ancilla failures |

The three errors are from `y3_global_codec.parse_schedule()`: it reads `ITERS=259` but finds 261 `SCHED_J2` entries and raises `ValueError`. The failure is `test_verifier_ceiling...`: expected `report["verdict"] == "green"`, got `"red"`.

The baseline discrepancy is now resolved for the exact current stream: full attempt `7f23f19ff3b3b5efa96847e922e5f4f41f00565a` establishes `1,477,492,400`, matching `grader.args.reference_best`. `06-research-status.md` / the exact-scorer backtest (`1,490,805,286`) and the source comment (`1,477,992,651`) are historical, non-current values. Any modified stream still requires its own fresh full evaluation; baseline evidence does not transfer across an operation-stream change.

## Mechanism
- `src/point_add/memory/06-research-status.md` identifies a promoted source and says retained tests are a re-entry gate, but the current checkout's schedule fixture has drifted from the parser's exact-length assertion.
- The codec tests therefore do not exercise their claimed terminal-tree or tape-size properties at all.
- The independent `exact_scorer.py --backtest results.tsv` validates score arithmetic and reports a historical frontier; it does not validate the current circuit stream or circuit semantics. The full trusted log for `7f23f19f` supplies that current-stream anchor.
- `cargo` is absent from the default `PATH` in this agent shell; `/home/makimakiver/.cargo/bin/cargo` exists, so callers must invoke it by absolute path or repair `PATH`.
- The repository tracks CPython 3.13 `__pycache__` artifacts under `memory/repro`; the default CPython 3.14 test invocation creates untracked 3.14 bytecode. Run retained Python diagnostics with `PYTHONDONTWRITEBYTECODE=1` to keep the worktree clean.

## What did not work
- **Full retained-suite gate** — failed before a candidate was built because 3 global-codec tests throw the schedule-length exception and one ceiling fixture reports red.
- **Plain Cargo build** — `/bin/bash: cargo: command not found`; no build evidence was obtained.

## Next
1. **Pre-eval step** — run `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s src/point_add/memory/repro -p 'test_*.py' -v` and record the current 1-failure/3-error baseline; do not call it green. Cost: under one second. Risk: none; this only identifies drift.
2. **Pre-eval step** — run `/home/makimakiver/.cargo/bin/cargo build --release --locked --offline` before a source candidate; it builds successfully in this shell. Cost: about 0.14 s warm. Risk: it catches compilation only, not 9,024-shot semantics.
3. **Upstream fix** — an owner of `src/point_add/memory/` should rebase `y3_global_codec.py` and `test_verifier_ceiling.py` fixtures to the active schedule, or explicitly mark the tests historical. This is outside my read-only controller role and must preserve source-only submission constraints.

## References
- [campaign admission](../focus/focus-campaign-admission.md) — orientation and evidence-gate context.
- `src/point_add/memory/06-research-status.md` — prescribed re-entry command and certified score.
- `src/point_add/memory/repro/y3_global_codec.py:51` — failing exact schedule-length assertion.
- `src/point_add/memory/repro/test_verifier_ceiling.py:16` — red-vs-green fixture failure.
- `src/point_add/memory/repro/exact_scorer.py --backtest results.tsv` — successful independent historical score backtest.
- `coral show 7f23f19f`; `.pi/eval_logs/7f23f19ff3b3b5efa96847e922e5f4f41f00565a/benchmark.stdout.log` — current trusted 9,024-shot baseline and all-zero failure channels.

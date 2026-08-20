---
creator: jack-aubrey
created: 2026-08-21T03:01:24+00:00
commit: 2d44cb0819c8fa2aff25d70905c0947d415d8a9e
type: experiment
claim: "TLM_TARGET_Q is forced to 1149 in src/point_add/mod.rs::build(), overriding install defaults; the 1150-qubit peak is structural and not reduced by lowering the cap further."
status: confirmed
confidence: high
evidence:
  attempt: 2d44cb0819c8fa2aff25d70905c0947d415d8a9e
  score_delta: 0 (1,477,492,400 → 1,477,492,400)
  verified: true
based_on:
  - 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
  - src/point_add/memory/05-qubit-reduction.md
  - src/point_add/memory/04-traps.md
touched:
  - src/point_add/trailmix_ludicrous/mod.rs
  - .pi/roles/jack-aubrey.md
  - .pi/notes/focus/focus-jack-aubrey-parameter-sweep.md
tags: [baseline, TLM_TARGET_Q, forced-defaults, qubit-cap, exchange-rate-trap]
---

# TLM_TARGET_Q is forced to 1149: install defaults do not control the live circuit

## Context
First real eval on this branch. Goal was to test whether the live qubit peak is sensitive to
`TLM_TARGET_Q`, following the exchange-rate-trap analysis in
`src/point_add/memory/05-qubit-reduction.md`. Baseline from `sinbad-the-sailor` attempt
`7f23f19f`: **1,477,492,400** = 1,284,776 avg executed Toffoli × 1,150 qubits, 0/9024 classical,
0/141 phase.

## Result

| Metric | Baseline | This attempt | Δ |
|---|---|---|---|
| attempt | `7f23f19f` | `2d44cb08` | — |
| avg executed Toffoli | 1,284,776 | **1,284,776** | 0 |
| peak qubits | 1,150 | **1,150** | 0 |
| score | 1,477,492,400 | **1,477,492,400** | 0 |
| classical mismatches | 0 / 9024 | 0 / 9024 | 0 |
| phase-garbage batches | 0 / 141 | 0 / 141 | 0 |
| emitted ops | 9,018,685 | 9,018,685 | 0 |

The `ops.bin` was byte-identical to the baseline. The edit to `install_q1153_submission_defaults`
was a no-op.

## Mechanism

- The live circuit is configured by **forced** `std::env::set_var` calls in
  `src/point_add/mod.rs::build()` (around line 2235), not by `install_q1153_submission_defaults`
  in `trailmix_ludicrous/mod.rs`. In particular:
  - `TLM_TARGET_Q = "1149"`
  - `TLM_SQUARE_PEAK_CAP = "1149"`
  - `TLM_APPLY_INV_CSWAP_SKIP_LAST = "2"`
  - `DIALOG_TAIL_NONCE = "9000624727621"`
  - `TLM_GAP_J2_DELTA = "2"`, `LO = "0"`, `HI = "200"`
- Because these are forced with `set_var`, editing the fallback values in
  `install_q1153_submission_defaults` has no effect unless the corresponding env var is set
  **before** `build()` runs. The grader/harness does not pre-set them, so the forced values always
  win.
- `TLM_TARGET_Q=1149` is already below the natural peak of 1,150. `target_qubit_headroom` is read
  by comparator chunk sizing, FFG vent sizing, Gidney vent sizing, and hybrid adder sizing; none of
  those decision points are constrained by the 1149 cap, so the emitted circuit is unchanged.

## What did not work

- **Editing `install_q1153_submission_defaults`.** This function is a fallback, not the source of
  truth. The real defaults are forced in `src/point_add/mod.rs::build()`.
- **Lowering the qubit cap alone.** Even when the cap is live, the exchange-rate trap documented in
  `05-qubit-reduction.md` applies: unless the persistent register set also shrinks, freed headroom
  is re-absorbed by cheaper (wider) adders.

## Surprises / open questions

- The current HEAD already forces `TLM_TARGET_Q=1149` and still peaks at 1,150. This means the cap
  is not binding; further qubit reductions must attack the persistent register set (SCHED_J2 tail
  narrowing, apply deferral, or a new representation) rather than the cap.
- The forced-defaults block also pins `DIALOG_TAIL_NONCE`, `TLM_GAP_J2_DELTA`, and other geometry
  knobs. This explains why the seed is reproducible across environments, but it also means that
  "environment-only" tuning of those knobs is impossible.

## Next

1. **Edit forced defaults in `src/point_add/mod.rs::build()`**, not in `install_q1153_submission_defaults`.
   Start with knobs that do not change the upstream op count before the q1150 action mask (line
   ~1941), e.g., tail-identity nonce or post-mask passes.
2. **Recompute or bypass the q1150 action mask** before changing geometry-affecting knobs such as
   `TLM_GAP_J2_DELTA` or `SCHED_J2`. See `eval-2-gap-j2-delta-1.md` for the crash that results when
   this is ignored.
3. **Re-test the qubit-reduction programme** now that the real control knobs are known.

## References

- attempt `7f23f19ff3b3b5efa96847e922e5f4f41f00565a` — baseline
- attempt `2d44cb0819c8fa2aff25d70905c0947d415d8a9e` — this eval
- `src/point_add/mod.rs:2230-2245` — forced live defaults
- note `src/point_add/memory/05-qubit-reduction.md` — exchange-rate trap
- note `src/point_add/memory/04-traps.md` — env-knob pitfalls
- note `.pi/notes/experiments/eval-2-gap-j2-delta-1.md` — follow-up GAP_J2 crash
- focus note `.pi/notes/focus/focus-jack-aubrey-parameter-sweep.md`

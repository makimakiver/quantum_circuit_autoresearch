---
creator: jack-aubrey
created: 2026-08-21T03:22:00+00:00
commit: c289eae7afe4e5ddac84caf8113841a247ff9059
type: experiment
claim: "Reducing TLM_GAP_J2_DELTA from 2 to 1 changes the upstream operation count and trips the pinned q1150 inverse-cswap action-mask assertion, so it is not a standalone parameter change."
status: refuted
confidence: high
evidence:
  attempt: c289eae7afe4e5ddac84caf8113841a247ff9059
  score_delta: FAILED
  verified: true
based_on:
  - 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
  - src/point_add/memory/04-traps.md
  - src/point_add/memory/05-qubit-reduction.md
touched:
  - src/point_add/mod.rs
  - .pi/notes/experiments/eval-1-target-q-1149.md
  - .pi/notes/focus/focus-jack-aubrey-parameter-sweep.md
tags: [TLM_GAP_J2_DELTA, pinned-mask, action-mask, structural-drift, tripwire]
---

# TLM_GAP_J2_DELTA=1 trips the q1150 pinned action-mask drift detector

## Context
Follow-up to `eval-1-target-q-1149`. After discovering that the live defaults are forced in
`src/point_add/mod.rs::build()` rather than in `install_q1153_submission_defaults`, I tried a
single-parameter change to the comparator narrowing knob `TLM_GAP_J2_DELTA` (2 → 1). This knob
controls how much the `GAP_J2` comparator window is narrowed over the first 200 divsteps and is
flagged in `src/point_add/memory/05-qubit-reduction.md` as part of the qubit-reduction programme.
Run in `--tune` mode.

## Result

| Metric | Baseline | TLM_GAP_J2_DELTA=1 | Δ |
|---|---|---|---|
| attempt | `7f23f19f` | `c289eae7` | — |
| status | OK | **FAILED** | — |
| build status | OK | panic at `src/point_add/mod.rs:1941` | — |
| parent ops before mask | 9,031,804 | 9,042,636 | +10,832 |
| output ops after mask | 9,018,685 | n/a | n/a |
| score | 1,477,492,400 | n/a | n/a |

`build_circuit` aborted with:
```
assertion `left == right` failed: q1150 action-mask parent operation-count drift
  left: 9042636
 right: 9031804
```

## Mechanism

- `apply_q1150_inverse_cswap_action_mask` (`src/point_add/mod.rs:1920-`) hard-codes an expected
  parent op count of **9,031,804** and an expected action mask of 16,850 lines. The mask is a
  curated list of CCX→CX downgrades and deletions discovered on a specific parent stream.
- Changing `TLM_GAP_J2_DELTA` from 2 to 1 widened the comparator window, which changes the
  number/size of comparison sub-circuits and therefore the operation count before the mask.
- This is the **occupancy tripwire** / ordinal-keyed strip issue documented in
  `src/point_add/memory/04-traps.md`: pinned certificates are keyed to an exact parent geometry;
  any upstream drift aborts the build instead of silently deleting the wrong gates.

## What did not work

- **Single-parameter schedule change without recomputing pinned masks.** The action mask is a
  cross-cutting dependency. A schedule tweak that changes op count is not a standalone tuning move.
- **Relying on `--tune` to catch geometry drift cheaply.** The grader reports that tune mode runs
  the full evaluation anyway, so the crash cost the same as a real eval. The only saving was budget
  classification.

## Surprises / open questions

- The parent-op drift is +10,832 ops, which is material. Even if the mask could be regenerated, the
  new geometry would need its own 16,850-action certificate.
- The same tripwire likely blocks other schedule-affecting knobs (`TLM_GAP_J2_HI`,
  `TLM_APPLY_INV_CSWAP_SKIP_LAST`, `TLM_TARGET_Q` if it changes adder geometry, etc.) unless the
  mask is recomputed or bypassed.

## Next

1. **Look for post-mask optimization levers** — changes that operate on the already-masked
   9,018,685-op stream or on the verifier/simulator side. These cannot drift the mask parent count.
   Candidates: tail-identity nonce, post-fanout rewrite passes, final uncompute scheduling.
   Expected payoff: small. Risk: low.
2. **Investigate whether the mask can be disabled or recomputed** — the comment says the mask is
   "part of the editable source tree," so the tooling to regenerate it may exist in the repo or in
   prior-agent notes. Expected payoff: large if it unlocks schedule tuning. Risk: high
   implementation cost; disabling it loses 16,850 optimizations (~2% Toffoli).
3. **Re-verify the forced-defaults discovery from eval-1** — update that note to point at
   `src/point_add/mod.rs::build()` as the real source of truth, since `install_q1153_submission_defaults`
   is overridden. This prevents future agents from making the same install-default no-op changes.

## References

- attempt `c289eae7afe4e5ddac84caf8113841a247ff9059` — crashed tune eval
- attempt `7f23f19ff3b3b5efa96847e922e5f4f41f00565a` — baseline
- `src/point_add/mod.rs:1920-` — `apply_q1150_inverse_cswap_action_mask`
- `src/point_add/memory/04-traps.md` — occupancy tripwire / ordinal-keyed strips
- `src/point_add/memory/05-qubit-reduction.md` — GAP_J2 narrowing programme
- `.pi/notes/experiments/eval-1-target-q-1149.md` — previous eval on this branch

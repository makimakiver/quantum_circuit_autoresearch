---
creator: sinbad-the-sailor
created: 2026-08-21T03:22:43+00:00
commit: 94e9e8fa451c28f0027dbbc8c3b4aa5b36188e95
type: experiment
claim: "Adding a diagnostic fire-census module does not change the score; the identity-keyed deep strip is currently disabled but would remove 7,660 dead and downgrade 1,872 gates if re-enabled without the q1150 action mask."
status: confirmed
confidence: high
evidence:
  attempt: 94e9e8fa451c28f0027dbbc8c3b4aa5b36188e95
  score_delta: 0
  verified: true
based_on:
  - 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
touched:
  - src/point_add/fire_census.rs
  - src/point_add/mod.rs
tags: [fire-census, tooling, deep-strip, action-mask]
---

# Added fire-census diagnostic module

## Context

Committed to the tooling-engineer lane from `focus/focus-sinbad-fire-census-reminer.md`. The first deliverable is a 64-lane classical fire-census re-miner that can be toggled with `SUB4_FIRE_CENSUS=1` and exits after printing candidate tables. The goal was to verify it builds cleanly and to probe the live stream for dead/downgradeable gates.

## Result

| Metric | Baseline | This | Δ |
|---|---|---|---|
| attempt hash | `7f23f19ff3b3...` | `94e9e8fa451c...` | — |
| score | 1,477,492,400 | 1,477,492,400 | 0 |
| avg executed Toffoli | 1,284,776 | 1,284,776 | 0 |
| peak qubits | 1,150 | 1,150 | 0 |
| emitted ops | 9,018,685 | 9,018,685 | 0 |

A local diagnostic run with `SUB4_FIRE_CENSUS=1 SUB4_FIRE_CENSUS_ROUNDS=1` reported:

- tracked 1,357,446 CCX/CCZ gates
- dead candidates with only 64 shots: 1,310,191 (spuriously high because 64 shots is far too few)

More importantly, a temporary local test that re-enabled `SUB4_APPLY_STRIP=1` showed:

- `[deep-strip-identity] removed 7660 / 10734 dead; downgraded 1872 / 2848 to CX/CZ; 4050 stale keys skipped`

So the shipped `deep_strip_keys.rs` table still has 7,660 valid dead keys and 1,872 valid downgrade keys on the live stream. Applying it changes the op count before the `q1150` action mask, so the mask assertion fires; the two cannot be used together without regenerating the mask.

## Mechanism

- The q1150 action mask is a pinned, position-keyed list of 16,850 edits (13,119 deletes + 1,512 `drop_q1` + 2,219 `drop_q2`) generated for one exact stream.
- `SUB4_APPLY_STRIP=0` is hardcoded in `build()`, disabling the identity-keyed strip that would otherwise run after constprop/fanout and before the mask.
- The identity-keyed table carries per-tuple occupancy tripwires; 4,050 keys are stale, but the remaining 9,532 keys still name valid dead/redundant gates.

## What did not work

- **1-round fire census as a dead-gate oracle.** With only 64 shots it flags ~96% of CCX/CCZ as dead; it is a candidate generator, not a verifier. More rounds or a SAT/proof step is required.
- **Blindly re-enabling the identity strip.** It saves 9,532 ops but breaks the q1150 mask, which is a hard assertion. Net score would likely worsen because the mask itself saves more ops.

## Surprises / open questions

- The identity-keyed strip is intentionally disabled, not accidentally stale. The 9,532 recoverable ops are real but cannot be combined with the existing mask without regenerating it.
- The q1150 mask subsumes some but not all of the identity-strip savings; the exact overlap is unknown.

## Next

1. **Regenerate or replace the q1150 action mask.** This is now the binding constraint. Options:
   - Write a source-level optimizer that replaces the mask (identify redundant inverse-cswap controls analytically).
   - Build a tool to re-derive the mask for a stream that includes the identity strip.
   Expected payoff: potentially ~1–2% score if both optimizations can be stacked.
2. **Improve the fire-census tool.** Make it run many more rounds efficiently, and rank candidates by survival count. Risk: compute cost; most candidates are false positives.
3. **Coordinate with captain-ahab** on controlled-addition factor-two gap; wait for their synthesis result before touching the adder bucket. Expected payoff: high if a sub-2n construction exists, but uncertain.
4. **Re-test schedule/ITERS tweaks** only after the mask can be regenerated; until then the mask blocks all geometry changes.

## References

- attempt `7f23f19ff3b3b5efa96847e922e5f4f41f00565a`: baseline verification.
- prior note: [`eval-1-baseline-verification.md`](eval-1-baseline-verification.md) — established the live baseline.
- focus note: [`focus-sinbad-fire-census-reminer.md`](../focus/focus-sinbad-fire-census-reminer.md) — tooling lane.

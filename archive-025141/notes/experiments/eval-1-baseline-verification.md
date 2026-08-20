---
creator: sinbad-the-sailor
created: 2026-08-21T02:57:56+00:00
commit: 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
type: experiment
claim: "The promoted seed circuit reproduces a clean 1150-qubit / 1.284M-Toffoli baseline and is the current leaderboard best."
status: confirmed
confidence: high
evidence:
  attempt: 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
  score_delta: 0
  verified: true
based_on: []
touched: []
tags: [baseline, audit, verification]
---

# Verified promoted seed baseline: score 1,477,492,400

## Context

First action after arrival was to audit the current promoted seed against the official scorer rather than relying on `memory/` prose. Ran a full real-mode `coral eval` on the unmodified repository (commit `31f9c58`, reset to the promoted seed). No source edits. Goal: establish the exact artifact identity and score that every subsequent claim must beat.

## Result

| Metric | Value |
|---|---|
| attempt hash | `7f23f19ff3b3b5efa96847e922e5f4f41f00565a` |
| commit | `7f23f19ff3b3` |
| avg executed Toffoli | 1,284,776 |
| peak qubits | 1,150 |
| score | **1,477,492,400** |
| classical mismatches | 0 / 9024 |
| phase-garbage batches | 0 / 141 |
| ancilla-garbage batches | 0 / 141 |

The verifier reported `improved` because this is the first attempt on this branch; it is effectively a tie with the current leaderboard best at 1.000×.

## Mechanism

- The seed is already on the narrow-160 / cap-1151 programme described in `src/point_add/memory/05-qubit-reduction.md` Step 5, with the occupancy tripwire active. Peak qubits are 1,150, not 1,154, so the baseline has moved down since the `1,490,805,286` figure quoted in `06-research-status.md`.
- Constprop removes 208 Toffoli in three iterations; the final `ops.bin` is byte-identical to the local `cargo run --release` artifact, confirming the grader and local build agree.
- No stale keys are reported in this build, so the shipped stream is already re-mined relative to the tripwire.

## What did not work

- **N/A** — this was a pure verification run. No alternative was tested.

## Surprises / open questions

- The live baseline (1,477,492,400) is already ~1.0% below the 1.48e9 target and ~13.3M below the `1,490,805,286` certified value in `06-research-status.md`. Either the seed advanced, or the certified value was rounded/measured under a different tripwire state. Treat this live score as the true target to beat.
- The other two active agents are occupying reviewer (`focus-campaign-admission.md`) and controlled-addition research (`focus-captain-ahab-controlled-add-gap.md`) lanes. Engineering headroom is likely in (a) joint-codec exact-eight synthesis, (b) further SCHED_J2 narrowing with a matching cap cut, or (c) a fresh qubit-programme search now that the tripwire is trusted.

## Next

1. **Re-mine the live stream for source implications / CCX→CX downgrades** — `memory/03-proven-floors.md` reports 2,050 downgrades were found on a prior re-mine vs 1,193 shipped; the current build may not have those. Expected payoff: up to ~0.05% score if downgrades are deterministic and not yet applied. Risk: may overlap with existing constprop / source-implication machinery.
2. **SCHED_J2 tail narrowing with a matching cap reduction** — `memory/05-qubit-reduction.md` found −0.49% at N=160 / q=1151, but λ rose to ~9.7. With a clean nonce grind, a slightly larger N might trade a small Toffoli gain for acceptable λ. Risk: λ is the real binding constraint; this is a grind-heavy path.
3. **Unrestricted exact-eight joint codec synthesis** — highest theoretical leverage, but requires SAT witnesses and compiled replay. Risk: very high implementation cost; complementary to the controlled-addition lane already taken.
4. **Fresh controlled-modular-addition decomposition** — wait for `captain-ahab` research result; do not duplicate.

## References

- prior note: [`src/point_add/memory/06-research-status.md`](../../../src/point_add/memory/06-research-status.md) — certified baseline claim; live score is 13.3M lower.
- prior note: [`src/point_add/memory/05-qubit-reduction.md`](../../../src/point_add/memory/05-qubit-reduction.md) — qubit programme history and the exchange-rate trap.
- prior note: [`src/point_add/memory/04-traps.md`](../../../src/point_add/memory/04-traps.md) — env-knob and tripwire pitfalls.
- focus note: [`.pi/notes/focus/focus-campaign-admission.md`](../focus/focus-campaign-admission.md)
- focus note: [`.pi/notes/focus/focus-captain-ahab-controlled-add-gap.md`](../focus/focus-captain-ahab-controlled-add-gap.md)

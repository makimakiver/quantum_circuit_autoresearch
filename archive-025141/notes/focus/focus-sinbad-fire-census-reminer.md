---
creator: sinbad-the-sailor
created: 2026-08-21T03:05:43+00:00
generation: 1
type: hypothesis
claim: "A fresh identity-keyed fire census against the current ITERS=259 stream will find dead/downgradeable gates that the shipped tables miss because they are stale."
status: untested
confidence: medium
tags: [fire-census, deep-strip, tooling, q1150-mask]
---

# Focus: fire-census re-miner for the live stream

## Posture
Tooling engineer / performance engineer. Build a reusable census tool and diagnostic scripts that the rest of the team can run on any geometry; ship score improvements only after the tool proves they are exact.

## Lane
Re-mine the live emitted op stream for never-firing CCX/CCZ gates and CCX→CX/CZ downgrades, producing new identity-keyed tables. Complements the controlled-addition research lane (captain-ahab) and the review gate (captain-nemo); does not duplicate them.

## Budget
3 real evals on the current geometry once a table is generated: one to measure the raw gate savings, one to verify 9024/9024 cleanliness, and one after any refinements. Tooling iterations (partial runs, smoke tests) do not count against the budget.

## Abandon-if
- A fresh census over ≥1e8 inputs finds zero new dead gates and zero new downgrades beyond the existing D2 positional strip.
- The q1150 inverse-cswap action mask is the only surface with remaining wins, because regenerating it is out of scope for this lane.
- Any table that passes the census fails the 9024-shot harness (indicating a tripwire or occupancy bug).

## Why this has positive EV
- `src/point_add/memory/05-qubit-reduction.md` Step 6 reports ~6,241 stale keys discarded by the tripwire on a prior stream; the current `deep_strip_keys.rs` table appears to be 100% stale on the live build (no identity-strip log line, early return with empty dead/down).
- The D2 positional strip still removes 1,999 gates, so the concept is sound; an identity-keyed table matched to the current stream should remove at least as many without the positional fragility.
- The prior workstream found 1,193 → 2,050 downgrades on a re-mine (`memory/03-proven-floors.md`), demonstrating the lever exists.
- The live circuit is pinned to ITERS=259 by the q1150 action mask; schedule exploration requires either a mask regeneration or abandoning the mask, both expensive. Census re-mining is a cheaper exact lever that does not touch schedules.
- A tooling deliverable (a script under `src/point_add/` that emits a `deep_strip_keys.rs`-format table) is reusable across future geometry changes and helps other agents avoid hand-mining.

## Update history
- 2026-08-21T03:05:43+00:00: created after verifying the promoted seed and discovering the identity-keyed strip is silent on the live stream.

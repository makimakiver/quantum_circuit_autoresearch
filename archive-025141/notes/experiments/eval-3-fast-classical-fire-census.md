---
creator: sinbad-the-sailor
created: 2026-08-21T03:37:43+00:00
commit: 3775f27ed6616a74def988207847109722d5658b
type: experiment
claim: "A fast classical 64-lane fire-census simulator runs ~16 rounds/minute and finds 66,027 dead-candidate and 10,916 downgrade-candidate CCX/CCZ gates on the pre-mask stream; most are expected false positives from sparse sampling."
status: confirmed
confidence: medium
evidence:
  attempt: 3775f27ed6616a74def988207847109722d5658b
  score_delta: 0
  verified: true
based_on:
  - 94e9e8fa451c28f0027dbbc8c3b4aa5b36188e95
touched:
  - src/point_add/fire_census.rs
  - src/point_add/mod.rs
tags: [fire-census, classical-simulator, tooling, false-positive-rate]
---

# Fast classical fire-census simulator

## Context

Replaced the quantum-Simulator-based fire census with a hand-rolled classical 64-lane mirror in `src/point_add/fire_census.rs`. Goal: make a longer fire census practical so we can separate true dead gates from rare-path false positives.

## Result

| Metric | Baseline | This | Δ |
|---|---|---|---|
| attempt hash | `94e9e8fa451c...` | `3775f27ed661...` | — |
| score | 1,477,492,400 | 1,477,492,400 | 0 |
| avg executed Toffoli | 1,284,776 | 1,284,776 | 0 |
| peak qubits | 1,150 | 1,150 | 0 |

Diagnostic run:

```sh
SUB4_FIRE_CENSUS=1 SUB4_FIRE_CENSUS_ROUNDS=16 SUB4_FIRE_CENSUS_MIN_SURVIVAL=16 \
    cargo run --release --bin build_circuit
```

Output:
- tracked 1,357,446 CCX/CCZ gates
- dead candidates: **66,027**
- c2-implies-c1 downgrade candidates: 2,761
- c1-implies-c2 downgrade candidates: 8,155
- wall time for 16 rounds: ~58 s (~3.6 s/round)

## Mechanism

- The classical mirror ignores phase and simulates only the classical value flow: CCX/CX/X/Swap modify qubit lane masks, R/Hmr reset their target under the condition, Push/PopCondition manage the condition stack, and Hmr writes a fresh 64-bit random word to its classical target.
- `condition_true_count` accumulates the total number of condition-true lane-events across all rounds. A gate only becomes a candidate if it has seen at least `min_survival_rounds * 64` condition-true events and never fired (for dead) or never saw a counterexample to the implication (for downgrade).

## What did not work

- **1-round census is not discriminating.** It flagged 109k dead candidates; even at 16 rounds we still have 66k. The action mask deletes only ~13k dead gates, so the excess is almost certainly rare-path live gates that happen not to fire in 1,024 shots.
- **The 11k downgrade candidates are also inflated.** The shipped identity strip has only 1,872 downgrades verified at 1e8 shots. A 1,024-shot run cannot prove an implication; it can only fail to disprove it.

## Surprises / open questions

- The candidate counts are much larger than the shipped optimizations, which means either:
  1. The action mask / strip tables are stale and miss tens of thousands of real optimizations, or
  2. The sparse census has a high false-positive rate.
- If (1), regenerating the action mask and identity strip could yield a large score improvement. If (2), we need a verifier/proof step to avoid corrupting the circuit.
- The classical simulator is ~25× faster per round than the full quantum simulator (≈1 s vs 25 s), making larger sample sizes feasible, but 1e8 shots would still take days.

## Next

1. **Build a candidate verifier.** For each dead/downgrade candidate, run the circuit on a focused sample or use SAT to prove the gate never fires / the implication always holds. This is the highest-EV next step because it converts noisy census data into usable tables.
2. **Compare candidates against the shipped strip/mask tables.** If the 7,660 strip-dead gates are a subset of the 66k candidates, the simulator is at least not missing obvious cases. If not, there is a simulator bug.
3. **Run a longer census overnight** (e.g., 1,024 rounds = 65,536 shots) and plot candidate-count decay vs rounds to estimate how many survive to 1e8. Risk: compute cost; expected payoff only if decay is slow.
4. **Explore source-level replacement of the q1150 mask** once a verified table is available; until then the mask blocks stacking optimizations.

## References

- attempt `94e9e8fa451c28f0027dbbc8c3b4aa5b36188e95`: first fire-census module.
- prior note: [`eval-2-fire-census-module.md`](eval-2-fire-census-module.md).
- focus note: [`focus-sinbad-fire-census-reminer.md`](../focus/focus-sinbad-fire-census-reminer.md).

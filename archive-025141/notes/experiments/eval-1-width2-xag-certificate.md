---
creator: captain-ahab
created: 2026-08-21T00:00:00+00:00
commit: f9eb386ed653
type: experiment
claim: "Retaining the width-two controlled-add XAG certificate changes no live circuit operations and therefore preserves the reference score exactly."
status: confirmed
confidence: high
evidence:
  attempt: f9eb386ed653
  score_delta: "1,477,492,400 -> 1,477,492,400; 0"
  verified: true
based_on: ["src/point_add/memory/03-proven-floors.md"]
touched: ["src/point_add/memory/06-research-status.md", "src/point_add/memory/repro/y7_controlled_add_xag.py", "src/point_add/memory/repro/test_y7_controlled_add_xag.py"]
tags: [controlled-addition, xag, research-artifact, no-live-circuit-change]
---

# Retained width-two XAG certificate: score unchanged at 1,477,492,400

## Context

This real evaluation commits the bounded structural research artifact from [the controlled-add focus](../focus/focus-captain-ahab-controlled-add-gap.md): a deterministic exhaustive two-AND XAG obstruction for the width-two, no-carry controlled-add map. It deliberately edits only `src/point_add/memory/`; the live `point_add::build()` route is untouched.

## Result

| Metric | Baseline | This | Δ |
|---|---:|---:|---:|
| Score | 1,477,492,400 | 1,477,492,400 | 0 |
| Executed Toffoli | 1,284,776 | 1,284,776 | 0 |
| Qubits | 1,150 | 1,150 | 0 |
| Two-AND width-two witnesses | n/a | 0 / 9,472 second-AND candidates | research result |

**score: 1,477,492,400**

The evaluator accepted the exact reference score. The local checker prints `compatible_first_and_functions=4`, `second_and_functions_examined=9472`, and `two_and_witnesses=0`; its unit test passes.

## Mechanism

- `y7_controlled_add_xag.py` represents all five-input Boolean functions as 32-bit truth tables and quotients out affine functions with exact GF(2) elimination.
- A two-AND XAG would have to span the two independent non-affine output components. Its first AND is restricted by exhaustive enumeration to one of four functions equal to `c*a0` modulo affine terms.
- Enumerating every affine-input second AND reachable from each such first function finds no function that supplies the remaining output direction, proving the three-CCX threaded cost tight only for this width/model.
- The script and its test live under `memory/repro`, so the result travels with future submissions without affecting circuit emission.

## What did not work

- **A score optimization** — no live emitter path was changed by design; the exact zero delta is expected, not evidence that controlled-addition has no larger-width headroom. Attempt `f9eb386ed653`.
- **SAT-backed width-three synthesis** — neither `kissat` nor `cadical` is available in this environment. The retained `y1_composite_synth.py` unit tests pass, but no solver verdict exists for width three.
- **Treating width two as a scalable lower bound** — the certificate is explicitly only a two-AND XAG obstruction for no-carry width two. It does not cover carry-out variants or arbitrary measurement protocols.

## Surprises / open questions

- The evaluator's established reference is `1,477,492,400`, lower than the `1,490,805,286` historical value documented in the retained research status. The latter must remain labelled historical; this attempt is the current stream evidence.
- The first useful unresolved instance is no-carry width three, whose live target is five CCX. A positive four-AND witness would be actionable only after it is compiled with exact clean ancilla and phase cleanup.

## Next

1. **Width-three exact XOR/AND search** — run a pinned SAT solver against a truth-table CNF for `<5` ANDs, then independently replay any witness. Expected payoff: directly falsifies the local `2n-1` family at its first nontrivial width. Risk: a Boolean witness may not admit the live HMR cleanup schedule.
2. **Carry-out width-two/three certificates** — model the separate `2s` baseline rather than reusing no-carry counts. Expected payoff: prevents false comparisons in chunked callers. Risk: output carry increases synthesis cost.
3. **Call-site-restricted map audit** — exploit actual GCD state invariants if generic width-three search is UNSAT. Expected payoff: may reduce only the high-frequency production subfamily. Risk: restrictions must be source-indexed and preserve replay direction.

## References

- attempt `f9eb386ed653`: exact baseline-score evaluation that retained the research artifact.
- [controlled-add focus](../focus/focus-captain-ahab-controlled-add-gap.md) — scope, invariant, and abandonment condition.
- [low-width ANF audit](../research/controlled-add/low-width-anf.md) — derivation and bounded-model caveat.
- [captured live source](../raw/controlled-add-live-source-2026-08-21.md) — live threaded-adder cost accounting.
- `src/point_add/memory/03-proven-floors.md` — recorded open factor-two controlled-addition gap.

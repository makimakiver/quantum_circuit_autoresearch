---
creator: jack-aubrey
created: 2026-08-20T18:00:23+00:00
generation: 1
type: hypothesis
claim: "The current HEAD parameter set is near a local basin, but small reachable knobs (fold vents, FFG max g, schedule deltas, target-q headroom) still hide 0.5-2% score headroom before touching core arithmetic."
status: untested
confidence: medium
tags: [parameter-sweep, qubit-headroom, fold-schedule, gcd-schedule]
---

# Focus: live-parameter sweep around the current HEAD

## Posture
Engineer / measured-parameter explorer. The team currently has no official attempts, so producing
score-bearing evals and recording baselines is the highest-EV posture.

## Lane
Sweep the environment knobs that `src/point_add/trailmix_ludicrous/mod.rs` reads, prioritising
low-risk single-parameter moves that do not change the circuit semantics (headroom targets, vent
widths, schedule offsets) and can be reversed in one line.

## Budget
6 real evals before judging whether the basin is exhausted:
- 1 baseline (current HEAD)
- 2-3 single-knob sweeps (TLM_TARGET_Q, LUD_EXTRA_FOLD_VENTS, FFG schedule)
- 1-2 combined knobs if a promising direction appears
- 1 revert/confirm if needed

## Abandon-if
- No real-mode score improves over 1,477,492,479 after 6 evals.
- Any promising direction increases classical mismatches beyond the intrinsic band (~10-30).
- Another agent publishes a structural idea with a higher measured or theoretically-projected EV.

## Why this has positive EV
- `src/point_add/memory/05-qubit-reduction.md` measured a 0.49% proxy win from narrowing the
  `SCHED_J2` tail and lowering the cap together, but the final shipped point was never re-mined and
  the tripwire tooling was lost. The current code may have absorbed part of this win, but the
  parameter space (tail width, cap, fold vents) has not been exhaustively searched on the current
  stream.
- `TLM_TARGET_Q` directly trades peak qubits against adder cost; the memory shows the trade-off is
  sharp but that the exchange-rate trap is avoidable when the cap and the persistent set move
  together.
- These sweeps are cheap to implement and fully reversible, so the downside is bounded to eval time.

## Update history
- 2026-08-20T18:00:23+00:00: created

# Notes Index

## Research
- [low-width controlled-add ANF](research/controlled-add/low-width-anf.md) — derives the exact live no-carry targets and separates the pending exact certificate from the width-three search.

## Experiments
- [baseline anchor](experiments/eval-1-baseline-anchor.md) — seed tree == platform record 1,477,492,400; prose score values in tree are stale.
- [eval 1 — width-two XAG certificate](experiments/eval-1-width2-xag-certificate.md) — committed deterministic two-AND obstruction; live score unchanged.
- [baseline verification](experiments/eval-1-baseline-verification.md) — verified promoted seed at score 1,477,492,400 (1150 qubits, 1.284M Toffoli).
- [TLM_TARGET_Q forced-defaults](experiments/eval-1-target-q-1149.md) — install defaults are overridden by forced defaults in `src/point_add/mod.rs::build()`.
- [TLM_GAP_J2_DELTA 1](experiments/eval-2-gap-j2-delta-1.md) — schedule narrowing trips the pinned q1150 action-mask drift detector; not a standalone change.
- [fast classical fire_census](experiments/eval-3-fast-classical-fire-census.md) — 64-lane classical simulator; 66k dead / 11k downgrade candidates at 1,024 shots, likely dominated by false positives.
- [fire-census module](experiments/eval-2-fire-census-module.md) — added diagnostic re-miner; identity strip disabled by q1150 mask, 9,532 ops recoverable if mask is regenerated.

## Infrastructure
- [retained repro contract drift](infra/repro-contract-drift.md) — the prescribed repro suite currently has three parser errors and one stale-fixture failure.

## Synthesis
- [current admission state](_synthesis/current-admission-state.md) — mask composition and lambda are the gates before any score candidate or nonce grind.

## Focus
- [campaign admission](focus/focus-campaign-admission.md) — read-only review gate and routing for the first candidate.
- [controlled-add factor-two gap](focus/focus-captain-ahab-controlled-add-gap.md) — bounded exact-synthesis research plan for the live controlled clean-adder.
- [fire-census re-miner](focus/focus-sinbad-fire-census-reminer.md) — tooling to re-mine dead/downgradeable gates on the live ITERS=259 stream.
- [parameter sweep around current HEAD](focus/focus-jack-aubrey-parameter-sweep.md) — measured knob search starting from the promoted seed.

## Open Questions
- Controlled-add factor-two gap: characterize and synthesize small-width alternatives.
- Can unrestricted eight-shear joint codec synthesis produce a compiled reversible witness?
- Can the q1150 positional action mask be regenerated or replaced so it composes with the live identity-keyed strip and permits geometry changes?

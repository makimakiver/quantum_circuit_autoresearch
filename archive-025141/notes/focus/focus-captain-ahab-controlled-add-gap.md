---
creator: captain-ahab
created: 2026-08-20T17:53:11+00:00
generation: 1
type: hypothesis
claim: "Exact small-width synthesis can determine whether a clean-ancilla/measurement controlled-adder family beats the current 2n CCX construction before any production rewrite."
status: untested
confidence: medium
tags: [controlled-addition, reversible-synthesis, structural]
---

# Focus: controlled-add factor-two gap

## Posture
Researcher. I will deliver an exact, bounded candidate specification rather than touch scored source or run `coral eval`.

## Lane
Controlled quantum addition `|c,a,b,0...⟩ → |c,a,b + c·a,0...⟩`, specifically whether the current threaded clean-carry construction's approximately `2n` CCX is an avoidable decomposition cost.

## Budget
Three bounded synthesis passes: (1) extract and truth-table the exact no-carry-out primitive for widths 2–5; (2) synthesize with clean ancillae and the HMR/conditional-CZ erase identity represented as a semantic primitive; (3) compile any sub-`2n−O(1)` witness into an implementer-ready forward/cleanup schedule. No production evals are part of this research lane.

## Abandon-if
Park the lane if exact widths through 5 show no witness below the threaded primitive after normalizing for the same input/output and clean-ancilla contract, or if every apparent gain relies on an uncleared quantum ancilla, a postselection assumption, or a measurement-conditioned operation unavailable in `B`.

## Why this has positive EV
- [`src/point_add/memory/03-proven-floors.md`](../../../src/point_add/memory/03-proven-floors.md) proves only an `n` CCX lower bound while the live controlled construction is about `2n`; this is the largest explicitly open multiplicative-complexity factor in the measured budget.
- The live primitive in [`gidney.rs`](../../../src/point_add/trailmix_ludicrous/gidney.rs) computes each carry with one CCX, then injects each controlled sum bit with one CCX. Its reverse carry erase is measurement based, so a candidate must preserve that exact cleanup advantage rather than compare against a unitary-only circuit.
- The controlled-permutation and codec neighborhoods are already tightly scoped or locally closed; this lane is complementary and has an inexpensive falsifier at low width.

## Initial invariant and candidate boundary
The candidate must preserve `c` and `a`, transform `b` by addition modulo `2^n` when `c=1`, leave `b` unchanged when `c=0`, return all quantum work wires to zero, and leave no relative/global phase. Classical measurement records may only be used through the existing `Hmr` plus classically conditioned phase repair pattern.

The first comparison target is `controlled_clean_add_threaded` in `gidney.rs:1187`. For a width `s` with a carry-out, it allocates `s` carry wires, emits one carry-producing CCX per bit, one controlled-sum CCX per bit, and emits one further controlled CCX into `cout`; cleanup uses HMR plus conditional CZ/CCZ. The live local target is therefore `2s + 1` CCX (and `2s - 1` without carry-out), excluding chunk-boundary comparators and downstream empirical gate strips.

## Update history
- 2026-08-21: claimed after orientation; no production source modification or score claim.
- 2026-08-21: completed passes 1–2 at width two in [low-width ANF audit](../research/controlled-add/low-width-anf.md). The true local baseline is `2s-1` without, and `2s+1` with, carry-out; the earlier `2s` statement was an accounting error corrected from the live loop structure. A deterministic enumeration proves no two-AND XAG implementation for the two-bit no-carry map (3 CCX is tight in that model). `kissat` and `cadical` are absent, so width-three remains blocked rather than inferred.

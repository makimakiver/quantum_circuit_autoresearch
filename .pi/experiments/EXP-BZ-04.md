# EXP-BZ-04 — cdouble s2-chain boundary trim, verify-only (STRIP-adjacent, small)

Pinned: commit a2067dcf, ops.bin 6519bd01…; fused.rs:1892-1900 (fwd chain), 1944-1956 (rev chain).

1. **experiment_id**: EXP-BZ-04
2. **search_radius**: the conditional-rotation chains `cswap(s2, w[i], w[i-1])` for i in 1..258 (257 gates/call,
   0% discount, ~67k emitted of the 167,958 apply-fold total). Specifically the top-of-chain entries where one operand
   is a fresh |0> overflow qubit (hi/hi2, allocated at fused.rs:1871-1872) and the existing
   `skip_structural_dead_fused_cdouble_shift0` bottom-skip (bit 0).
3. **mechanism**: a Fredkin whose one operand is a known-zero fresh ancilla decomposes with one dead CCX control; the
   constprop pass (`ccx_final_cancel`, constprop.rs) should already strip these. This card only VERIFIES that the strip
   stack actually removes them at a2067dcf (the census-mined certificates were retired at ITERS!=BAKED_ITERS per
   schedule.rs baked_artifacts_valid, so some previously-stripped gates may now survive).
4. **predicted direction**: T 0 to −500 executed if any top-of-chain gates survive the post-pass; Q 0; lambda 0
   (identity-preserving strips only).
5. **cheapest_discriminator**: count-only probe with `TLM_ADD_CONST_SKIP_STRUCTURAL_DEAD_CARRIES`-style toggles is not
   applicable here; use POINT_ADD_COUNT_ONLY=1 plus a grep of the constprop stats output (`folded_cx/dropped` counters,
   constprop.rs:1186-1216) for the fold call indices in question.
6. **predeclared_falsifier**: constprop stats show the top-of-chain cswap CCX already cancelled (no surviving gates)
   — card closes as already-done.
7. **evidence_debt**: none; verification only.
8. **trial_budget / time_budget**: 30 min.
9. **unresolved_directions**: a *census-based* re-mine of never-firing s2-chain gates (the 1,290-key fire-census of
   03-proven-floors.md) is a separate strip-lane effort, not this card.
10. **reopening_trigger**: any change to fused.rs chain order or overflow-bit allocation reopens the verification.
11. **reproduction_commands**: `CARGO_TARGET_DIR=/tmp/bezout-scout-target POINT_ADD_COUNT_ONLY=1 cargo run --release --bin build_circuit`
    and compare op count vs 8,958,690 baseline after toggling `TLM_FOLD_RELEASE_CONTROLS` (mod.rs:2222 default 1) off/on.
12. **classification**: STRIP (verification of existing post-pass coverage; no new mechanism).

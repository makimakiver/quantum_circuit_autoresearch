# EXP-BZ-02 — Deferred reduction: absorb the mod_add clean/fold into the cdouble fold (STRUCTURAL, downgraded)

Pinned: commit a2067dcf, ops.bin 6519bd01…, apply-side ledger (executed): fwd reg 147,799.0 / inv reg 148,392.5 /
mod_add folds 29,717.5 / cleans 4,943.0 (50% discount) / cswaps 132,096.0 (0%) / cdouble folds 165,741.5.

1. **experiment_id**: EXP-BZ-02
2. **search_radius**: in `apply_step_forward` (gcd.rs:1740-1776) and `apply_step_reverse` (gcd.rs:1778-1814), delete the
   post-add reduction stages `add_f_window`+`controlled_lt_msbs_conditional` (fwd, arith.rs:1513-1536) and their inverse
   twins in `controlled_mod_sub_vented` (gcd.rs:1816-1860), letting the *next* `fused_double_cdouble` /
   `fused_double_cdouble_reverse` (fused.rs:1865/1921) fold absorb a 3-overflow-bit, 7f-wide addend. Paired change in
   both directions mandatory (M^-T inverse pairing).
3. **mechanism**: today each iteration reduces y twice (once after the add, once after the doubling). The strong version
   of this idea (defer the add's *fold window* too) is FALSIFIED at scout time: the `cswap(swp, x, y)` between add and
   doubling would have to swap a 257-bit (y|cout) pair, costing +256 full-price Toffoli/iteration/direction — more than
   the ~66.7 saved. The surviving weak version defers only the 19-bit MSBS clean (`mod_add_clean` phases, 4,943 executed
   at exactly 50% discount = single hmr condition) into the cdouble fold's existing window machinery, worth roughly
   −10..−15 executed/iteration/direction ≈ −6k executed total ≈ −0.5% T. The cdouble fold gains 1-2 conditional
   subtract terms (wider addend, up to 7f vs 3f: 36 vs 35 bits, both inside the LSBS=53 window).
4. **predicted direction**: T −5k..−8k executed (weak version) with Q unchanged (transient overflow bits already exist:
   hi/hi2 in fused.rs:1871-1872; may need one more). lambda: window carry-escape term (2.18mm at 258-era) grows with the
   wider addend — expect +0.5..+2.5mm; the 50%-discounted clean phases being removed were also the width-growth guard,
   so a value-bounds proof is part of implementation.
5. **cheapest_discriminator**: POINT_ADD_COUNT_ONLY=1 build diff (expect ops −3k..−6k emitted) plus TRACE_TLM_TOF
   phase deltas on `tlm_apply_forward_mod_add_clean`/`tlm_apply_inverse_mod_sub_clean` (must go to ~0) and
   `tlm_apply_*_fold` (must not grow by more than the clean saving).
6. **predeclared_falsifier**: (a) the count-only probe shows fold-phase growth >= clean-phase removal (win evaporates);
   (b) value-bounds analysis shows the post-fold residual can exceed 2^256 entering the next iteration's 256-bit cout
   adder (correctness break — the reason the clean exists); (c) dirty-scan classical mismatches rise > 1 sigma over n=12.
7. **evidence_debt**: no measurement exists of the post-fold value distribution tail (how close to 2^256 the cdouble
   output actually runs) — the bounds proof or a 1e6-sample classical emulation must repay this before implementation
   is trusted; promotion gate = four checks green + paired lambda n>=12.
8. **trial_budget / time_budget**: 2 days implementer time for the paired fwd/inv restructure + proofs; 1 h probes.
   Hard cap: one falsification cycle — if (a) or (b) fires, close as LOCAL_PLATEAU and hand the folded-adder idea
   (merging the f-addend into the adder's low chunk) to the shared-primitives cell, where the adder internals live.
9. **unresolved_directions**: does not decide the folded-adder merge (adder internals = shared-primitive lane); does not
   decide whether the inverse side's `compare_geq_chunked_middle` clean can be replaced by the fwd-side
   `controlled_lt_msbs_conditional` shape (asymmetric cleanup machinery — needs its own card if count-only shows a gap).
10. **reopening_trigger**: any change to LSBS (fold-window width) or to the cdouble fold internals reopens; a measured
    value distribution with >=8 bits of headroom below 2^256 reopens the strong version (which is otherwise dead by the
    257-bit cswap argument).
11. **reproduction_commands**: baseline `CARGO_TARGET_DIR=/tmp/bezout-scout-target TRACE_TLM_TOF=1 cargo run --release --bin build_circuit`
    — cite phases `tlm_apply_forward_mod_add_clean emitted=4943 expected=2471.5 discount=0.500` and
    `tlm_apply_inverse_mod_sub_clean emitted=4943 expected=2471.5 discount=0.500` as the removal targets.
12. **classification**: STRUCTURAL (T-primary, small lambda exposure) — weak variant only; strong variant closed by
    scout-time falsifier (257-bit cswap cost).

# EXP-ARCH-03 — Deferred-reduction weak composite with fold-boundary value proof (B3×P×λ-model; promotion form of EXP-BZ-02)

Status: SCOPED-COMPOSITE. Inherits the strong-version falsification (257-bit cswap, +256/iter/dir) from
EXP-BZ-02; only the weak variant (defer the 19-bit MSBS clean into the cdouble fold) is carded. Pinned
a2067dcf / ops.bin 6519bd01… (no new measurement in the finishing session).

1. experiment_id: EXP-ARCH-03
2. search_radius: delete `add_f_window`+`controlled_lt_msbs_conditional` clean stages in
   `apply_step_forward` (gcd.rs:1756) and their inverse twins in `controlled_mod_sub_vented`
   (gcd.rs:1844)/`apply_step_reverse` (gcd.rs:1804), letting the next `fused_double_cdouble` /
   `fused_double_cdouble_reverse` (fused.rs:1865/1921) absorb a wider (up to 7f vs 3f; 36 vs 35 bits, both
   inside LSBS=53) addend. Paired fwd/inv change mandatory (M^-T inverse pairing).
3. mechanism: EXP-BZ-02 §3. Composite addition: the fold-boundary VALUE PROOF is promoted to a first-class
   stage (the clean being removed is the width-growth guard for the next iteration's 256-bit cout adder);
   and the λ gate is wired to EXP-ARCH-01 stage-0's baseline (shared nonce sets, shared σ).
4. predicted direction: T −5k..−8k executed (ΔS −5.8M..−9.2M at Q=1154); Q 0..+1 (transient overflow bits
   hi/hi2 already exist at fused.rs; one extra live bit at a plateau window costs +1 Q = 1,112 executed
   break-even — Q-guard census mandatory); λ +0.5..+2.5mm (window carry-escape term grows; 258-era reference
   2.18mm) — must be quoted against the stage-0 baseline and priced into grind effort.
5. cheapest_discriminator: `POINT_ADD_COUNT_ONLY=1` probe — `tlm_apply_forward_mod_add_clean` /
   `tlm_apply_inverse_mod_sub_clean` counters (4,943 emitted each, 50% discount, 2,471.5 executed each side)
   must go to ~0 while `tlm_apply_*_fold` grows by strictly less than the removal.
6. predeclared_falsifier: (a) fold growth ≥ clean removal (win evaporates); (b) value-bounds proof or 1e6-sample
   classical emulation shows post-fold residual can reach 2^256 entering the next cout adder (correctness
   break — the reason the clean exists); (c) paired dirty-scan λ worse >1σ over n≥12; (d) Q-guard breach.
7. evidence_debt: post-fold value-distribution tail unmeasured (repaid by the proof stage); λ movement
   unmeasured until stage-0 baseline exists + promotion paired scan; strip re-key unmeasured.
8. trial_budget / time_budget: proof stage 0.5–1 day; 2 days implementer for the paired restructure; 1 h
   probes; one falsification cycle max. If (a) or (b) fires → LOCAL_PLATEAU, hand the folded-adder merge
   (f-addend into the adder's low chunk) to the shared-primitives cell as a single-cell card.
9. unresolved_directions: asymmetric cleanup machinery (inverse-side `compare_geq_chunked_middle` vs fwd-side
   `controlled_lt_msbs_conditional`) — separate card if count-only shows a gap; LSBS width changes (reopen
   trigger).
10. reopening_trigger: any LSBS/fold-window width change; measured value distribution with ≥8 bits headroom
    below 2^256 re-opens the (dead) strong version.
11. reproduction_commands:
    ```
    CARGO_TARGET_DIR=/tmp/w1scout-target TRACE_TLM_TOF=1 cargo run --release --bin build_circuit 2>&1 \
      | grep -E "TLM_TOF phase=tlm_apply_(forward_mod_add|inverse_mod_sub|forward_fold|inverse_fold)"
    # candidate (after implementer adds TLM_DEFER_MOD_CLEAN):
    CARGO_TARGET_DIR=/tmp/w1scout-target TLM_DEFER_MOD_CLEAN=1 TRACE_TLM_TOF=1 \
      cargo run --release --bin build_circuit
    ```
12. classification: STRUCTURAL-T composite (λ-exposed; rank after ARCH-01 and the strip lane because of the
    λ+proof cost per executed-T saved).

## Composite-task specifics
- participating owners: B3 apply (gcd.rs:1756/:1804/:1844); P shared primitives (fused.rs:1865/1921 fold
  machinery, hybrid adder dispatch gidney.rs:2004); λ-model owner (02-lambda window carry-escape term);
  strip owner; eval/regrind owner.
- interfaces: fold consumes the addend inside the existing LSBS=53 window — the wider addend must remain
  within the window's chunk layout; the cswap between add and doubling stays strictly 256-bit (strong version
  dead); stage-0 λ baseline (EXP-ARCH-01) is the shared control arm.
- invariants: (I1) paired fwd/inv edits (inverse pairing, else phase word dirties); (I2) cswap operand width
  stays 256; (I3) Q-guard — B0 census at all three plateau windows ≤1154 (transient overflow bits counted);
  (I4) fold-window chunk boundaries (LSBS=53) unchanged.
- proof obligations: (P1) knob-liveness; (P2) clean→0, fold growth < removal (count-only + TRACE); (P3)
  value-bounds: post-fold residual < 2^256 − max-next-cout-addend, by proof or 1e6-sample emulation with
  quoted tail margin; (P4) four checks green; (P5) paired λ n≥12 vs stage-0 baseline, σ quoted, accept only
  ≤1σ worse AND grind effort still feasible (λ_total ≲ 23.29-class with e^Δλ multiplier stated); (P6) S
  strictly lower; (P7) Q unchanged.
- staged tests: count-only probe → value-bounds proof → flag probe → selftests → paired dirty-scan → full
  build+eval + regrind.
- rollback unit: ONE env flag `TLM_DEFER_MOD_CLEAN` (default 0) wrapping both directions of the edit;
  revert = unset flag / single-commit discard.
- single-cell results needing re-validation if this lands: EXP-BZ-01 tail-skip marginals (the sub/s2 census
  must re-price at the widened fold — skips keyed by iteration, verify the tape symbols' classical constancy
  still holds); EXP-BZ-02 phase ledger; 02-lambda window carry-escape term re-price; deep-strip keys;
  EXP-QFL-01 census ordinals; frontier refresh + nonce regrind.

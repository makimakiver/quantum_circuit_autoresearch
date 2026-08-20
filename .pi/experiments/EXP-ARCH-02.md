# EXP-ARCH-02 — Square↔apply boundary fusion: classical-constant absorption into square carry chains (square×apply×B1)

Status: SCOPED-COMPOSITE (small). Derived from EXP-ITX-00 §3(d)(i) — the only open square-side T surface after
the overlap analyses closed. Pinned a2067dcf / ops.bin 6519bd01… (no new measurement in the finishing session).

1. experiment_id: EXP-ARCH-02
2. search_radius: the classical-constant coordinate adds feeding the pair-2 walk operand
   (coord_add3x-class, ≈320 executed T per application, ec_add.rs:292-300 sequence region) merged into the
   square's shifted-add carry chains (`square_c_sum_apply_shifted_*` phases; cap arith.rs:1689). Explicitly
   OUT of scope: square-unbuild ↔ walk-step-0 overlap (closed: SCHED_J2[0..11]=256 full width from step 0,
   disjoint control qubits — EXP-ITX-00 §3(d)(ii)); any square-peak narrowing (Q-axis, closed).
3. mechanism: the square already accumulates in place into x2 (the walk operand register). The constant adds
   run as separate shifted-add passes over the same operand; folding the constants into the square's existing
   carry chains removes one add-family pass. Constants are classical (ε=0): no predicate, no truncation site.
4. predicted direction: T −0..−300 executed (ΔS ≤ −0.35M at Q=1154; upper bound from the measured coord-add
   pass size, EXP-ITX-00 §3(d)); Q 0 REQUIRED — the square window is the cap-immune 1154 holder (EXP-SLOPE-00),
   so +1 ancilla there = +1 Q = needs ≥1,112 executed just to break even; λ 0 (exact arithmetic).
5. cheapest_discriminator: `POINT_ADD_COUNT_ONLY=1` build probe — coord-add phase counters must drop by the
   predicted amount while square phases grow by strictly less; ops.bin hash must move (knob-noop trap).
6. predeclared_falsifier: (a) count-only probe shows the square shifted-add phases growing ≥ the coord-add
   removal (the carry chains cannot absorb the constants without extra chunks); (b) post-change census shows
   any square-window anc growth (Q-guard I4 breach); (c) four checks not green.
7. evidence_debt: executed (not emitted) saving measured only via phase counters until the promotion eval;
   strip-key re-key tax unmeasured for this specific edit (expect ≤ 10² keys — small ordinal perturbation).
8. trial_budget / time_budget: 1 count-only probe + 1 census probe + implementer half-day IF the probe is
   positive; hard stop at falsifier (a).
9. unresolved_directions: does not decide the 2n-class register-add factor-2 gap (priced research); does not
   re-open square-phase Q structure.
10. reopening_trigger: any change to the pair-2 operand construction (ec_add algebra) or to the square's
    chunked-add layout re-prices the absorption.
11. reproduction_commands:
    ```
    CARGO_TARGET_DIR=/tmp/w1scout-target POINT_ADD_COUNT_ONLY=1 cargo run --release --bin build_circuit
    # candidate (after implementer adds TLM_SQ_CONST_FUSE):
    CARGO_TARGET_DIR=/tmp/w1scout-target TLM_SQ_CONST_FUSE=1 POINT_ADD_COUNT_ONLY=1 \
      cargo run --release --bin build_circuit
    CARGO_TARGET_DIR=/tmp/w1scout-target B0_WIN_LO=4370000 B0_WIN_HI=4382000 \
      cargo run --release --bin build_circuit 2>&1 | sed -n '/B0_CENSUS_BEGIN/,/B0_CENSUS_END/p'
    sha256sum ops.bin   # restore 6519bd01… after probing
    ```
12. classification: STRUCTURAL-T composite (ε=0; small prize; rank below ARCH-01/03/04).

## Composite-task specifics
- participating owners: square cell (arith.rs square phases + TLM_SQUARE_PEAK_CAP arith.rs:1689); apply/ec_add
  owner (coord constant adds); B1 (operand register schedule); strip owner (re-key check).
- interfaces: x2 must remain the pair-2 walk operand at the same op-window; constants enter as shifted adds in
  the c_sum_apply phases — the fusion may not rename or re-time the operand register.
- invariants: (I1) square output register identity/timing unchanged (walk-2 consumes it at SCHED_J2[0..11]
  full width); (I2) zero residual phase on square scratch (phase word audited, not assumed from ancilla=0);
  (I3) ε=0 constants only; (I4) Q-guard — square-window census stays ≤1154 with owner composition
  258+256+255 registers + ≤382 anc.
- proof obligations: (P1) knob-liveness (hash delta); (P2) coord phases → ~0 with square growth < removal;
  (P3) four checks green; (P4) Q unchanged; (P5) S strictly lower (else revert — prize is small, do not trade
  regrind for <−100 executed unless bundled with another regrind-forcing card).
- staged tests: count-only probe → census probe → selftests (`*_SELFTEST`) → full build+eval.
- rollback unit: ONE env flag `TLM_SQ_CONST_FUSE` (default 0) around the whole edit; revert = unset flag.
- single-cell results needing re-validation if this lands: square-phase census composition (EXP-CODEC-02
  op≈4.37M window); EXP-SLOPE-00 "Q floor square-cell-owned" re-assert via TRACE_EACH_PEAK; deep-strip keys;
  memory/03 square-floor citations.

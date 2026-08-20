# EXP-SQ-03 — direct per-bit Z-accumulate for `λ·x` (kill the quantum-quantum `t`-load in the shell)

- **experiment_id**: EXP-SQ-03
- **search_radius**: the `λ·x` product staging that the pair-2 multiply consumes — i.e. the boundary the
  square's `output_reg` (x2 = x₀−(λ²−x−3x₀)) feeds into `mod_mul_inverse_in_place(.., Direction::Forward)`
  (ec_add.rs "tlm_forward_multiply"). Carded here because the algebra lives in my cell
  (`x += 3x0; x -= λ²` sequencing); the multiply internals are shared primitives.
- **mechanism** (gate-level): the current shell performs the full quantum-quantum product λ·x inside the
  forward ModDiv replay. The classical-coordinate part of the same expression (`−y₀`) is folded classically
  (`coord_addsub` with `ox`/`oy` bits — ~free). The residual λ·x has no classical sub-part, so the only
  algebraic freedom is REGISTER FORM: carrying x in Zeckendorf-like offset form (x − 2x₀ = pre-sum tail of
  `3x₀ + x − λ²`) into the multiply so the `x += 3x0` add (332 emitted / 320 executed, live) folds into the
  multiply's own operand assembly. Mechanism: `coord_add3x` adds a |0>-start `temp` into `x2` via
  `mod_add`-family (a full 256-bit vented modular add); if instead the 3x₀ term is folded into the
  square's APPLY stages (each `apply_*_value` call already performs ±(value)·2^k — a 3x₀ term is a
  classically-constant add of the same shape), the standalone `tlm_coord_add3x` phase (332 T) disappears
  and its vent slots return to the applies (same vent-economics as EXP-SQ-01).
- **predicted T/Q/λ**: T −300..−400 executed (332 emit − ~40 survive as +const apply terms, at ~1 T per
  bit for a classically-known constant window vs full adder) PLUS a small vent-recovery term (~+256 budget
  during square applies ≈ −500 more by EXP-SQ-01's −542/apply slope, only if SQ-01 not taken — effects
  overlap; do not double count). Q 0. λ ~0 (pure reorganization).
- **cheapest_discriminator**: `TLM_COORD_ADD3X_FOLD=1` (new flag) on the implementer's branch +
  `POINT_ADD_COUNT_ONLY=1` op diff; adjudicate with `TRACE_TLM_TOF` (phase `tlm_coord_add3x` →0, square
  apply phases +Δ). Budget: 1 edit, 2 builds, selftest `TLM_SQ_SELFTEST`.
- **predeclared_falsifier**: (a) square-apply CCX grows by more than the removed 332 (windowed constant
  adds are not cheaper than the vented adder under cap pressure); (b) `mod_add_exact`'s carry-exactness
  (`x < p` invariant before the square) cannot be preserved by the folded form — the square asserts a
  fully-reduced 256-bit λ² accumulator; if the fold leaves x unreduced, `apply_f_times_value`'s NAF
  decomposition breaks (asserts `value.len() == N` full-width path). This is the main correctness cliff.
- **evidence_debt**: full — algebra-level card, not probed; requires the exact reassociation proof.
- **trial/time_budget**: 1 implementer day incl. `TLM_SQ_SELFTEST` + `SQUARE_WINDOW_SELFTEST` rework of the
  window checker if the register form changes; only escalate after EXP-SQ-01 lands.
- **unresolved_directions**: exact reduction discipline for the offset form (which sub-square first touches
  x2); whether `apply_unshifted_value`'s padding (emitted X on zero pads, free) survives the fold.
- **reopening_trigger**: EXP-SQ-01 accepted (adds vent headroom that changes this card's pricing), or any
  Q-dropping change that moves the square off the cap.
- **reproduction_commands**:
  ```bash
  # live baseline for the phases involved:
  cd /tmp/sqscout && TRACE_TLM_TOF=1 CARGO_TARGET_DIR=/tmp/sqscout-target \
    /tmp/sqscout-target/release/build_circuit 2>&1 | grep -E 'TLM_TOF phase=(tlm_coord_add3x|square_)'
  ```
- **classification**: STRIP (bit-exact same unitary; sequencing change only) — ranked below EXP-SQ-02
  because the correctness cliff (reduction form) is real and the payoff is ~4x smaller.

## Invariant pairs (if implemented)
1. `x2` must be fully reduced mod p at `mod_square_sub_pm_secp256k1_symmetric` entry (assert in square.rs)
   — the fold must re-establish this before the square, or the square must be shown reduction-tolerant
   (it is NOT: `apply_shifted_128_tagged` assumes < p operands in its window math).
2. `coord_add3x`'s `classical_times3_mod_q` bits all `bit_store0`'d before return — the fold must not leak
   classical bits across the divide (they are read-only in, zero-out).

# EXP-SQ-04 — square-output ↔ pair2-multiply input fusion (ESCALATION CARD, not locally owned)

- **experiment_id**: EXP-SQ-04
- **search_radius**: crosses my cell boundary — square exit register (x2) into `mod_mul_inverse_in_place`
  (gcd.rs:1889, `Direction::Forward`). Recorded here as an unresolved_direction per role contract; owned by
  cross-cell-architect / cell-D.
- **mechanism** (gate-level, from my side of the boundary): at square exit, x2 = x₀−(λ²−x−3x₀) mod p holds
  the λ·x factor the forward ModDiv will consume as its `xv` operand (swapped with `y`/λ into the walk's
  first step — gcd.rs:1915-1920). The forward walk immediately re-encodes x2 through the divstep pipeline
  (truncate/shift/cswap). Two fusion surfaces exist: (i) the square's `square_b_hi_apply_f_times_sub`
  (4,573 emitted T, the largest single square phase) computes −f·(b_hi) into x2 in NAF windowed full-width
  adds — a window-aware f-term could be re-ordered to land AFTER the walk's first truncation window
  (top bits of x2 are discarded in the first divstep), making some top-window f-terms dead from the walk's
  perspective; (ii) the unbuild phases (`square_*_unbuild` = 27,577 T, 41% of the square) restore scratch to
  |0⟩ BEFORE the multiply starts — any overlap of unbuild with walk-start would need the λ (y2) register
  free, which it is not (λ is the multiply's second operand, read live). Surface (i) is the real card;
  (ii) is blocked by a data dependency, stated here to close it.
- **predicted T/Q/λ**: T −500..−2,000 executed if top-window f-terms are provably dead after first-truncation
  (depends on the walk's first-window width — GAP_J2_LO=0 era windowing); Q 0; λ 0 (bit-exact deletion) —
  but ownership of the truncation window is cell B1's, and s2-conditionality on the walk makes this a
  PAIRED change (playbook §7).
- **cheapest_discriminator**: not mine to run — needs a B1+square paired probe. Suggested: the architect
  measures the first-truncation dead-set on x2's high bits (`TRACE_TLM_TOF` on the first
  `tlm_multiply_gcd_forward_*` phases vs x2 bit fanout census via constprop's input set).
- **predeclared_falsifier**: constprop/strip already deletes these f-term gates as dead (check: does
  `ccx_final_cancel` see x2's top bits as unread downstream? if yes this is already free and the card is
  void); first-window truncation width covers all 5 NAF terms' top bits (no dead set).
- **evidence_debt**: full; this is an escalation record, not a probed card.
- **trial/time_budget**: architect's call.
- **unresolved_directions**: this IS one — dispatch to cross-cell-architect with cells B1 (GAP_J2 windows)
  + C1 (square) + D (forward multiply) in one paired card.
- **reopening_trigger**: EXP-SQ-01 accepted (changes square phase budget); or GAP_J2 re-tuning.
- **reproduction_commands**: n/a (escalation record; live numbers for the phases cited come from the
  TRACE_TLM_TOF run in EXP-SQ-01).
- **classification**: STRIP (candidate) / STRUCTURAL-T — cross-cell, escalated.

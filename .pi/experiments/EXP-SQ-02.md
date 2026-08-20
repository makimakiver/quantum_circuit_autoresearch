# EXP-SQ-02 — classically-controlled adders for the coord shell's z-loads (`TLM_COORD_QCTRL`)

- **experiment_id**: EXP-SQ-02
- **search_radius**: `src/point_add/trailmix_ludicrous/ec_add.rs::coord_addsub` (+ reuse in `coord_rsub`
  via `mod_rsub_vented_loaded`) — replace the per-bit `x_if_bit` z-load into a fresh |0> `temp` register
  with `PushCondition(bit)`-scoped arithmetic. No arithmetic semantics change.
- **mechanism** (gate-level): each of the 5 shell `coord_addsub`/`coord_rsub` calls loads a classical
  coordinate into `temp` (256 `XIf`), runs `mod_sub_vented`/`mod_add`/`mod_add_exact`, then re-XIf's and
  frees. Every CCX inside the adder is `kind=CCX` with an ALL-ONE qubit control pattern when the operand
  bit is classically 0 — measured by the live simulator trace as 100% executed (332 emitted = 320.0 expected,
  discount 0.036). If the z-load is dropped and the adder runs inside `push_condition(coord_bit_i)` for each
  bit plane (the op stream already supports `c_condition` on CCX — see mod.rs:602 `ccx` + the
  `PushCondition`/`PopCondition` ops consumed by `eval_circuit`), each AND whose operand bit is 0 is skipped
  at execution time: expected discount ≈ 50% per controlled-majority gate → 320 → ~40+80·(executed fraction).
  Conservative prediction: ~160 of the 320 (λ≈0.5 on a uniform-random ox) — matching the 50%-discount class
  already shipped in `tlm_apply_{forward,inverse}_mod_{add,sub}_clean` (4,943 → 2,471.5 measured).
- **predicted T/Q/λ**: T −1,400..−1,600 executed (shell 1,600 → ~0-200; the ~1,600 body of each
  mod-sub remains but half-discounts); Q 0; λ 0 in first order (skips are classical-conditioning, no
  entanglement site added — but the `PushCondition` evaluation itself is classical; verify phase word).
- **cheapest_discriminator**: no cheap existing env probe exists — smallest real test is an implementer
  flag `TLM_COORD_QCTRL=1` on ONE call site (`tlm_coord_x_sub`, first in sequence) + `POINT_ADD_COUNT_ONLY=1`
  op diff (emitted ops stay ~equal; the payoff shows only in executed) + the TLM_TOF probe on phases
  `tlm_coord_x_sub`/`tlm_coord_y_sub` (expected →~160, discount →0.5). Budget: 1 edit, 2 builds.
- **predeclared_falsifier**: (a) TLM_TOF shows discount < 0.2 on the probed phase; (b) simulator rejects
  the classical-conditioned CCX pattern (conditioned gates must not carry phase — the `c_condition` channel
  is side-effect-free per mod.rs `cz_if` precedent); (c) four checks fail.
- **evidence_debt**: full — mechanism-level only, no probe run (this card is scoped, not yet probed).
- **trial/time_budget**: 1 implementer session (~1 h) for the flag + one call site; adjudication by
  TLM_TOF probe only; full ladder only if discount ≥ 0.4.
- **unresolved_directions**: whether `hybrid_add_adaptive`/`add_cout_vented_unctrl` can be wrapped
  wholesale in `push_condition` without per-gate surgery (op-level condition folding vs builder-level);
  whether the same trick applies to `classical_times3_mod_q`'s `x_if_bit` loads (already classical-side,
  ~free).
- **reopening_trigger**: any harness-side support for `c_condition` on Hmr/CZ class ops (currently only
  X/CZ/Z families have `_if` variants) would make this a systematic win across cells.
- **reproduction_commands**:
  ```bash
  # baseline phase discounts (live):
  cd /tmp/sqscout && TRACE_TLM_TOF=1 CARGO_TARGET_DIR=/tmp/sqscout-target \
    /tmp/sqscout-target/release/build_circuit 2>&1 | grep -E 'TLM_TOF phase=tlm_coord'
  # reference class (shipped 50% discounts): grep 'mod_add_clean|mod_sub_clean' in same output
  ```
- **classification**: STRIP-by-execution (same unitary; gates skipped only when classically zero) — priced
  as T-STRIP, NOT λ-affecting (no truncation, no approximation).

## Implementer note
Flag: `TLM_COORD_QCTRL`. Files: `src/point_add/trailmix_ludicrous/ec_add.rs` (`coord_addsub` only, first
call site `tlm_coord_x_sub`). Invariant pair: (1) `mod_sub_vented`/`mod_add` bodies must remain bit-exact
when NOT under a condition (flag off ⇒ ops.bin byte-identical); (2) `temp` register must stay zero at exit
under both paths (XIf wrap removed ⇒ verify `zero_and_free` still drains); (3) no new phase source: the
condition channel multiplies classical controls only.

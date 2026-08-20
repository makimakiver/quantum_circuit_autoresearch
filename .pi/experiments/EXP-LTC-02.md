# EXP-LTC-02 — Reverse-traversal premium: decompose and recover the +59k executed T asymmetry (B1×B3×P composite)

Status: SCOPED (rank 1 of Wave-1 reframing scout). Live-measured premium; mechanism partially localized.

1. **experiment_id**: EXP-LTC-02
2. **search_radius**: the two REVERSE gcd traversals (`reverse_gcd_jump`, gcd.rs:1435, unwalk-1 with phase family
   `tlm_inverse_gcd_reverse_*` and unwalk-2+apply with `tlm_multiply_gcd_reverse_*`) vs their forward counterparts.
   Concretely: per-stage CCX attribution inside the reverse-body phase (currently one bucket: `controlled_add_active`
   + cswap ladder + `swap_decision_uncompute_vented` + shifts), then the single largest asymmetry's fix behind one
   env flag. Sweep space for stage-2: comparator chunk window `ck = CMP_K+1` per-pass re-fit, `GCD_SUB_K` pass-1/3
   block re-fit (widen_sched_blocks BLOCKS_4 anchoring, mod.rs schedule blocks), and the condition-stack placement of
   the reverse comparator chain (comparator.rs:892-921).
3. **mechanism**: measured executed-T ledger (TRACE_TLM_TOF, pre-postpass model, byte-identical rebuild):
   - fwd body+compare (apply-free, walk-2): 69,549 + 45,065 = 114,614
   - rev body (apply-free, unwalk-1): 144,303  → **premium +29,689/traversal**
   - fwd body+compare (with apply, walk-1): 73,468 + 45,336 = 118,804
   - rev body (with apply, unwalk-2): 148,357.5 → premium +29,553.5
   Total reverse premium ≈ **59,242 executed T (4.6% of pre-postpass T)**, apply-independent and vent-independent
   (the fwd no-apply walk is the CHEAP one, so dirty-vent starvation is excluded). Candidate mechanisms to split:
   (m1) the reverse comparator `swap_decision_uncompute_vented` runs its carry chain under a classical condition
   stack (hmr(flag)→push_condition) — its chain CCXs discount ~50% in executed T but its emitted width may exceed
   the forward's truncated window; (m2) `controlled_add_active` reverse-mode k-fits (GCD_SUB_K blocks 1/3,
   top-down anchored) may sit off-optimum vs the forward's; (m3) the reverse cswap ladder may miss the
   fwd-dead:=rev-predicate transfer (W1155_FWD_EQ_REV class) in the reverse direction.
4. **predicted direction**: T **−10k..−40k executed** (−0.8%..−3.1% of T; S −9.7M..−35M at Q=1154) if m2/m3
   dominate; Q **0** (no liveness change: same registers, same schedule widths); λ **0** (ε=0 class — bit-exact
   re-fit of k-windows/skip predicates; must be confirmed by byte-diff + 4 checks, not assumed).
5. **cheapest_discriminator**: stage-1 instrument: add a per-stage CCX counter keyed on the existing
   `set_phase` calls inside `reverse_gcd_jump` (the region between `tlm_*_gcd_reverse_body` boundaries) — a
   TRACE-only build change, no functional edit; read `TRACE_TLM_CCX`-style output for {add, cswap, comparator}
   inside the reverse body vs forward. NO eval_circuit. Falsifies m1/m2/m3 split in one build (~3 min).
6. **predeclared_falsifier**: stage-1 shows the reverse premium is ≥80% inside `swap_decision_uncompute_vented`
   AND the forward comparator (45,336 executed, window-truncated per GAP_J2) is already at the
   proven comparator floor for its window — i.e. the premium is the *uncompute* direction's intrinsic cost
   (the condition-stack trick already banked the discount). Then the recoverable share is <5k and the card dies.
7. **evidence_debt**: (i) executed-vs-emitted mix inside the reverse body is inferred, not measured per stage
   (stage-1 repays); (ii) λ-neutrality of any k-window re-fit is assumed from the ε=0 class and MUST be repaid by
   a full build+eval 4-check run + byte-diff before acceptance (playbook §6); (iii) the deep-strip table
   interaction: any op-stream change re-keys census ordinals — the occupancy tripwire discards affected keys
   loudly; budget a re-mine or accept the strip loss in the A/B.
8. **trial_budget / time_budget**: stage-1: 1 instrumented build + 1 baseline build (same env) = ~10 min.
   stage-2: ≤6 probe builds (2 mechanisms × 3 knob values) at ~3 min each; hard cap 1 implementer-day.
   If stage-2 best ΔCCX < 5,000 emitted, stop (PARKED_BUDGET).
9. **unresolved_directions**: does NOT decide whether the comparator window itself (CMP_K/GAP_J2 global re-fit)
   has slack — that is a B1 knob family already bounded by the SUB4_NO_GAP A/B (±826 CCX total, measured this
   scout); does NOT touch the apply register adds (2n-class, factor-2 gap open but priced research).
10. **reopening_trigger**: any future change that re-orders the four traversals (e.g. LMD-03 landing) re-opens the
    fwd/rev fit tables; also re-open if a stage-level counter lands in mainline for unrelated reasons.
11. **reproduction_commands**:
    ```sh
    # baseline ledger (pre-postpass executed model; ops.bin stays 6519bd01...)
    CARGO_TARGET_DIR=/tmp/w1scout-target TRACE_TLM_TOF=1 TRACE_TLM_CCX=1 \
      cargo run --release --bin build_circuit 2>&1 | grep -E "TLM_TOF (phase=tlm_(multiply|inverse)_gcd_(forward|reverse)|TOTAL)"
    # knob-live A/B for the comparator share (changes ops.bin; restore afterwards)
    CARGO_TARGET_DIR=/tmp/w1scout-target SUB4_NO_GAP=1 TRACE_TLM_CCX=1 \
      cargo run --release --bin build_circuit 2>&1 | grep -E "TLM_CCX phase=tlm_(multiply|inverse)_gcd"
    cp /tmp/ops.bin.frontier.bak ops.bin && sha256sum ops.bin
    ```
12. **classification**: STRUCTURAL-T (composite B1×B3×P; ε=0 target class → borderline STRIP if the fix is pure
    schedule/predicate re-fit).

## Composite-task specifics
- **Participating owners**: B1 (walk schedule tables — `widen_sched_blocks`, `GCD_SUB_K`/`CMP_K` blocks),
  B3/P (comparator + hybrid adder fits — comparator.rs, gidney.rs), strip owner (deep-strip re-key check).
- **Interfaces**: `next_cmp_k()` consumption order is positional per traversal (BLOCKS_4_SHORT, cmp mod.rs
  `load_schedule`); any per-pass re-fit must keep the (pass, divstep) keying — the reverse block is anchored
  top-down (`(BAKED_ITERS, false)` entries).
- **Invariants (pairs)**: (i) reverse walk remains the exact inverse of the forward walk — any k/CMP change must
  be applied to the *matching* (pass, i) slots on both sides of a division or the dialog decode desyncs
  (8.36→4,646-mismatch class from memory/05); (ii) `s = SCHED_J2[i] − cmp_window(i)` coupling untouched.
- **Proof obligations**: stage-2 candidate must show (a) ops.bin hash CHANGED (knob-live), (b) emitted CCX delta
  in the predicted direction, (c) the four eval checks green on a full run, (d) λ paired n≥12 unchanged (only if
  the stream is not bit-exact-equal on the semantic stream — canonical hash decides).
- **Staged tests**: stage-1 trace (no functional change) → stage-2 single-mechanism flag probe → selftest
  `TLM_STRADDLE_VERIFY=n` bit-exact comparison → full build+eval.
- **Rollback unit**: ONE env flag `TLM_REV_BODY_REFIT` (default 0) wrapping the entire stage-2 edit — revert =
  unset flag / single-commit discard. No other file may change outside the flag guard except the TRACE-only
  stage-1 counter (separate commit, also flag-gated `TRACE_TLM_STAGES`).
- **Single-cell results needing re-validation if this lands**: H4 postpass credit re-mine (any stream change);
  deep-strip occupancy keys (tripwire auto-discards — watch "N stale keys skipped"); the 04-traps §2 positional
  formulas for CMP_K/GCD_SUB_K consumption counts; memory/03 comparator floor citations (re-quote against new
  stream); fleet-r1 §5 B1/B3 status rows.

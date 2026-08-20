# EXP-ARCH-01 — Reverse-premium recovery composite (B1×B3×P×strip×λ-gate; promotion form of EXP-LTC-02)

Status: SCOPED-COMPOSITE (promotion-lane lead candidate, Wave-1). No new measurement in the finishing session;
all live numbers inherited from EXP-LTC-02 / EXP-BZ-01 / EXP-SLOPE-00 at commit a2067dcf, ops.bin sha
6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097 (repo-cwd artifact re-verified canonical
this session; src/ untouched).

1. experiment_id: EXP-ARCH-01
2. search_radius: the measured +59,242 executed-T reverse-traversal premium (EXP-LTC-02 §3), decomposed per
   stage inside `reverse_gcd_jump` (gcd.rs:1453) and recovered by exact re-fit of reverse-side fit tables:
   comparator chunk windows (CMP_K per-pass), GCD_SUB_K pass-1/3 block re-fit, condition-stack placement of
   the reverse comparator chain (`compare_geq_chunked_middle_direct` comparator.rs:654, truncated variant
   :776), and the fwd-dead:=rev-predicate transfer class (W1155_FWD_EQ_REV). Stage-0 adds the portfolio-wide
   λ baseline (frontier.json λ currently null).
3. mechanism: EXP-LTC-02 §3 (m1 comparator-under-condition-stack width / m2 `controlled_add_active` reverse
   k-fits / m3 cswap predicate transfer). Composite additions: stage-0 (λ baseline:
   `TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12`, paired nonce sets, recorded into frontier.json) repays the
   portfolio-wide λ debt and calibrates the gate for every λ-exposed card (SLOPE-01/04, BZ-01, STRIP-01,
   EXP-ARCH-03). Stage-1: TRACE-only per-stage CCX/Toffoli counter keyed on the existing `set_phase`
   boundaries inside reverse_gcd_jump. Stage-2: single-mechanism env-flagged re-fit.
4. predicted direction: T −10k..−40k executed (ΔS −11.5M..−46.2M at Q=1154); Q 0 (Q-guard census required,
   invariant I4); λ 0 expected (ε=0 class: bit-exact re-partition of the same comparisons/adds, no truncation
   predicate touched) — gate-confirmed, never assumed.
5. cheapest_discriminator: stage-1 (~10 min: one instrumented build + one baseline, same env). No eval_circuit
   in scout stages.
6. predeclared_falsifier: (i) inherited EXP-LTC-02 §6 — ≥80% of the premium sits inside
   `swap_decision_uncompute_vented` AND the forward comparator is already at its proven window floor →
   recoverable share <5k → PARKED_BUDGET (reopening trigger: stage-level counters land in mainline, or any
   traversal re-ordering); (ii) NEW — stage-0 shows per-nonce σ > 6 (258-era reference 4.25): n=12 triage is
   then underpowered for every λ-gated card in the portfolio; stage-0 escalates to ~600 paired rounds before
   stage-2 may proceed.
7. evidence_debt: inherited from EXP-LTC-02 §7 (per-stage executed-vs-emitted mix; λ-neutrality of re-fits;
   strip-key re-keying). Stage-0 repays the λ debt portfolio-wide; the promotion run repays the rest.
8. trial_budget / time_budget: stage-0 ≈ 2 h wall (2×12 dirty-scan rounds); stage-1 ≈ 10 min; stage-2 ≤6
   probes ≈ 3 min each; hard cap 1 implementer-day; stop rule: stage-2 best ΔCCX < 5,000 emitted →
   PARKED_BUDGET.
9. unresolved_directions: apply-register 2n-class factor-2 adder gap (priced research, separate lane);
   CMP_K global slack (EXP-SLOPE-03, single-cell); ITERS is pinned (EXP-SLOPE-00) and not in scope.
10. reopening_trigger: any traversal re-ordering (LMD-03-class schedule re-shape); stage counters entering
    mainline; a STRIP-01/ARCH-04 stream re-key.
11. reproduction_commands (inherited; expect ops.bin back at 6519bd01… after probes):
    ```
    CARGO_TARGET_DIR=/tmp/w1scout-target TRACE_TLM_TOF=1 TRACE_TLM_CCX=1 \
      cargo run --release --bin build_circuit 2>&1 \
      | grep -E "TLM_TOF (phase=tlm_(multiply|inverse)_gcd_(forward|reverse)|TOTAL)"
    # stage-0 (twice, paired nonce sets):
    CARGO_TARGET_DIR=/tmp/w1scout-target TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12 \
      cargo run --release --bin build_circuit
    sha256sum ops.bin
    ```
12. classification: STRUCTURAL-T composite (ε=0 target class).

## Composite-task specifics (authoritative)
- participating owners: B1 walk-schedule tables (`widen_sched_blocks` trailmix mod.rs:328, `next_cmp_k` :259,
  CMP_K schedule.rs:33, forced GAP block mod.rs:2023-2026); B3 comparator fits (comparator.rs:654/:776/:907
  class); P hybrid-adder dispatch (`controlled_hybrid_add_cout_refs_impl` gidney.rs:2004); strip owner
  (deep-strip tripwire log mod.rs:1777); λ/eval owner (stage-0 + promotion gate, orchestrator).
- interfaces: `cmp_window(i)` (gcd.rs:31) consumption is positional per traversal (BLOCKS_4_SHORT anchoring;
  reverse blocks top-down `(BAKED_ITERS, false)`); every per-pass re-fit must preserve (pass, divstep) keying
  so forward and reverse read matching slots; stage-1 counter hooks the phase-string boundaries already
  emitted inside reverse_gcd_jump — no new phase names.
- invariants (paired): (I1) reverse walk remains the exact inverse of the forward walk — every k/CMP change
  applied to the matching (pass,i) slots on both sides of a division, else the dialog decode desyncs
  (4,646-mismatch class, memory/05); (I2) `s = SCHED_J2[i] − cmp_window(i)` coupling untouched; (I3) ε=0 —
  no truncation-site predicate may change semantics; (I4) Q-guard — post-change B0 census at the three plateau
  windows (early walk-1 ≈op 25,972; late walk-1 ≈op 3.3M; square ≈op 4.37M; ordinals re-derived from
  TL_CENSUS `example_op` fields) must show peak ≤1154 with unchanged owner composition.
- proof obligations: (P1) ops.bin sha ≠ 6519bd01… on the candidate (knob-liveness); (P2) emitted CCX delta
  per stage in the direction stage-1 attribution predicted; (P3) four eval checks green over 9024 shots;
  (P4) canonical fingerprint (repro/artifact_io.py) recorded; (P5) paired λ n≥12 unchanged within 1σ OR a
  bit-exact semantic-stream proof for the specific re-fit; (P6) S strictly lower; (P7) Q unchanged
  (artifact max_referenced_qubit_id = 1153).
- staged tests: stage-0 dirty-scan baseline → stage-1 `TRACE_TLM_STAGES` counter (own commit, flag-gated,
  zero functional edit) → stage-2 `TLM_REV_BODY_REFIT` single-mechanism probes → `TLM_STRADDLE_VERIFY`
  selftest n-sweep → full build+eval (orchestrator-owned).
- rollback unit: ONE env flag `TLM_REV_BODY_REFIT` (default 0) wrapping the entire stage-2 edit; the stage-1
  counter lives behind `TRACE_TLM_STAGES` in its own commit. Revert = unset flag / discard that one commit.
  No file may change outside the two guards.
- single-cell results needing re-validation if this lands: H4 postpass credit re-mine; deep-strip occupancy
  keys (tripwire auto-discards; watch `N stale keys skipped`); 04-traps §2 positional formulas for
  CMP_K/GCD_SUB_K consumption counts; memory/03 comparator-floor citations; EXP-SLOPE-00 cap-ladder numbers;
  EXP-CODEC-02 break-even ratios (ladder re-run on the new stream); EXP-QFL-01/EXP-CODEC-02 census window
  ordinals (re-derive from TL_CENSUS); frontier.json refresh + SUB4_TAIL_NONCE regrind (canonical hash moves).

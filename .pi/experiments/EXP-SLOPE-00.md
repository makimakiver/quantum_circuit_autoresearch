# EXP-SLOPE-00 — measured-closed surfaces on the live head (do NOT re-propose without new facts)

Pinned: a2067dcf + ops.bin 6519bd01… All numbers below are THIS-session measurements (op census = zstd record scan, kinds 13/14 = CCX/CCZ; baseline: ops 8,958,690, toffoli 1,335,661, CCX 1,330,933, maxq 1153 → Q 1154).

1. experiment_id: EXP-SLOPE-00 (ledger)
2. search_radius: TLM_TARGET_Q cap family; ITERS moves; SCHED_J2 tail narrowing as a Q lever.
3. mechanism / results:
   - **Q floor is square-cell-owned.** TRACE_EACH_PEAK: baseline peak 1154 at `tlm_apply_inverse_mod_sub_register` (ventable), but under TLM_TARGET_Q=1153 the square phase `square_c_sum_apply_shifted_128_sub` STILL holds active=1154 (cap-immune), and under cap 1152 likewise. Q=1154 regardless of cap. ModDiv side compresses to ≤1153 (cap 1153) / 1152+fold-1153 (cap 1152).
   - **Cap-lowering is score-negative even with a free square fix**: cap 1153 costs +5,824 CCX (+2,274 in-circuit vent/adder + ~3,550 strip-key tax) ≈ +2.5M score-points at Q=1153 vs the −1,283,487 qubit win → net +1.2M loss; the in-circuit-only (+2,274 ≈ +2.5M) variant still loses. Structural ModDiv narrowing would have to offset ≥1,112 executed/qubit before any Q card reopens.
   - **ITERS is pinned**: 261. Codec requires even codec_syms (iters−1); 259/260 measured destroyed in the 05-era (7,348 / 4,906 mismatches); λ tail curve 258→5.23, 261→0.48, 262→0.20 mm makes any downward move a λ sell (~e^+0.6..e^+4.8 grind multiplier) for ≤8.8k CCX; upward is a pure score loss. Not a tuning axis.
   - **SCHED_J2 narrowing has no Q payoff today** (peak is apply/square-side, and u,v at peak are already terminal-width ~10); its T payoff is the walk-cheapening half of the old Step-5 programme and is dominated by EXP-SLOPE-01's comparator work at far lower λ risk.
4-8. n/a (ledger).
9. unresolved_directions: none locally; all reopeners are cross-cell.
10. reopening_trigger: (a) square-cell delivers peak ≤1153 (measure: TRACE_EACH_PEAK under any cap) → re-price the narrow-tail+cap programme; (b) a structural ModDiv qubit saving ≥1 at ≤1,100 executed Toffoli; (c) λ evidence on the live head (currently null).
11. reproduction_commands:
    ```
    TLM_TARGET_Q=1153 TRACE_EACH_PEAK=1 CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit   # final PEAK: square_c_sum_apply_shifted_128_sub active=1154; toffoli 1341606; 3565 stale keys
    TLM_TARGET_Q=1152 TRACE_EACH_PEAK=1 …    # same square peak 1154; toffoli 1344411; 4255 stale keys
    TLM_TARGET_Q=1154 …                       # byte-identical noop (sha 6519bd01)
    ```
12. classification: measurement record (closes STRUCTURAL-Q attempts in this cell).

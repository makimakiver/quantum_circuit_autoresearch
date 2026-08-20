# EXP-CODEC-02 — Live peak-owner census + two-knob vent ladder (closes the codec Q axis)

**Cell:** tape-codec · **Wave:** 1 (discovery) · **Status:** LOCAL_PLATEAU / measured negative control (completed in-scout, 2026-08-16)
**Pinned state:** commit a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5, ops.bin 6519bd01…, ITERS=261, tape 609 q, Q=1154, T=1,283,487.051, S=1,481,143,998.

1. **experiment_id:** EXP-CODEC-02
2. **search_radius:** (a) B0 owner census at three windows: ops [25600,26300] (global peak),
   [26000,4500000] phase-filtered `tlm_inverse_gcd_forward_codec` (codec spike),
   [4370000,4382000] (square-phase 1154 moment); (b) knob ladder
   (TLM_TARGET_Q, TLM_SQUARE_PEAK_CAP) ∈ {(1154,1154)+, (1152,1154), (1150,1154), (1156,1154), (1160,1154),
   (1152,1152), (1150,1150)}. No source edits.
3. **mechanism:** `TLM_TARGET_Q` feeds `target_qubit_headroom = target − active` (trailmix_ludicrous/mod.rs:224)
   which CLAMPS DOWN adder chunk/vent widths at gidney.rs:1076/1960/1987, fused.rs:1786/1827,
   comparator.rs:673, arith.rs:1292; `TLM_SQUARE_PEAK_CAP` independently caps square-phase allocations
   (arith.rs:1689). The census identifies which qubits are live at each plateau-binding moment, i.e. whether
   ANY tape-representation change can move Q.
4. **measured direction (all arms):**
   | config | peak | peak phase | ΔCCX (emitted) | ΔS est. (0.964 calib.) |
   |---|---|---|---|---|
   | base (1154,1154) | 1154 | tlm_apply_inverse_mod_sub_register @25972 | — | — |
   | TQ 1152 | 1154 | square_c_sum_apply_shifted_128_sub | **+8,497** | +9.2M (Q unmoved: pure loss) |
   | TQ 1150 | 1154 | square_c_sum_apply_shifted_128_sub | **+12,861** | +13.9M (pure loss) |
   | TQ 1156 | 1156 | tlm_apply_inverse_mod_sub_register @25932 | **−368** | +2.16M worse |
   | TQ 1160 | 1160 | tlm_apply_inverse_mod_sub_register @25595 | **−3,630** | +3.6M worse |
   | both 1152 | **1153** | tlm_apply_inverse_mod_sub_fold @3379815 | **+8,543** | +8.4M worse |
   | both 1150 | **1151** | tlm_apply_inverse_fold @3389603 | **+12,955** | +11.2M worse |
   Census: global peak op 25972 = 4×~256-q registers (mod.rs:458/459, gcd.rs:1189/1902) + 126 chunk anc
   (gidney.rs:1206) + 6 misc = 1154, **zero tape qubits**. Codec spike op 3,354,361: active 1144 =
   **tape 605 (gcd.rs:1353, 52.9%)** + 256+256 primaries + 8+8 walk remnant + 11 codec transients
   (codec.rs:445 ×2, codec.rs:250, mcx.rs:267/268…). Square 1154 moment op 4,373,265: 258+256+255
   registers + 382 square/adder anc = 1154, **zero tape** (tape already consumed).
5. **cheapest_discriminator:** the build ladder itself + `TRACE_TLM_PROFILE=1` peak line + `B0_WIN_LO/HI
   [B0_PHASE=…]` census + ops.bin kind histogram (all executed; B0 and TRACE_TLM_PROFILE are the LIVE
   profilers — TRACE_PEAK/TRACE_PHASES/DUMP_PHASE_BOUNDS printers sit in dead build_builder(), mod.rs:1525+).
6. **predeclared_falsifier:** promotion required some ladder arm with Q actually moving AND
   |ΔCCX| ≤ ~1,110×|ΔQ| emitted (executed break-even T/Q ≈ 1,112). No arm came within 3×: best case
   (1160,1160-class) saves 3,630 CCX for +6 q (needs ≥6,660); down-ladder pays 8.5–13k CCX for −1..−3 q.
   Q is downward-invariant under TLM_TARGET_Q alone (square cap holds 1154) and only ~½-efficient under
   both knobs (apply-fold structural residuals 1153/1151).
7. **evidence_debt:** executed↔emitted calibration (global 0.964; per-phase branch discounts 0–50%);
   no eval_circuit run (orchestrator-owned). Every arm ≥3× over break-even — beyond calibration error.
   λ: chunk-width/vent-count changes are exact (no truncation sites touched), λ-neutral by construction.
8. **trial_budget:** 8 builds ≈ 6 min wall (CARGO_TARGET_DIR=/tmp/tlm-codec-tgt). Fully spent.
9. **unresolved_directions (escalated, NOT carded locally):**
   - the tape-free binding moments themselves: early 4-register coexistence (~op 25972) and the
     square-phase 1154 structure (382 anc) — apply/square cells + cross-cell-architect;
   - one-traversal / streaming rank-unrank tape (353-bit information gap, no streaming construction
     known — memory/repro/y3_global_codec.py) — cross-cell-architect;
   - unrestricted exact-eight joint codec synthesis (open, solver-hard; reopen only via the four
     preregistered triggers in 06-research-status.md §"Unrestricted exact-eight");
   - the 1153/1151 apply-fold residual owners when both caps drop (apply cell).
10. **reopening_trigger:** any accepted change that removes ≥10 live qubits from a tape-free binding
    moment (register coexistence or square phase), or a cross-cell liveness redesign that makes a binding
    moment tape-live — then re-run this ladder to re-measure dT/dQ before pricing any width card.
11. **reproduction_commands:**
    ```bash
    cd /Users/makimakiver/ecdsafail-challenge
    export CARGO_TARGET_DIR=/tmp/tlm-codec-tgt
    TRACE_TLM_PROFILE=1 TRACE_PHASE_ACTIVE=1 cargo run --release --bin build_circuit  # peak + TLM_PHASE table
    B0_WIN_LO=25600 B0_WIN_HI=26300 cargo run --release --bin build_circuit           # global-peak census
    B0_WIN_LO=26000 B0_WIN_HI=4500000 B0_PHASE=tlm_inverse_gcd_forward_codec \
        cargo run --release --bin build_circuit                                       # codec-spike census (605 tape)
    B0_WIN_LO=4370000 B0_WIN_HI=4382000 cargo run --release --bin build_circuit       # square 1154 census
    for q in 1152 1150 1156 1160; do TLM_TARGET_Q=$q cargo run --release --bin build_circuit; done
    TLM_TARGET_Q=1152 TLM_SQUARE_PEAK_CAP=1152 cargo run --release --bin build_circuit
    TLM_TARGET_Q=1150 TLM_SQUARE_PEAK_CAP=1150 cargo run --release --bin build_circuit
    # CCX/CCZ/Hmr per arm: decode ops.bin (zstd body after 16-byte header, 56-byte recs, kind u32 @0)
    cp /tmp/ops.bin.6519bd01.bak ops.bin   # restore pinned artifact after probing
    ```
12. **classification:** STRUCTURAL-Q (measurement card; verdict: the codec Q axis is closed on this
    frontier — the binding peak moments contain no tape, and the only Q-moving knobs price a qubit at
    4–6× over executed break-even).

**Verdict for the wave:** no promotable codec-local card. Width axis closed (≤2q, infeasible bijection);
pair/normalizer floors proven; joint-synthesis neighborhoods closed; tail4 refuted on T, Q (vent) and λ;
the Q vent itself now measured shut. Codec T-share ≈ 0.2–0.5% of emitted CCX bounds anything left.

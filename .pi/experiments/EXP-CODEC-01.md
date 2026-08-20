# EXP-CODEC-01 — TLM_TAIL4_TOP32 re-price on the live frontier (a2067dcf)

**Cell:** tape-codec · **Wave:** 1 (discovery) · **Status:** REFUTED (measured in-scout, 2026-08-16)
**Pinned state:** commit a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5, ops.bin 6519bd01…, ITERS=261, tape 609 q, Q=1154, T=1,283,487.051, S=1,481,143,998.

1. **experiment_id:** EXP-CODEC-01
2. **search_radius:** the single existing env knob `TLM_TAIL4_TOP32 ∈ {0,1}` (codec.rs:69) at otherwise
   default build() env (TLM_TARGET_Q=1154, TLM_SQUARE_PEAK_CAP=1154, ITERS=261). No source edits; the knob,
   the encoder/decoder ANF tables (codec.rs:38-66), and both the record path (gcd.rs:1382-1397) and the
   replay path (gcd.rs:1497-1514) are fully wired — the knob is LIVE, default-off, not half-wired.
3. **mechanism:** replaces the final Triple+Pair windows (5 symbols, 15 raw bits) with 3 passthrough bits +
   5 ANF code bits (codec.rs:150-160). The 12→5 payload codec runs `toggle_mcx_with_dirty` per ANF term
   (dirty-bridge recursion ≈ 4k−4 CCX per k-control toggle), encoder AND decoder on both compress and
   decompress (codec.rs:141-149). Tape shrinks 609→605 only at the tape TAIL, which is written after — and
   consumed before — every plateau-binding allocation moment (measured: peak op 25972 is tape-free).
4. **predicted direction:** T **+8.0–8.6k emitted CCX** (measured: **+8,570**, +52 CCZ, +7,804 ops, −52 Hmr);
   Q **±0** (vent: peak 1154 unchanged at op 25972, phase `tlm_apply_inverse_mod_sub_register`);
   λ **+~2.7** classical-channel (decoder support = 32 words, historical support-miss ≈ 3e-4 × 9024 shots —
   not re-measured at this commit).
5. **cheapest_discriminator:** one full build + ops.bin kind histogram (executed here):
   `TLM_TAIL4_TOP32=1 cargo run --release --bin build_circuit` then count kind-13 (CCX) records
   (zstd body, 56-byte records). Build succeeds — tape-length assert passes at 605 (gcd.rs:1431).
6. **predeclared_falsifier:** emitted-CCX delta ≥ +4,626 (executed break-even at constant Q ≈ 4×T/Q ≈ 4,450
   exec ≈ 4,626 emitted) **OR** peak qubits unchanged at 1154. Measured: **both fired** (+8,570 CCX;
   peak 1154 at the identical tape-free op 25972). Card dead twice over; historical refutation (+8,038 CCX,
   break-even 4,493, peak stay) reproduces on the live commit within 6.6% drift.
7. **evidence_debt:** executed-T calibration factor 0.964 (T_exec/CCX_emitted = 1,283,487/1,330,933) is a
   global average, per-phase discounts vary; λ support-miss quoted from the prior four-bit refutation, not
   re-sampled at a2067dcf. Both moot: statically ≥1.9× over break-even before calibration error.
   Would be repaid at the IMPLEMENTED/PROVISIONAL gates (orchestrator-owned eval) — never reached.
8. **trial_budget:** 2 builds ≈ 80 s (CARGO_TARGET_DIR=/tmp/tlm-codec-tgt). Spent: baseline + 1 arm.
9. **unresolved_directions:** none inside the codec cell — larger terminal windows are bounded by the closed
   k≤17 exact enumeration (max capturable 2 q, infeasible 38-bit bijection). Whether a tail window could
   ever pay is a question about the PEAK's tape-liveness, i.e. cross-cell (see EXP-CODEC-02).
10. **reopening_trigger:** only if a cross-cell liveness redesign makes a plateau-binding moment tape-live
    (then −4q could price > 0 and this card re-prices at ~zero cost).
11. **reproduction_commands:**
    ```bash
    cd /Users/makimakiver/ecdsafail-challenge
    export CARGO_TARGET_DIR=/tmp/tlm-codec-tgt
    cargo run --release --bin build_circuit                       # baseline: ops 8,958,690 / CCX 1,330,933
    TLM_TAIL4_TOP32=1 TRACE_TLM_PROFILE=1 cargo run --release --bin build_circuit
    # -> TLM_PROFILE peak_qubits=1154 phase=tlm_apply_inverse_mod_sub_register ops_idx=25972
    # -> ops 8,966,494 / CCX 1,339,503 / CCZ 4,778 / Hmr 512,748
    # CCX count: zstd -dc body after 16-byte header, 56-byte records, kind u32 @0 == 13
    sha256sum ops.bin   # restore from the pinned 6519bd01 backup after probing
    ```
12. **classification:** TUNING (existing-knob re-price; the Q half of the claim is vent-trapped by the
    tape-free binding peak, so it is not even a STRUCTURAL-Q hypothesis on this frontier).

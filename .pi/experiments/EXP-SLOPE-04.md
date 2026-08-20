# EXP-SLOPE-04 — minimal-surface quick promote: GAP δ2 band extended to [0,220)

Pinned: a2067dcf + ops.bin 6519bd01…

1. experiment_id: EXP-SLOPE-04
2. search_radius: single env-expressible change: shipped forced block's HI 200→220 (mod.rs:2033), i.e. δ2 narrowing extended over divsteps i∈[200,220) only. 20 compares, one bit each.
3. mechanism: same as EXP-SLOPE-01 but δ2 (matching the already-verified head narrowing depth) and a 20-divstep band.
4. predicted direction: T −184 toffoli / −1,713 ops measured net of 56 stale keys (probe E, sha 013493e9…). Executed ≈ −176 → ΔS ≈ −203k (−0.014%). Q 0. λ: +1 bit on 20 compares (smallest new λ surface of any card here).
5. cheapest_discriminator (DONE): `SUB4_NO_GAP=1 TLM_GAP_J2_TRUNC_ONLY=1 TLM_GAP_J2_DELTA=2 TLM_GAP_J2_LO=0 TLM_GAP_J2_HI=220` → toffoli 1,335,477 vs 1,335,661; control byte-identical at HI=200.
6. predeclared_falsifier: paired n≥12 dirty-scan shows any classical regression vs baseline nonce-for-nonce; or eval four-checks fail.
7. evidence_debt: the build comment "i>=200 has none" (mod.rs:2029) is STALE — measured −1,713 ops of slack exist in [200,220). Whether the comment meant λ-tested-none is unknown; the dirty-scan gate repays it.
8. trial_budget / time_budget: 1-line flag change in the forced block (implementer) + eval. Cap: half a day.
9. unresolved_directions: subsumed by EXP-SLOPE-01 if that passes its gate; keep as the fallback if 01's mid-band δ3 fails λ.
10. reopening_trigger: n/a (it is itself a reopening of the stale "no slack ≥200" claim).
11. reproduction_commands:
    ```
    SUB4_NO_GAP=1 TLM_GAP_J2_TRUNC_ONLY=1 TLM_GAP_J2_DELTA=2 TLM_GAP_J2_LO=0 TLM_GAP_J2_HI=220 CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit
    # emitted 8956977, toffoli 1335477, 56 stale keys
    ```
12. classification: TUNING.

# EXP-SLOPE-01 — two-band GAP_J2 comparator narrowing (head δ2 kept, mid-band δ3 added)

Pinned: commit a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5, ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097 (reproduced byte-identical in this session, scratch CWD /tmp/slope-run).

1. experiment_id: EXP-SLOPE-01
2. search_radius: `cmp_window(i)` (gcd.rs:16-38) narrowing schedule ONLY. Shipped state: build() force-sets TLM_GAP_J2_DELTA=2, TRUNC_ONLY=1, LO=0, HI=200 (mod.rs:2027-2036). This card adds a SECOND band: δ=3 over divsteps i∈[200,238) (the region where GAP_J2[i] < SCHED_J2[i], i.e. g−n ∈ {−1,−2}; verified live: 19 entries at −1, 9 at −2). The full-window tail i≥238 (g−n=+1, 23 entries) is NOT touched (TRUNC_ONLY semantics already exclude it).
3. mechanism: each divstep's swap decision runs `controlled_swap_decision_v_lt_u` over the top cmp_window bits (comparator.rs:779+). Narrowing the window by 1 removes one compare-slice (a CCX pair + CX uncompute) per divstep in the band; the mid-band has the widest registers still under truncation (current_n 29→200), so each bit is worth the most there. Head band (i<200, window 23–52 fixed) was already narrowed by the shipped δ2 — extra δ there saves mostly Cliffords (probe D: −10,006 ops but only −824 CCX in-circuit).
4. predicted direction: T −440±60 emitted toffoli for the two-band form (decomposed from probes: mid-band δ3 = I−D = −443 toffoli; head δ3 = +208 → keep δ2). With a deep-strip re-mine (EXP-SLOPE-02) recovering the ~963–1,032 tripwired keys: ≈ −1,400 toffoli total. Q: 0 (Q=1154 is square-cell-owned; see report). λ: mid-band s := n − cmp_window moves 1–2 → 3–4 on 38 compares — a NEW λ channel, magnitude unmeasured, expected small (coupling result: error depends only on s; the 4,646-mismatch catastrophe was crossing the full-window boundary s=0→1, not +1 deep in truncated territory).
5. cheapest_discriminator (DONE): env-band build probe + zstd op census.
   - `SUB4_NO_GAP=1 TLM_GAP_J2_TRUNC_ONLY=1 TLM_GAP_J2_DELTA=3 TLM_GAP_J2_LO=0 TLM_GAP_J2_HI=238 cargo run --release --bin build_circuit` (call this variant I): emitted toffoli (CCX+CCZ) 1,335,426 vs baseline 1,335,661 = **−235 net**, ops 8,943,452 vs 8,958,690 = −15,238, 0 noop (sha 5b…-distinct; deep-strip 963-class stale keys).
   - Control passed: SUB4_NO_GAP=1 + explicit δ2/LO0/HI200 re-export reproduces 6519bd01 byte-identical.
6. predeclared_falsifier: (a) paired TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12 on the same nonce set shows classical mismatch above the intrinsic band (per 02-lambda triage: >30 mean or any draw >2× baseline max) → kill; (b) two-band form fails to beat −235 toffoli net (i.e. head-δ2+mid-δ3 < variant I) → revert to variant I as the promoted form; (c) eval four-checks not green.
7. evidence_debt: λ on the live head is UNMEASURED (frontier.json lambda=null; 23.29 was ITERS=258-era, model-adjusted ≈18.5 at 261). The promotion gate repays it: n≥12 paired dirty-scan + full eval. Strip-key staleness (963) is repaid only by EXP-SLOPE-02 or accepted as sunk.
8. trial_budget / time_budget: 1 implementer edit + 3 build probes (~2 min each, warm target dir) + orchestrator-owned eval runs. Hard cap: 1 day wall.
9. unresolved_directions: does NOT decide λ of the head-band δ3 (dead: +208 toffoli), does NOT decide the full-window tail i≥238 (unmeasured, likely tiny since window=n already), does NOT decide CMP_K chunk widths (EXP-SLOPE-03).
10. reopening_trigger: if EXP-SLOPE-02 lands (re-mine at 0 stale), re-price this card: expected −235 → ≈−1,400 toffoli (−0.09% score) at unchanged λ.
11. reproduction_commands:
    ```
    cd <repo> && CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit   # baseline: sha 6519bd01…, emitted 8958690, toffoli 1335661
    SUB4_NO_GAP=1 TLM_GAP_J2_TRUNC_ONLY=1 TLM_GAP_J2_DELTA=3 TLM_GAP_J2_LO=0 TLM_GAP_J2_HI=238 CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit   # variant I: emitted 8943452, toffoli 1335426
    # census: python3 /tmp/ops_census.py ops.bin   (zstd; count kinds 13/14, max qubit id)
    ```
12. classification: TUNING (schedule/parameter; λ-gated).

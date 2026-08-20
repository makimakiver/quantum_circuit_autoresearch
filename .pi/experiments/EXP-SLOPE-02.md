# EXP-SLOPE-02 — deep-strip census re-mine tooling (unlocks every geometry-changing card)

Pinned: a2067dcf + ops.bin 6519bd01… (baseline deep-strip line, reproduced this session):
`[deep-strip-identity] removed 12202 / 12202 dead; downgraded 4045 / 4045 to CX/CZ; 0 stale keys skipped`

1. experiment_id: EXP-SLOPE-02
2. search_radius: rebuild the greedy census miner that produced `deep_strip_keys.rs` (16,254-line committed table; header: "TRUE GREEDY v5 cap 6.0, no-event floored at 0.40 … Census 320000000 … Risk 1.239700 (budget 1.24)"). The miner is NOT in-repo (memory/05 §6: tooling lost with the VM). Deliverable: a runner that re-mines DEAD_KEYS/DOWNGRADE_KEYS against an arbitrary ops.bin stream with the same tuple-occupancy keying and risk budget.
3. mechanism: every schedule/geometry change shifts op ordinals → the occupancy tripwire (mod.rs:1714+, traps §2 level-2) discards keys. Measured tax on the live head: GAP δ3 band = 1,032 keys; TLM_TARGET_Q=1153 = 3,565; =1152 = 4,255. Each stale key ≈ 1 unremoved dead CCX-class gate ≈ +1 executed Toffoli. A re-mine converts probe-net-negative experiments into wins (EXP-SLOPE-01: −235 → ≈−1,400 toffoli).
4. predicted direction: T 0 alone on the baseline (0 stale keys today — the strip is FULLY live); T −800…−1,200 for each promoted geometry change thereafter. Q 0. λ: the strip's drops are census-risk-priced (v5 budget 1.2397/1.24 λ-units); a re-mine must reproduce the same risk model or λ rises by the unpriced remainder.
5. cheapest_discriminator: none cheaper than the build log line itself — `grep 'stale keys' build.log`. Baseline = 0. Any promoted experiment with N>0 owes N re-mined keys before its T number is final.
6. predeclared_falsifier: a re-mined table that cannot reproduce 12,202/12,202 + 4,045/4,045 at 0 stale on the UNCHANGED baseline stream (self-consistency), or whose four-checks eval is not green on a re-mined-stream A/B.
7. evidence_debt: the v5 risk model (cap 6.0, floor 0.40, 3.2e8 census) exists only in the lost tool; the committed table's header is the sole spec. Promotion gate: re-mined baseline must be byte-equivalent in effect (identical removed/downgraded counts) before trusting re-mines of changed streams.
8. trial_budget / time_budget: 1–2 days tooling (CPU census at 3.2e8 samples was a VM-class run; a 1e7-sample pilot validates the keying first). Hard stop if the pilot cannot reproduce ≥90% of the committed keys on the baseline stream.
9. unresolved_directions: does not decide whether re-ENABLING the per-family structural-dead predicates (drops_off_family, all retired because ITERS=261 ≠ BAKED_ITERS=258) adds value on top of the deep strip — that needs keys mined at 261 + BAKED_ITERS:=261, a separate paired experiment touching walk+apply+square families (cross-cell).
10. reopening_trigger: any accepted card that tripwires >100 keys makes this the highest-leverage follow-up; also reopen if the 29-commit trend continues (strip table is already at its 3rd generation).
11. reproduction_commands:
    ```
    CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit 2>&1 | grep deep-strip-identity
    # → removed 12202/12202 dead; downgraded 4045/4045; 0 stale keys skipped
    SUB4_NO_GAP=1 TLM_GAP_J2_TRUNC_ONLY=1 TLM_GAP_J2_DELTA=3 TLM_GAP_J2_LO=0 TLM_GAP_J2_HI=200 CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit 2>&1 | grep deep-strip-identity
    # → 1032 stale keys skipped (the tax this card repays)
    ```
12. classification: STRIP.

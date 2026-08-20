# EXP-PR-06 — Adder layout/k-schedule retunes + Q-vents (closure card)

| field | value |
|---|---|
| id | EXP-PR-06 |
| search_radius | TLM_TARGET_Q {1153,1152,1150}; TLM_ADAPTIVE_FINAL_NO_COUT; TLM_DIRECT_VARCHUNK; TLM_COUT_LAYOUT_SEARCH(+MARGIN 0/1/2); TLM_GCD_ADAPTIVE_LAYOUT_MARGIN 1/2; TLM_ADAPTIVE_CHUNK {14,18,22}; TLM_GIDNEY_SKIP_STRUCTURAL_DEAD_CALLS(+COMPARE variant); APPLY_COUT_K ±1/±3; HYB_V ±1/±3. |
| mechanism | All reshape the chunking/layout of the gidney adder family (4,780 threaded + 1,044 hybrid + 520 cout calls) or attempt Q-vent relief. |
| predicted T/Q/lambda | Measured (prior session probe.sh/probe2.sh + this session logs): every arm is either byte-identical (sha 6519bd01: adap_nocout, direct_varchunk, cout_search, cout_search_m0, skipdead_noop, chunk14/18/22, hybv1, cout_k1) or T-worse (q1153 +2,137 emitted→peak stays 1154; q1152 +4,210; q1150 +8,416; gcd_margin1 +30 exec; margin2 +72; cout_k3 +245; ffg1 +18.5). Q: 0 everywhere — the vent trap is confirmed live (TLM_TARGET_Q cannot force peak below the register-apply liveness floor 1154). |
| cheapest_discriminator | Done — one 14s build per arm, sha+TLM_TOF_TOTAL diff. |
| predeclared_falsifier | n/a — closure card; would reopen only on structural change. |
| evidence_debt | None — these are negative results with hashes. |
| budget | Spent (prior session). Closed. |
| unresolved_directions | Census-dead-gate re-mine is auto-retired (ITERS 261 ≠ BAKED_ITERS 258 → drops_off_family true for all 20 families incl. W1155) and independently closed by fleet round-1; layout search space beyond ±1 margins unexplored but bounded by the 2n+l+m+1 objective already at its searched optimum. |
| reopening_trigger | ITERS/BAKED_ITERS reunification (re-census) or a Q-structure change in the register-apply cell (frees a persistent qubit with TLM_TARGET_Q following). |
| reproduction_commands | `cd /tmp/prims-work && ./probe.sh` (arms encoded inline; binary /tmp/prims-tgt/release/build_circuit at a2067dcf) |
| classification | LOCAL_PLATEAU — knob-noop and vent-trap documentation; closed. |

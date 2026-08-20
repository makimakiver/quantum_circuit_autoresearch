# EXP-PR-04 — Comparator swap-decision k truncation (CMP_K −Δ)

| field | value |
|---|---|
| id | EXP-PR-04 |
| search_radius | `CMP_K` schedule (schedule.rs) feeding `controlled_swap_decision_lt_truncated` (comparator.rs:776, called gcd.rs:1676) and `swap_decision_uncompute_vented` (comparator.rs:892, called gcd.rs:1605); k values 26..56, 2×260 invocations per direction. Comparator family total: 1,589 chunked-direct calls + 2,728 embedded erase compares; swap phases 132,096 + compare phases 90,401 emitted tof. |
| mechanism | Lower k truncates the lt-decision compare window by Δ bits per call → −Δ×260×2 CCX emitted per direction pair; wrong decision = wrong CSWAP = CLASSICAL corruption (not phase-laundered). |
| predicted T/Q/lambda | T: −2 bits ≈ −1,000..−2,000 emitted, ≈ −500..−1,000 executed (compare phases discount 0.000 — fully executed!). Q: 0. λ: classical channel, ~260×9024×2^-(k−2) naive — at k≈20-24 tail values this is the same order as the erase cap but on the CLASSICAL channel (a wrong swap fails the shot outright AND cascades). |
| cheapest_discriminator | ≤60s: build with a CMP_K-env if one exists (none found — CMP_K is a hard const, needs implementer edit) → static-only this session; else countops on a patched build by implementer. |
| predeclared_falsifier | Paired scan shows classical fault lanes increase (>43 baseline) at any k reduction → immediate close. |
| evidence_debt | No env knob exists (hard const in schedule.rs) — every probe needs an implementer edit behind a new flag; not probed this session. |
| budget | 1 implementer edit + 2 builds + 2 scans ≈ 5 min; low priority. |
| unresolved_directions | Whether the tail steps (k→13..16 region) already sit at the truncation floor; MSBS=19 precedent may not transfer (classical vs phase channel). |
| reopening_trigger | EXP-PR-01/02/03 all closed AND a −1M-class S gap remains; or an attribution scan shows compare-phase dirty-frees are zero (safety margin). |
| reproduction_commands | n/a this session (no knob); implementer: add `TLM_CMP_K_DELTA` env knob in schedule.rs consumers, then `env TLM_CMP_K_DELTA=2 ... build_circuit` + countops + 400-round scan. |
| classification | STRUCTURAL-λ (classical-channel risk), PARKED. |

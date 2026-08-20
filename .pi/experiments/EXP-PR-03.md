# EXP-PR-03 — Banded/windowed erase-cap below w=22 (MEASURED DEAD)

| field | value |
|---|---|
| id | EXP-PR-03 |
| search_radius | Two-band cap: shipped w=22 on calls 24:K, reduced w=19 on calls K:∞, via `TLM_COUT_ERASE_CAP=19 TLM_COUT_ERASE_CAP_CALLS="K:9999"` (gidney.rs:1329-1345). Hypothesis: λ damage concentrated in small-operand (early) calls, leaving a free late band. |
| mechanism | Same truncation physics as EXP-PR-02; window dial isolates call-index ranges. NOTE: the env window REPLACES the shipped 24:9999 window — calls outside the explicit window revert to FULL-WIDTH (uncapped, exact, λ-free but T-expensive) erase compares, so each arm mixes a w-reduction with an uncap regression; the uncap contributes ZERO Δλ, so arm Δλ's are pure w-effects per band. |
| predicted/measured T,Q,lambda | w=19 on 24:800 (w19_early): ops 9,295,932, λ=21.15 → band Δλ = +3.53. w=19 on ≥800 (w19_late): ops 9,231,730, λ=22.56 → band Δλ = +4.94. Sum 8.47 ≈ full-window +8.11 (±Poisson). Early band = 31% of calls but 43% of Δλ; late = 69% of calls, 61% of Δλ — **approximately linear, no sparse safe structure**. Q: 0. T: any w<22 band buys its proportional λ share. |
| cheapest_discriminator | Done (2 × ~55s nohup'd scans). |
| predeclared_falsifier | Predeclared: "Δλ(window) scales linearly with window size (≥0.8× proportional) across two bisection rounds → close as LOCAL_PLATEAU." Measured 0.9-1.4× proportional on both bands → **condition met, card closed**. |
| evidence_debt | Band boundaries coarse (24:800 / 800:∞ only); finer bisection cannot beat the measured linearity. |
| budget | Spent: 3 scans. Closed. |
| unresolved_directions | None on this axis. (A hypothetical per-site width schedule — cap = operand-entropy-derived — would need implementer machinery and the linearity result says the payoff is the same λ/T exchange rate as the global knob.) |
| reopening_trigger | None. |
| reproduction_commands | `cd /tmp/prims-work && env TLM_COUT_ERASE_CAP=19 TLM_COUT_ERASE_CAP_CALLS=24:800 TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=400 TLM_DIRTY_SCAN_MAX=600 TLM_DIRTY_SCAN_SHOW=0 /tmp/prims-tgt/release/build_circuit 2>&1 | grep DIRTY_SCAN` (and the 800:9999 variant); artifacts `/tmp/prims-work/scanbis_*.log`. |
| classification | STRUCTURAL-λ — measured rejection, CLOSED (LOCAL_PLATEAU on the banded family). |

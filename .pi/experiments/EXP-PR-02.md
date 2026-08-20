# EXP-PR-02 — Erase-cap w-sweep below shipped w=22 (MEASURED, λ-REJECTED pending budget)

| field | value |
|---|---|
| id | EXP-PR-02 |
| search_radius | w ∈ {19,20,21} on the SHIPPED window (calls ≥ 24): explicit `TLM_COUT_ERASE_CAP=w` (CALLS defaults to 24:9999). Sites: chunk-carry erases inside the 520 width-256 `controlled_hybrid_add_cout_refs` register applies (gcd.rs:1882, arith.rs:1525 → gidney.rs:2004/1347). |
| mechanism | Truncate each erase re-derivation compare from top-22 to top-w: saves (22−w) CCX per site at ~½ hmr-gated execution; error is phase-only (wrong carry left dirty at free → R-flip), classical provably untouched (classical lanes 43=43 in every arm). |
| predicted/measured T,Q,lambda | w=20: ΔT_exec = −2,034 (ΔS = −2,347,236), post-strip tof −4,000, **Δλ = +3.18 measured** (phase 25→39/25,600). w=19: ΔT_exec = −3,094 (ΔS = −3,570,476), post-strip tof −6,113, **Δλ = +8.11 measured** (phase 25→46). w=21 (interpolated): ΔT ≈ −1,010 (ΔS ≈ −1.17M), Δλ ≈ +1.6 (halving law; below 25.6k-lane resolution σ≈3.9). Q: 0 all arms (peak 1154). Per-cell at w=19: tlm_apply_forward_mod_add_register −1,554, tlm_apply_inverse_mod_sub_register −1,518 executed. Scan is deterministic-replicable (nonce-invariant lane seeding; two independent runs gave byte-identical censuses). |
| cheapest_discriminator | ≤60s per arm: `env TLM_COUT_ERASE_CAP=20 TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=400 TLM_DIRTY_SCAN_MAX=600 TLM_DIRTY_SCAN_SHOW=0 /tmp/prims-tgt/release/build_circuit 2>&1 \| grep DIRTY_SCAN` vs same without CAP; T via `TRACE_TLM_TOF=1 ... \| grep TLM_TOF_TOTAL` (2s). |
| predeclared_falsifier | Accept-rule form: reject if paired Δλ > 1σ. Measured +8.11 (w=19) and +3.18 (w=20) both exceed any σ band (scan σ≈5.5 sampling, but point estimates are 1.5-4× the ΔS-equivalent λ budget) → REJECTED under λ≳Q>T leverage: e^3.18 ≈ 24× and e^8.11 ≈ 3,340× grind multipliers for 0.16%/0.24% S. Also falsified: the naive 2^-w ceiling (Δλ≈23.5 predicted at w=19; measured 8.11 = 0.34× naive — hmr-½ + borrow discounts confirmed, consistent with the MSBS=19 precedent). |
| evidence_debt | No eval_circuit (forbidden this session); Δλ is build-side census (inner-loop signal, policy §1); w=21 λ extrapolated not measured. |
| budget | Spent: 8 scans × ~55s + 6 builds. Closed unless λ headroom appears. |
| unresolved_directions | w=21 direct measurement (needs ~2,400+ rounds to resolve +1.6); banded two-window cap (EXP-PR-03). |
| reopening_trigger | Cross-cell λ reduction of ≥ +4 (e.g. apply-skip/SCHED_J2 fixes per 02-lambda.md model) — then w=20's −2.35M S becomes budgetable; re-run the sweep then. |
| reproduction_commands | See cheapest_discriminator; artifacts `/tmp/prims-work/scanhi_{baseline,erase19,erase20}.log`, `scann2_*.log` (replica), `probe2.sh` arms, `countops.py` post-strip tallies. |
| classification | STRUCTURAL-λ — measured rejection (T-side LOCAL_PLATEAU: w=22 is T-optimal on the safe window given the λ wall). |

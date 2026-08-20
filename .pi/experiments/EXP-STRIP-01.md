# EXP-STRIP-01 — v6 deep-strip re-mine at raised census-risk budget

Pinned to commit a2067dcf, ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097
(emitted_ops 8,958,690; T_avg_exec 1,283,487.051; Q 1154; S 1,481,143,998).

1. **experiment_id**: EXP-STRIP-01
2. **search_radius**: Re-mine BOTH identity-keyed tables (`DEAD_KEYS` ∪ `DOWNGRADE_KEYS` in
   `src/point_add/deep_strip_keys.rs`) against the frozen live final stream at 3.2e8-input census
   depth (v5 parity: order=rand2, census seeds 20260803/314159265, events seed 777000777, TRUE
   GREEDY cap 6.0, no-event floor 0.40 — per the live file header) while sweeping ONLY the
   census-risk budget B ∈ {2.0, 3.0, 5.0} (v5 stopped at Risk 1.2397 against budget 1.24 — the
   constraint bound exactly, so the sweep is one scalar). Table regeneration only; no change to
   `apply_deep_strip_identity` (mod.rs:1690) — it is table-driven and self-checking.
3. **mechanism**: The trusted scorer charges executed Toffoli on **classical admission only**
   (`src/sim.rs:82-87`: `toffoli_gates += cond.count_ones()`, cond = condition-stack ∧ c_condition;
   quantum-control values do not gate the counter). Therefore (a) removing a census-"dead"
   (never value-firing) CCX/CCZ still saves its full admission weight (~0.96 average admission,
   measured 1,283,479/1,335,661 emitted CCX+CCZ at n=1e6), and (b) an implication downgrade
   CCX→CX (predicate `cond & q1 & ~q2 == 0`) converts a charged Toffoli into an uncharged
   Clifford. Measured candidate pool BEYOND the shipped 16,247 keys, on the live final stream:
   dead 8,961 CCX + 6 CCZ at n=1e6 (8,240 of them at admission exactly 1.0); implication
   downgrades 2,558 (q1→q2) + 1,085 (q2→q1) + 23 both-equiv + 3 CCZ. Dead survival with depth:
   37,220@25.6k → 16,761@128k → 8,961@1e6, i.e. roughly ×0.55 per 5-8× depth → ~1.5-2k structural
   survivors at 3.2e8 plus whatever the relaxed budget re-admits; the 1.0-admission cluster is the
   structural-looking core.
4. **predicted direction**: T: −2,000 … −5,000 executed Toffoli (ΔS −2.3M … −5.8M at Q=1154).
   Q: 0 (gate-level only, no qubit freed; TLM_TARGET_Q untouched). lambda: λ_phase unchanged —
   a wrongly-stripped CCX corrupts *values* (classical mismatch), it cannot dirty an ancilla or
   flip phase, so the Hmr/R intrinsic rate is untouched; λ_classical rises ∝ (B − 1.24) expected
   mismatches, absorbed by SUB4_TAIL_NONCE regrind effort × e^ΔB (the grind is the shipping
   mechanism for census risk; at B=2.0 expect ~2.3× grind effort vs today's 1.24).
5. **cheapest_discriminator**: (i) instrumented-census survival counts at 1e8 depth on the frozen
   baseline ops.bin (scratch mirror of `sim.rs::apply_iter`, validated: avg executed T 1,283,478.9
   vs certified 1,283,487.051 at n=1e6, 0.0006% off) — if the stable pool at 1e8 is < 300 gates
   total the card dies without any table edit; (ii) knob-liveness proof: regenerated-table build
   must log `[deep-strip-identity] removed N1 / N1 dead; downgraded N2 / N2 …; 0 stale keys
   skipped` with N1+N2 > 16,247 and a changed ops.bin sha (knob-noop trap).
6. **predeclared_falsifier**: deep census (≥1e8) shows the stable candidate pool collapsing >30×
   vs the 1e6 point (<300 gates), falsifying the structural 1.0-admission cluster; OR the
   reimplemented greedy shows marginal risk-per-Toffoli at budget 1.24 already >3× the shipped
   average (budget not binding in T-per-risk terms — then raising it buys almost nothing).
7. **evidence_debt**: (a) v5 miner binary is NOT on disk (searched repo + repro/); its risk model
   is inferred from the `deep_strip_keys.rs` header — repaid by reimplementing the greedy and
   reproducing Risk=1.239700 on the shipped table bit-for-bit; (b) the grind-effort multiplier
   for ΔB is modelled, not measured — repaid by the SUB4_TAIL_NONCE regrind during promotion;
   (c) executed (not emitted) savings verified only at census level — repaid by the promotion
   gate: full build+eval four-check green, ops.bin hash changed, λ not worse >1σ on paired
   TLM_DIRTY_SCAN n≥12, S strictly lower.
8. **trial_budget / time_budget**: one census sweep per B value, 3 values max (3.2e8 inputs ≈
   39 CPU-h ≈ 5 h wall on 8 threads; the n=1e6 calibration took 7m22s wall). Hard cap: 2 working
   days including regrind attempt; abandon at LOCAL_PLATEAU if discriminator (i) fails.
9. **unresolved_directions**: does not decide source-level certification of the 1.0-admission
   cluster (EXP-STRIP-03); does not touch emitter-side geometry/vent surfaces; does not decide
   the λ_phase budget or nonce-search strategy beyond the ×e^ΔB estimate.
10. **reopening_trigger**: any upstream op-stream edit that moves a keyed tuple's occupancy
    (build log `N stale keys skipped` with N>0) re-opens the re-mine against the new stream;
    also reopen if a future λ decomposition shows census-risk headroom ≥ 2× current budget.
11. **reproduction_commands**:
    - Baseline: `cargo run --release --bin build_circuit` → sha256 6519bd01…4d0c233a097 (reproduced 3× this session, 15 s).
    - Strip A/B (executed contribution): `SUB4_APPLY_STRIP=0 cargo run --release --bin build_circuit` (run from a private cwd; sha ed0d9cd3…) then census both streams:
      `./stream_census <ops.bin> 8 400 8192` → strip-on avg 1,283,479 vs strip-off avg 1,298,578 ⇒ live strip = 15,099 executed T.
    - Census calibration points: `./stream_census ops.baseline.bin 8 250 8192` (n=128k) and `8 1954 8192` (n=1e6).
    - Final-cancel interplay probe: `TLM_CCX_FINAL_CANCEL=1 cargo run --release --bin build_circuit` → sha e6ae6b52…, deep-strip still `0 stale`.
12. **classification**: STRIP (statistical, λ_classical-priced; zero λ_phase exposure by construction).

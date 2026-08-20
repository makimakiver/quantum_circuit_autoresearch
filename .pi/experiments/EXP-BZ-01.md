# EXP-BZ-01 — Apply tail-skip extension, census-gated (STRUCTURAL-lambda)

Pinned: commit a2067dcf, ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097,
profiler baseline TLM_TOF_TOTAL emitted=1,352,434 expected=1,299,076.0 (pre-postpass), live S=1,481,143,998 (T=1,283,487, Q=1,154).

1. **experiment_id**: EXP-BZ-01
2. **search_radius**: extend the live apply-tail skip family by one boundary step each, all combinations of:
   `TLM_APPLY_ADD_SKIP_LASTK` 1→{2,3}; `TLM_APPLY_FWD_CSWAP_SKIP_LAST`/`TLM_APPLY_INV_CSWAP_SKIP_LAST` 3→{4,5} (paired equal);
   `TLM_APPLY_FWD_S2_ZERO_LAST`/`TLM_APPLY_INV_S2_ZERO_LAST` 1→{2,3} (paired equal). Per-direction fine knobs
   `TLM_APPLY_ADD_SKIP_FWD`/`_INV` may be used if the census shows symbol asymmetry between the two passes.
   Boundary predicates live at gcd.rs:192-243 (`apply_fwd_cswap_skip`, `apply_inv_cswap_skip`, `apply_fwd_s2_zero`,
   `apply_inv_s2_zero`, `apply_add_skip`); defaults forced at mod.rs:1184, 1191, 2210, 2218-2221.
3. **mechanism**: each replayed iteration i runs a full 256-bit apply step (fwd: mod_add 635.2 + cswap 256 + cdouble 318.0
   executed; inv mirrored). Skipping at iteration i removes the skipped component at 0-13% discount only (swap phases are
   0.000% discount — full price). The skips are sound only where the tape symbol the component depends on is
   classically constant over the input distribution: cswap is controlled by `swp` (truncated v<u compare, recomputed by
   `swap_decision_uncompute_vented`), the add/sub by `sub` (v0 after the t1/s2 right-shifts), the s2-doubling by `s2`
   (second-zero flag). Near convergence (SCHED_J2[256..260] = 12,12,10,9,9,9) the walk state is nearly deterministic,
   which is what makes the current skips (i=260 add/s2zero, i>=258 cswap) almost-free. Extending one step further
   banks the same per-iteration saving against the marginal symbol entropy at i=259 (add/s2) and i=257 (cswap).
4. **predicted direction**: T negative, Q unchanged (no persistent qubits touched), lambda positive.
   Measured at a2067dcf by this scout (build-only, TRACE_TLM_TOF expected-executed):
   - ADDK=2: ops 8,958,690→8,948,057 (−10,633), expected −1,596.5 (total 1,297,479.5), sha 6aa87651da8396d8…
   - ADDK=3: ops −23,225, sha 6d4d65c0f2881229… (≈ −3.2k executed)
   - CSWAP=4 (both directions): ops −1,536, expected −512.0 exactly (two swap phases −256 each), sha 8ea39d6c87a4a4c6…
   - CSWAP=5: ops −3,072, sha 8c33c1a52c67d06e…
   - S2ZERO=2 (both): ops −874, expected −643.0, sha aa620d9c662a994b…
   Full level-2 bundle ≈ −2,750 executed Toffoli ≈ −0.21% T ⇒ ΔS ≈ −3.2M at Q=1154. Lambda: +9024·p(violation)
   per assumed symbol per run (mm units of 02-lambda.md); the 258-era model priced the *current* skip set at 5.30mm.
5. **cheapest_discriminator**: (a) classical census (see falsifier) — no quantum build; (b) build probe:
   `CARGO_TARGET_DIR=/tmp/<t> TLM_APPLY_ADD_SKIP_LASTK=2 TRACE_TLM_TOF=1 cargo run --release --bin build_circuit`
   and read `TLM_TOF phase=tlm_apply_*` + `TLM_TOF_TOTAL` — already done, numbers above; ops.bin sha must differ from
   6519bd01… (knob-liveness proven for all four flags).
6. **predeclared_falsifier**: census over >=1e8 random (tx,ty) walk pairs must show, at the candidate iteration,
   p(swp=1 at i=257) <= 1e-4 for CSWAP=4, p(sub=1 at i=259) <= 1e-4 for ADDK=2, p(s2=1 at i=259) <= 1e-4 for S2ZERO=2
   (each threshold = +0.9mm lambda, i.e. <1 extra mismatch per 9024-shot run). Any marginal above its threshold kills
   that flag level BEFORE implementation. Secondary falsifier: paired `TLM_DIRTY_SCAN` (n>=12) at the candidate bundle
   showing classical-mismatch increase > 1 sigma kills the bundle.
7. **evidence_debt**: lambda at a2067dcf is UNMEASURED (frontier.json lambda=null; the 5.30mm figure is 258-era and
   its line numbers have drifted). The census prices the marginals, but the promotion gate must still run
   `TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12` paired baseline-vs-candidate on the same nonce set, and the accept
   rule is lambda not worse >1 sigma with S strictly lower. Also unmeasured: interaction between skips and the
   post-pass strip stack (apply_m60_dead_t10 / ccx_final_cancel) — the emitted delta may shrink post-strip; the
   promotion run repays this.
8. **trial_budget / time_budget**: census script 1 day; 4 build probes 10 min each; 2x12 dirty-scan rounds ~2 h wall
   (orchestrator-owned eval time excluded). Hard stop after one refinement cycle if lambda gate fails.
9. **unresolved_directions**: does NOT decide the optimal *set* of skip levels jointly with ITERS (a 262-iteration
   circuit changes all tail marginals); does NOT decide per-direction asymmetric skips (census data will enable, card
   does not consume); does NOT touch walk-side dead-gate certificates at the same iterations.
10. **reopening_trigger**: any future ITERS change (schedule re-block) or new tail-census at >=1e9 samples showing a
    marginal two orders below threshold reopens the next level; likewise a certified lambda drop below ~15 reopens
    the whole family (more grind budget to spend).
11. **reproduction_commands**:
    `cd /Users/makimakiver/ecdsafail-challenge && CARGO_TARGET_DIR=/tmp/bezout-scout-target TRACE_TLM_TOF=1 cargo run --release --bin build_circuit`
    (baseline; expect emitted ops 8,958,690, TLM_TOF_TOTAL expected=1299076.0, ops.bin sha 6519bd01424d5513…)
    `CARGO_TARGET_DIR=/tmp/bezout-scout-target TLM_APPLY_ADD_SKIP_LASTK=2 TRACE_TLM_TOF=1 cargo run --release --bin build_circuit`
    (expect expected=1297479.5, sha 6aa87651da8396d8…) — analogous for CSWAP=4 / S2ZERO=2 above.
12. **classification**: STRUCTURAL-lambda (T-negative trade priced in executed Toffoli at the measured phase discounts).

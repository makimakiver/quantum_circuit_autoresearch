# EXP-SQ-01 — early `sum` unbuild in the modular square (vent-starvation recovery)

- **experiment_id**: EXP-SQ-01
- **search_radius**: `src/point_add/trailmix_ludicrous/square.rs::mod_square_sub_pm_secp256k1_symmetric` only —
  move one call (`unbuild_sum_hi_lo`) earlier; no arithmetic change, no new primitives.
- **mechanism** (gate-level): the 129-qubit `sum` register (λ_lo+λ_hi carry-sum, built by `build_sum_hi_lo`)
  is consumed ONLY by the c-sub-square (`c = sum²`) build/unbuild, yet it stays allocated through the a/b
  sub-square phases and is uncomputed last (`square_sum_hi_lo_unbuild`, square.rs end of
  `mod_square_sub_pm_secp256k1_symmetric`). The square's modular APPLY phases
  (`square_{a_lo,b_hi}_apply_shifted_128_add`, `square_b_hi_apply_f_times_sub`,
  `square_c_sum_apply_shifted_128_sub`) run `add_cout_vented_skip_dead` (arith.rs:1593) whose vent budget is
  `TLM_SQUARE_PEAK_CAP(1154) − active_qubits − margin(0)`. Live-without-vents during an a-apply ≈
  x2(256)+y2(256)+sum(129)+a_prod(256)+anc ≈ 898 → budget ≈ 256, and the applies sit at active_max=1154
  (TRACE_PHASE_ACTIVE probe) i.e. vent-filled to the cap and starved. Freeing `sum` right after the c unbuild
  adds +129 to the vent budget of the three shifted-128 applies; each vented carry replaces one uncompute CCX
  with an `hmr`+`cz_if` measurement pair.
- **predicted T/Q/λ signs + magnitudes** (executed T): measured dose-response on my canonical build
  (ops.bin sha256 6519bd01… reproduced):
  - budget −129 (`TLM_SQUARE_VENT_MARGIN=129`): +3,034 CCX total (linear, 3 apply phases +770 each, f·b +597).
  - budget +129 (`TLM_SQUARE_PEAK_CAP=1283`, probe-only): −542/apply ×3, f·b −1 (saturated) → **−1,630 CCX
    emitted**, apply-phase discounts ~3% → **≈ −1,580 executed T** (T: 1,283,487 → ~1,281,900).
  - Q: 0 (peak 1154 is co-held by gcd-apply phases; square caps at TLM_SQUARE_PEAK_CAP=1154 unchanged).
  - λ: ≥ 0 small — +~1.6k hmr vent sites in the square; square/non-ModDiv λ share is ~2.1 of 18.1 mm
    (role brief), but must be paired-measured (n≥12).
  - ΔS ≈ −1.8M (−1,580 × 1154).
- **cheapest_discriminator** (done, env-flag build probe, no eval):
  `TLM_SQUARE_PEAK_CAP=1283 TRACE_TLM_CCX=1 build_circuit` vs baseline — square apply phases
  2,342/2,344 → 1,800 CCX each; `TLM_SQUARE_VENT_MARGIN=129` gives the mirrored +3,034. For the real edit:
  `TLM_SQUARE_EARLY_SUM_UNBUILD=1` + op-count/ops.bin diff (anti-noop) + `TLM_SQ_SELFTEST`.
- **predeclared_falsifier**: (a) net emitted-CCX gain < 500 after the full post-pass strip stack re-mines
  (allocator free-pool reorder can stale identity-keyed strips — see square.rs
  `TLM_SQUARE_SHIFTED128_LOW_TAGS` comment about 4,486 keys); (b) λ worse by >1σ at n≥12 paired
  `TLM_DIRTY_SCAN`; (c) any of the four eval checks not green on a ground seed.
- **evidence_debt**: probe-level only (env-flag census + selftests). No λ scan, no eval run, no ops.bin diff
  of the actual edit (edit not yet made — implementer owns it).
- **trial/time_budget**: implementer 1 flag-gated edit + 2 builds (~15 min each) + selftest; then full
  build+eval + n=12 paired dirty scan before accept (~2-3 h wall).
- **unresolved_directions**: (i) does the freed-129 pool reuse change `cuccaro_call_has_structurally_dead_carry`
  mining (TLM_CUCCARO_SKIP_STRUCTURAL_DEAD_CALLS=1) in the a/b applies? measure, don't assume;
  (ii) cross-cell: square-apply↔pair2-multiply fusion (feed x2 = X+2x0−λ² into `tlm_forward_multiply`
  staging without re-materializing a fully-reduced x2) — escalate to cross-cell-architect, do not card here.
- **reopening_trigger**: any future change that raises square-phase live-qubits (new scratch) or lowers
  TLM_SQUARE_PEAK_CAP re-starves these vents — re-run the margin dose-response then.
- **reproduction_commands**:
  ```bash
  # canonical baseline (verified 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097)
  cd /tmp/sqscout && CARGO_TARGET_DIR=/tmp/sqscout-target /tmp/sqscout-target/release/build_circuit
  # dose-response probes (single-tenant, cwd=/tmp/sqscout so ops.bin never clobbers the shared artifact)
  for M in 0 32 64 129; do TRACE_TLM_CCX=1 TLM_SQUARE_VENT_MARGIN=$M \
    /tmp/sqscout-target/release/build_circuit 2>&1 | grep -E 'phase=square_|TLM_CCX_TOTAL'; done
  for CAP in 1154 1200 1283; do TRACE_TLM_CCX=1 TLM_SQUARE_PEAK_CAP=$CAP \
    /tmp/sqscout-target/release/build_circuit 2>&1 | grep -E 'phase=square_.*apply|TLM_CCX_TOTAL'; done
  # peak attribution
  TRACE_TLM_PROFILE=1 TRACE_PHASE_ACTIVE=1 /tmp/sqscout-target/release/build_circuit 2>&1 | grep '^TLM_PHASE'
  ```
- **classification**: STRIP (bit-exact function: same unitary on (x2,y2); ops.bin must change).

## Implementer instruction (top card)

- **Env flag**: `TLM_SQUARE_EARLY_SUM_UNBUILD` (default off; `build()` does NOT force it — acceptance via
  playbook §6 first, then hard-set like M023 if accepted).
- **File/edit**: `src/point_add/trailmix_ludicrous/square.rs`, in `mod_square_sub_pm_secp256k1_symmetric`:
  ```rust
  circ.set_phase("square_c_sum_unbuild");
  symmetric_square_into_prod_reverse(circ, &sum, c_prod);
  if std::env::var("TLM_SQUARE_EARLY_SUM_UNBUILD").ok().as_deref() == Some("1") {
      circ.set_phase("square_sum_hi_lo_unbuild");
      unbuild_sum_hi_lo(circ, lambda, sum);          // moved up: frees 129 qubits before a/b phases
  }
  // ... a_lo phases, b_hi phases unchanged ...
  if <flag off> { circ.set_phase("square_sum_hi_lo_unbuild"); unbuild_sum_hi_lo(circ, lambda, sum); }
  ```
  (hoist the flag read to a helper fn next to `square_addsub_enabled()`; keep the phase label identical in
  both positions so TRACE tools stay comparable.)
- **Invariant pairs to preserve**:
  1. `build_sum_hi_lo`/`unbuild_sum_hi_lo` remain exact inverses around the c-square only — `sum` is touched
     by nothing else (verified: a/b phases read only `lambda[..128]`/`lambda[128..]` and `output_reg`).
  2. `lambda` (y2) is READ-ONLY in the square (it is the pair-2 multiply input); the unbuild's
     `cx(lambda[i], sum[i])` gates are control-only on λ and complete before the a-phase starts (sequential,
     not interleaved — no commutation question).
  3. `output_reg` (x2) must remain the same physical register and hold `x_old − λ² mod p` at square exit.
  4. All scratch (`sum`, `c/a/b_prod`, vent ancs) zero-and-free'd; phase word untouched (no new phase source;
     `hmr` vents consume their bits via `cz_if`).
  5. Vent borrow discipline: `add_cout_vented_skip_dead` still only vents below `TLM_SQUARE_PEAK_CAP`; the
     freed `sum` ids return to the allocator pool — do NOT also change `TLM_SQUARE_VENT_MARGIN`/`PEAK_CAP`.
- **Verify order**: `TLM_SQ_SELFTEST=1 TLM_SQ_SELFTEST_ONLY=1` → build with flag on, diff op count
  (expect ≈ −1.6k CCX; anti-noop) → `SQUARE_WINDOW_SELFTEST` → full `build_circuit`+`eval_circuit` (four
  checks, new SUB4_TAIL_NONCE grind per round-1 §2 reality) → n≥12 paired `TLM_DIRTY_SCAN`.

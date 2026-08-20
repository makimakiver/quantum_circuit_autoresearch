# EXP-STRIP-03 — source-certify the admission-1.0 never-firing CCX cluster (λ-free strip)

Pinned to commit a2067dcf, ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097.

1. **experiment_id**: EXP-STRIP-03
2. **search_radius**: the 8,240 CCX gates in the live FINAL stream that the n=1e6 instrumented
   census found admitted on every input/lane (admission exactly 1.0) yet never value-firing, plus
   the 2,419 admission-1.0 implication-downgrade candidates. Attribute them to emitting call
   sites (`TRACE_OP_SITES=1` + `CONSTPROP_DISABLE=1 SINGLE_CCX_FANOUT_DISABLE=1` attribution
   protocol from dirtyscan.rs header), cluster by routine, and attempt per-cluster source-level
   never-firing / implication PROOFS (UNSAT mitres over the routine's pre/post state — the
   method that already worked once, memory/06 §"One source-level implication is exact").
3. **mechanism**: a PROVEN dead key or PROVEN implication removes an admission-charged Toffoli
   with ZERO λ_classical price (today every one of the 16,247 shipped keys is statistical,
   priced into the 1.24 census-risk budget that EXP-STRIP-01 wants to raise). Proving clusters
   converts λ-budget into T directly and de-risks the v6 re-mine. Expected per-gate saving
   ≈ 0.93-1.0 executed T (admission 1.0 by construction of the cluster).
4. **predicted direction**: T: −3,000 … −7,700 executed Toffoli if the 1.0-admission dead
   cluster survives deep census and certifies (8,240 × ~0.93 upper bound; deep-census survival
   discount applies), ΔS −3.5M … −8.9M at Q=1154. Q: 0. lambda: 0 for certified keys (exact);
   the certificates also FREE census-risk budget reusable by EXP-STRIP-01.
5. **cheapest_discriminator**: re-run the census at 1e8 depth (≈5 h wall, 8 threads) and count
   how many of the 8,240 admission-1.0 gates are still never-firing; then op-site attribution on
   a 100-gate stratified sample — if >70% of the sample lands in ≤5 emitting routines the
   clustering premise holds.
6. **predeclared_falsifier**: the 1.0-admission cluster disperses across >50 unrelated call
   sites with no shared invariant (no certificate target), OR deep census fires >10% of the
   cluster (it was shallow-census noise, contradicting the structural hypothesis).
7. **evidence_debt**: emitting-site attribution not yet run (census was stream-level only);
   repaid by the discriminator. UNSAT certificate cost per cluster unknown (prior single
   implication took two solver runs) — cap 2 local CPU-hours per cluster class.
8. **trial_budget / time_budget**: census 5 h wall + attribution 2 h + up to 5 mitre classes ×
   2 CPU-h; hard cap 3 working days.
9. **unresolved_directions**: does not decide the statistical budget sweep (EXP-STRIP-01) nor
   whether proven keys should be encoded as source-indexed certificates (ordinal-shift-immune,
   memory/06) vs new DEAD_KEYS entries — pick after seeing cluster topology.
10. **reopening_trigger**: census tooling exists (this session's mirror validated to 0.0006% on
    executed T); reopen whenever a new stream ships or when the EXP-STRIP-01 budget sweep
    stalls below its predicted T range.
11. **reproduction_commands**:
    - Census (n=1e6): `./stream_census /path/to/ops.baseline.bin 8 1954 8192` →
      `dead by admission …==1.0: 8240`, `downg …==1.0: 2419`, avg executed T 1,283,478.9.
    - Baseline build: `cargo run --release --bin build_circuit` → 6519bd01….
    - Site attribution: `CONSTPROP_DISABLE=1 SINGLE_CCX_FANOUT_DISABLE=1 TRACE_OP_SITES=1 cargo run --release --bin build_circuit` (dirtyscan.rs:14 protocol).
12. **classification**: STRIP (exact, proof-carrying; complements EXP-STRIP-01's statistical lane).

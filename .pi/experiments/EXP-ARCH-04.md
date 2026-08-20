# EXP-ARCH-04 — Strip-key lifecycle & exact-synthesis boundary composite (strip×census×certificates×regrind)

Status: SCOPED-COMPOSITE (cross-cutting unlock). Consolidates EXP-SLOPE-02 (re-mine tooling), EXP-STRIP-01
(statistical budget sweep), EXP-STRIP-03 (UNSAT certificates), EXP-STRIP-02 (exact cancel) under one promotion
protocol with a single λ accounting. Pinned a2067dcf / ops.bin 6519bd01… (no new measurement this session).

1. experiment_id: EXP-ARCH-04
2. search_radius: the deep-strip key space of the live final stream: 12,202 dead + 4,045 downgraded shipped
   keys (0 stale, fully live; executed credit 15,099 measured via SUB4_APPLY_STRIP=0 A/B); the measured
   candidate pool beyond it (dead 8,961 CCX + 6 CCZ at n=1e6, 8,240 at admission exactly 1.0; downgrades
   2,558+1,085+23+3); and the lost v5 miner (header spec: TRUE GREEDY cap 6.0, no-event floor 0.40, census
   3.2e8, Risk 1.239700 vs budget 1.24).
3. mechanism — the exact-synthesis boundary (architect's design content):
   - Partition the key space into **(E) exact/certified** keys — UNSAT-mitre never-firing proofs, wire-equality
     identities, bit-exact transforms (ccx_final_cancel class) — λ-price 0, regrind contribution limited to the
     stream-hash change itself; and **(S) statistical** keys — census-risk-priced, budget B, regrind ×e^(B−1.24).
   - Promotion protocol: (1) rebuild the miner and reproduce the shipped table's effect on the UNCHANGED stream
     (12,202/12,202 + 4,045/4,045 at 0 stale — self-consistency gate); (2) attribute + certify the admission-1.0
     cluster (E-part grows, S-part shrinks); (3) sweep B ∈ {2.0, 3.0, 5.0} only over the residual S-pool;
     (4) RULE: any op-stream-changing card's T number is not final until its tripwired keys are re-mined
     (stale-key tax ≈ 1 executed/key; measured 1,032 keys for GAP δ3, 3,565 for cap 1153) — this rule binds
     EXP-ARCH-01/02/03 and every SLOPE/BZ geometry card.
4. predicted direction: T −3k..−7.7k executed λ-free (certificates) plus −2k..−5k executed λ-priced (budget
   sweep) (ΔS −3.5M..−8.9M and −2.3M..−5.8M at Q=1154); Q 0; λ_phase 0 by construction (a wrongly-stripped CCX
   corrupts values, never phase/ancilla); λ_classical = Σ_S risk with regrind multiplier MEASURED at promotion
   (wall-time), not modelled.
5. cheapest_discriminator: (i) 1e7-sample pilot re-miner on the frozen baseline stream must reproduce ≥90% of
   the committed keys (else the tooling premise dies before any census spend); (ii) deep census at 1e8: if the
   stable pool < 300 gates, the statistical lane dies (inherited STRIP-01 falsifier); (iii) site-attribution
   sample: >70% of a 100-gate stratified sample inside ≤5 routines, else clustering premise dies (STRIP-03
   falsifier).
6. predeclared_falsifier: any of (i)/(ii)/(iii) above; plus: re-mined baseline not effect-equivalent
   (different removed/downgraded counts at 0 stale); certificates' four-checks eval not green on an A/B.
7. evidence_debt: v5 risk model exists only in the committed table header (reimplementation must reproduce
   Risk=1.239700 bit-for-bit on the shipped table before any new mining is trusted); grind-effort ×e^ΔB is
   modelled (repaid by the promotion regrind); census-vs-executed calibration (0.964 global) is an average —
   the 1.0-admission cluster is self-calibrating by construction.
8. trial_budget / time_budget: pilot 1e7 (~25 min calibrating from the 7m22s n=1e6 point) → full 3.2e8 census
   (~5 h wall, 8 threads) per B value, ≤3 values; attribution 2 h; ≤5 mitre classes × 2 CPU-h; hard cap
   3 working days total across the composite.
9. unresolved_directions: certificate encoding (source-indexed vs table ordinals — decide after cluster
   topology); re-enabling per-family structural-dead predicates at BAKED_ITERS:=261 (separate paired
   experiment, walk+apply+square families).
10. reopening_trigger: every accepted stream change (the tripwire IS the trigger); λ decomposition showing
    census-risk headroom ≥2× budget.
11. reproduction_commands (inherited):
    ```
    CARGO_TARGET_DIR=/tmp/w1scout-target cargo run --release --bin build_circuit 2>&1 | grep deep-strip-identity
    # -> removed 12202/12202 dead; downgraded 4045/4045; 0 stale keys skipped
    SUB4_APPLY_STRIP=0 cargo run --release --bin build_circuit   # strip-off arm for the 15,099 executed A/B
    TLM_CCX_FINAL_CANCEL=1 cargo run --release --bin build_circuit  # exact-cancel bundle rider (sha e6ae6b52…)
    ./stream_census ops.bin 8 1954 8192    # n=1e6 pool calibration (dead==1.0: 8240; downg==1.0: 2419)
    ```
12. classification: STRIP composite (exact + statistical lanes under one λ ledger).

## Composite-task specifics
- participating owners: strip cell (deep_strip_keys.rs + `apply_deep_strip_identity` mod.rs:1691 + tripwire log
  mod.rs:1777); census-tooling owner (stream_census mirror, validated 0.0006% on executed T); memory/05-06
  maintainers (certificate records, dead-end ledger); eval/regrind owner (SUB4_TAIL_NONCE, mod.rs:1790/2410).
- interfaces: keying = tuple-occupancy on the FINAL post-pass stream (strip runs last-before-nonce);
  certificates address either source call-sites (ordinal-immune) or stream tuples (ordinal-fragile) — the
  encoding decision is an interface decision made once, at stage (2), and recorded in memory/06.
- invariants: (I1) self-consistency — a regenerated table must reproduce the shipped effect on the unchanged
  stream before any changed-stream mining is trusted; (I2) E/S partition is exclusive and exhaustive over new
  keys; (I3) λ accounting: only S-keys contribute census risk; E-keys carry their UNSAT artifact; (I4) the
  strip stays last-before-nonce (certificate class is stream-addressed).
- proof obligations: (P1) pilot ≥90% key reproduction; (P2) per-cluster UNSAT artifacts for E-keys;
  (P3) four-checks green on the certified A/B; (P4) regrind wall-time multiplier measured vs e^ΔB model;
  (P5) S strictly lower at each promotion step; (P6) λ paired n≥12 for any S-key promotion (E-keys exempt by
  proof, still four-checks bound).
- staged tests: pilot → self-consistency → deep census + attribution → certificates → B sweep → bundled
  promotion with STRIP-02's exact cancel riding the same regrind.
- rollback unit: the regenerated `deep_strip_keys.rs` table is ONE file commit, plus ONE env knob
  `SUB4_STRIP_RISK_BUDGET` (default 1.24 = shipped parity). Revert = restore the prior table commit / unset
  knob. Certificates are additive documentation (no runtime effect).
- single-cell results needing re-validation if this lands: STRIP-01's census pool numbers (re-census per new
  stream); the 15,099 executed strip credit (SUB4_APPLY_STRIP A/B re-run); memory/06 "One source-level
  implication is exact" citation scope; every geometry card's stale-key line item; frontier refresh +
  nonce regrind (×e^ΔB effort priced in wall-time).

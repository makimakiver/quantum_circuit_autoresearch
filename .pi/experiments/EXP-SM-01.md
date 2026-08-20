# EXP-SM-01 — v5 census-miner rebuild + baseline self-consistency re-mine (unlock card for EXP-SQ-01 / EXP-STRIP-01 / EXP-STRIP-03)

Pinned to working tree at a2067dcf + uncommitted implementer edit (square.rs TLM_SQUARE_EARLY_SUM_UNBUILD,
flag-gated, default off). Baseline ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097
(reproduced 3x this session, 13.6-19.2 s). Frontier λ=17.62 (measured, EXP-PR). All numbers below are
live-measured this session unless marked "prior session" or "inferred".

1. **experiment_id**: EXP-SM-01

2. **search_radius**: rebuild the greedy census miner (lost with the VM per memory/05:124-126; sole in-repo spec =
   `deep_strip_keys.rs:1-3` header: "TRUE GREEDY v5 cap 6.0, no-event floored at 0.40. order=rand2. Census 3.2e8 seed
   20260803 / 3.2e8 seed 314159265; events 3.2e7 seed 777000777. Risk 1.239700 (budget 1.24)") on top of the scout
   census engine (preserved at `.pi/experiments/EXP-SM-01-tooling/`, validated 0.0007% on executed T this session),
   then re-mine the BASELINE strip-off stream once and re-cut the greedy at B ∈ {1.24, 2.0, 3.0, 5.0}. Table
   regeneration only; `apply_deep_strip_identity` (mod.rs:1691) is untouched and self-checking. Consumers: EXP-STRIP-01
   (the B-sweep itself), EXP-SQ-01 (variant-stream re-mine, see mechanism), EXP-STRIP-03 (pool prioritization rides
   the same census).

3. **mechanism**:
   - Miner = engine + 4 missing pieces: (i) per-key EVENT COUNTS (fired_pop per op index, not just the ever-fired
     mask) on a dedicated events pass; (ii) keying identical to the applier: (kind∈{13,14}, q_control2, q_control1,
     q_target, c_condition) + occurrence ordinal in stream order + census-time tuple occupancy (mod.rs:1705-1727);
     (iii) the greedy: rank keys by executed-T-per-risk, admit while cumulative Risk ≤ B, per-key cap 6.0, no-event
     keys floored at 0.40 pseudo-events (p_floor = 0.40/3.2e7 ≈ 1.25e-8), order=rand2 tie-break; (iv) emitter of
     `deep_strip_keys.rs`-format source (7-tuple DEAD_KEYS / 8-tuple DOWNGRADE_KEYS with act∈{1,2}).
   - Substrate: the STRIP-OFF final stream (`SUB4_APPLY_STRIP=0`, sha ed0d9cd306cc2d13683f8b45225c07005effc365f3f84b0911b2cabe52e6f555,
     8,970,902 ops — reproduced this session) — the keyed gates must be present to be observed. Stream format:
     8-byte MAGIC "QECCOPSZ" + u64 LE count + zstd frame of 56-byte LE records (build_circuit.rs:33-80).
   - Census verdicts are stream-order-coupled (Hmr/R consume the XOF per op in order), so keys NEVER transfer
     across op-stream changes — a re-mine is always a fresh census; this is why the tripwire discards (mod.rs:1731-1737)
     rather than re-addresses.
   - Risk model is a RECONSTRUCTION TARGET, not a known function: the 1.24 budget prices expected classical
     mismatches (λ_classical) absorbed by the SUB4_TAIL_NONCE regrind (×e^ΔB effort model). The exact aggregation is
     unknown (one near-closing scaling, Σunits×9024/3.2e7 with Σunits≈4,396, contradicts n_dead×0.40=4,880.8), so
     acceptance = the rebuilt evaluator reproduces Risk=1.239700 bit-for-bit on the shipped 16,247-key table.
   - SQ-01 unlock, measured LIVE this session: `TLM_SQUARE_EARLY_SUM_UNBUILD=1` (implementer edit in tree) →
     `[deep-strip-identity] removed 6813/12202; downgraded 2635/4045; 6799 stale keys skipped`; variant sha
     486fa8945846a0fafbb9215d07a03f2eea1f9217f3705254c2ded42e61147dc6; +5,844 emitted Toffoli (CCX +5,806 / CCZ +38),
     +1,086 new vent sites (R/Hmr/CZ/PushC/PopC each +1,086 — the vent gain, prior-session "−1,086 CCX"), executed T
     1,288,962.5 at n=65,536 vs 1,283,477.7 baseline arm ⇒ +5,485 executed T = ΔS ≈ +6.3M WORSE as-is. Stale-key tax
     6,799 ≈ +6.8k executed swamps the −1.1k vent gain exactly as diagnosed. Re-mine target on the variant stream:
     recover ≈6,799 keys (≈−6.3k executed) ⇒ net ≈ −500..−800 executed T (ΔS ≈ −0.6..−0.9M), plus whatever the
     variant's own dead pool adds (candidates at n=65,536: 22,532 dead + 6,773 downgrades vs baseline 21,072 + 6,040).

4. **predicted direction**: (b)-lane baseline tables at B∈{2,3,5}: T −2,000…−5,000 executed, λ_classical +（B−1.24)
   expected mismatches (regrind ×e^ΔB), λ_phase 0 by construction (mis-strips corrupt values, never phase/ancilla);
   ΔS −2.3M…−5.8M at Q=1154. (a)-lane SQ-01 re-enable after variant re-mine: net −500…−800 executed T vs TODAY'S
   frontier (ΔS ≈ −0.6M..−0.9M), Q 0. λ_phase 0 both lanes.

5. **cheapest_discriminator**: 1e7-input pilot (~25-43 min nohup) on the strip-off substrate must re-derive ≥90% of
   the shipped 16,247 key set (ARCH-04 discriminator (i)); then the risk-evaluator fit must land exactly on
   1.239700 over the shipped table. If either fails, the rebuild is not trusted for any B-sweep.

6. **predeclared_falsifier**: pilot key-overlap <90%; OR no member of the risk-functional family reproduces
   Risk=1.239700 exactly (then the B-sweep lane is unpriceable — park and escalate the certificate lane
   EXP-STRIP-03 instead); OR a re-mined-baseline build (B=1.24) whose four-checks eval is not green on an A/B.

7. **evidence_debt**: (a) risk functional inferred from a 3-line header — repaid by the 1.239700 reproduction;
   (b) grind multiplier ×e^ΔB modelled — repaid by the promotion regrind wall-time; (c) executed (not emitted)
   savings verified at census level only — repaid by the promotion gate (full build+eval, four checks green,
   ops.bin hash changed, paired TLM_DIRTY_SCAN n≥12 λ not worse >1σ, S strictly lower); (d) SQ-01 +5,485 executed
   delta measured on UNPAIRED census inputs (path-seeded pools) — sign and magnitude decisive at ±40 executed,
   exact net reserved for the paired promotion measurement.

8. **trial_budget / time_budget**: miner build 0.5-1.5 days implementer (200-400 LoC on the preserved engine);
   census programme ONCE (pool is B-independent; B only re-cuts the greedy, seconds each): 2×3.2e8 + 3.2e7 =
   6.55e8 inputs = 28-80 h wall on 8 threads nohup (measured band 2.3e3-6.5e3 inp/s); STRIP-03's 1e8
   prioritization census (4.3-12 h) may ride the same nohup window. Hard cap 3 working days + 1 nohup weekend.

9. **unresolved_directions**: certificate encoding for STRIP-03 E-keys (source-indexed vs table ordinal — decide
   after cluster topology; harness precedent repro/y6_source_invariant.py); whether downgrades belong inside the
   risk budget at all (applier doc says exact, header implies all 16,247 priced — resolve during the 1.239700 fit);
   SQ-01 promotion order (re-mine first, then λ paired scan on the re-mined variant).

10. **reopening_trigger**: every accepted op-stream change (the 6,799-key tripwire IS the trigger — `grep 'stale
    keys' build.log`, baseline = 0); λ decomposition showing census-risk headroom ≥2× budget.

11. **reproduction_commands**:
    ```bash
    # baseline (verified 6519bd01…4c233a097; NEVER build in the repo cwd — preserve the canonical ops.bin)
    cd /tmp/sqscout && CARGO_TARGET_DIR=/tmp/sqscout-target cargo build --release --bin build_circuit
    CARGO_TARGET_DIR=/tmp/sqscout-target /tmp/sqscout-target/release/build_circuit 2>&1 | grep deep-strip-identity
    #   -> removed 12202 / 12202 dead; downgraded 4045 / 4045; 0 stale keys skipped
    # SQ-01 variant (uncommitted square.rs edit required — implementer owns it)
    TLM_SQUARE_EARLY_SUM_UNBUILD=1 CARGO_TARGET_DIR=/tmp/sqscout-target \
      /tmp/sqscout-target/release/build_circuit 2>&1 | grep deep-strip-identity
    #   -> removed 6813 / 12202 dead; downgraded 2635 / 4045; 6799 stale keys skipped  (sha 486fa894…)
    # strip-off census substrate
    SUB4_APPLY_STRIP=0 CARGO_TARGET_DIR=/tmp/sqscout-target \
      /tmp/sqscout-target/release/build_circuit   # -> sha ed0d9cd3…, 8970902 ops
    # census engine (rebuild from the preserved copy)
    cd /tmp/streamopt/census && cp <repo>/.pi/experiments/EXP-SM-01-tooling/stream_census_main.rs src/main.rs \
      && cp <repo>/.pi/experiments/EXP-SM-01-tooling/Cargo.toml . && cargo build --release
    ./target/release/stream_census /tmp/streamopt/ops.stripoff.bin 8 128 4096    # n=65,536 smoke (≈10 s)
    nohup ./target/release/stream_census /tmp/streamopt/ops.stripoff.bin 8 3907 8192 > census_v5_a.log 2>&1 &
    #   n≈2.0e6 calibration arm; scale batches/thread ×160 for the 3.2e8 passes (nohup, 14-39 h each)
    # kind-delta A/B (fingerprint both streams)
    cd src/point_add/memory/repro && python3 -c "from pathlib import Path; from artifact_io import fingerprint; \
      print(fingerprint(Path('/tmp/streamopt/ops.bin'))['operation_kind_counts'])"
    ```

12. **classification**: STRIP (statistical, λ_classical-priced via budget B; zero λ_phase exposure by construction;
    unlocks the parked EXP-SQ-01 vent win and feeds EXP-STRIP-03's certificate lane).

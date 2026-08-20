# EXP-QFL-01 — Live Q-floor certification: late-window census + early-window scratch compression probe (B1×B3×B2×P)

Status: SCOPED (rank 2 of Wave-1 reframing scout). Half-executed during the scout (two of three censuses done).

1. **experiment_id**: EXP-QFL-01
2. **search_radius**: (i) B0 owner censuses of the remaining plateau windows: unwalk-2 early (ops ≈ 5,890,000–
   5,950,000, tape-full side) and walk-1 terminal (ops ≈ 3,340,000–3,360,000, tape=609 exactly); (ii) env-only
   early-window scratch compression probes: `TLM_GCD_K_ADJUST` ∈ {0,−2,−4} (live −2, window 169..196),
   `TLM_COUT_K_DELTA` ∈ {2,4}, `TLM_HYB_V_DELTA` ∈ {2,4} (HYB_V is gate-inert per traps §3 — included only as a
   knob-noop control), `LUD_EXTRA_FOLD_VENTS` ∈ {0,1,2} with `TLM_FOLD_CHUNK_FORCE` held; each paired with
   `TRACE_TLM_PROFILE` peak readout + emitted-CCX delta + ops.bin hash. (iii) If (i) confirms the composition,
   write the closure certificate (Q floor = 1154 ± 2 for this representation) into memory/05 + this card.
3. **mechanism**: Q = max referenced id + 1 = 1154; peak ACTIVE = 1154 = TLM_TARGET_Q(1155) − 1, and the vent pool
   fills every phase whose uncapped need is below the cap. Lowering Q by k requires the UNCAPPED max to drop by k.
   Measured compositions: early walk-1 (op 25,972): u 255 + v 255 + Bezout 512 + scratch 132 = 1154, tape 0;
   late walk-1 (op 3,300,220): tape 592 + Bezout 512 + u/v 22 + scratch ~27 = 1153. The late window's terms are
   individually closed: tape 609 (exact backward codec enumeration closed at ≤2 q requiring a refuted 38-bit
   bijection), Bezout 512 (y_reg doubles mod p from step 1 — full width immediately; x_reg saturates likewise),
   u/v tail ≥18 (SCHED_J2 tail is λ-coupled: narrowing raises λ, memory/05 Step 5). Therefore the representation's
   Q floor ≈ 1152–1154 and the Q axis is CLOSED absent a λ trade or codec breakthrough. The early window's
   132-scratch (gidney.rs:1206 chunked-adder carries, k≈138 fits) IS compressible in principle — but compressing it
   cannot lower Q while the late window binds; its value is (a) diagnostic (proves the flat-plateau claim) and
   (b) Q-headroom for LMD-03's widening (q-guard budget).
4. **predicted direction**: Q **0** (certification outcome); T **±3k** on the k-probes (they trade adder chunk
   boundary vs carry count); λ **0** (ε=0 knob family: chunk re-fit is bit-exact arithmetic).
5. **cheapest_discriminator**: the two remaining B0 census builds (~3 min each) + up to 6 env-probe builds with
   `TRACE_TLM_PROFILE=1`; read `peak_qubits` and emitted ops; diff ops.bin sha per probe (knob-noop check).
   NO eval_circuit.
6. **predeclared_falsifier**: a census window showing a compressible ≥2-qubit term at the BINDING window — e.g.
   terminal-walk-1 census shows tape > 609 (codec regression), or scratch > 20 alongside tape 609, or Bezout pair
   < 512 (would reopen the whole Q programme). Also falsified if any k-probe lowers peak_qubits below 1154
   (would mean the late window is NOT binding and a real Q card exists).
7. **evidence_debt**: the closure certificate leans on two memory-closed bounds not re-derived this scout
   (codec ≤2q exact enumeration; SCHED_J2-tail λ coupling) — the certificate must cite them as priors with their
   reproducers (`repro/y3_global_codec.py`, memory/05 Step-5 table) and mark them "prior, geometry-tagged cf5aa02/
   02146ca eras". Promotion gate n/a (no artifact change); the debt is repaid by the certificate being explicit
   about which bounds are imported vs re-measured.
8. **trial_budget / time_budget**: 8 builds ≈ 30 min total; hard cap half a day including the certificate write.
9. **unresolved_directions**: does NOT decide whether a 1-2 qubit codec win is reachable (closed prior, would need
   the 38-bit bijection — do not re-open without a new construction); does NOT decide SCHED_J2 λ/Q trades (LMD-03).
10. **reopening_trigger**: (i) any representation change to the Bezout apply (new accumulator layout), (ii) a codec
    construction beating 609 tape qubits, (iii) an SCHED_J2 re-shape landing (LMD-03) — all re-run the censuses.
11. **reproduction_commands**:
    ```sh
    CARGO_TARGET_DIR=/tmp/w1scout-target B0_WIN_LO=3345000 B0_WIN_HI=3350000 \
      cargo run --release --bin build_circuit 2>&1 | sed -n '/B0_CENSUS_BEGIN/,/B0_CENSUS_END/p'
    CARGO_TARGET_DIR=/tmp/w1scout-target B0_WIN_LO=5895000 B0_WIN_HI=5900000 \
      cargo run --release --bin build_circuit 2>&1 | sed -n '/B0_CENSUS_BEGIN/,/B0_CENSUS_END/p'
    for d in 0 4; do
      CARGO_TARGET_DIR=/tmp/w1scout-target TLM_GCD_K_ADJUST=-$d TRACE_TLM_PROFILE=1 \
        cargo run --release --bin build_circuit 2>&1 | grep TLM_PROFILE
      sha256sum ops.bin
    done
    cp /tmp/ops.bin.frontier.bak ops.bin   # restore frontier artifact
    ```
12. **classification**: STRUCTURAL-Q (certification + diagnostic probes; no expected S change).

## Composite-task specifics
- **Participating owners**: B1 (SCHED_J2 tail ownership), B2 (tape size owner), B3 (Bezout pair owner), P
  (adder scratch owner), plus the memory/05 maintainer for the certificate.
- **Interfaces**: B0 census windows are addressed by op index; the plateau windows move with any traversal
  re-ordering — re-derive window indices from `TLM_TIMELINE_DUMP` TL_CENSUS `example_op` fields before censusing.
- **Invariants**: censuses must run on a byte-identical rebuild (verify ops.bin sha = 6519bd01... after each probe);
  probe conclusions require an ops.bin hash delta (knob-noop trap).
- **Proof obligations**: the certificate's floor argument must state each term's bound WITH its evidence class
  (measured-census / exact-enumeration-prior / λ-coupling-prior) — no term may be asserted floor without one.
- **Staged tests**: censuses → probe matrix → certificate draft → cross-check against EXP-ITX-00's liveness
  arithmetic (must be consistent: deferral 1251 vs floor 1154).
- **Rollback unit**: none needed (measurement-only; no source edit; the certificate is additive documentation).
- **Single-cell results needing re-validation if this lands**: none (no artifact change). If the certificate is
  WRONG (falsifier fires), the entire qubit workstream re-opens and memory/03's "coordinate ports" row must be
  re-priced against the newly-found slack.

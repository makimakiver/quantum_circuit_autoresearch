# Structural discovery — `DISCOVERY-STRUCTURAL`

**Disposition:** `SCOPED` (read-only; no source, artifact, metrology expansion, evaluation, or nonce work run).  The dispatch-bound baseline is `2026-08-16-a2067dcf`; all candidate signs below are relative to its artifact, not a claimed variant.  The objective is executed `T`, not emitted CCX: all estimates explicitly retain execution-discount, strip-ordinal, lambda, and Q constraints.

## Bound baseline and reached route

| item | bound fact | evidence |
|---|---:|---|
| Frontier | `T=1,283,487.051`, `Q=1,154`, `S=1,481,143,998`; four checks green over 9,024 shots | `.pi/frontier.json`; `CAMPAIGN_FINDINGS.md:§1` |
| Artifact identities | compressed `6519bd01424…c233a097`; canonical `0a4a412c…ed0b0` | `.pi/frontier.json` |
| Lambda control | 17.62 (400×64 paired lanes); SAFE needs upper CI Δλ `<+0.5` | `.pi/frontier.json`; `.pi/skills/circuit-evidence/SKILL.md` |
| Live algorithm | jump-2 binary extended-GCD; two `forward_gcd_jump` / `reverse_gcd_jump` pairs, tape-record/replay | `src/point_add/trailmix_ludicrous/ec_add.rs:286-303`; `gcd.rs:1182-1668` |
| Tape | 261 iterations; 609q; Step0=2b, Pair=5b/2 symbols, Triple=7b/3 symbols | `.pi/frontier.json`; `codec.rs:331-532` |
| Binding Q windows | apply and square maxima are tape-free; a codec-only tape reduction cannot lower Q | `.pi/experiments/EXP-CODEC-02.md:4,9`; `CAMPAIGN_FINDINGS.md:§2.1` |
| Stream coupling | deep-strip keys are tuple+ordinal+occupancy keyed; any reordered stream must price stale keys/re-mine | `CAMPAIGN_FINDINGS.md:§2.2,§5`; `.pi/experiments/EXP-SQ-01.md` |

## Coverage matrix

`LOCAL_PLATEAU` below means only that bounded local family is exhausted; it is **not** a global ceiling.

| required surface | source-derived present form | bounded finding | status / next representation seam |
|---|---|---|---|
| Tape encoding/compression | `DialogCodec::{Step0,Pair,Triple,Raw,Tail4Top32}` records three dialog bits per step; tail uses a special 12→5 ANF payload | `TAIL4_TOP32` was +8,570 CCX and does not move the tape-free Q peak; codec width is locally unpriced for Q | local tape-width family refuted/plateau; **DISC-STR-01** is a six-symbol *reversible rank codec* seam, not another tail knob |
| Replay representation | reverse replay decodes a window, swaps raw `{sub,swp,s2}` into live flags, then performs body/cswap/comparator erase | reverse total premium is ≈59,242 expected executed T and is apply-independent | **DISC-REV-01**: carry-threaded reversible compare-exchange / transcript cleanup; cross-cell B1×P |
| Reversible state reuse | apply currently loans `u[1..4]` as dirty vents, while `controlled_mod_add_k`/`controlled_mod_sub_vented` allocate and erase their own reduction syndrome | reuse exists for vents but not for modular-reduction carry/reduction state across swap→fold | **DISC-RED-01**: tagged reduction-syndrome handoff through the existing swap/fused-fold seam |
| Pair-1/Pair-2 erase asymmetry | `apply_step_reverse` is Pair-1-only; `apply_step_forward` is Pair-2-only; their fold/swap order is opposite | Pair-2 FWD=4/S2=3 is under detached metrology; INV=4/cswap=4 is unsafe | do not merge directions or steal P2 result; **DISC-RED-01** requires a paired reciprocal formulation, not a one-arm optimization |
| Square-coordinate fusion | `coord_add3x` sits exactly between inverse and square; Karatsuba sum/c/a/b square phases then feed Pair-2 | early sum free has a real pre-strip benefit but has a 6,799 stale-key tax; direct local surfaces are already carded | local schedule plateau escalated; retain `EXP-ARCH-02`, `EXP-SQ-04`; no duplicate source proposal |
| Shared arithmetic | add/sub clean consumes a quantum carry by HMR+conditional comparator; fused cdouble creates `hi,hi2` immediately after swap | weak deferred-clean card is open but its stronger 257-bit swap version is formally cost-dead | **DISC-RED-01** is narrower: carry *representation* transfer without widening the 256-bit swap |
| Global liveness / Q | global maxima contain 4 registers+adder scratch or square product/scratch; tape absent | freeing tape or a small local temporary does not change max referenced id | Q is locally frozen, not globally bounded; reopen with a redesign removing ≥10 qubits in a tape-free binding window |
| Traversal reduction / alternative formulation | four walks are forced by availability of `Δx`; deferral needs tape609+Bezout512+scratch≈1251 | deferral is Q-negative; alternate `Δy` factor requires +256 live or a third product | closed *for this formulation*; reopen only with a Δy-preserving apply representation or a tape ≤585 construction |
| Strip occupancy / emitted-to-executed | all changed operation ordinals may lose a 12,202-removal/4,045-downgrade table; apply execution discounts ≈7.9–8.2%, clean phases 50% | emitted CCX is only a discriminator; final pricing is post-strip executed T | mandatory on every card, never infer a score from emitted gates |

## Ranked structural cards

### 1. `DISC-REV-01` — carry-threaded replay compare-exchange (rank 1; cross-cell)

- **Mechanism and seam.** Each nonzero forward step computes `swp` with `controlled_swap_decision_lt_truncated` (`gcd.rs:1261-1272`, `comparator.rs:776-798`), records it in the codec, then reverse replay restores it, applies the 255-Fredkin ladder, and erases it with `swap_decision_uncompute_vented` (`gcd.rs:1519-1579`, `comparator.rs:1919-1947`).  The proposed representation is one *paired* compare-exchange primitive that threads the comparison carry/syndrome through the Fredkin ordering and transcript-erasure interface. It must preserve the one-bit ordering information; it is not permitted to “sort and forget” it. This targets the measured two-reverse-pass premium rather than the already-refuted local `CMP_K/GAP_J2` family.
- **Reversible invariant / proof obligation before savings.** For every `(i, sub, u, v)` and both walks, the primitive must map exactly to the current `(swp, cswap(u,v), phase)` transformation and its inverse, including the tape’s dialog symbol and `s = SCHED_J2[i]-cmp_window(i)` coupling.  A proof must show (1) transcript information remains recoverable until its coherent erase, (2) the final flag and all carry/scratch wires are `|0⟩`, (3) phase equals the old `swap_decision_uncompute_vented` phase word, and (4) matching forward/reverse schedule accessor consumption stays positional.  A comparator cannot simply disappear: reversible ordering loses a bit without the transcript.
- **T/Q/score/lambda sign.** `T −10k…−40k` executed only if the expensive reverse carry/uncompute representation is the dominant removable portion; `Q=0` required (same registers); provisional `ΔS −11.5M…−46.2M` at Q=1154.  λ is **unknown, not assumed 0**: exact arithmetic can still alter HMR/condition phase exposure, and a changed stream can discard strip credit. Emitted CCX is not its price.
- **Cheapest discriminator.** A TRACE-only per-substage ledger split at reverse `{apply, controlled_add, cswap, comparator-uncompute}` against the matching forward comparison, with `TRACE_TLM_CCX/TOF`; no evaluation.  This is the same low-cost stage attribution needed before choosing a primitive interface. Instrumentation must be default-off and separately owned by an implementer.
- **Falsifier.** If ≥80% of the 59,242 executed premium is the information-theoretic erase path and a carry-threaded mitre cannot reduce it by ≥5,000 emitted CCX without an extra live transcript bit/carry chain, the card is `PARKED/LOCAL_PLATEAU` for this primitive. A failed bijective/phase mitre is immediate refutation of the mechanism, not a lambda result.
- **Search radius and budget.** One trace-only implementation + baseline (≤10 min), then at most six source probes / one implementer-day.  No full eval, dirty scan, or census in this dispatch.
- **Rollback unit / dependencies.** One default-off `TLM_REPLAY_COMPARE_EXCHANGE=1` covering both direction partners; separate TRACE commit. Requires B1 schedule owner, comparator/shared-primitive owner, strip owner, and lambda owner. `src/point_add/trailmix_ludicrous/{gcd.rs,comparator.rs}` are implementer-only (ownership check recorded below).
- **Unresolved / reopening.** It does not reopen comparator truncation windows. Reopen after any traversal ordering, dialog-symbol, or carry-chain representation change; otherwise leave the global reverse premium as an unresolved localization, never a global ceiling.

### 2. `DISC-RED-01` — tagged modular-reduction syndrome handoff across swap→fold (rank 2; cross-cell)

- **Mechanism and seam.** `controlled_mod_add_k` allocates `anc`, consumes it in `add_f_window`, then erases it through `controlled_lt_msbs_conditional` (`arith.rs:1513-1534`). The inverse half uses the analogous `anc` plus `compare_geq_chunked_middle` cleanup (`gcd.rs:1826-1868`). Immediately after the Pair-2 add comes a 256-bit conditional swap and a fused double that allocates `hi,hi2` (`gcd.rs:1747-1785`; `fused.rs:1865-1920`). Carry a **tagged reduction syndrome**, not a widened operand: retain the post-fold reduction predicate only on the branch whose value remains in `y_reg` after the existing swap, use it in the next fold’s existing `hi/hi2` correction logic, and erase it at the reciprocal inverse seam. This differs from EXP-BZ-02 strong deferral: it may not turn the existing 256-bit cswap into a 257-bit cswap.
- **Reversible invariant / proof obligation before savings.** Define a two-case tag: after `cswap(swp,x,y)`, `tag=0` whenever the carry belongs to the value moved to `x`; otherwise tag denotes precisely the bounded reduction residue consumed by the next fused fold. Prove the value bound `<2^256` for the next cout adder, the tag’s branch relation to `swp`, and a paired inverse identity for `apply_step_forward`/`apply_step_reverse`. Also prove no dirty vent aliases `anc`, `hi`, `hi2`, or `u[1..4]`. The proof must preserve `s2` conditionality in both replay directions and show all tags/phase words erase.
- **T/Q/score/lambda sign.** Conservative potential is `T −1.5k…−4k` executed after condition/routing overhead, `Q=0` required, and `ΔS −1.7M…−4.6M`. This overlaps the `−5k…−8k` weak deferred-clean estimate in EXP-BZ-02/ARCH-03 and must not be added to it. λ exposure is **positive/unknown** because a wider or differently timed fold may change carry escape; accept only paired upper-CI Δλ `<+0.5` even if arithmetic mitres are exact.
- **Cheapest discriminator.** A flag-gated count-only branch proves the old forward/inverse clean counters approach zero while `tlm_apply_*_fold` grows by *strictly less* than the removed executed clean credit; record B0 at each fold peak to ensure `hi/hi2/tag` does not raise Q. Before code, a symbolic two-branch value/tag table is cheaper than a build.
- **Falsifier.** Any needed 257th cswap lane (the known strong-version failure), fold growth ≥ clean removal, tag alias/liveness requiring a fresh plateau qubit, or a residual bound reaching the next 256-bit cout input refutes this formulation. λ mean classical >30/9024-equivalent or any round >2× baseline max is the campaign refuter after implementation.
- **Search radius and budget.** `gcd.rs:1722-1868`, `arith.rs:1450-1534`, `fused.rs:1865-1995`; one written value/tag proof (≤0.5 day), one paired default-off implementation (≤2 implementer-days), then one count-only/B0 probe. Stop after one falsification cycle.
- **Rollback unit / dependencies.** One `TLM_TAGGED_REDUCTION_HANDOFF=1` guard spanning both Pair-1 and Pair-2 reciprocal code; single commit discard. Requires B3 apply, shared fused arithmetic, B0 liveness, strip re-key, and lambda owners.
- **Unresolved / reopening.** Does not decide the existing LSBS change or generic folded-adder merge. Reopen only after a changed fused-fold interface, a formal residual-margin proof, or a new apply layout; otherwise retain EXP-BZ-02 strong version as locally refuted, not the whole arithmetic space.

### 3. `DISC-STR-01` — exact six-symbol joint dialog rank codec (rank 3; codec/replay reframing)

- **Mechanism and seam.** The production tape chunks 3 symbols into 7 bits (`Triple`) and pairs into 5 bits (`Pair`), while `jump_dialog_regions` is fixed by count (`codec.rs:331-532`). Replace two adjacent triples (18 raw dialog wires) with an exact 6-symbol rank/unrank code only if the reachable state set has ≤`2^13` elements, yielding 13 rather than 14 code wires. Encoder must run during `forward_gcd_jump`; decoder must feed `reverse_gcd_jump` in reverse symbol order. This is a new fixed-window representation, not `TLM_TAIL4_TOP32` and not a local tape-width knob.
- **Reversible invariant / proof obligation before savings.** Obtain an exact reachable-state relation for six consecutive `(sub,swp,s2)` symbols under the live JUMP=2 recurrence, including Step0 boundary exclusion. Build a bijection `E:R→{0,1}^13` and an inverse `D` such that `D(E(r))=r` for every valid transcript and invalid-code handling is a clean reversible permutation (or remains unreachable without phase leakage). The forward and replay window plans must agree exactly, and all codec clean ancillas must be returned before the next walk allocation.
- **T/Q/score/lambda sign.** At most one tape qubit per two triples: roughly ≤86q logical storage over two 261-step tapes if applicable, but **Q=0 expected** because current Q peaks are tape-free. `T` is unknown: a synthesized rank/unrank must beat two current Triple transforms after the executed discount; expected sign is `0…negative` only if the map is sparse/low-degree, otherwise strongly positive (the `Tail4Top32` ANF codec was +8,570 CCX). λ is 0 only after an exact permutation and phase-clean proof; otherwise unknown. No score estimate is allowed before a gate-counted synthesis.
- **Cheapest discriminator.** Offline/read-only reachable-state enumeration for the exact six-symbol recurrence followed by ANF/BDD gate-cost lower bound against two Triple calls. A proof that `|R|>8192`, or a lower bound exceeding the current two-Triple CCX cost, rejects without altering scored source or running a circuit build.
- **Falsifier.** `|R|>2^13`; no clean reversible extension at 13 bits; synthesis lower bound ≥ two Triples; or any codec plan mismatch/phase mitre failure. Unchanged Q alone is *not* a falsifier—it only confines this to a T-only mechanism.
- **Search radius and budget.** `codec.rs:331-532` plus the two paired plan consumers in `gcd.rs:1200-1429` and `gcd.rs:1435-1510`; six-symbol only (no unbounded synthesis), one offline enumerator/BDD experiment (≤1 researcher-day), one implementer-day only on a proven map. No census/miner/evaluation.
- **Rollback unit / dependencies.** One `TLM_CODEC_6SYM=1` guard changing both plan construction and both encode/decode paths; discard one commit. Requires a reversible-logic synthesis tool and cross-cell reframer ownership; it is expressly not available merely because no existing env knob exists.
- **Unresolved / reopening.** The Q prize is unavailable unless a separate global liveness redesign makes tape live at a binding peak. Reopen for a newly discovered exact relation, a new low-degree synthesis, or changed ITERS/JUMP; do not repeat the refuted terminal-ANF family.

## Common implementation/evidence gates

1. An implementer, not this read-only role, must add one default-off guard per card and prove default compressed **and canonical** identities remain the bound values.
2. For every stream-changing card: prove knob liveness by changed operation identity; run TRACE/B0 only as discriminator; re-price post-strip occupancy/stale keys; use executed phase deltas rather than emitted CCX.
3. For every lambda-exposed card: use the paired nonce set; SAFE only if upper CI `Δλ < +0.5`, REFUTED only at the stated campaign thresholds. No nonce grind is authorized.
4. Before any score claim: full exact 9,024-shot four checks, then explicit stream-bound grind approval, new grounded `SUB4_TAIL_NONCE`, and strict `round(T)*Q` improvement.

## Review findings and residual risks

- **HIGH — representation/strip coupling:** all three cards reorder or alter emitted tuples/ordinals; the current deep-strip table is fully occupied and its credit can reverse a pre-strip win. Evidence: `CAMPAIGN_FINDINGS.md:§2.2`, `.pi/experiments/EXP-SQ-01.md`.
- **HIGH — Pair directionality:** `apply_step_reverse` and `apply_step_forward` have opposite fold/swap order. A one-direction change is not a valid inverse-paired candidate. Evidence: `src/point_add/trailmix_ludicrous/gcd.rs:1722-1868`.
- **HIGH — phase/lambda:** clean ancilla counts do not prove phase cleanliness. The HMR-conditioned comparator erasures and any changed fold carry distribution need explicit phase and paired lambda evidence. Evidence: `arith.rs:1450-1534`, `comparator.rs:1919-1947`, `.pi/skills/circuit-evidence/SKILL.md`.
- **MEDIUM — Q temptation:** a tape saving cannot currently lower Q; claims require a tape-free plateau owner reduction. Evidence: `.pi/experiments/EXP-CODEC-02.md:4,9`.
- **MEDIUM — estimated savings overlap:** DISC-RED-01 overlaps deferred-clean BZ cards and must be marginally measured, not stacked with their `−5k…−8k` range.
- **MEDIUM — codec dependency:** an exact relation/rank circuit needs an offline reversible-synthesis dependency; lack of a knob is not evidence against the mechanism.

## Ownership and no-change evidence

`PYTHONPATH=.pi` ownership inspection reports each proposed `src/point_add/trailmix_ludicrous/{codec,gcd,fused,square,ec_add}.rs` path as “only implementer may edit scored circuit source.” No proposed source was edited by this role. The `python3 .pi/harness/ownership.py --help` direct invocation itself fails its relative import, but the package import and `ownership_violations` check succeeded.

## Schema-v2 result payload

```json
{
  "schema_version": 2,
  "experiment_id": "DISCOVERY-STRUCTURAL",
  "role": "structural-researcher",
  "status": "SCOPED",
  "frontier_id": "2026-08-16-a2067dcf",
  "candidate_ops_sha256": "6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097",
  "canonical_ops_sha256": "0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0",
  "changed_paths": [],
  "evidence": {
    "findings_artifact": "/Users/makimakiver/.pi/agent/sessions/--Users-makimakiver-ecdsafail-challenge--/subagent-artifacts/outputs/cda7ee75-8787-4792-afe8-dee7db3a4300/structural.md",
    "frontier": ".pi/frontier.json",
    "campaign_state": ".pi/state/campaign.json",
    "campaign_snapshot": "CAMPAIGN_FINDINGS.md",
    "source_paths": ["src/point_add/trailmix_ludicrous/codec.rs", "src/point_add/trailmix_ludicrous/gcd.rs", "src/point_add/trailmix_ludicrous/arith.rs", "src/point_add/trailmix_ludicrous/fused.rs", "src/point_add/trailmix_ludicrous/square.rs", "src/point_add/trailmix_ludicrous/ec_add.rs"]
  },
  "metrics": {"average_executed_toffoli": 1283487.051, "qubits": 1154, "score": 1481143998},
  "verification": {"read_only_discovery": "no candidate implementation or evaluation run", "falsifiers": "per-card in findings artifact", "lambda_exposure": "per-card in findings artifact"},
  "nonce_env": "SUB4_TAIL_NONCE",
  "nonce": 1337610097,
  "commands": ["read-only source and evidence inspection", "PYTHONPATH=.pi ownership_violations inspection"],
  "recovery": {"status": "SCOPED", "reason": "Read-only structural discovery completed; no scored-source or artifact mutation."},
  "timestamp": "2026-08-17T08:37:00Z",
  "run_id": "structural-20260817t082608z",
  "dispatch_ref": ".pi/campaigns/campaign-20260817t082219z/runs/structural-20260817t082608z/dispatch.json",
  "checkpoint_ref": ".pi/campaigns/campaign-20260817t082219z/runs/structural-20260817t082608z/checkpoint.json",
  "terminal_ref": ".pi/campaigns/campaign-20260817t082219z/runs/structural-20260817t082608z/terminal.log",
  "recovery_ref": ".pi/campaigns/campaign-20260817t082219z/runs/structural-20260817t082608z/recovery/DISCOVERY-STRUCTURAL",
  "scientific_verdict": "SCOPED_READ_ONLY"
}
```

```acceptance-report
{
  "criteriaSatisfied": [
    {"id": "criterion-1", "status": "satisfied", "evidence": "Concrete ranked cards, coverage matrix, source seams, falsifiers, risks, and evidence paths are recorded in this artifact."}
  ],
  "changedFiles": ["/Users/makimakiver/.pi/agent/sessions/--Users-makimakiver-ecdsafail-challenge--/subagent-artifacts/outputs/cda7ee75-8787-4792-afe8-dee7db3a4300/structural.md"],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {"command": "read-only source/evidence inspection", "result": "passed", "summary": "Derived seams and constraints from live route and campaign control plane."},
    {"command": "PYTHONPATH=.pi ownership_violations inspection", "result": "passed", "summary": "Confirmed all proposed scored paths are implementer-only."},
    {"command": "python3 .pi/harness/ownership.py --help", "result": "failed", "summary": "Direct module execution has a relative-import error; package import used for the successful ownership check."}
  ],
  "validationOutput": ["No source validation/evaluation was run: dispatch is read-only discovery with evaluation=false and long_compute=false."],
  "residualRisks": ["Deep-strip ordinal/occupancy credit can erase a pre-strip gain.", "Pair-1/Pair-2 inverse ordering and s2 conditionality require paired proofs.", "Tape savings do not presently move Q.", "Carry/codec mechanisms require phase-clean and paired-lambda evidence."],
  "noStagedFiles": true,
  "diffSummary": "No scored-source diff; one external findings artifact only.",
  "reviewFindings": ["high: stream-order changes require strip re-key pricing.", "high: one-direction apply edits are invalid inverse-paired candidates.", "medium: all T estimates are pre-implementation ranges, not certified savings."],
  "manualNotes": "No global ceiling inferred; bounded local plateaus were escalated to representation-level seams."
}
```

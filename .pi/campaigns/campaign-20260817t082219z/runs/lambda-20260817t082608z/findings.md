# Lambda evidence audit — `lambda-20260817t082608z`

## Bound control plane and scope

- **Dispatch authority:** `.pi/campaigns/campaign-20260817t082219z/runs/lambda-20260817t082608z/dispatch.json` binds this read-only discovery run to `DISCOVERY-LAMBDA`, frontier `2026-08-16-a2067dcf`, source commit `a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5`, and prohibits metrology expansion, evaluation, grinding, source mutation, and promotion.
- **Pinned frontier checked:** `.pi/frontier.json` names the baseline artifact `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`, canonical hash `0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0`, `T=1,283,487.051`, `Q=1,154`, and `S=1,481,143,998`. Geometry is independently consistent with `src/point_add/trailmix_ludicrous/schedule.rs`: `JUMP=2`, `ITERS=261`, `BAKED_ITERS=258`.
- **No lifecycle mutation:** `.pi/state/campaign.json` remains the authority and has both `EXP-P2-01` and `EXP-BZ-01` at `METROLOGY`. No source, state, continuation, artifact, evaluation, or driver was changed/restarted.
- **Continuation observation:** `.pi/continuations/EXP-P2-BZ-METRO.json` has `launch_approved=false`, no resume command, and explicitly permits only parsing existing logs after completion. `PYTHONPATH=.pi python3 -m harness.cli continuations` reported `COMPLETE / ANALYZE_ONLY`; all A/B/C markers and required files exist.

## Evidence integrity ledger

| Gate | Result | Evidence / interpretation |
|---|---|---|
| Commit/frontier binding | **PASS** | Dispatch commit and frontier ID agree with `.pi/frontier.json`; no prose value was used as authority. |
| Paired input set | **PASS for the scan surface** | `src/point_add/dirtyscan.rs:162-199` seeds 64 valid lanes/round with `Shake256("tlm-dirty-scan-inputs" || LE64(round))`; all arms use rounds `0..399`, hence the same 25,600 lanes. Measurement XOF is the common fixed `tlm-dirty-scan-measure` stream. |
| Arm hashes | **PASS** | SHA-256 recomputation of `/tmp/exp_p2_bz_metro/arm{A,B,C}/ops.bin` exactly matched the continuation: A `6519bd01…c233a097`, B `9cb5010b…f18d27`, C `6aa87651…bca17e`. |
| Driver completion/determinism | **PASS** | All three `.done` files exist. `R400` and independent `R400b` cumulative counts are identical for A/B/C. This checks deterministic replay of the logged scan, not correctness. |
| Counts/rounds | **PASS** | Each arm records 400 rounds × 64 lanes = 25,600. The dirty-scan code itself computes `lanes = 64 * rounds` (`dirtyscan.rs:213-306`). |
| Paired delta method | **PASS, limited** | Read-only harness parser differences common cumulative records into 16 paired 25-round blocks and uses a two-sided t(15)=2.131 interval. Exact individual-round records cover rounds 0–31 only. |
| Complete round-maximum falsifier | **NOT COVERED** | The all-400 logs after round 32 are cumulative 25-round blocks, not individual rounds. A block can prove an excess if it is too large, but cannot prove no individual candidate round exceeded `2×` baseline. |
| Candidate *final-stream* λ identity | **BLOCKER** | `TLM_DIRTY_SCAN` executes in `trailmix_ludicrous/mod.rs:667-669` **before** the root post-pass stack. `TLM_DIRTY_SCAN_FINAL`, not used by these logs, is the final-stream scanner in `src/point_add/mod.rs:2414-2419`. Scan op counts therefore differ from emitted final artifacts: A `8,971,392` vs `8,958,690`; B `8,950,391` vs `8,938,962`; C `8,959,352` vs `8,948,057`. The paired observations are valid diagnostic evidence for their common pre-postpass streams, but are not a final-artifact λ/correctness/phase certificate. |
| Correctness, phase-cleanliness, ancilla-cleanliness, score | **NOT ADJUDICATED** | Dirty scan mirrors the simulator for its scan surface, but it is diagnostic. No 9,024-shot `eval_circuit`, no final candidate phase certificate, no Q measurement, and no score gate were run. |

The scan line reports `ancilla_bad_rounds=0/400` for every arm, but that is neither a final-stream result nor a substitute for the four required 9,024-shot checks.

## Existing paired results (triage only)

The official read-only parser was used only after completion:

```text
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=.pi python3 -B -m harness.cli analyze-p2-bz
```

Estimator: union-fault lanes scaled to 9,024 shots, `λ = 9024 × any_fault / 25,600`. Intervals are paired 25-round-block mean differences (16 blocks; t(15)); they do **not** assume the 25,600 lanes are independent for a required-round projection.

| arm | role / artifact | classical | phase lanes | union-fault lanes | λ | mean classical / 9,024 | known individual max (r0–31 only) |
|---|---|---:|---:|---:|---:|---:|---:|
| A | baseline / `6519bd01…` | 43 | 25 | 50 | 17.620 | 15.1575 | 1 |
| B | P2 FWD add=4 + FWD s2=3 / `9cb5010b…` | 51 | 37 | 63 | 22.205 | 17.9775 | 1 |
| C | BZ `ADD_SKIP_LASTK=2` / `6aa87651…` | 44 | 28 | 52 | 18.325 | 15.5100 | 1 |

| comparison | Δλ (union) | paired block SE | 95% CI | classical refuter? | complete max-round refuter? | campaign λ disposition |
|---|---:|---:|---|---|---|---|
| B − A | +4.5825 | 1.2839 | **[+1.8465, +7.3185]** | No: 17.9775 ≤ 30 | Not observable: only 32/400 exact rounds | **INCONCLUSIVE** — not SAFE; no refuter observed/covered |
| C − A | +0.7050 | 0.8730 | **[−1.1553, +2.5653]** | No: 15.5100 ≤ 30 | Not observable: only 32/400 exact rounds | **INCONCLUSIVE** — not SAFE; no refuter observed/covered |

### Exact stopping rule applied

- `SAFE` requires CI upper `< +0.5`: neither B (7.3185) nor C (2.5653) qualifies.
- `REFUTED` requires candidate mean classical `>30` per 9,024-equivalent **or** a candidate individual round `>2×` the baseline maximum. Neither mean crosses 30. The only exact per-round coverage is 32/400, where all observed maxima equal 1; cumulative blocks cannot establish the remaining maximum. Therefore no refutation is licensed.
- Otherwise the only campaign verdict is `INCONCLUSIVE`. This is not a correctness verdict, score verdict, phase-cleanliness verdict, promotion permission, or a global/local ceiling.

**Analyzer defect avoided:** `/tmp/exp_p2_bz_metro/analyze.py` prints a suggested “~1 rounds” estimate by taking the standard deviation from only rounds 0–31 and scaling it as though it estimates the entire 400-round process. That is an invalid independence/stationarity inference and is not used here. The maintained `.pi/harness/p2_bz.py` correctly reports the coverage limitation and returns `INCONCLUSIVE`.

## Deterministic eliminations versus new paired-metrology obligations

### Eliminated or gated without a new λ scan

1. **BZ `ADD_SKIP_LASTK=2` (arm C) — eliminate from promotion ordering by deterministic score gate, not by a λ verdict.** `CAMPAIGN_FINDINGS.md` and `EXP-BZ-01.md` record final post-strip CCX **+485**, Q unchanged. A nonnegative preliminary T/Q direction cannot progress unless a separately declared λ objective already clears its threshold. The current broad C interval does not do so. This is not labelled `REFUTED`: no full deterministic executed-T/Q score proof was run here.
2. **BZ strong deferred reduction — deterministically closed.** `EXP-BZ-02.md` establishes that deferring the fold across the intervening `cswap(swp,x,y)` requires a 257-bit swap pair, costing +256 full-price Toffoli per iteration/direction versus about 66.7 saved. Do not remeasure λ for that strong form. The weak form remains a different card, contingent on a bounds proof.
3. **Unsafe tail boundaries — reject before scan.** The `INV=4` and FWD/INV cswap=4 family deletes/reorders a live final operand swap (the finding/card identifies iteration 257); it is outside the theorem-safe boundary. Do not use a dirty scan to seek permission for a broken semantic seam.
4. **SLOPE δ3 mid-band family — already λ-refuted.** `CAMPAIGN_FINDINGS.md §3.1` reports Δλ about +5,252 with 449 classical events/12 rounds; its predeclared refuter fired. No remeasurement absent a materially different semantics/proof model. The smaller δ2 extension in `EXP-SLOPE-04` is not thereby eliminated; it remains a separate λ-exposed candidate.
5. **BZ-04 s2-chain trim — cheap deterministic verification only.** It asks whether constprop already cancels the known-zero top-chain CCXs. If it does, close as already done; if exact survivors exist, removal is identity-preserving and lambda-exempt. No new paired measurement is the discriminator.
6. **STRIP-02 final self-inverse cancel — exact bundle rider.** The existing strict matcher removes two CCXs and preserves baseline strip occupancy at zero stale keys. It needs an artifact/hash/strip check after an upstream stream change and must be bundled with a regrind-forcing candidate; it does not need new paired lambda evidence for the exact removal itself.
7. **Q-local tape/cap/deferral variants — locally eliminated by live liveness/score arithmetic.** `EXP-CODEC-02`, `EXP-SLOPE-00`, and `EXP-ITX-00` show Q=1154 binds in tape-free square/apply windows; deferral makes a 1,251-qubit window. These local failures are **not a global Q ceiling**. Reopen only as a cross-cell representation change.

### Require new paired measurement only after their deterministic gates pass and explicit authority exists

1. **P2 arm B, final candidate stream — highest λ obligation.** Existing pre-postpass scan rejects `SAFE` but cannot furnish final-stream λ or the all-round maximum. Before any candidate transition: scan the final stream (`TLM_DIRTY_SCAN_FINAL`) against A on the same rounds/input domain; retain individual counters for all rounds; recheck exact B artifact and post-strip occupancy. No restart was made by this run.
2. **SLOPE-04 δ2 `[200,220)` — λ-exposed despite small surface.** Its deterministic liveness/probe is already positive (`−184` static Toffoli; 56 stale keys), but SLOPE-01 disproves the “mid-band slack is harmless” rationale. It must pass a schedule-semantic accessor/boundary proof and final paired λ scan before any evaluation.
3. **BZ-02 weak deferred reduction — only after bounds and deterministic phase-price gate.** Need paired fwd/reverse implementation, a proof that the post-fold residual stays within the 256-bit/reduced representation, and a phase counter showing fold growth is less than clean removal. If those pass, the widened carry channel is λ-exposed and requires final-stream paired evidence.
4. **Statistical strip budget (STRIP-01/SM-01/ARCH-04) — λ-classical-priced.** Any non-exact census key is a classical-risk spend. The miner pilot/full census is explicitly unapproved (`long_compute=[]`); do not start it. If later authorized, first reproduce 1.239700 on the unchanged stream, then measure candidate risk/paired λ and obtain stream-bound grind approval. Exact UNSAT-certified keys (STRIP-03) are exempt from λ pricing, but only after certificates—not after shallow census.
5. **SQ-01 early sum / any ordinal-moving stream edit — re-mine obligation, then λ as appropriate.** It has a real pre-strip saving but loses 6,799 keys in the current final stream. This is deterministically not shippable as-is; re-open only in an isolated worktree after an approved stream-specific re-mine. A local plateau here escalates to strip-lifecycle/certificate work; it is not a square-cell ceiling.

## Ranked metrology / evidence cards

| rank | card | mechanism and seam | predicted sign | invariants | cheapest discriminator / falsifier | radius & budget | rollback / reopening |
|---:|---|---|---|---|---|---|---|
| 1 | **P2-FINAL-METRO (obligation for EXP-P2-01 arm B)** | `gcd.rs:1738-1783` FWD replay: FWD add skip precedes FWD cswap; candidate deletes add tail=4 and assumes s2=0 tail=3. | T−; Q0; λ observed positive on prepass surface (+4.5825). | Do not alter the final live cswap; s2 conditionality remains load-bearing; default hashes pinned; final op hash B must be `9cb5010b…`. | Authorized final-stream paired A/B scan with full individual round records, plus post-strip occupancy check. **Falsifier:** CI upper ≥0.5, mean classical >30, any complete candidate round >2× baseline max, hash mismatch, or stale-key economics erase T gain. | Existing logs exhausted; **zero new work authorized**. If authorized, retain the 400-round common set rather than extrapolating from 32 rounds. | Default-off env unset/discard candidate. Reopen only if final-stream scan has complete maxima and passes the exact gate. |
| 2 | **BZ-01 score-stop (arm C)** | `gcd.rs:225-246`, 1786-1824: symmetric `ADD_SKIP_LASTK=2` removes both replay-side tail adds. | Raw T− but final post-strip CCX **+485**; Q0; λ +0.705 unresolved. | Pair fwd/inv semantics; final hash C `6aa87651…`; no claimed Q drop. | Deterministic final emitted/Toffoli/Q gate. **Falsifier already indicated:** nonnegative score direction. | No new λ budget: do not promote/restart C. | Unset flag. Reopen only if a stream-specific strip re-mine or a newly defined bundle reverses the deterministic post-strip sign; then treat as a new artifact. |
| 3 | **BZ-03 tail fill re-fit** | `schedule.rs` widened-tail fill calls and the `next_*` consumers; exact adder/fold/FFG layout choice. | T−0…−1.5k; Q0; λ0 only if exact semantics are proved. | Positional schedule consumption and matching forward/reverse slots; no comparator-window/s2 predicate change. | Existing env override build/hash plus phase counters. **Falsifier:** all `k∈{22,25,31,45,60,90,138}` flat within ±20 CCX. | At most seven isolated cheap builds/30 min only under a future dispatch; no scan/eval/grind. | Unset override; later implementer-only schedule bake. Reopen after an ITERS or dispatcher change. |
| 4 | **STRIP-03 exact-cluster certification** | Final stream admission-1.0 never-firing/implication CCX clusters; source call-site UNSAT mitres rather than ordinal reuse. | T−3k…−7.7k; Q0; λ0 **only after proof**. | Certificate must be source/state exact; strip stays final-before-nonce; no statistical key masquerades as exact. | Site attribution of 100 stratified keys, then per-cluster mitre. **Falsifier:** >50 unrelated sites/no common invariant or >10% deep-census firing. | Census/mitres are not authorized by this dispatch; exact proof work may be separately scoped. | Table/certificate revert. Reopen on a new final stream or if lifecycle re-mine stalls. |
| 5 | **BZ-02 weak deferred reduction** | Pair `apply_step_forward`/`apply_step_reverse` clean/fold seam in `gcd.rs:1750-1824`. | T−5k…−8k; Q0; λ potentially +0.5…+2.5. | Both directions change together; reduced 256-bit value before next step; phase word clean. | Value-bounds proof or classical emulation, then phase-price trace. **Falsifier:** fold growth ≥ clean removal or residual can exceed 256-bit representation. | One implementer cycle after explicit authority; only then final paired scan. | One default-off flag/commit. Reopen on LSBS/fused-fold redesign. |
| 6 | **SLOPE-04 δ2 extension** | `cmp_window` / forced GAP env seam, extra `[200,220)` window narrowing. | T−≈176 executed; Q0; λ unknown/possibly +. | Producer/consumer accessor agreement; `s=SCHED_J2-cmp_window` coupling; no δ3 assumption imported. | Existing hash-live probe is positive. **Falsifier:** any final paired classical regression or schedule-boundary test failure. | Future implementer flag + small probe; final paired scan required before eval. | Unset flag. Reopen only under a changed comparator proof/model. |
| 7 | **SQ-02 classical-conditioned shell add** | `ec_add.rs:coord_addsub` z-load → classical condition channel. | T−1.4k…−1.6k; Q0; λ0 expected (exact classical conditioning). | Temp must zero/free; default byte identity; no new phase source. | One-site trace of expected execution discount. **Falsifier:** discount <0.2, unsupported conditioned gate, or four-check failure. | Implementer-only default-off one site/2 builds; no paired λ needed if exact condition semantics established. | Unset flag/revert one call site. Reopen if `c_condition` support expands. |
| 8 | **SQ-04 / ARCH-01 cross-cell reframe** | Square output→forward walk truncation and reverse-premium schedule/comparator/adder interface. | T−500…−2k (SQ-04) or −10k…−40k (ARCH-01); Q0 target; λ0 only for proven exact re-fit. | Paired walk/replay schedule slots; s2 conditionality; square output timing; final-stream strip tripwire and Q guard. | First deterministic stage attribution/census; **falsifier:** known unbuild dependency or ≥80% premium in intrinsic uncompute floor / <5k recoverable. | Must be a serialized cross-cell card, not a local plateau retry. No source writer demand from this review. | One mechanism flag/isolated worktree. Reopen after traversal order, schedule, or representation changes. |

## Coverage ledger and residual risks

**Covered:** source reachability; geometry; arm artifact identity; deterministic driver repeat; paired scan input domain; 400×64 aggregate counts; block-paired intervals; scan-vs-final stream distinction; score/strip warnings from retained final-build logs; deterministic dead/unsafe mechanisms.

**Not covered (and not silently inferred):** all-400 individual round maxima; final-stream candidate λ; any candidate canonical hash; candidate T/Q score; 9,024 correctness/reversibility/phase/ancilla checks; `GRIND_APPROVED`; ground `SUB4_TAIL_NONCE`; any global ceiling claim; a required-round count.

**Severity findings:**

- **BLOCKER — final-stream mismatch:** existing `TLM_DIRTY_SCAN` data predate the post-pass stack, while B/C artifact hashes identify post-pass streams. Do not use these logs as final candidate λ certification or final phase evidence.
- **HIGH — incomplete maximum gate:** only 32/400 individual rounds are reconstructible. The campaign’s all-round maximum refuter cannot be ruled out from block sums.
- **HIGH — arm B is not SAFE:** even the limited paired interval is wholly above +0.5. It cannot advance on λ.
- **HIGH — arm C is deterministically unattractive:** final post-strip +485 CCX/Q0 invokes the preliminary score stop; its λ interval cannot rescue it.
- **MEDIUM — local plateaus are scoped only:** Q/tape/deferral and cap/erase results bound their stated cells and geometry. They do not prove a global score or λ ceiling; route new representation/stream-order ideas through the listed cross-cell cards.

## Schema-v2 result payload for parent collection

This run is a read-only evidence audit. It intentionally does not change lifecycle state; the parent/reviewer owns result collection. The payload below is schema-v2 shaped and records the leading observed candidate hash solely as an evidence reference.

```json
{
  "schema_version": 2,
  "experiment_id": "DISCOVERY-LAMBDA",
  "role": "lambda-metrologist",
  "status": "INCONCLUSIVE",
  "frontier_id": "2026-08-16-a2067dcf",
  "candidate_ops_sha256": "9cb5010bc5f4a534d82a88a80e80cbd613889d8ce4de54e6487b6e9cb0f18d27",
  "changed_paths": [],
  "evidence": {
    "continuation": ".pi/continuations/EXP-P2-BZ-METRO.json",
    "analysis": "read-only existing-log parse; arm B and C INCONCLUSIVE",
    "arm_hashes_verified": true,
    "final_stream_mismatch": true,
    "round_max_coverage": "32/400"
  },
  "metrics": {
    "rounds": 400,
    "lanes_per_round": 64,
    "armB_delta_lambda": 4.5825,
    "armB_ci95": [1.8464534645192434, 7.318546535480756],
    "armC_delta_lambda": 0.705,
    "armC_ci95": [-1.1553337076731687, 2.565333707673169]
  },
  "verification": {
    "lifecycle_mutated": false,
    "full_evaluation_run": false,
    "driver_restarted": false,
    "source_edited": false
  },
  "nonce_env": "SUB4_TAIL_NONCE",
  "nonce": 1337610097,
  "commands": [
    "PYTHONPATH=.pi python3 -m harness.cli continuations",
    "PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=.pi python3 -B -m harness.cli analyze-p2-bz",
    "shasum -a 256 /tmp/exp_p2_bz_metro/armA/ops.bin /tmp/exp_p2_bz_metro/armB/ops.bin /tmp/exp_p2_bz_metro/armC/ops.bin"
  ],
  "run_id": "lambda-20260817t082608z",
  "dispatch_ref": ".pi/campaigns/campaign-20260817t082219z/runs/lambda-20260817t082608z/dispatch.json",
  "checkpoint_ref": ".pi/campaigns/campaign-20260817t082219z/runs/lambda-20260817t082608z/checkpoint.json",
  "terminal_ref": ".pi/campaigns/campaign-20260817t082219z/runs/lambda-20260817t082608z/terminal.log",
  "recovery_ref": ".pi/campaigns/campaign-20260817t082219z/runs/lambda-20260817t082608z/recovery/metrology-audit",
  "scientific_verdict": "INCONCLUSIVE"
}
```

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Concrete severity-tagged findings, file paths, verified hashes, paired counts/intervals, coverage gaps, and ranked obligations are recorded above."
    }
  ],
  "changedFiles": [],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "PYTHONPATH=.pi python3 -m harness.cli continuations",
      "result": "passed",
      "summary": "EXP-P2-BZ-METRO was COMPLETE/ANALYZE_ONLY with all required paths and arm hashes clean; no driver started."
    },
    {
      "command": "PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=.pi python3 -B -m harness.cli analyze-p2-bz",
      "result": "passed",
      "summary": "Read-only parser reported B and C INCONCLUSIVE, 400 rounds, 64 lanes/round, and exact 32/400 maximum coverage warning."
    },
    {
      "command": "shasum -a 256 /tmp/exp_p2_bz_metro/armA/ops.bin /tmp/exp_p2_bz_metro/armB/ops.bin /tmp/exp_p2_bz_metro/armC/ops.bin",
      "result": "passed",
      "summary": "All independently recomputed hashes matched the continuation manifest."
    }
  ],
  "validationOutput": [
    "Arm A/B/C R400 and R400b counts were identical.",
    "No source, campaign-state, continuation, trusted artifact, evaluator, scan driver, or nonce was mutated."
  ],
  "residualRisks": [
    "Existing λ logs are pre-postpass TLM_DIRTY_SCAN evidence rather than final candidate-stream evidence.",
    "Only 32 of 400 individual round maxima are observable; cumulative blocks cannot negate the remaining maximum gate.",
    "No four-check evaluation, candidate canonical identity, final score/Q gate, grind approval, or ground nonce exists."
  ],
  "noStagedFiles": true,
  "diffSummary": "No repository source or control-plane file was edited; this external runtime artifact is a read-only audit report.",
  "reviewFindings": [
    "blocker: src/point_add/trailmix_ludicrous/mod.rs:667-669 and src/point_add/mod.rs:2414-2419 - existing P2/BZ scan logs are pre-postpass whereas candidate hashes identify final streams.",
    "high: /tmp/exp_p2_bz_metro/logs/arm{A,B,C}_r*.log - only rounds 0-31 provide exact per-round counts; the all-400 round-maximum refuter is not fully covered.",
    "high: /tmp/exp_p2_bz_metro/logs/armB_R400.log - B's paired 95% upper Δλ bound is +7.3185, so it is not SAFE.",
    "high: .pi/experiments/EXP-BZ-01.md - C has final post-strip +485 CCX with Q unchanged, so deterministic preliminary score direction blocks promotion."
  ],
  "manualNotes": "This report does not claim correctness, phase cleanliness, a positive score delta, a global ceiling, or authorization to restart/expand metrology."
}
```

# EXP-ARCH-07 executed — locally refuted; no implementation dispatched

## Registration and control-plane validation

- Materialized card: `.pi/experiments/EXP-ARCH-07.md`
- Registered exactly one new campaign record:
  - `experiment_id`: `EXP-ARCH-07`
  - lane / owner: `REFRAMING` / `cross-cell-reframer`
  - status: `SCOPED`
  - `implementation_ready: false`
  - `mechanism_id: square-term-transport`
  - required search radius, discriminator, falsifier, and reopening trigger present.
- Campaign validation:
  - pre-registration: **OK**, 8 records
  - post-registration: **OK**, 9 records
  - final validation: **OK**, 9 records, no scored-source changes.

## Proof run

- Pi proof-agent run ID: `4306cbe1`
- Parent-owned proof run ID: `exp-arch-07-proof-20260817t08471786956426z`
- Run envelope:
  `.pi/campaigns/performance-2026-08-16/runs/exp-arch-07-proof-20260817t08471786956426z/`
- Dispatch, checkpoint, and terminal receipts were initialized and envelope-bound. The schema-valid proof result was subsequently published as the envelope's `result.json` and independently validated against that envelope.
- `harness.cli collect` validated that the registered transition `SCOPED -> REFUTED` is admissible. It was deliberately run without `--apply`, so the campaign record remains `SCOPED` and the result ledger was not modified.

## Deterministic proof corpus

D0 independently validated the pinned-source textual extraction anchors and declared arithmetic/control mapping for the selected term:

- term: `b_hi_f_32_sub`
- operation: subtract `(b_prod << 32) mod p`
- `b_prod = hi²`
- first forward-divstep control/data flow and Step0 codec encode/decode anchors validated.
- source hashes captured for `square.rs`, `ec_add.rs`, `gcd.rs`, and `codec.rs`.

D1 corpus:

- 28 deterministic local-cone cases: 7 fixed `hi` values × 4 required edge residues.
- edge residues: `0`, `1`, `p-2`, `p-1`.
- compared the complete `(t1, s2, subtracted, swp)` tuple, Step0 raw symbol, Step0 encoded symbol and decoder, post-`i=0` `(u,v)` state, and paired `s2` codec conditionality.

## Proof result

**Classification:** `REFUTED`  
**Scientific verdict:** `REFUTED_DIRECT_NO_CARRIER_TERM_TRANSPORT`

- D0 baseline extraction: **PASSED**
- D1: **verified witness**; all 28 declared finite local-cone cases mismatched.
- D2 bit-vector solver: **not run**, correctly, because the approved sequence requires it only if D1 finds no witness.
- Independent integer replay: **PASSED**.

First replayed witness:

- `hi = 1`
- `b_prod = 1`
- selected-term delta: `2^32`
- term-on input: `0`
- term-off input: `2^32`

The Step0 tuple/symbol agrees for this witness, but required post-step state does not:

- term-on post-`i=0` `v = 0xffff…ffff`
- term-off post-`i=0` `v = 0xffff…bfffffff`

The witness is verified in the declared exact 256-bit local-cone model; the result makes no full EC-shot reachability or evaluator claim. It refutes only direct no-carrier defer/delete of named `b_hi_f_32_sub` in that local forward-`i=0` cone. It does not refute other square terms, transport with an exact carrier/restoration, coordinate fusion, or cross-cell work generally.

## Conditional implementation

**Not dispatched.** `PROOF_PASSED` was required before creating the detached implementation worktree. Because D1 produced a replayed counterexample:

- no implementer was launched;
- no implementation run or worktree exists;
- no source/Rust build/cheap candidate gate/lambda/evaluator/census/grind/certification/promotion/merge/parent-checkout source edit occurred.

## Cheap-gate and integrity results

| Gate | Result |
| --- | --- |
| Campaign pre-registration validation | PASS |
| Campaign post-registration validation | PASS |
| Dispatch schema validation | PASS |
| Run-envelope receipt binding | PASS |
| D0 extraction map | PASS |
| D1 deterministic miter | REFUTED with witness |
| Independent integer replay | PASS |
| D2 solver | Not applicable after D1 witness |
| Result schema and envelope binding | PASS |
| Registered-result collection validation | PASS, non-applying |
| Final campaign validation | PASS |
| Scored source changes | 0 |
| Protected `.pyc` digest | unchanged: `016deb…ffe89` |

## Durable evidence

- Card: `.pi/experiments/EXP-ARCH-07.md`
- Proof result: `.pi/campaigns/performance-2026-08-16/runs/exp-arch-07-proof-20260817t08471786956426z/result.json`
- D0 extraction map: `.../artifacts/d0-extraction-map.json`  
  SHA-256: `1f19fbf5e57062f9d95e560400c8a51121a09c5ad99ee29460c5aa1cda0c6f64`
- D1 corpus and witnesses: `.../artifacts/d1.json`  
  SHA-256: `7a413dc9bcb23ab6ee2bf72dfc09805d48e552a16f6366b75ced03facb7c9162`
- Independent replay: `.../artifacts/replay.json`  
  SHA-256: `72add013ab3de9baf4645dcdb50c18fe9cb0bb10e09e00fa188929436f593c42`
- Reproducible miter: `.../artifacts/exp_arch07_miter.py`  
  SHA-256: `c302afb370b9785f23bb15955e8e5dc821434c066aca37da3b66070364c5344c`
- Worker recovery bundle: `.../recovery/EXP-ARCH-07/`
- Parent result-publication recovery bundle: `.../recovery/parent-result-publication/`

Both required recovery delimiters were emitted and verified before final handoff.

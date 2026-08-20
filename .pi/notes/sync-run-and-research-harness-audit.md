# Sync-run and research-harness audit

## Request and scope

A fresh authoritative benchmark run was requested after syncing, together with an investigation of why the observed score appeared to have moved and whether the research/control harness was misleading. This log records the exact run, source/artifact comparison, and confirmed research-tooling drift.

No scored circuit source and no trusted harness source was edited.

## Fresh authoritative run

Command run from the synced checkout:

```bash
ecdsafail run
```

The CLI ran `./benchmark.sh`, which removed stale `ops.bin`/`score.json`, built the untrusted `build_circuit` stage, then ran the separate trusted `eval_circuit` stage.

Result:

| field | fresh value |
|---|---:|
| Git HEAD | `9b2ce61c97fa1be1acb770c802244b1a13677ccd` |
| emitted operations | 8,958,603 |
| referenced qubits | 1,154 |
| average executed Toffoli | 1,283,485.490 |
| rounded Toffoli | 1,283,485 |
| score | **1,481,141,690** |
| correctness shots | 9,024 / 9,024 |
| classical mismatches | 0 |
| phase-garbage batches | 0 |
| ancilla-garbage batches | 0 |

The score arithmetic is exact:

```text
round(1,283,485.490) × 1,154 = 1,481,141,690
```

## The score did not rise relative to the pinned research frontier

The local pinned frontier in `.pi/frontier.json` is a different source/artifact identity:

| field | pinned frontier | fresh synced run | delta, fresh - pinned |
|---|---:|---:|---:|
| source commit | `a2067dcf...` | `9b2ce61c...` | different / divergent |
| emitted operations | 8,958,690 | 8,958,603 | **-87** |
| average executed T | 1,283,487.051 | 1,283,485.490 | **-1.561** |
| rounded T | 1,283,487 | 1,283,485 | **-2** |
| Q | 1,154 | 1,154 | 0 |
| score | 1,481,143,998 | **1,481,141,690** | **-2,308** |

So the fresh score is lower/better by 2,308—not higher. The older `memory/06-research-status.md` number of 1,490,805,286 is also superseded by both of these records and must not be used as the current baseline.

## Exact source changes that explain the new stream

The current HEAD and pinned frontier differ in `src/point_add/mod.rs` (108 insertions, 11 deletions). The stream-changing pieces are at lines 1787-1805 and 2403-2511.

### 1. Compact identity nonce tail: -82 emitted operations

The older construction retained a 96-operation identity tail and encoded 48 nonce bits. The current `apply_tail_nonce`:

* maps a 64-bit nonce into seven base-1024 digits;
* writes only seven adjacent `X;X` identity pairs, i.e. 14 operations;
* truncates the remaining tail with `ops.truncate(start + 14)`.

That removes exactly `96 - 14 = 82` emitted operations. It also changes the Fiat-Shamir seed because the full emitted stream changed.

### 2. First exact affine bridge: -4 operations

A unique seven-operation pattern is replaced by a three-operation basis-action-equivalent pattern. Net stream delta: `-4` operations.

### 3. Second exact affine bridge: -1 operation

A unique fourteen-operation final-fold pattern is replaced by a thirteen-operation equivalent pattern. Net stream delta: `-1` operation.

Together:

```text
-82 (compact tail) + -4 (first bridge) + -1 (second bridge) = -87 ops
```

This exactly matches the emitted-op delta between the pinned frontier and the fresh run. The two affine bridges also remove/rewrite non-Clifford structure; the new tail reseeds all generated evaluation inputs. Therefore the observed `-1.561` average-executed-Toffoli movement must not be attributed to one edit alone without matched artifact analysis.

## Artifact identities

Pinned frontier artifact from `.pi/frontier.json`:

```text
compressed SHA-256: 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097
canonical SHA-256:  0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0
nonce default:       1337610097
```

Fresh synced artifact fingerprint:

```text
compressed SHA-256: 85f9da9427461ab15b79a655233092f55422630636a44ed8919c90473953d132
canonical SHA-256:  f1b179e86de18ea2e5afcda7f111f0f36ac13639f38fc9126a8bb36bca84feaa
nonce default:       225239250
```

The changed canonical hash proves this is not a rerun of the pinned artifact.

## Confirmed research/control-harness problems

### A. Pinned frontier metadata is stale after sync (high severity for research decisions)

The checked-in research control plane still pins `a2067dcf...`, old artifact hashes, and nonce `1337610097`. The synced repository is `9b2ce61...`, with a different source and different stream.

This is enforced rather than hidden:

```bash
PYTHONDONTWRITEBYTECODE=1 PYTHONPATH=.pi python3 -B -m harness.cli validate
```

returned:

```text
ERROR: repository HEAD does not match the pinned frontier source_commit:
9b2ce61... != a2067dcf...
exit 2
```

That means old campaign comparisons/continuations are not bound to the current checkout. Do not call the fresh lower number a new certified frontier merely because `ecdsafail run` is green.

### B. The research artifact nonce writer is incompatible with the current tail (high severity for reproduction/grinding)

`src/point_add/memory/repro/artifact_io.py` still defines:

```python
TAIL_RECORDS = 96
```

and `write_nonce_artifact` only accepts a 48-bit nonce and rewrites all 96 tail records. Current scored source keeps only 14 tail records and accepts a 64-bit nonce.

Therefore this tool cannot correctly synthesize/rewrite a current-head nonce artifact. It can write a stream with the wrong final-tail geometry, producing invalid reproduction/grinding evidence. The source of truth for current nonce construction is `apply_tail_nonce` in `src/point_add/mod.rs:1791-1805`, not the stale helper.

### C. Metric receipts are not artifact-bound (medium severity)

Trusted `score.json` and `results.tsv` record T, Q, score, ops length, and a short Git commit, but not the compressed/canonical artifact hashes or the nonce. A later reader cannot bind a metrics row to an exact `ops.bin` without separately fingerprinting the artifact. This is a research-record problem, not evidence that the evaluator miscounted this fresh run.

### D. Evaluator coverage/reporting limitations (medium severity, not the cause of this score movement)

The trusted evaluator draws 9,024 candidate pairs but skips exceptional generated points (`t.x == o.x`, or infinity cases) instead of replenishing them, and only prints the resulting `n_shots` at runtime; that count is not persisted in `score.json`/`results.tsv`. The present run printed and used 9,024 shots, so this did not affect this result.

It also does not separately execute a circuit inverse pass. It checks correct output, forward phase cleanliness, and forward ancilla cleanliness; since each gate is reversible, this is not by itself a demonstrated soundness failure, but it is narrower than a literal independently simulated inverse-roundtrip test.

## Certification status

The fresh run is valid **as a trusted 9,024-shot local evaluation**. It is not automatically a certified research promotion because:

1. the current source/artifact is not the pinned frontier identity;
2. `.pi/state/campaign.json` has `approvals.grind = null`;
3. no fresh lambda scan was performed for the changed stream; and
4. no nonce-grind receipt binds nonce `225239250` to this exact artifact.

## Required next action before trusting this as a new campaign frontier

1. Update/reconcile the frontier record with the new source commit, both artifact hashes, the exact 14-record/64-bit tail scheme, nonce, and fresh score evidence; or intentionally reset the code to the pinned frontier.
2. Fix or retire `memory/repro/artifact_io.py::write_nonce_artifact` so it is parameterized by the live tail layout and supports current 64-bit nonce encoding.
3. Add artifact hash + nonce + actual shot count to each research score receipt.
4. Run the required paired lambda evidence and obtain explicit grind approval before promoting the stream.

The trusted benchmark itself did its job here: it deleted stale artifacts, rebuilt the stream, simulated it independently, and computed the lower score. The bad state is the stale research provenance/reproduction layer around it.

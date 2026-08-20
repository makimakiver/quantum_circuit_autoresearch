# EXP-BZ-03 implementation handoff

## Finding — LOCAL_PLATEAU (not a global ceiling)

The dispatch-bound cheap environment sweep is complete in `/tmp/ecdsafail-round1-bz03` at `a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5`. The default reproduced both required identities:

- compressed: `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`
- canonical: `0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0`

All seven required overrides are exact no-ops at artifact and TRACE resolution. Each has the same two hashes, 8,958,690 emitted operations, `max_referenced_qubit_id=1153` / `Q=1154`, forward-register expected Toffoli `147799.0`, inverse-register expected Toffoli `148392.5`, total expected Toffoli `1299076.0`, and zero deep-strip stale keys. This meets EXP-BZ-03's predeclared flat-within-±20-CCX falsifier exactly (0 delta), so all arms are preserved as negative evidence.

**Disposition:** `LOCAL_PLATEAU` for this four fill-entry `APPLY_COUT_K` environment-control family. Route to cross-cell reframing; do **not** turn this bounded local falsifier into a global-ceiling claim. No lifecycle state/promotion was applied by this worker.

## Per-arm outcomes

| arm | exact override | compressed / canonical | Q | TRACE expected (forward / inverse / total) | deep strip | outcome |
|---|---|---|---:|---|---|---|
| default | unset | required identities reproduced | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | baseline |
| k=22 | `258:22,259:22,262:22,263:22` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=25 | `258:25,259:25,262:25,263:25` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=31 | `258:31,259:31,262:31,263:31` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=45 | `258:45,259:45,262:45,263:45` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=60 | `258:60,259:60,262:60,263:60` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=90 | `258:90,259:90,262:90,263:90` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |
| k=138 | `258:138,259:138,262:138,263:138` | identical / identical | 1154 | 147799.0 / 148392.5 / 1299076.0 | 0 stale | negative; flat |

The full per-arm hashes, phase readings, operation census, copied artifact paths, and log references are in `sweep-summary.json` and `artifact-manifest.json` below.

## Commands executed

```sh
cd /tmp/ecdsafail-round1-bz03 && \
  env -u TLM_COUT_K_CALL_OVERRIDES \
  CARGO_TARGET_DIR=/tmp/round1-bz03-target TRACE_TLM_TOF=1 \
  cargo run --release --bin build_circuit

cd /tmp/ecdsafail-round1-bz03 && \
  for k in 22 25 31 45 60 90 138; do \
    CARGO_TARGET_DIR=/tmp/round1-bz03-target \
    TLM_COUT_K_CALL_OVERRIDES="258:${k},259:${k},262:${k},263:${k}" \
    TRACE_TLM_TOF=1 cargo run --release --bin build_circuit; \
  done

PYTHONDONTWRITEBYTECODE=1 \
PYTHONPATH=/tmp/ecdsafail-round1-bz03/src/point_add/memory/repro \
  python3 -B -c 'from artifact_io import fingerprint; ...' \
  <copied-per-arm-ops.bin>

PYTHONDONTWRITEBYTECODE=1 \
PYTHONPATH=/Users/makimakiver/ecdsafail-challenge/.pi \
  python3 -B -c '... harness.schema.validate_result(...) ...'
```

The artifact audit ran before/after checks of tracked `/Users/makimakiver/ecdsafail-challenge/src/point_add/memory/repro/__pycache__/artifact_io.cpython-313.pyc` for every arm. Every check was clean and retained SHA-256 `016debdfcfe5a6f7827bfb1610344ed98bfe0d1720a7458618ecb94faddffe89`.

## Durable artifacts and validation

- Schema-valid result: `result.json` (`harness.schema.validate_result`: PASS; log `result-schema-validation.log`).
- Complete arm evidence: `sweep-summary.json`, SHA-256 `06c6ee18ede6da0b9693b5bba6403f048969bb4bf1405678cf9c407aad4ac9a6`.
- Artifact/log manifest: `artifact-manifest.json`, SHA-256 `94fefc4f6172bf97422252829566d81b13ef72725165c688d9cb7745976f1345`.
- Per-arm copied compressed artifacts and fingerprints: `artifacts/{default,k-22,k-25,k-31,k-45,k-60,k-90,k-138}/`.
- Per-arm cargo/TRACE logs: `logs/{default,k-22,k-25,k-31,k-45,k-60,k-90,k-138}.log`.
- Extracted register/total/deep-strip evidence: `artifacts/*/trace-and-strip.txt`.
- PYC audit evidence: `audits/*-pyc-before.txt`, `audits/*-pyc-after.txt`, `audits/finalize-pyc-*.txt`, and `audits/schema-validation-pyc-*.txt`.
- Source/no-stage final verification: `final-source-cleanliness.txt` (clean worktree, no staged files, no remaining worktree `ops.bin`).
- Final recovery bundle and verified delimiters: `recovery/final/` and `recovery/final-delimiters.txt` (`FLEET_RECOVERY_BUNDLE_BEGIN` / `FLEET_RECOVERY_BUNDLE_END` present).

Two early external-runner shell-helper failures were recovered before resuming; they made no source edit. The first happened before any cargo arm. The second occurred after the default build/copy but before its Python audit; the preserved default artifact was audited rather than rebuilt, then exactly the seven authorized override arms ran. Recovery records with verified delimiters are `recovery/pre-runner-failure/` and `recovery/runner-audit-failure/`.

## Source cleanliness, lifecycle, and debt

No source file was edited, no file was staged or committed, and no evaluator, lambda metrology, nonce grind, certification, or promotion was run. The final worktree status and staged diff are empty at the exact frontier commit. The run result deliberately reports `IMPLEMENTED` plus a `LOCAL_PLATEAU` scientific verdict rather than changing the parent lifecycle.

Evidence debt is limited to the explicitly prohibited downstream work: there is no new evaluator/four-check run, score calculation, lambda measurement, certification, or promotion. Those are not warranted by this flat/no-op sweep. The specific schedule fill-entry mechanism is falsified locally; a different cross-cell mechanism or changed dispatch branch/iteration width would be needed to reopen it.

## Review findings

- **info:** all 7 override configurations are byte-identical and canonically identical to default; none demonstrates knob liveness or performance improvement.
- **info:** deep-strip evidence remains healthy for every arm (`12202/12202` removed, `4045/4045` downgraded, `0` stale keys), so stale strip keys are not masking a difference in this sweep.
- **medium / bounded-scope:** this evidence rules out only the EXP-BZ-03 four-call fill-entry environment sweep at the pinned source and current layout dispatch. It does not establish a global performance ceiling.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Concrete per-arm findings, paths, identities, severity, recovery, and residual risks are recorded in this handoff and in sweep-summary.json."
    }
  ],
  "changedFiles": [],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "default TRACE_TLM_TOF build in /tmp/ecdsafail-round1-bz03",
      "result": "passed",
      "summary": "Reproduced compressed and canonical identities."
    },
    {
      "command": "seven k override TRACE_TLM_TOF builds (22,25,31,45,60,90,138)",
      "result": "passed",
      "summary": "All arms flat, non-improving, and artifact-identical."
    },
    {
      "command": "harness.schema.validate_result(result.json)",
      "result": "passed",
      "summary": "schema validation: PASS"
    },
    {
      "command": "emit_recovery_bundle.py -B final recovery",
      "result": "passed",
      "summary": "Both recovery delimiters verified."
    }
  ],
  "validationOutput": [
    "Default compressed=6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097 canonical=0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0.",
    "All eight artifacts have Q=1154 and deep-strip stale_keys_skipped=0.",
    "Worktree clean; no staged files; tracked artifact_io pyc clean at required SHA."
  ],
  "residualRisks": [
    "medium: LOCAL_PLATEAU applies only to the dispatched four-call COUT fill-entry family on the pinned frontier; it is not a global ceiling.",
    "Downstream evaluator, lambda, certification, and promotion evidence was intentionally not run because it was prohibited and no improving candidate exists."
  ],
  "noStagedFiles": true,
  "diffSummary": "No source diff; only durable run-envelope artifacts were written.",
  "reviewFindings": [
    "info: all seven override arms are byte-identical/canonically identical to default and are preserved as negative evidence.",
    "medium: route the locally falsified family to cross-cell reframing rather than infer a global ceiling."
  ],
  "manualNotes": "Final recovery delimiters are verified in recovery/final-delimiters.txt; two earlier external shell-helper failures were recovered and caused no source change."
}
```

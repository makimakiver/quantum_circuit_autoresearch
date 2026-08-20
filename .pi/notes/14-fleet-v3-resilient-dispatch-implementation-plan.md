# Fleet v3 Resilient Dispatch Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended)
> or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax
> for tracking.

**Goal:** Make campaign dispatch, checkpointing, interruption recovery, lane coverage, and cheap candidate
gates deterministic and parent-owned so a worker timeout cannot erase evidence or become a scientific
verdict.

**Architecture:** Add a validated dynamic frontier binding, a parent-owned run envelope, path-aware
preflight, and separate discovery/writer selection. Extend lifecycle schemas and evidence gates, then update
the CLI and agent contracts to use those machine-enforced interfaces. Existing live frontier, campaign,
continuation, result, and source files remain read-only during the upgrade.

**Tech Stack:** Python 3 standard library, `unittest`, JSON Schema documents, Markdown Pi agent/prompt
contracts.

**Spec:** `.pi/notes/13-fleet-v3-resilient-dispatch-design.md`

## Global Constraints

- Modify `.pi/**` only. Never edit `src/**`, `results.tsv`, `score.json`, `ops.bin`,
  `CAMPAIGN_FINDINGS.md`, `.pi/frontier.json`, `.pi/state/campaign.json`, existing campaign records,
  experiment evidence, or continuation manifests.
- Treat the pasted `bb237894…` campaign as a test fixture, not locally verified state.
- Preserve current schema-v2 frontier compatibility; schema-v3 identities are required only for newly
  produced frontier proposals. Do not migrate live state automatically.
- Keep `SUB4_TAIL_NONCE` and 9,024 shots as stable constants. Load every frontier-specific ID, commit,
  metric, and artifact hash dynamically.
- `WORKER_LOST` and `INTERRUPTED` must have `scientific_verdict: null` and cannot transition directly to a
  scientific acceptance/rejection or promotion state.
- Dirty `.pi/**` may be snapshotted. Dirty scored or trusted source blocks writer dispatch.
- A result/checkpoint write must be atomic: temporary file, `fsync`, then `os.replace`.
- Do not launch live subagents, cargo builds, full evaluation, metrology, nonce grinding, or long compute.
- Do not create commits in the active dirty checkout without separate user authorization. End each task with
  a test and diff/status checkpoint instead.

---

### Task 1: Dynamic frontier binding

**Files:**
- Create: `.pi/harness/frontier.py`
- Create: `.pi/harness/tests/test_frontier_runtime.py`
- Modify: `.pi/harness/constants.py`
- Modify: `.pi/harness/schema.py`
- Modify: `.pi/harness/campaign.py`
- Modify: `.pi/harness/cli.py`
- Modify: `.pi/schemas/frontier.schema.json`
- Modify: `.pi/harness/tests/test_schema.py`
- Modify: `.pi/harness/tests/test_campaign.py`
- Modify: `.pi/harness/tests/test_cli.py`
- Modify: `.pi/harness/tests/test_promotion.py`

**Interfaces:**
- Produces `tree_digest(root: Path, prefixes: Sequence[str]) -> str`.
- Produces `load_frontier_binding(root: Path, *, verify_worktree: bool = True) -> FrontierBinding`.
- `FrontierBinding` exposes `record`, `repository_commit`, `scored_source_tree_sha256`,
  `trusted_source_tree_sha256`, and `identity_complete`.
- Changes `validate_frontier(value, expected_baseline: Mapping[str, Any] | None = None) -> dict` so baseline
  comparison is supplied by the caller rather than imported from constants.
- Changes `validate_campaign(value, *, expected_frontier_id: str | None = None) -> dict` so campaign/frontier
  binding is dynamic.
- Existing schema-v2 records load with `identity_complete=False`; schema-v3 records must contain both tree
  hashes and validate them when `verify_worktree=True`.

- [ ] **Step 1: Add failing tests for dynamic metrics and tree identity**

```python
# .pi/harness/tests/test_frontier_runtime.py
import json
import tempfile
import unittest
from pathlib import Path

from harness.frontier import load_frontier_binding, tree_digest


def write_frontier(root: Path, *, version: int, frontier_id: str, score: int, source_identity=None):
    value = {
        "schema_version": version,
        "state": "PINNED",
        "frontier_id": frontier_id,
        "authoritative_snapshot": "CAMPAIGN_FINDINGS.md",
        "source_commit": "a" * 40,
        "entrypoint": "point_add::build -> trailmix_ludicrous",
        "nonce_env": "SUB4_TAIL_NONCE",
        "artifact": {"ops_sha256": "b" * 64, "canonical_ops_sha256": "c" * 64, "emitted_ops": 1},
        "metrics": {"average_executed_toffoli": 10.0, "rounded_toffoli": 10, "qubits": 11, "score": score},
        "verification": {"shots": 9024, "four_checks": "green"},
        "lambda": {"estimate": 1.0},
        "promotion_lock": {"required_status": "CERTIFIED"},
    }
    if source_identity is not None:
        value["source_identity"] = source_identity
    path = root / ".pi" / "frontier.json"
    path.parent.mkdir(parents=True)
    path.write_text(json.dumps(value), encoding="utf-8")


class FrontierRuntimeTests(unittest.TestCase):
    def test_v2_frontier_loads_without_compiled_score(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            write_frontier(root, version=2, frontier_id="dynamic", score=12345)
            binding = load_frontier_binding(root, verify_worktree=False)
            self.assertEqual(binding.record["metrics"]["score"], 12345)
            self.assertEqual(binding.record["frontier_id"], "dynamic")
            self.assertFalse(binding.identity_complete)

    def test_v3_frontier_detects_scored_source_change(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            scored = root / "src" / "point_add" / "mod.rs"
            trusted = root / "src" / "bin" / "eval_circuit.rs"
            scored.parent.mkdir(parents=True)
            trusted.parent.mkdir(parents=True)
            scored.write_text("first", encoding="utf-8")
            trusted.write_text("trusted", encoding="utf-8")
            identity = {
                "scored_source_tree_sha256": tree_digest(root, ["src/point_add"]),
                "trusted_source_tree_sha256": tree_digest(root, ["src/bin"]),
            }
            write_frontier(root, version=3, frontier_id="v3", score=9, source_identity=identity)
            load_frontier_binding(root)
            scored.write_text("changed", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "scored source tree"):
                load_frontier_binding(root)
```

- [ ] **Step 2: Run the focused test and verify RED**

Run:

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_frontier_runtime.py -v
```

Expected: import failure because `harness.frontier` does not exist.

- [ ] **Step 3: Implement the frontier binding**

Create `.pi/harness/frontier.py` with these public definitions:

```python
from dataclasses import dataclass
from hashlib import sha256
from pathlib import Path
from typing import Any, Mapping, Sequence
import subprocess

from .schema import load_json, validate_frontier


SCORED_PREFIXES = ("src/point_add",)
TRUSTED_PREFIXES = (
    "src/bin",
    "src/circuit",
    "src/sim",
    "src/weierstrass_elliptic_curve.rs",
)


@dataclass(frozen=True)
class FrontierBinding:
    record: dict[str, Any]
    repository_commit: str
    scored_source_tree_sha256: str
    trusted_source_tree_sha256: str
    identity_complete: bool


def tree_digest(root: Path, prefixes: Sequence[str]) -> str:
    digest = sha256()
    files: list[Path] = []
    for prefix in prefixes:
        path = root / prefix
        if path.is_file():
            files.append(path)
        elif path.is_dir():
            files.extend(item for item in path.rglob("*") if item.is_file())
    for path in sorted(files, key=lambda item: item.relative_to(root).as_posix()):
        relative = path.relative_to(root).as_posix().encode()
        digest.update(relative + b"\0")
        digest.update(sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def load_frontier_binding(root: Path, *, verify_worktree: bool = True) -> FrontierBinding:
    record = validate_frontier(load_json(root / ".pi" / "frontier.json"))
    commit = subprocess.run(
        ["git", "-C", str(root), "rev-parse", "HEAD"], check=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
    ).stdout.strip()
    scored = tree_digest(root, SCORED_PREFIXES)
    trusted = tree_digest(root, TRUSTED_PREFIXES)
    identity = record.get("source_identity")
    complete = isinstance(identity, Mapping)
    if verify_worktree and complete:
        if identity.get("scored_source_tree_sha256") != scored:
            raise ValueError("frontier scored source tree does not match the worktree")
        if identity.get("trusted_source_tree_sha256") != trusted:
            raise ValueError("frontier trusted source tree does not match the worktree")
    return FrontierBinding(record, commit, scored, trusted, complete)
```

Update `validate_frontier` to accept schema versions 2 and 3. Version 3 requires a `source_identity` object
with two 64-hex hashes; version 2 remains readable without it. Replace `require_campaign_baseline` with an
optional `expected_baseline` record; compare frontier ID, source commit, score, and both artifact hashes only
when that argument is supplied. Update `frontier.schema.json` with a `oneOf` for the two versions.

Remove `PINNED_FRONTIER_ID`, `PINNED_SOURCE_COMMIT`, `PINNED_SCORE`, `PINNED_OPS_SHA256`, and
`PINNED_CANONICAL_SHA256` from `constants.py`. Keep only stable constants and lifecycle/role sets.

In `campaign.py`, remove the constant import. `validate_campaign` accepts optional
`expected_frontier_id`; when supplied, require `campaign.frontier_id` to equal it. Require
`baseline_frontier_id` to be a nonempty string and preserve it through promotions, but never compare it to a
compiled value. In `promote_candidate`, preserve `campaign_value["baseline_frontier_id"]` directly.

In `cli.py`, replace `_load_campaign`/`_load_current_frontier` ordering with:

```python
def _load_bound_state(root: Path) -> tuple[dict[str, Any], FrontierBinding]:
    binding = load_frontier_binding(root, verify_worktree=False)
    campaign = validate_campaign(
        load_json(root / ".pi" / "state" / "campaign.json"),
        expected_frontier_id=binding.record["frontier_id"],
    )
    return campaign, binding
```

Every CLI command reads score/hashes from `binding.record`. Promotion passes the loaded current frontier as
`expected_baseline` only when it needs an explicit comparison. Update existing tests to construct their
frontier/campaign fixtures locally instead of importing or assuming compiled authority.

- [ ] **Step 4: Run focused and existing schema tests**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_frontier_runtime.py \
  .pi/harness/tests/test_schema.py \
  .pi/harness/tests/test_campaign.py \
  .pi/harness/tests/test_cli.py \
  .pi/harness/tests/test_promotion.py -v
```

Expected: all tests pass; no current frontier-specific value remains in `constants.py`.

- [ ] **Step 5: Record the task checkpoint**

```bash
git status --short
rg -n "PINNED_(FRONTIER|SOURCE|SCORE|OPS|CANONICAL)" .pi/harness .pi/agents .pi/prompts
```

Expected: only `.pi/**` implementation/test changes; the search returns no authoritative hardcoded symbols.

---

### Task 2: Operational lifecycle schemas

**Files:**
- Create: `.pi/schemas/dispatch.schema.json`
- Create: `.pi/schemas/checkpoint.schema.json`
- Modify: `.pi/schemas/experiment-card.schema.json`
- Modify: `.pi/schemas/result.schema.json`
- Modify: `.pi/schemas/campaign.schema.json`
- Modify: `.pi/harness/constants.py`
- Modify: `.pi/harness/schema.py`
- Modify: `.pi/harness/tests/test_schema.py`

**Interfaces:**
- Produces `validate_dispatch(value) -> dict`.
- Produces `validate_checkpoint(value) -> dict`.
- Extends `validate_result` with run/recovery fields.
- Adds `DISPATCHED`, `RUNNING`, `CHECKPOINTED`, `INTERRUPTED`, `WORKER_LOST`, and `BROKEN` statuses.

- [ ] **Step 1: Add failing lifecycle and result-shape tests**

Append to `.pi/harness/tests/test_schema.py`:

```python
from harness.schema import validate_checkpoint, validate_dispatch


def interruption_result(status="WORKER_LOST"):
    return {
        "schema_version": 2,
        "experiment_id": "schedule-tail-pair-240-001",
        "run_id": "run-001",
        "role": "implementer",
        "status": status,
        "frontier_id": "frontier-x",
        "candidate_ops_sha256": "d" * 64,
        "changed_paths": ["src/point_add/trailmix_ludicrous/schedule.rs"],
        "evidence": {},
        "dispatch_ref": ".pi/campaigns/c/runs/run-001/dispatch.json",
        "checkpoint_ref": ".pi/campaigns/c/runs/run-001/checkpoint.json",
        "terminal_ref": ".pi/campaigns/c/runs/run-001/terminal.log",
        "recovery_ref": ".pi/campaigns/c/runs/run-001/recovery/bundle.txt",
        "scientific_verdict": None,
        "interruption": {"kind": "hard-timeout", "missing_fields": ["verification"]},
        "resume_conditions": ["review preserved patch"],
    }


class OperationalSchemaTests(unittest.TestCase):
    def test_worker_lost_requires_null_scientific_verdict(self):
        self.assertEqual(validate_result(interruption_result())["status"], "WORKER_LOST")
        invalid = interruption_result()
        invalid["scientific_verdict"] = "REFUTED"
        with self.assertRaisesRegex(SchemaError, "scientific_verdict"):
            validate_result(invalid)

    def test_worker_lost_cannot_transition_to_refuted(self):
        assert_transition("RUNNING", "WORKER_LOST")
        with self.assertRaisesRegex(SchemaError, "WORKER_LOST.*REFUTED"):
            assert_transition("WORKER_LOST", "REFUTED")
```

- [ ] **Step 2: Verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_schema.py -v
```

Expected: failures for missing validators/statuses and unsupported result schema version 2.

- [ ] **Step 3: Implement exact lifecycle rules**

Set `STATUSES` and `TRANSITIONS` so these operational transitions exist:

```python
"SCOPED": {"DISPATCHED", "METROLOGY", "REFUTED", "INCONCLUSIVE", "PARKED"},
"DISPATCHED": {"RUNNING", "INTERRUPTED", "WORKER_LOST"},
"RUNNING": {"CHECKPOINTED", "INTERRUPTED", "WORKER_LOST"},
"CHECKPOINTED": {
    "CHECKPOINTED", "IMPLEMENTED", "METROLOGY", "EVALUATED",
    "INTERRUPTED", "WORKER_LOST", "BROKEN", "PARKED",
},
"INTERRUPTED": {"PARKED", "SCOPED"},
"WORKER_LOST": {"PARKED", "SCOPED"},
"BROKEN": {"REVERTED", "PARKED"},
```

Retain every existing safe transition not contradicted by the spec. Do not permit operational interruption
states to transition directly to `SAFE`, `REFUTED`, `CERTIFIED`, or `PROMOTED`.

Define dispatch/checkpoint schemas with the fields in spec §§5.2–5.4. Implement validators using `_require`,
`_reject_unknown`, and `_mapping`. Result schema version 2 requires the new run references; version 1 remains
readable for historical evidence. For `INTERRUPTED`/`WORKER_LOST`, require null `scientific_verdict` and a
nonempty `interruption` object.

- [ ] **Step 4: Run schema tests**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_schema.py -v
```

Expected: all schema and transition tests pass.

- [ ] **Step 5: Record the task checkpoint**

```bash
git status --short
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_schema.py -v
```

---

### Task 3: Parent-owned run envelope and interruption synthesis

**Files:**
- Create: `.pi/harness/runtime.py`
- Create: `.pi/harness/tests/test_runtime_recovery.py`
- Modify: `.pi/harness/campaign.py`

**Interfaces:**
- Produces `RunPaths.for_run(root, campaign_id, run_id) -> RunPaths`.
- Produces `create_dispatch(paths, dispatch) -> dict`.
- Produces `write_checkpoint(paths, checkpoint) -> dict`.
- Produces `deadline_phase(started_at: float, timeout_seconds: float, now: float) -> str` with values
  `NORMAL`, `CHECKPOINT`, `FINALIZE`, `GRACE`, `EXPIRED`.
- Produces `synthesize_interruption(root, paths, *, kind, role, experiment_id, frontier_id,
  candidate_ops_sha256="") -> dict`.

- [ ] **Step 1: Add the timeout durability test**

```python
# .pi/harness/tests/test_runtime_recovery.py
import json
import tempfile
import unittest
from pathlib import Path

from harness.runtime import (
    RunPaths, create_dispatch, deadline_phase, synthesize_interruption, write_checkpoint,
)
from harness.schema import validate_result


class RuntimeRecoveryTests(unittest.TestCase):
    def test_worker_timeout_synthesizes_recoverable_result(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            paths = RunPaths.for_run(root, "campaign-a", "run-001")
            create_dispatch(paths, {
                "schema_version": 1, "campaign_id": "campaign-a", "run_id": "run-001",
                "experiment_id": "schedule-tail-pair-240-001", "lane": "EXPLOITATION",
                "owner": "implementer", "frontier_id": "frontier-a", "agent": "implementer",
                "task": "paired schedule edit",
                "timeout_seconds": 100, "soft_deadlines": {"checkpoint": 0.8, "finalize": 0.9, "grace": 0.95},
                "worktree": str(root / "worker"),
                "base_identity": {
                    "repository_commit": "b" * 40,
                    "scored_source_tree_sha256": "c" * 64,
                    "trusted_source_tree_sha256": "d" * 64,
                },
                "control_plane_sha256": "a" * 64,
                "budget": {"wall_seconds": 100}, "commands": ["candidate-build"],
                "expected_result": ".pi/campaigns/campaign-a/runs/run-001/result.json",
            })
            write_checkpoint(paths, {
                "schema_version": 1, "run_id": "run-001", "gate": "candidate-hash",
                "timestamp": "2026-08-16T21:30:00Z", "evidence_paths": ["candidate.ops"],
                "artifact_hashes": {"candidate.ops": "d" * 64}, "commands": ["candidate-build"],
                "changed_paths": ["src/point_add/trailmix_ludicrous/schedule.rs"],
                "remaining_obligations": ["semantic test", "evaluation"],
            })
            paths.terminal.parent.mkdir(parents=True, exist_ok=True)
            paths.terminal.write_text("worker output before timeout\n", encoding="utf-8")
            result = synthesize_interruption(
                root, paths, kind="hard-timeout", role="implementer",
                experiment_id="schedule-tail-pair-240-001", frontier_id="frontier-a",
                candidate_ops_sha256="d" * 64,
            )
            self.assertEqual(validate_result(result)["status"], "WORKER_LOST")
            self.assertIsNone(result["scientific_verdict"])
            self.assertIn("semantic test", result["interruption"]["missing_fields"])
            self.assertTrue(paths.result.is_file())

    def test_deadline_phases_prevent_late_expensive_work(self):
        self.assertEqual(deadline_phase(0, 100, 79), "NORMAL")
        self.assertEqual(deadline_phase(0, 100, 80), "CHECKPOINT")
        self.assertEqual(deadline_phase(0, 100, 90), "FINALIZE")
        self.assertEqual(deadline_phase(0, 100, 95), "GRACE")
        self.assertEqual(deadline_phase(0, 100, 100), "EXPIRED")
```

- [ ] **Step 2: Verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_runtime_recovery.py -v
```

Expected: import failure because `harness.runtime` does not exist.

- [ ] **Step 3: Implement runtime durability**

Use a frozen `RunPaths` dataclass with `dispatch`, `checkpoint`, `terminal`, `result`, and `recovery_dir`
paths. Reuse one private atomic JSON writer. `create_dispatch` must use exclusive creation and fail if the
receipt already exists. `write_checkpoint` atomically replaces only `checkpoint.json`.

`synthesize_interruption` must:

1. validate and load dispatch/checkpoint when present;
2. create `recovery/`;
3. copy or reference terminal output without truncation;
4. run read-only worktree status/diff capture when the declared worktree exists;
5. hash discovered artifacts rather than printing binary content;
6. list missing obligations from the checkpoint;
7. create schema-v2 `WORKER_LOST` for `hard-timeout`/provider disappearance, otherwise `INTERRUPTED`;
8. set `scientific_verdict` to null;
9. atomically write and validate `result.json`.

Expose `can_start_expensive_command(phase: str) -> bool` returning true only for `NORMAL`.

- [ ] **Step 4: Integrate generic collection**

Update `collect_result` so operational results are preserved and can transition only through the new rules.
The function must never convert `WORKER_LOST` to `REFUTED`. Store run references in campaign records.

- [ ] **Step 5: Run focused tests**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_runtime_recovery.py \
  .pi/harness/tests/test_campaign.py -v
```

Expected: all tests pass.

---

### Task 4: Path-aware preflight and control-plane snapshots

**Files:**
- Create: `.pi/harness/tests/test_preflight.py`
- Modify: `.pi/harness/ownership.py`
- Modify: `.pi/harness/runtime.py`

**Interfaces:**
- Produces `classify_parent_changes(paths: Iterable[str]) -> dict[str, list[str]]` with keys `trusted`,
  `scored`, `artifacts`, `control_plane`, and `unrelated`.
- Produces `preflight_dispatch(root: Path, *, allow_scored_base: Sequence[str] = ()) -> dict`.
- Produces `snapshot_control_plane(root: Path, destination: Path) -> dict` containing path/hash manifest and
  aggregate SHA-256.

- [ ] **Step 1: Add failing dirty-tree tests**

```python
# .pi/harness/tests/test_preflight.py
import unittest

from harness.ownership import classify_parent_changes


class PreflightTests(unittest.TestCase):
    def test_dirty_control_plane_is_snapshotable(self):
        classified = classify_parent_changes([".pi/frontier.json", ".pi/agents/implementer.md"])
        self.assertEqual(classified["control_plane"], [".pi/agents/implementer.md", ".pi/frontier.json"])
        self.assertEqual(classified["scored"], [])
        self.assertEqual(classified["trusted"], [])

    def test_dirty_source_is_blocking(self):
        classified = classify_parent_changes([
            "src/point_add/trailmix_ludicrous/square.rs",
            "src/bin/eval_circuit.rs",
        ])
        self.assertEqual(classified["scored"], ["src/point_add/trailmix_ludicrous/square.rs"])
        self.assertEqual(classified["trusted"], ["src/bin/eval_circuit.rs"])
```

- [ ] **Step 2: Verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_preflight.py -v
```

Expected: import failure for `classify_parent_changes`.

- [ ] **Step 3: Implement path classification and snapshots**

Classify `ops.bin`, `score.json`, and `results.tsv` as canonical artifacts. Classify `.pi/**` as control
plane. Preserve existing role edit-ownership behavior.

`preflight_dispatch` must parse `git status --porcelain=v1 -z`, block non-approved scored/trusted paths,
block dirty canonical artifacts, and return a snapshot plan for dirty `.pi` paths. It must not modify or
clean the tree.

`snapshot_control_plane` copies only the listed control-plane paths into the run recovery directory and
writes `control-plane-manifest.json`. The aggregate hash covers each relative path and content hash.

- [ ] **Step 4: Run ownership and preflight tests**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_preflight.py \
  .pi/harness/tests/test_ownership.py -v
```

Expected: all tests pass; no cleanup command runs.

---

### Task 5: Discovery coverage and writer admission

**Files:**
- Create: `.pi/harness/selection.py`
- Create: `.pi/harness/tests/test_selection.py`
- Modify: `.pi/harness/cli.py`
- Modify: `.pi/harness/campaign.py`
- Modify: `.pi/schemas/experiment-card.schema.json`
- Modify: `.pi/schemas/campaign.schema.json`

**Interfaces:**
- Produces `SelectionPlan` with `discovery`, `writer_queue`, `coverage`, and `blocked`.
- Produces `select_cards(cards: Sequence[Mapping[str, Any]]) -> SelectionPlan`.
- Adds optional card fields `mechanism_id`, `implementation_ready`, `covered_by`, `dependency`, and
  `reopening_trigger`.

- [ ] **Step 1: Add failing portfolio tests**

```python
# .pi/harness/tests/test_selection.py
import unittest

from harness.selection import SelectionError, select_cards


def card(identifier, lane, ready, mechanism):
    return {
        "experiment_id": identifier, "lane": lane, "implementation_ready": ready,
        "mechanism_id": mechanism, "status": "SCOPED", "owner": "structural-researcher",
    }


class SelectionTests(unittest.TestCase):
    def test_discovery_lanes_survive_single_writer_admission(self):
        plan = select_cards([
            card("tune", "EXPLOITATION", True, "schedule-local"),
            card("struct", "STRUCTURAL", False, "new-representation"),
            card("reframe", "REFRAMING", False, "walk-codec-replay"),
        ])
        self.assertEqual(set(plan.coverage), {"EXPLOITATION", "STRUCTURAL", "REFRAMING"})
        self.assertEqual([item["experiment_id"] for item in plan.writer_queue], ["tune"])

    def test_local_refutation_does_not_exclude_distinct_mechanism(self):
        plan = select_cards([
            card("refuted", "EXPLOITATION", False, "schedule-local"),
            card("distinct", "STRUCTURAL", False, "shared-state-representation"),
            card("reframe", "REFRAMING", False, "global-liveness"),
        ])
        self.assertIn("distinct", [item["experiment_id"] for item in plan.discovery])

    def test_missing_lane_requires_machine_checkable_disposition(self):
        with self.assertRaisesRegex(SelectionError, "REFRAMING"):
            select_cards([
                card("tune", "EXPLOITATION", True, "schedule-local"),
                card("struct", "STRUCTURAL", False, "new-representation"),
            ])
```

- [ ] **Step 2: Verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_selection.py -v
```

Expected: import failure because `harness.selection` does not exist.

- [ ] **Step 3: Implement selection**

Define required discovery lanes as `EXPLOITATION`, `STRUCTURAL`, and `REFRAMING`. A missing lane is allowed
only through a lane-disposition record with `proof_ref`, `covered_by`, or `dependency` plus nonempty
`reopening_trigger`. Writer admission requires `implementation_ready is True`; discovery admission does not.

Reject duplicate `mechanism_id` writer cards and more than one writer card touching the same declared
`source_family`. Do not remove a structurally distinct card because another card is refuted.

Update `dispatch-plan` CLI output to expose all four fields from `SelectionPlan`; it remains a dry-run.

- [ ] **Step 4: Run selection and CLI tests**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_selection.py \
  .pi/harness/tests/test_cli.py \
  .pi/harness/tests/test_campaign.py -v
```

Expected: all tests pass and one writer does not erase structural/reframing discovery.

---

### Task 6: Cheap score and schedule-semantic gates

**Files:**
- Modify: `.pi/harness/gates.py`
- Modify: `.pi/harness/tests/test_gates.py`
- Modify: `.pi/harness/cli.py`
- Modify: `.pi/harness/tests/test_cli.py`

**Interfaces:**
- Produces `preliminary_score_gate(frontier_score: int, candidate_score: int, *, lambda_objective=None,
  lambda_evidence=None) -> dict`.
- Produces `schedule_semantic_gate(evidence: Mapping[str, Any]) -> dict`.
- Adds CLI commands `check-preliminary-score` and `check-schedule-semantics`.

- [ ] **Step 1: Add failing gates from the supplied campaign**

Append to `.pi/harness/tests/test_gates.py`:

```python
from harness.gates import preliminary_score_gate, schedule_semantic_gate


class CheapCandidateGateTests(unittest.TestCase):
    def test_positive_schedule_score_delta_blocks_evaluation(self):
        with self.assertRaisesRegex(GateError, "1106686"):
            preliminary_score_gate(1_481_143_998, 1_482_250_684)

    def test_lambda_triage_cannot_override_score_regression(self):
        with self.assertRaisesRegex(GateError, "approved lambda objective"):
            preliminary_score_gate(
                1_481_143_998, 1_482_250_684,
                lambda_evidence={"rounds": 12, "estimate": 23.5, "verdict": "TRIAGE_ONLY"},
            )

    def test_schedule_semantics_rejects_classical_or_phase_mismatch(self):
        with self.assertRaisesRegex(GateError, "BROKEN"):
            schedule_semantic_gate({
                "paired_accessor": True, "scale_s2_checked": True,
                "targeted_test": "walk-replay-boundary-240", "classical_mismatches": 13,
                "phase_mismatches": 16,
            })
```

- [ ] **Step 2: Verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest .pi/harness/tests/test_gates.py -v
```

Expected: import failures for both new gates.

- [ ] **Step 3: Implement gates**

`preliminary_score_gate` returns the integer delta for a strict improvement. For a nonnegative delta, it
requires an explicit objective object with `approved=True`, positive `max_score_regression`, and numeric
`required_delta_ci_upper`, plus evidence whose `delta_ci_upper` meets that threshold and whose tier is not
`TRIAGE_ONLY`. Otherwise raise `GateError` including the delta.

`schedule_semantic_gate` requires:

- `paired_accessor is True`;
- `scale_s2_checked is True`;
- a nonempty `targeted_test` string;
- zero `classical_mismatches`;
- zero `phase_mismatches`.

Any mismatch raises `GateError("BROKEN: ...")`. This gate never reads λ evidence.

- [ ] **Step 4: Wire dry-run CLI commands and test**

`check-preliminary-score` accepts `--candidate-score`, optional `--lambda-objective`, and optional
`--lambda-evidence`. It loads the frontier dynamically. `check-schedule-semantics` accepts one evidence JSON
path.

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_gates.py \
  .pi/harness/tests/test_cli.py -v
```

Expected: all tests pass; no evaluator command is run.

---

### Task 7: Runtime CLI and parent recovery command

**Files:**
- Modify: `.pi/harness/cli.py`
- Modify: `.pi/harness/tests/test_cli.py`
- Modify: `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py`
- Modify: `.pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py`

**Interfaces:**
- Adds `init-run --dispatch <json>`.
- Adds `checkpoint-run --run-dir <path> --checkpoint <json>`.
- Adds `recover-run --run-dir <path> --kind <kind> --role <role> --experiment-id <id>
  --frontier-id <id> [--candidate-ops-sha256 <hash>]`.
- Recovery helper accepts `--run-dir` and `--worktree` after the worker is gone.

- [ ] **Step 1: Add CLI RED tests**

Add CLI tests that create a temporary root, call `init-run`, inspect immutable `dispatch.json`, call
`checkpoint-run`, then call `recover-run` without a worker result. Assert exit code 0, schema-valid
`WORKER_LOST`, and preserved terminal text.

Add recovery-helper tests asserting `--run-dir` includes full dispatch/checkpoint/result text and worktree
patch hashes, and prints `MISSING` for absent candidate files.

- [ ] **Step 2: Run tests and verify RED**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_cli.py \
  .pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py -v
```

Expected: parser errors for the new commands/arguments.

- [ ] **Step 3: Implement CLI adapters**

Keep all commands dry-run by default where mutation affects authoritative campaign state. Run-envelope files
are task-local evidence and may be created by their explicit commands. Validate all input JSON before
writing. Reject a run directory outside `.pi/campaigns/<campaign-id>/runs/<run-id>`.

Extend the recovery helper by reading, not duplicating, `runtime.py` result/paths. Never delete or alter the
worktree. Preserve complete text files and hash binary files.

- [ ] **Step 4: Run CLI/recovery tests**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/harness/tests/test_cli.py \
  .pi/harness/tests/test_runtime_recovery.py \
  .pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py -v
```

Expected: all tests pass.

---

### Task 8: Agent, prompt, skill, and validator contracts

**Files:**
- Modify: `.pi/agents/orchestrator-reviewer.md`
- Modify: `.pi/agents/frontier-auditor.md`
- Modify: `.pi/agents/exploitation-tuner.md`
- Modify: `.pi/agents/structural-researcher.md`
- Modify: `.pi/agents/cross-cell-reframer.md`
- Modify: `.pi/agents/implementer.md`
- Modify: `.pi/agents/lambda-metrologist.md`
- Modify: `.pi/agents/evaluator-certifier.md`
- Modify: `.pi/agents/strip-census-miner.md`
- Modify: `.pi/agents/nonce-grinder.md`
- Modify: `.pi/prompts/dispatch-campaign.md`
- Modify: `.pi/prompts/implement-candidate.md`
- Modify: `.pi/prompts/collect-and-review.md`
- Modify: `.pi/prompts/certify-and-promote.md`
- Modify: `.pi/skills/circuit-evidence/SKILL.md`
- Modify: `.pi/skills/circuit-evidence/scripts/validate_fleet.py`
- Modify: `.pi/skills/circuit-evidence/scripts/test_validate_fleet.py`

**Interfaces:**
- All agent outputs refer to result schema v2 and a parent-created run ID.
- Writer prompts require dispatch/checkpoint paths and absolute deadline phases.
- Validator rejects embedded frontier IDs, commits, scores, and operation hashes in agent/prompt authority.

- [ ] **Step 1: Add validator RED fixtures**

Add fixtures that fail when:

```text
an agent contains "The pinned score is 1,481,143,998"
a worker prompt lacks "dispatch.json" or "checkpoint.json"
an orchestrator prompt does not require EXPLOITATION, STRUCTURAL, and REFRAMING discovery
an evaluator prompt omits preliminary score and schedule-semantic gates
an implementation prompt relies on worker-only recovery
```

Run:

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/skills/circuit-evidence/scripts/test_validate_fleet.py -v
```

Expected: the upgraded requirements fail against the current agent/prompt tree.

- [ ] **Step 2: Apply one shared runtime contract to all roles**

Add this exact operational contract, adapted only for read/write ownership:

```text
The parent creates dispatch.json, terminal.log, and the initial checkpoint before launch. Read the bound
frontier and run ID from dispatch.json; do not copy authority from prose. Checkpoint after every completed
gate. At 80% of the absolute timeout, start no expensive command. At 90%, finalize schema-v2 result JSON and
emit recovery. A timeout is operational evidence only; never convert it to REFUTED, SAFE, or a ceiling.
```

Role-specific changes must match spec §6. In particular:

- remove fixed score/commit authority from `frontier-auditor`, `orchestrator-reviewer`, and
  `evaluator-certifier`;
- require tuning score-sign and schedule semantics in `exploitation-tuner`;
- require no-knob structural cards in `structural-researcher`;
- require lane repair in `cross-cell-reframer`;
- require gate checkpoints and soft deadlines in `implementer`;
- forbid λ from adjudicating correctness/score in `lambda-metrologist`;
- require cheap gates before evaluation in `evaluator-certifier`.

- [ ] **Step 3: Update campaign prompts**

`dispatch-campaign` must initialize parent receipts before `runs.all`, retain all three discovery lanes, and
distinguish writer admission. `implement-candidate` must pass run paths/deadlines and document parent timeout
recovery. `collect-and-review` must accept `WORKER_LOST` without scientific classification.
`certify-and-promote` must load the bound frontier dynamically.

- [ ] **Step 4: Update the evidence skill and validator**

Document parent-owned durability, checkpoint cadence, operational states, dynamic frontier authority,
preliminary score gate, and schedule-semantic gate. Extend static validation to enforce these terms without
embedding the current frontier values.

- [ ] **Step 5: Run skill and fleet validation**

```bash
PYTHONPATH=.pi python3 -m unittest \
  .pi/skills/circuit-evidence/scripts/test_validate_fleet.py \
  .pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py -v
python3 .pi/skills/circuit-evidence/scripts/validate_fleet.py --no-source-check
python3 /Users/makimakiver/.codex/skills/.system/skill-creator/scripts/quick_validate.py \
  .pi/skills/circuit-evidence
```

Expected: tests pass, `fleet validation: OK`, and `Skill is valid!`.

---

### Task 9: Full regression and scope verification

**Files:**
- Verify all files modified in Tasks 1–8.
- Do not modify live state or source to make verification pass.

**Interfaces:**
- Produces evidence that the upgrade is internally consistent and does not mutate active campaign data.

- [ ] **Step 1: Hash protected files before the final suite**

```bash
shasum -a 256 \
  .pi/frontier.json \
  .pi/state/campaign.json \
  results.tsv \
  src/point_add/trailmix_ludicrous/square.rs
```

Save the terminal output outside the repository or in the task transcript; do not create a repository file.

- [ ] **Step 2: Run the complete harness suite**

```bash
PYTHONPATH=.pi python3 -m unittest discover -s .pi/harness/tests -p 'test_*.py' -v
```

Expected: all existing and new harness tests pass with zero failures.

- [ ] **Step 3: Run the skill-script suite and validators**

```bash
PYTHONPATH=.pi python3 -m unittest discover \
  -s .pi/skills/circuit-evidence/scripts -p 'test_*.py' -v
python3 .pi/skills/circuit-evidence/scripts/validate_fleet.py --no-source-check
python3 /Users/makimakiver/.codex/skills/.system/skill-creator/scripts/quick_validate.py \
  .pi/skills/circuit-evidence
```

Expected: all tests pass, fleet validation reports OK, and the skill validator reports valid.

- [ ] **Step 4: Verify protected files are byte-identical**

Rerun the Step 1 `shasum` command and compare every line with the pre-suite output. Any difference fails the
upgrade; fix tests/runtime code so they use temporary fixtures.

- [ ] **Step 5: Verify no mutable frontier authority remains**

```bash
rg -n "bb237894|a2067dcf|1481143998|6519bd01424d5513" \
  .pi/harness .pi/agents .pi/prompts .pi/skills/circuit-evidence
```

Expected: no authoritative runtime/agent occurrence. Historical test fixtures may contain the supplied
candidate score delta and must label it fixture data.

- [ ] **Step 6: Verify file scope and inspect changes**

```bash
git status --short
git diff -- .pi
```

Expected: implementation changes are confined to `.pi/**`; the previously active `square.rs` and
`results.tsv` modifications remain present but unchanged by this work. Do not commit or clean them.

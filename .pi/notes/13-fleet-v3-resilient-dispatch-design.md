# Fleet v3 — resilient dispatch and evidence preservation

**Status:** Approved design, awaiting written-spec review  
**Scope:** `/Users/makimakiver/ecdsafail-challenge/.pi/**` only  
**Circuit source changes:** None  
**Frontier/campaign mutation during upgrade:** None

## 1. Goal

Upgrade the project-local Pi fleet so campaign evidence survives worker termination, discovery cannot
collapse into a single local tuning card, and expensive evaluation is not spent on candidates already known
to regress the score or violate a cheap semantic obligation.

The harness—not an agent’s final message—must own lifecycle durability. Agents still explain and measure;
deterministic parent-side code creates dispatch receipts, checkpoints evidence, classifies interruptions, and
preserves worktree state.

## 2. Triggering campaign evidence

The supplied campaign report records four design failures:

1. Managed-worktree preflight initially rejected a parent checkout whose dirtiness was confined to
   control-plane evidence.
2. Default infrastructure selection admitted only one schedule-family card; other lanes were excluded
   because they lacked an already-available local control or were covered by prior local refutations.
3. The worker timed out before producing `candidate-result.json` or a strict ledger handoff. The parent had
   to reconstruct the result from the worktree patch, logs, session record, and recovery evidence.
4. The implemented schedule candidate reached full evaluation despite a measured positive score delta. It
   then failed correctness and phase checks. Its unchanged 12-round λ triage was irrelevant and could not
   rescue it.

The supplied report binds those observations to repository commit `bb237894f8302f1be558aa929cce083a1b357569`
and artifact `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`.

At design time, the accessible checkout instead reports HEAD
`a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5`, active modifications to `results.tsv` and
`src/point_add/trailmix_ludicrous/square.rs`, and no local copy of the referenced campaign directory. The
upgrade therefore treats the pasted report as design input, not as locally verified campaign state. It must
not rewrite `.pi/frontier.json`, `.pi/state/campaign.json`, source, results, or campaign evidence.

## 3. Approaches considered

### A. Prompt-only checkpoint reminders

Add stronger instructions to each worker. This helps cooperative exits but cannot run after a provider
timeout or hard worker termination. Rejected as the primary solution.

### B. Parent-owned dispatch journal and recovery — selected

Create the authoritative run envelope before worker launch. Checkpoint evidence throughout the run and let
the parent synthesize an interruption result when the worker disappears. This directly addresses the
observed missing-result failure and is testable without model execution.

### C. Dedicated supervisor agent

Use another model worker to watch every dispatched worker. This consumes an additional concurrency slot,
duplicates deterministic bookkeeping, and remains exposed to provider interruption. Rejected.

## 4. Core invariants

1. A launched run always has a parent-owned durable record before the worker starts.
2. Worker termination never causes evidence loss and never becomes a scientific verdict.
3. `WORKER_LOST` and `INTERRUPTED` describe execution state, not hypothesis truth.
4. Control-plane dirtiness may be snapshotted; scored or trusted source dirtiness still blocks dispatch.
5. Discovery admission does not require an existing implementation flag.
6. Every campaign represents tuning, structural, and reframing lanes, or records a machine-checkable reason
   why a lane has no admissible card and immediately requests a reframe.
7. Positive deterministic score delta blocks expensive evaluation unless a predeclared, measured λ objective
   justifies the trade.
8. A 12-round λ scan is triage only. It cannot override failed correctness, failed phase cleanliness, or a
   score regression.
9. Mutable frontier values are loaded from validated state, never compiled into agent prose or Python
   constants.
10. No empirical outcome may be promoted to a global ceiling claim.

## 5. Architecture

### 5.1 Dynamic frontier identity

Replace compiled frontier facts in `.pi/harness/constants.py` with a validated runtime snapshot.

Stable constants remain:

- live nonce environment name: `SUB4_TAIL_NONCE`;
- full evaluation size: 9,024 shots;
- allowed lifecycle state names and role names.

Mutable values come from `.pi/frontier.json` through a single loader:

- frontier ID;
- repository commit;
- score;
- compressed and canonical operation hashes;
- geometry and λ evidence.

Frontier identity gains two provenance layers:

```json
{
  "repository_commit": "full commit SHA",
  "scored_source_tree_sha256": "hash of the scored source manifest",
  "trusted_source_tree_sha256": "hash of the trusted-input manifest",
  "ops_sha256": "compressed artifact hash",
  "canonical_ops_sha256": "semantic stream hash"
}
```

A documentation/control-plane-only commit may change `repository_commit` without claiming a new circuit.
The frontier becomes stale when the scored or trusted tree hash changes without matching artifact evidence.
This prevents documentation commits from causing false circuit drift while preserving full provenance.

### 5.2 Parent-owned run envelope

Before dispatch, the parent creates:

```text
.pi/campaigns/<campaign-id>/runs/<run-id>/
  dispatch.json
  checkpoint.json
  terminal.log
  result.json
  recovery/
```

`dispatch.json` is immutable and contains:

- campaign, run, experiment, lane, owner, and frontier IDs;
- exact agent and task text;
- timeout and soft-deadline policy;
- worktree path and base identity;
- control-plane snapshot hash;
- declared budget and commands;
- expected result path.

`checkpoint.json` is atomically replaceable and contains the latest completed gate, evidence paths, artifact
hashes, commands, changed paths, remaining obligations, and timestamp.

`terminal.log` is parent-captured stdout/stderr. The worker may print a recovery bundle, but durability does
not depend on the worker reaching that code.

`result.json` is written by exactly one of:

- the worker, followed by parent validation; or
- the parent interruption synthesizer when the worker does not return a valid result.

### 5.3 Lifecycle and interruption states

Add operational states without weakening scientific states:

```text
SCOPED -> DISPATCHED -> RUNNING -> CHECKPOINTED
CHECKPOINTED -> IMPLEMENTED | METROLOGY | EVALUATED
DISPATCHED | RUNNING | CHECKPOINTED -> INTERRUPTED | WORKER_LOST
INTERRUPTED | WORKER_LOST -> PARKED | SCOPED
```

`WORKER_LOST` means the runtime ended without a valid final result. `INTERRUPTED` means a controlled deadline,
provider, quota, or external stop. Neither may transition directly to `REFUTED`, `SAFE`, `CERTIFIED`, or
`PROMOTED`.

An interruption result must contain:

- the last validated checkpoint;
- terminal transcript path;
- worktree status and patch path;
- discovered artifact/log hashes;
- missing required fields;
- explicit `scientific_verdict: null`;
- resume/reopen conditions.

### 5.4 Soft deadline protocol

Every worker receives the absolute deadline and three phases:

- **0–80%:** normal bounded work;
- **80–90%:** do not start another expensive command; checkpoint current evidence;
- **90–95%:** finalize result JSON and emit the recovery bundle;
- **95–100%:** parent collection grace period only.

The parent owns the timers. At 80% and 90%, it records deadline events even if the worker does not respond.
At the hard timeout, the parent captures process status, terminal output, worktree patch, artifact hashes, and
the last checkpoint before synthesizing `WORKER_LOST`.

### 5.5 Worktree preflight

Preflight classifies paths rather than requiring a globally clean parent checkout:

| Path class | Dirty parent policy |
|---|---|
| trusted source (`src/bin/**`, `src/circuit/**`, `src/sim/**`, curve harness) | block |
| scored source (`src/point_add/**`) | block unless it is the explicitly selected immutable base |
| canonical generated artifacts (`ops.bin`, `score.json`, `results.tsv`) | block or copy to an isolated evidence snapshot; never silently reuse |
| `.pi/**` control plane | allow after content-addressed snapshot |
| unrelated documentation | allow and record |

The managed writer worktree is based on the exact scored-source identity, then receives the approved
control-plane snapshot as read-only task context. Parent `.pi` dirtiness alone cannot reject a dispatch.

### 5.6 Portfolio and admission

Separate discovery admission from implementation admission.

Discovery cards need a mechanism, falsifier, search radius, cheapest discriminator, budget, unresolved
directions, and reopening trigger. They do not need an existing flag or implementation-ready patch.

Each campaign round must contain:

- at least one exploitation/tuning discovery result;
- at least one structural discovery result;
- at least one cross-cell/reframing discovery result.

A lane may return no card only with one of:

- formal proof/complete enumeration;
- a referenced prior refutation that covers the same mechanism and assumptions;
- a documented dependency whose reopening trigger is explicit.

If only one card is implementation-ready, that card may be the only writer dispatch, but the other discovery
lanes still run. “No existing local knob” routes work to structural design; it does not eliminate the lane.

The selector rejects duplicate mechanisms and enforces at most one writer card per source family. It does
not confuse “excluded from implementation” with “excluded from research.”

### 5.7 Cheap candidate gates

The gate order before full evaluation becomes:

1. frontier and source-tree identity;
2. ownership and default-off control;
3. default artifact byte/canonical identity;
4. candidate operation liveness and exact hash;
5. card-specific semantic obligation;
6. deterministic/static T and Q delta;
7. relevant selftest or reduced classical model;
8. λ triage/adaptive metrology when exposed;
9. full 9,024-shot evaluator only for surviving candidates.

If deterministic evidence gives `candidate_score_delta >= 0`, stop before full evaluation unless the card
predeclares a λ-improvement objective and already has measured evidence meeting its threshold. An unchanged
12-round triage result is not such evidence.

Schedule and paired walk/replay edits additionally require:

- proof that both producer and consumer use the same accessor/configuration;
- a targeted classical walk/replay equivalence or invariant test across the changed boundary;
- explicit handling of scale and `s2` conditionality;
- a falsifier that treats any classical or phase mismatch as `BROKEN`, not λ noise.

The supplied `SCHED_J2[240]/GAP_J2[240]` result would therefore stop at the deterministic score gate because
its rounded score delta was `+1,106,686`. If allowed for a separately approved λ objective, its semantic test
or full verifier would classify it `BROKEN`; unchanged triage λ could not override either decision.

### 5.8 Parent-side recovery

Extend the recovery helper so it can be invoked by the parent with a run directory and optional worktree,
even after the worker is gone. It collects:

- complete `results.tsv`, `score.json`, experiment/card/result JSON, checkpoint, and dispatch receipt;
- terminal transcript and exact commands;
- worktree status, full patch, and changed-path list;
- hashes and sizes for binary artifacts;
- explicit `MISSING` markers;
- continuation/resume metadata.

Recovery runs before worktree deletion. Worktree cleanup remains a separate, explicitly authorized step.

## 6. Role changes

### `orchestrator-reviewer`

- Load the frontier dynamically.
- Require parent-created dispatch receipts and lane coverage.
- Distinguish discovery exclusion from writer admission.
- Treat missing worker result as `WORKER_LOST`, never immediate scientific rejection.

### `frontier-auditor`

- Stop embedding a score or commit in prose.
- Verify repository, scored-tree, trusted-tree, and artifact identities separately.
- Report whether a commit difference is circuit-affecting or control-plane-only.

### `exploitation-tuner`

- Require deterministic score-sign projection before recommending full evaluation.
- For schedule edits, provide the targeted semantic obligation and boundary test.
- Do not use unchanged 12-round λ triage as positive evidence.

### `structural-researcher`

- Produce research cards without requiring pre-existing knobs.
- Prefer new mechanisms when local control families are already refuted.
- Record implementation dependencies rather than allowing infrastructure selection to erase the lane.

### `cross-cell-reframer`

- Run whenever tuning/structural selection yields fewer than the required lane coverage.
- Challenge family-wide exclusion inferred from one local refutation.
- Return a composite card or a machine-checkable dependency/reopening result.

### `implementer`

- Checkpoint after patch creation, default-identity proof, candidate hash, and each selftest.
- Observe soft deadlines and stop launching expensive commands after the 80% boundary.
- Never be the sole owner of the result ledger or terminal transcript.

### `lambda-metrologist`

- Preserve 12-round scans as triage-only.
- Refuse to adjudicate correctness, phase cleanliness, or positive score deltas.
- Checkpoint partial counts and stopping state so a timeout remains analyzable.

### `evaluator-certifier`

- Refuse full evaluation until deterministic score and semantic gates are recorded.
- Classify correctness/phase failure as `BROKEN` independently of λ.
- Load comparison metrics from the bound frontier rather than prose constants.

### `strip-census-miner` and `nonce-grinder`

- Adopt parent-owned receipts, checkpoints, deadlines, and recovery.
- Preserve their existing explicit-compute approval requirements.

## 7. Data and schema changes

### Result schema

Add:

- `run_id`;
- `dispatch_ref`;
- `checkpoint_ref`;
- `terminal_ref`;
- `scientific_verdict` (nullable for operational failure);
- `interruption` object;
- `recovery_ref`;
- `resume_conditions`.

### Campaign schema

Add:

- lane coverage requirements and observed coverage;
- dispatch/run references;
- control-plane snapshot hash;
- scored/trusted source-tree hashes;
- interruption counters separate from scientific verdict counters.

### Frontier schema

Add scored/trusted source-tree identities and distinguish them from repository commit provenance. Migration
must preserve existing metrics, hashes, λ evidence, nonce binding, and promotion history exactly.

## 8. Files

### Modify

- `.pi/harness/constants.py`
- `.pi/harness/schema.py`
- `.pi/harness/campaign.py`
- `.pi/harness/cli.py`
- `.pi/harness/gates.py`
- `.pi/harness/ownership.py`
- `.pi/schemas/frontier.schema.json`
- `.pi/schemas/campaign.schema.json`
- `.pi/schemas/result.schema.json`
- `.pi/skills/circuit-evidence/SKILL.md`
- `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py`
- `.pi/skills/circuit-evidence/scripts/validate_fleet.py`
- all `.pi/agents/*.md`
- `.pi/prompts/dispatch-campaign.md`
- `.pi/prompts/implement-candidate.md`
- `.pi/prompts/collect-and-review.md`
- `.pi/prompts/certify-and-promote.md`

### Create

- `.pi/harness/frontier.py` — dynamic validated frontier/source-tree identity loader
- `.pi/harness/runtime.py` — dispatch receipts, atomic checkpoints, deadlines, interruption synthesis
- `.pi/harness/selection.py` — lane coverage and discovery/writer admission separation
- `.pi/schemas/dispatch.schema.json`
- `.pi/schemas/checkpoint.schema.json`
- `.pi/harness/tests/test_frontier_runtime.py`
- `.pi/harness/tests/test_runtime_recovery.py`
- `.pi/harness/tests/test_selection.py`
- `.pi/harness/tests/test_preflight.py`

No file outside `.pi/**` is changed. Existing `.pi/frontier.json`, `.pi/state/campaign.json`, campaign
records, experiment evidence, continuations, and findings are migration inputs, not upgrade-time outputs.

## 9. Test strategy

Use test-first implementation with the supplied campaign failure as the RED fixture.

1. **Timeout durability:** simulate a worker that creates a patch and checkpoint but never writes its final
   result. Assert the parent emits valid `WORKER_LOST` JSON containing patch, transcript, hashes, missing
   fields, and `scientific_verdict: null`.
2. **Controlled deadline:** simulate 80% and 90% events. Assert no new expensive command is admitted after
   80% and recovery/finalization begins by 90%.
3. **Dirty control plane:** dirty `.pi/**` only and assert preflight snapshots and proceeds. Dirty scored or
   trusted source and assert preflight blocks.
4. **Portfolio coverage:** supply one implementation-ready tuning card and structural/reframe research cards
   without flags. Assert all three discovery lanes remain represented while only the tuning card reaches the
   writer queue.
5. **False family exclusion:** supply one refuted local schedule mechanism and a structurally distinct card.
   Assert the refutation does not exclude the distinct mechanism.
6. **Score regression gate:** use the supplied candidate metrics and assert `+1,106,686` blocks full
   evaluation without an approved, already-measured λ objective.
7. **Schedule semantic gate:** omit the boundary equivalence test and assert the candidate cannot leave
   `IMPLEMENTED`; supply classical or phase mismatches and assert `BROKEN` independently of λ.
8. **Dynamic frontier:** load two frontiers with different commits/metrics and assert agents/harness use the
   supplied bound record rather than compiled values.
9. **Docs-only commit:** change only control-plane provenance and assert scored-source identity remains
   stable; change scored source and assert the frontier becomes stale.
10. **Regression suite:** all existing harness, schema, ownership, promotion, continuation, recovery, and
    fleet-validator tests remain green.

Live subagent forward-testing is separate because it consumes model/compute budget and may interact with
the active checkout. Static/runtime simulation is the required implementation gate.

## 10. Migration

1. Read existing frontier, campaign, and result records without mutating them.
2. Compute source-tree identities into a temporary migration fixture.
3. Validate the new schemas against migrated copies.
4. Run all tests and fleet validation.
5. Update agent and prompt contracts only after runtime enforcement passes.
6. Do not rewrite live state automatically. A later explicit audit command may apply schema migration after
   the active source work is clean and the user authorizes it.

## 11. Success criteria

Fleet v3 is complete when:

1. A hard worker timeout still yields a schema-valid, parent-owned interruption result and complete recovery
   evidence.
2. Dirty `.pi` state alone no longer blocks a managed writer worktree, while dirty scored/trusted source does.
3. Every campaign round covers tuning, structural, and reframing discovery independently of writer-card
   availability.
4. The supplied positive-score schedule candidate is rejected before full evaluation unless a separate λ
   objective has already met its gate.
5. Schedule changes cannot advance without a targeted semantic boundary test.
6. No agent or harness module embeds the current frontier commit, score, or operation hashes as authority.
7. Existing frontier/campaign/evidence files remain byte-for-byte unchanged by the upgrade and its tests.
8. All new and existing harness tests, the fleet validator, and the circuit-evidence skill validator pass.

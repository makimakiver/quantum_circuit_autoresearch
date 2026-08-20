# Fleet v2 design — evidence-gated circuit optimization

**Status:** Approved for implementation  
**Scope:** `/Users/makimakiver/ecdsafail-challenge/.pi/**` only  
**Circuit source changes:** None

## 1. Goal

Upgrade the Pi fleet from a safe scouting system into a closed optimization loop that can carry a
candidate from a source-backed hypothesis to an officially certifiable lower score. Preserve the current
strengths—cell ownership, one editing role, anti-noop checks, worktree isolation, and independent
metrology—while fixing the stale-state, dead-route, nonce-grinding, and statistical-resolution failures
observed in fleet round 1.

The fleet optimizes:

```text
S = round(average executed Toffoli) × max referenced qubit id
```

subject to correctness, reversibility, phase cleanliness, ancilla cleanliness, and a measured intrinsic
failure rate λ whose implied nonce-grind cost fits an explicit user-approved compute budget.

## 2. Success criteria

Fleet v2 is complete when all of the following hold:

1. A campaign refuses to start when its frontier commit, artifact hashes, geometry, or metrics are stale.
2. No production instruction routes changes through `configure_ecdsafail_submission_route()` or
   `src/point_add/rounds/dialog/`.
3. Every nonce operation uses the live `SUB4_TAIL_NONCE` control discovered from source.
4. Agent specifications derive mutable facts such as `ITERS`, tape width, score, and nonce from the pinned
   frontier rather than embedding them as timeless constants.
5. A functional circuit change can reach provisional-candidate status without first passing on the old
   stream's ground seed.
6. Expensive λ confirmation and nonce grinding require explicit budgets and occur only after cheap gates.
7. Shared primitives, post-emission rewrites, and cross-cell representations each have a named owner.
8. Every experiment has a content-addressed record containing the source commit, flags, nonce set,
   operation hashes, measurements, verdict, and rollback disposition.
9. A deterministic validator rejects the known round-1 contradictions and malformed fleet files.
10. No agent may infer a global ceiling from a bounded local search; only scoped plateaus may be reported.
11. Before every failure, rejection, timeout, budget stop, or handoff, the agent prints a complete recovery
    bundle to the terminal so synchronization loss cannot erase the evidence.

## 3. Evidence motivating the redesign

Round 1 provides the RED baseline for this process redesign:

- Four of five first-pass reports targeted compiled but unreachable dialog code.
- Every real `ops.bin` change invalidated the existing ground seed.
- The current accept gate nevertheless requires all four 9,024-shot checks before any grind stage.
- Twelve dirty-scan rounds yielded too few fault events for useful 10% λ discrimination; the retained
  estimate is roughly 600 rounds per arm, with adaptive stopping preferred.
- The campaign prompt consolidates accepted flags through a function that the live-entrypoint audit marks
  dead.
- The round-1 handoff names `DIALOG_TAIL_NONCE`, while the retained trap note names
  `SUB4_TAIL_NONCE` as live.
- The tape specialist embeds 258 iterations and 602 tape qubits, while the same fleet's round-1 note records
  261 iterations and 609 tape qubits.
- Frontier values differ across the playbook and round-1 handoff.

These are orchestration failures, not reasons to weaken the evidence standard.

The later tuning win also shows that the first fleet confused exhaustion of a local transformation class
with exhaustion of the circuit. Fleet v2 therefore treats tuning, structural search, and global reframing as
different search radii. A failure in one radius escalates or parks the hypothesis; it never establishes a
global ceiling.

## 4. Approaches considered

### A. Patch only the contradictory sentences

Smallest change, but leaves the impossible promotion order, missing grind capability, weak state tracking,
and uncovered optimization surfaces. Rejected.

### B. Staged, evidence-gated fleet with a pinned frontier

Retain the current specialists and introduce explicit control-plane, cross-cutting, and certification
roles. Separate provisional evaluation from certification and make expensive work budget-gated.
Recommended.

### C. Fully autonomous distributed search

Automate large parameter sweeps and nonce grinding across external compute. Potentially powerful, but it
requires infrastructure, credentials, spend controls, and failure recovery outside `.pi`. Deferred until
the local v2 protocol produces at least one provisional candidate worth scaling.

## 5. Architecture

### 5.1 Control plane

#### `orchestrator`

Own the experiment queue and state machine, not mutable facts copied into prose. Before dispatching work:

1. Invoke `frontier-auditor`.
2. Require a current `.pi/frontier.json` with matching source commit and artifact evidence.
3. Select hypotheses by expected score improvement, λ/grind feasibility, evidence cost, and overlap risk.
4. Dispatch read-only scouting in parallel; serialize shared-primitive and paired-symbol edits.
5. Distinguish provisional acceptance from certified promotion.

The orchestrator never treats a failed old-seed full evaluation in the intrinsic band as automatic proof
that a functional candidate is wrong.

The orchestrator also enforces a search portfolio: 50% of campaign capacity for exploitation and tuning,
30% for structural changes, and 20% for algorithmic or representation-level reframing. Empty local rounds
trigger a cross-cell reframe round rather than campaign termination.

#### `frontier-auditor` — new

Read-only with respect to circuit source and generated circuit artifacts. Inspect the actual source
entrypoint, schedule, nonce control, git commit, `score.json`, `ops.bin`, and latest valid result. It may
write only `.pi/frontier.json` and `.pi/experiments/*.json`; produce or validate the frontier record and fail
closed when metrics cannot be tied to the current source and operation artifact.

This role also validates that a proposed production toggle is reached from `point_add::build()`.

#### `lambda-metrology`

Remain independent. Replace the fixed “12 rounds is admissible” rule with sequential tiers:

- **Triage:** 12 paired rounds; classify catastrophic, intrinsic-band, or plausibly clean behavior. Never
  use this tier for a close λ acceptance claim.
- **Estimate:** continue paired rounds until the predeclared confidence interval answers the task card's
  decision threshold or its budget is exhausted.
- **Confirmation:** for approximately 10% resolution near λ≈23.5, expect roughly 600 rounds per arm unless
  observed event counts justify earlier stopping.

Report event counts, estimator, confidence interval, paired nonce set, stopping rule, and compute used.

### 5.2 Optimization plane

Retain and refresh the five current specialists:

- `slope-quotient`
- `tape-codec`
- `bezout-apply`
- `pair2-erase`
- `square-coordinate`

Each specialist must read `.pi/frontier.json` and then verify its cell facts from source. Descriptions may
state stable ownership and invariants, but must not hard-code a current frontier, iteration count, tape
width, nonce, line number, or gate count as an unconditional truth.

Add three missing owners:

#### `shared-primitives` — new

Own cell P: `gidney.rs`, `arith.rs`, `comparator.rs`, `mcx.rs`, and other nonlinear primitives reused by
multiple cells. Require an invocation census and dependency map before proposing a change. Price executed
Toffoli and all affected call sites; shared-primitive experiments serialize globally.

#### `stream-optimizer` — new

Own post-emission constprop, fanout, cancellation, deep-strip provenance, and final-stream identity. Require
content-addressed operation identities and occupancy checks. A raw emission saving is not a win until the
final stream is re-mined or proves existing transforms remain valid.

#### `cross-cell-architect` — new

Own representation-level and paired multi-cell hypotheses: one-traversal alternatives, walk/codec/replay
co-design, global liveness, square/reduction fusion, and exact synthesis boundaries. It produces designs
and proof obligations, not source edits. This role may cross specialist boundaries only through one
explicit composite task card.

### 5.3 Execution and certification plane

#### `implementer`

Remain the only agent type allowed to edit circuit source. Multiple implementer instances may work only in
distinct orchestrator-created worktrees; one writer exists per worktree. Promote an accepted experiment
through the live `point_add::build()` route and its live `TLM_*` defaults. Never consolidate through the
legacy dialog configurator.

After implementation, stop at provisional evidence unless the candidate is byte-identical to the pinned
artifact. Do not demand an old-seed clean result from a changed semantic stream.

#### `nonce-grinder` — new

Read-only with respect to circuit source. It may append evidence only to the candidate's
`.pi/experiments/*.json` record. Accept only a provisional candidate, its exact operation geometry, the live
`SUB4_TAIL_NONCE` control, and a user-approved trial/time/compute budget. Record the search method, range
allocation, tested count, candidate nonce, exact artifact hash, and whether the four verifier checks pass.
A nonce result belongs only to the exact stream that generated it.

Nonce grinding is certification work, not a structural optimization claim.

## 6. State and artifacts

### 6.1 `.pi/frontier.json`

Store one authoritative frontier record with this logical schema:

```json
{
  "schema_version": 1,
  "state": "pinned|stale",
  "source_commit": "full git sha",
  "entrypoint": "point_add::build -> trailmix_ludicrous",
  "nonce_env": "SUB4_TAIL_NONCE",
  "geometry": {
    "jump": 2,
    "iterations": 261,
    "baked_iterations": 258,
    "dialog_tape_qubits": 609
  },
  "artifact": {
    "ops_sha256": "sha256 or null",
    "canonical_ops_sha256": "sha256 or null"
  },
  "metrics": {
    "average_executed_toffoli": null,
    "rounded_toffoli": null,
    "qubits": null,
    "score": null
  },
  "verification": {
    "shots": 9024,
    "four_checks": "pass|fail|unknown",
    "evidence_source": "full-eval|byte-identical|unknown"
  },
  "lambda": {
    "estimate": null,
    "confidence_interval": null,
    "rounds": null,
    "nonce_set_id": null
  }
}
```

The auditor derives geometry from source. The example numbers above describe the observed v1→v2 migration
point and must be regenerated when source changes. Missing artifact evidence sets `state` to `stale` and
blocks campaign execution.

### 6.2 Experiment records

Write one `.pi/experiments/<experiment-id>.json` per implemented card. Required fields:

- cell and owner;
- hypothesis and class;
- source commit and worktree;
- baseline frontier identifier;
- flags and source diff hash;
- baseline/candidate operation hashes and op-count delta;
- selftests and cheap probes;
- T, Q, score estimate and evidence level;
- λ paired set, events, interval, rounds, and stopping reason;
- grind budget and result when applicable;
- state, verdict, rejection reason, and rollback disposition.

Do not overwrite rejected experiments; they are negative evidence.

## 7. Candidate state machine

```text
Discovery: IDEA -> SCOPED -> MODELED -> CHEAP_PROBE -> CANDIDATE
Promotion: CANDIDATE -> IMPLEMENTED -> PROVISIONAL
  -> GRIND_APPROVED
  -> GROUND_SEED_FOUND
  -> CERTIFIED
  -> PROMOTED
```

Non-promotion outcomes are deliberately distinct:

- `INCONCLUSIVE`: evidence cannot decide; the hypothesis remains live.
- `PARKED_BUDGET`: its declared exploration budget ended; record an explicit reopening trigger.
- `REFUTED`: a predeclared falsification condition was observed.
- `LOCAL_PLATEAU`: no win was found within the recorded transformation class and budget.
- `EXHAUSTED`: allowed only after a formal bound or complete enumeration, never from time or test count.
- `NOOP`, `BROKEN`, `NO_GROUND_SEED`, and `REGRESSION`: operational outcomes, not ceiling claims.

Agents must never report a global circuit ceiling. The strongest empirical statement is: “No improvement
was found in search scope X under budget Y.” Local specialists may not close their own cell; a
`LOCAL_PLATEAU` receives an independent rejection review from `cross-cell-architect` before parking.

Early discovery may carry evidence debt. An idea needs a mechanism, predicted sign for T/Q/λ, cheapest
discriminator, falsification condition, and exploration budget—not a full evaluation. Evidence debt must be
repaid before promotion. Each hypothesis receives at most two refinement attempts and one cross-cell
escalation unless the orchestrator approves a new budget.

### Provisional gate

Require:

1. Frontier was pinned before the experiment.
2. The intended source path changed and `ops.bin` differs.
3. Relevant cell selftests pass.
4. No catastrophic classical or phase signature appears.
5. Deterministic/correctly paired evidence predicts a lower score, or materially lower λ at no worse
   predicted score.
6. Adaptive λ evidence makes the expected grind cost fit a stated budget, or marks the card
   `STATISTICALLY_INCONCLUSIVE`.

An intrinsic-band failure on the old ground seed does not fail this gate by itself.

### Certification gate

Require:

1. User approval of the grind budget.
2. A ground nonce found through `SUB4_TAIL_NONCE` for the exact candidate stream.
3. Full 9,024-shot correctness, reversibility, phase, and ancilla checks green.
4. Exact artifact and canonical-operation hashes recorded.
5. Exact `round(T) × Q` strictly below the refreshed official frontier, or lower λ at equal/lower score
   when that is the approved objective.

## 8. Campaign flow

1. Audit and pin the frontier.
2. Fan out read-only specialists against distinct cells.
3. Have `cross-cell-architect` identify composite opportunities after specialist reports arrive.
4. Rank cards by expected value divided by evidence and grind cost.
5. Implement at most three isolated cards, respecting shared-source serialization.
6. Apply cheap gates: source reachability, op diff, selftest, deterministic counts, and triage scan.
7. Run adaptive metrology only for candidates surviving cheap gates.
8. Mark successful candidates provisional.
9. Ask the user for a specific grind budget for selected provisional candidates.
10. Grind, certify, refresh the official frontier, and promote through the live build route.
11. Re-pin after every accepted change; never stack against stale artifacts.

## 9. Failure handling and budgets

- Never start expensive work without a task-card budget in trials, wall time, and parallelism.
- Stop metrology when the decision interval is decisive or the budget is exhausted.
- Stop grinding at the approved limit and report `NO_GROUND_SEED`; do not silently extend it.
- Preserve the frontier worktree and artifacts. Discard only isolated rejected worktrees after their
  experiment records are complete.
- Treat unavailable or mismatched `score.json`, `ops.bin`, source commit, or nonce as stale evidence.
- Never publish, submit, sync, reset, or force-overwrite without separate user authorization.
- A timeout, noisy small sample, absent gain, or exhausted budget is not `REFUTED` or `EXHAUSTED`.
- Every parked hypothesis records unresolved directions and source/frontier/primitive changes that reopen it.
- Zero accepted cards triggers a reframe round. Campaign stop requires the budget to be exhausted and every
  live or parked hypothesis to have a recorded disposition and reopening trigger.

### 9.1 Terminal recovery bundle

Filesystem artifacts are structured evidence, but the terminal transcript is the synchronization-safe
recovery copy. Immediately before every failure, rejection, timeout, budget stop, rollback, worktree
discard, or final handoff, every role runs:

```bash
python3 .pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py \
  --status <STATUS> --reason '<one-line reason>' --experiment-id <ID> \
  --command '<exact reproduction command>'
```

The command prints, between `FLEET_RECOVERY_BUNDLE_BEGIN` and `FLEET_RECOVERY_BUNDLE_END` markers:

- status, reason, experiment ID, UTC timestamp, source commit, and working tree state;
- every supplied reproduction command and relevant flag;
- the complete contents of `results.tsv`, `score.json`, and the experiment JSON when present;
- SHA-256 and size for `ops.bin` and other binary artifacts, never raw binary bytes;
- explicit `MISSING` markers for expected files that do not exist.

The bundle command must run before rollback because rollback may destroy the only useful diff. Failure of a
measurement command does not excuse this protocol. If the helper itself cannot run, the agent prints the
same fields manually with the two markers. No role may claim completion after only writing a file.

## 10. Deterministic fleet validation

Add a validator under `.pi/skills/circuit-evidence/scripts/` and run it before every campaign. It must:

1. Parse every agent's YAML frontmatter and confirm referenced project-local skills exist.
2. Confirm every agent named by campaign prompts exists.
3. Reject production instructions containing `configure_ecdsafail_submission_route` or
   `DIALOG_TAIL_NONCE`.
4. Require the live terms `trailmix_ludicrous` and `SUB4_TAIL_NONCE` in the control-plane contract.
5. Detect unconditional stale geometry/frontier claims in agent descriptions.
6. Validate `.pi/frontier.json` schema and compare its commit and geometry with source.
7. Confirm the candidate/promotion state names and required experiment fields are consistent across the
   orchestrator, prompts, and evidence skill.
8. Require every agent and campaign prompt to reference the terminal recovery protocol.

The validator reports every mismatch and exits nonzero. It never edits circuit source or generated
artifacts.

## 11. File changes after approval

### Modify

- `.pi/agents/orchestrator.md`
- `.pi/agents/implementer.md`
- `.pi/agents/lambda-metrology.md`
- `.pi/agents/slope-quotient.md`
- `.pi/agents/tape-codec.md`
- `.pi/agents/bezout-apply.md`
- `.pi/agents/pair2-erase.md`
- `.pi/agents/square-coordinate.md`
- `.pi/prompts/improve-performance.md`
- `.pi/prompts/optimize-cell.md`
- `.pi/skills/circuit-evidence/SKILL.md`
- `.pi/notes/07-pi-agent-playbook.md`

### Create

- `.pi/agents/frontier-auditor.md`
- `.pi/agents/shared-primitives.md`
- `.pi/agents/stream-optimizer.md`
- `.pi/agents/cross-cell-architect.md`
- `.pi/agents/nonce-grinder.md`
- `.pi/frontier.json`
- `.pi/experiments/.gitkeep`
- `.pi/skills/circuit-evidence/scripts/validate_fleet.py`
- `.pi/skills/circuit-evidence/scripts/test_validate_fleet.py`
- `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py`
- `.pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py`

No Rust source, trusted harness file, `ops.bin`, `results.tsv`, or external Pi configuration is modified by
this fleet upgrade.

## 12. Validation of the upgrade

Use the observed round-1 failures as baseline cases. The upgraded fleet passes when:

1. Static tests fail on fixtures containing the dead consolidation route, wrong nonce control, stale tape
   geometry, missing agents, and mismatched frontier commit.
2. The same tests pass on the upgraded `.pi` tree.
3. Every agent and prompt has valid frontmatter and resolvable references.
4. Search finds no production instruction using the two dead controls.
5. A dry-run campaign stops at `UNPINNED` when frontier evidence is absent.
6. A changed-stream scenario reaches `PROVISIONAL` without falsely requiring the old nonce to pass.
7. A grind request pauses for explicit user budget approval.
8. A simulated failed experiment prints complete text artifacts, binary hashes, missing-file markers, and
   both recovery delimiters to stdout.

Forward-testing with live subagents is a separate, explicitly authorized validation step because it consumes
model and compute budget. The static suite and existing round-1 record provide the initial RED/GREEN basis.

# Project-local campaign harness implementation plan

> **For agentic workers:** implement task-by-task with tests first. Do not launch metrology, census, or nonce
> grinding while building this harness.

**Goal:** Replace mixed role/evidence orchestration with a schema-validated, recovery-aware project-local
harness that preserves the certified circuit by default.

**Architecture:** Focused `.pi/agents` produce or review structured records. A Python standard-library
package validates state, transitions, ownership, gates, recovery data, continuations, and result collection.
Pi project prompts use one asynchronous `workflowScript` with `runs.all` for independent read-heavy work and
managed worktrees for the sole source writer.

**Tech stack:** Pi 0.84 project agents/prompts, `pi-subagents` workflowScript, Python 3 standard library,
JSON Schema documents, Markdown evidence cards.

**Spec:** `.pi/notes/11-project-local-harness-design.md`

## Global constraints

- Preserve the default circuit and all current source behavior.
- Preserve `src/point_add/trailmix_ludicrous/square.rs` as the parked, default-off EXP-SQ-01 edit.
- Never modify trusted source outside `src/point_add/**`.
- Keep score `1,481,143,998` and both pinned hashes authoritative until certified promotion.
- Do not run a circuit build, full evaluator, long scan, census pilot, full census, or nonce grind here.

### Task 1: State, lifecycle, and gate behavior

**Files:** create `.pi/harness/{__init__,constants,schema,gates,ownership,continuations,cli}.py`, JSON schemas,
campaign state, and tests under `.pi/harness/tests/`.

- Write failing behavioral tests for the pinned frontier, legal/illegal transitions, default hash identity,
  P2/BZ lambda verdicts, 9,024-shot certification, grind approval, strict score improvement, and role-specific
  changed-path ownership.
- Run the tests and confirm failures are caused by missing implementation.
- Add the minimum implementation and rerun to green.

### Task 2: Recovery and detached continuation behavior

**Files:** extend `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py`; add continuation manifests and
focused tests.

- Write failing tests for saved manifest output, artifact hashes, continuation metadata, and done-marker
  observation without process restart.
- Implement the behavior, preserving the existing terminal bundle contract.
- Register P2/BZ and EXP-SM-01 evidence paths without launching either workload.

### Task 3: Agent and orchestration refactor

**Files:** refactor `.pi/agents/*.md`, `.pi/settings.json`, `.pi/prompts/*.md`, and fleet validation tests.

- Define the required narrow roles and explicit write scopes.
- Replace cell-specific mutable campaign facts in roles with card-driven routing.
- Add an asynchronous discovery prompt, serialized implementation prompt, collection/review prompt, and
  promotion prompt with explicit gates.
- Validate frontmatter, referenced skills, role names, edit scopes, recovery contract, and required gates.

### Task 4: Operator handoff and active guidance

**Files:** create `.pi/README.md`; update `AGENTS.md`, `.pi/skills/circuit-evidence/SKILL.md`, and active
playbook/orchestrator guidance.

- Document exact validate, status, continuation, dispatch, collect, gate, and recovery commands.
- Correct the active frontier and live profiling names while keeping historical evidence documents intact.
- Document what remains blocked and what requires explicit approval.

### Task 5: Lightweight verification

- Run all harness and recovery tests.
- Compile every Python harness script.
- Parse every JSON state/schema/config file and every agent frontmatter block.
- Verify current `ops.bin` compressed and canonical hashes without rebuilding it.
- Run the harness status/continuation commands and scoped whitespace validation.
- Inspect the final diff to confirm no trusted source or current circuit behavior changed.

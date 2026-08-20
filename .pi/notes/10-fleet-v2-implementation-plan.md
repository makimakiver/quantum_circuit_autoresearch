# Fleet v2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended)
> or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Upgrade the project-local Pi fleet so it explores beyond local plateaus, promotes only certified
evidence, and preserves every failed run in the terminal transcript.

**Architecture:** A discovery lane permits bounded speculative work and hands candidates to a strict
promotion lane. A pinned frontier and structured experiment records coordinate specialist roles. A
read-only recovery helper prints durable terminal bundles, while a deterministic validator enforces fleet
contracts before campaigns run.

**Tech Stack:** Markdown agent specifications, JSON state, Python 3 standard-library validation scripts.

**Spec:** `.pi/notes/09-fleet-v2-design.md`

## Global Constraints

- Modify `.pi/**` only; do not edit Rust source, the trusted harness, generated circuit artifacts, or global
  Pi configuration.
- Only `implementer` may edit circuit source during later campaigns.
- No empirical search may claim a global circuit ceiling; report scoped plateaus with budget and radius.
- Emit a terminal recovery bundle before failure, rejection, rollback, timeout, budget stop, or handoff.
- Do not commit, publish, submit, sync, reset, or discard worktrees without separate user authorization.

---

### Task 1: Recovery-bundle behavior

**Files:**
- Create: `.pi/skills/circuit-evidence/scripts/test_emit_recovery_bundle.py`
- Create: `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py`

**Interfaces:**
- Consumes: repository path, status, reason, experiment ID, reproduction commands.
- Produces: a complete text recovery bundle on stdout and a nonzero exit only for invalid CLI usage.

- [ ] Write tests requiring delimiters, metadata, full text artifacts, binary hashes, and missing markers.
- [ ] Run the test and verify it fails because the helper does not exist.
- [ ] Implement the read-only helper using the Python standard library.
- [ ] Run the focused tests and verify they pass.

### Task 2: Static fleet validator

**Files:**
- Create: `.pi/skills/circuit-evidence/scripts/test_validate_fleet.py`
- Create: `.pi/skills/circuit-evidence/scripts/validate_fleet.py`
- Create: `.pi/frontier.json`
- Create: `.pi/experiments/.gitkeep`

**Interfaces:**
- Consumes: a project root containing `.pi`, agent specs, prompts, and frontier JSON.
- Produces: all validation errors on stderr and exit status 1, or `fleet validation: OK` and status 0.

- [ ] Write fixture tests for dead routes, wrong nonce names, missing agents, stale geometry, missing
  recovery contracts, and a valid tree.
- [ ] Run the test and verify it fails because the validator does not exist.
- [ ] Implement frontmatter, reference, forbidden-term, frontier-schema, geometry, state, and recovery checks.
- [ ] Run focused tests and verify they pass.

### Task 3: Shared evidence and recovery contract

**Files:**
- Modify: `.pi/skills/circuit-evidence/SKILL.md`
- Modify: `.pi/notes/07-pi-agent-playbook.md`

**Interfaces:**
- Consumes: measurements and agent dispositions.
- Produces: consistent discovery/promotion evidence levels and mandatory terminal recovery behavior.

- [ ] Add discovery evidence debt, adaptive metrology, scoped plateau terminology, and the exact recovery
  command to the shared skill.
- [ ] Add the dual-lane state machine, non-ceiling policy, and recovery-before-rollback rule to the playbook.
- [ ] Run the validator tests.

### Task 4: Control and certification roles

**Files:**
- Modify: `.pi/agents/orchestrator.md`
- Modify: `.pi/agents/implementer.md`
- Modify: `.pi/agents/lambda-metrology.md`
- Create: `.pi/agents/frontier-auditor.md`
- Create: `.pi/agents/nonce-grinder.md`

**Interfaces:**
- Consumes: frontier JSON, task cards, measurements, and user-approved budgets.
- Produces: governed experiment transitions and exact certification records.

- [ ] Encode portfolio allocation, escalation, scoped plateau review, and recovery enforcement.
- [ ] Separate provisional evaluation from ground-seed certification.
- [ ] Replace fixed-small-sample conclusions with adaptive λ tiers.
- [ ] Run the fleet validator and fix every control-plane mismatch.

### Task 5: Optimization ownership roles

**Files:**
- Modify: `.pi/agents/slope-quotient.md`
- Modify: `.pi/agents/tape-codec.md`
- Modify: `.pi/agents/bezout-apply.md`
- Modify: `.pi/agents/pair2-erase.md`
- Modify: `.pi/agents/square-coordinate.md`
- Create: `.pi/agents/shared-primitives.md`
- Create: `.pi/agents/stream-optimizer.md`
- Create: `.pi/agents/cross-cell-architect.md`

**Interfaces:**
- Consumes: pinned frontier and live source facts.
- Produces: bounded hypotheses with mechanism, falsifier, budget, search radius, and recovery bundle.

- [ ] Remove unconditional mutable geometry and metric claims.
- [ ] Add local-plateau escalation and reopening triggers to every specialist.
- [ ] Define missing ownership surfaces without granting additional source writers.
- [ ] Run the fleet validator and search for stale/dead production controls.

### Task 6: Campaign prompts and full verification

**Files:**
- Modify: `.pi/prompts/improve-performance.md`
- Modify: `.pi/prompts/optimize-cell.md`

**Interfaces:**
- Consumes: all fleet roles and validation scripts.
- Produces: repeatable discovery, provisional, grind, certification, and failure-recovery campaigns.

- [ ] Replace the dead consolidation route with live-route promotion.
- [ ] Add discovery quotas, reframe rounds, adaptive evidence gates, and recovery-before-rollback.
- [ ] Run all Python tests and the validator against the real `.pi` tree.
- [ ] Search the resulting tree for forbidden production instructions and inspect the final diff.

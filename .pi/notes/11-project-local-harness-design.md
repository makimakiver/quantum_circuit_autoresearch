# Project-local campaign harness design

**Date:** 2026-08-16
**Status:** approved for implementation by the continuation request
**Authority:** `CAMPAIGN_FINDINGS.md` and `.pi/frontier.json`

## Goal

Refactor the existing `.pi` subagent fleet into one project-local, evidence-gated harness without changing
the default circuit, the parked default-off `EXP-SQ-01` edit, or any trusted harness source. The pinned
frontier remains score `1,481,143,998`, compressed operation hash
`6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`, and canonical operation hash
`0a4a412c939b05f59fa4bc33c2410e5bc110b07c7faded5e6db5f6b5db3ed0b0` until an independently certified
promotion is recorded.

## Architecture

The harness has four layers:

1. **Role layer:** narrow Pi agents for frontier audit, exploitation/tuning, structural research,
   cross-cell reframing, lambda metrology, strip/census/mining, implementation, evaluation/certification,
   nonce grinding, and orchestration/review. Only `implementer` may change `src/point_add/**`. No role may
   change trusted source outside that tree.
2. **State layer:** JSON-schema-backed frontier, campaign, experiment-card, result, approval, and
   continuation records. Existing Markdown experiment cards remain immutable evidence inputs rather than
   executable agent instructions.
3. **Gate layer:** deterministic Python checks for default-artifact identity, legal lifecycle transitions,
   edit ownership, lambda verdicts, 9,024-shot evaluation, explicit `GRIND_APPROVED`, exact-stream nonce
   binding, certification, and promotion.
4. **Orchestration layer:** Pi `workflowScript`/`runs.all` prompts fan out independent read-heavy lanes and
   collect structured outputs. Source writers are serialized and isolated in managed worktrees.

## Lifecycle

Canonical statuses are `SCOPED`, `IMPLEMENTED`, `METROLOGY`, `SAFE`, `REFUTED`, `INCONCLUSIVE`, `PARKED`,
`REVERTED`, `EVALUATED`, `GRIND_APPROVED`, `GROUND_SEED_FOUND`, `CERTIFIED`, and `PROMOTED`. Transitions are
validated. `REFUTED` requires a predeclared falsifier; a timeout/provider limit is `PARKED` or
`INCONCLUSIVE`. `PROMOTED` requires a `CERTIFIED` result that strictly beats the current frontier and then
creates a new frontier revision; it never mutates the historical pinned record.

## Recovery and continuation

Recovery bundles contain repository identity, commands, diffs, text evidence, binary hashes, continuation
metadata, and log tails. They are printed between terminal delimiters and may also be saved outside scored
source. Provider limits and timeouts preserve a resumable continuation record. Detached jobs are observed
through declared log/done-marker paths; the harness never silently restarts them.

The initial continuation registry preserves:

- `EXP-P2-BZ-METRO` at `/tmp/exp_p2_bz_metro`, including arm hashes, cumulative logs, analyzer, and done
  markers. The campaign snapshot status remains in-progress/parked; live observation is reported separately.
- `EXP-SM-01` at `.pi/experiments/EXP-SM-01.md` and `.pi/experiments/EXP-SM-01-tooling/`. Miner construction
  may resume, but the 10-million-input pilot and full census remain blocked without explicit compute approval.

## Failure behavior

All commands fail closed with actionable errors. Unknown schema fields are rejected in authoritative state
and tolerated only in external lifecycle observations. Missing artifacts produce `INCONCLUSIVE`/`PARKED`,
never success. A default hash mismatch blocks every downstream gate. An ownership violation or trusted-source
change blocks collection and promotion.

## Verification

Implementation uses Python standard-library tests and static agent-contract validation. Lightweight checks
cover JSON parsing/schema behavior, legal transitions, ownership, lambda gates, continuation detection,
recovery output, agent discovery configuration, Python compilation, existing artifact hashes, and scoped
whitespace checks. No circuit build, long metrology/census run, or nonce grind is part of this refactor.

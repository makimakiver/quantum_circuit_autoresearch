# EXP-PR-05 — Fold-schedule micro-retune (FOLD_SCHED ±1 / FFG_G ±1)

| field | value |
|---|---|
| id | EXP-PR-05 |
| search_radius | `FOLD_SCHED` (schedule.rs, 514 entries) and `FFG_G` (schedule.rs) consumed by fused.rs fold/window primitives (TLM_FOLD 520 + TLM_FFG 607 invocations; fold phases 168,077 emitted tof; probes fold1/fold3/ffg1 from prior session). |
| mechanism | ±1 on individual schedule entries shifts the LSBS=53 fold-window split; carry-escape λ term (2.18 mm per 02-lambda.md model) lives here. |
| predicted T/Q/lambda | Measured: fold1 −8 executed, fold3 +20, ffg1 +18.5 — all within schedule-noise. Q: 0. λ: FOLD_SCHED carries −5 markers (drop steps) — touching them risks the 2.18-mm carry-escape term. |
| cheapest_discriminator | Done (prior session probe2.sh arms, ≤60s each). |
| predeclared_falsifier | Any |ΔT_exec| < 100 per single-entry retune → plateau confirmed (already met). |
| evidence_debt | None material. |
| budget | Closed. |
| unresolved_directions | Coordinated multi-entry retunes (combinatorial, needs a search harness) — poor ROI vs erase axis. |
| reopening_trigger | A pruning search over FOLD_SCHED vectors with scan-in-the-loop becomes cheap (scan ≈ 50s/400 rounds). |
| reproduction_commands | `cd /tmp/prims-work && env TLM_FOLD_SCHED_DELTA=1 /tmp/prims-tgt/release/build_circuit 2>&1 | grep TLM_TOF_TOTAL` (knob names per probe2.sh) |
| classification | LOCAL_PLATEAU (single-entry), closed. |

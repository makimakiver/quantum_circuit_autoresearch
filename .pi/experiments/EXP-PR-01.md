# EXP-PR-01 — Erase-cap window extension to early calls (MEASURED DEAD)

| field | value |
|---|---|
| id | EXP-PR-01 |
| search_radius | `TLM_COUT_ERASE_CAP`/`TLM_COUT_ERASE_CAP_CALLS` (gidney.rs:1322-1345) over the ~2,728 gated erase sites. **Load-bearing context discovered this session**: the certified baseline ALREADY ships `set_default_env("TLM_COUT_ERASE_CAP","22")` + `("TLM_COUT_ERASE_CAP_CALLS","24:9999")` (mod.rs:2254-2256, with comment "Set TLM_COUT_ERASE_CAP=0 to disable"). Therefore `TLM_COUT_ERASE_CAP=22` alone is byte-identical to baseline (knob-noop trap — set_default_env does not override an explicit equal value, and CALLS defaults to the shipped window). |
| mechanism | Prior-session arm `erase22_all` (CAP=22 + CALLS=0:9999) actually measures WIDENING the shipped window to cover erase calls 0-23 — the early-walk calls where Bezout operands are ≤ ~24 bits, so a top-22 truncation compares near-empty prefixes. |
| predicted T/Q/lambda (measured) | T: −593.5 executed pre-strip (ΔS ≈ −685k). Q: 0. λ: **5295.26** (vs baseline 17.62) at 400 rounds/25,600 lanes — phase_shots 14,999/25,600 (58.6%), phase_bad 400/400 rounds, classical UNCHANGED at 43 (pure phase channel). Catastrophic: grind probability e^-5295 ≈ 0. |
| cheapest_discriminator | The 60s-capped single scan (this session, ran inline at 59s): `env TLM_COUT_ERASE_CAP=22 TLM_COUT_ERASE_CAP_CALLS=0:9999 TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=400 TLM_DIRTY_SCAN_MAX=600 TLM_DIRTY_SCAN_SHOW=0 /tmp/prims-tgt/release/build_circuit 2>&1 \| grep DIRTY_SCAN` |
| predeclared_falsifier | Δλ > +20 at n=400 rounds → reject. Measured +5278 → reject with finality; also explains why the shipped window begins at 24 (the safe prefix was already mined and shipped). |
| evidence_debt | None for the rejection (DIRTY_SCAN is inner-loop per policy §1, but a +5278 λ is not a marginal call). |
| budget | Spent: 1 scan. Closed. |
| unresolved_directions | Whether a MIDDLE band (e.g. calls 24:K capped at 22, K:∞ capped at 19) recovers w=19's T at λ≈0 — see EXP-PR-03 (bisection in flight). |
| reopening_trigger | None realistic — early-call truncation is structurally unsound (operand width < cap). |
| reproduction_commands | See cheapest_discriminator; artifact `/tmp/prims-work/scan_*.log` (12-round) and this session's inline run; baseline replica arm `scanrun.sh:erase22all` (λ=17.62, ops identical) is the paired control. |
| classification | STRUCTURAL-λ — measured rejection, CLOSED. |

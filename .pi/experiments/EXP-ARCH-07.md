# EXP-ARCH-07 — Square f-term modular-wrap cone certificate at the first forward-divstep boundary

**Status:** SCOPED. This is a no-build dependency proof, not a circuit deletion or an implementation candidate.

- **experiment_id:** `EXP-ARCH-07`
- **lane / owner:** `REFRAMING` / `cross-cell-reframer`
- **mechanism_id:** `square-term-transport`
- **implementation_ready:** `false`
- **search radius:** one selected square `b_hi` NAF term from `square.rs::apply_f_times_value_tagged(value.len()==N)` / `apply_shifted_hi_term`, through `ec_add.rs::tlm_forward_multiply` and `gcd.rs::forward_gcd_jump` iteration `i=0`; includes the first `v[0]`-controlled shifts, `(t1, s2, subtracted, swp)`, Step0 raw/encoded symbol, post-step active `(u,v)`, and matching reverse-decode requirement. Excludes schedule tuning, codec width, local square add/sub, early-sum freeing, deep-strip mutation, and census.
- **mechanism:** a selected high-positioned f-term is still a mod-p update: carry or correction can alter low `v[0]`, then the first-divstep control tuple and Step0 dialog symbol. The necessary-condition proof compares exact term-on versus selected-term-deferred behavior before any hypothetical later restoration.
- **invariants / proof obligations:** for every modeled input, the two paths must agree on complete `i=0` `(t1,s2,subtracted,swp)`, Step0 raw and encoded symbol, and post-step `(u,v)`. The Step0 encoder/decoder must remain paired, including `s2` conditionality. A future implementation still needs a source-use/paired-restore certificate, default identities, Q/occupancy, phase/ancilla, lambda, evaluator, and candidate nonce gates.
- **cheapest discriminator:** deterministic exact-integer term-on/term-off miter after independently validating the baseline extraction map, followed only on a no-witness D1 outcome by one bounded exact bit-vector SAT/UNSAT query. SAT models require independent integer replay.
- **falsifier:** any independently replayed reachable mismatch in the complete `i=0` control tuple, Step0 symbol, or post-step state refutes direct no-carrier defer/delete for this named term only. Sampling no-witness, timeout, extraction ambiguity, or unreplayed SAT is inconclusive.
- **budget:** one temporary pure-model script and one solver query; maximum 30 minutes wall; no Rust build, source edit, lambda scan, evaluator, census, or grind.
- **predicted direction:** proof stage itself has `T/Q/score/lambda = 0`. No candidate direction is established; future transport is separately priced and re-gated.
- **unresolved direction:** whether a selected term has a modular-wrap-safe first-step cone and a later exact existing restore carrier; later source uses, replay, phase, allocation, strip ordinals, Q, lambda, and evaluation remain open.
- **reopening trigger:** a different transported term, extraction map, restoration boundary, proven existing dialog/replay carrier restored before every consumer, changed paired first-divstep control map, or exact source certificate showing a witness lies outside the candidate domain.
- **rollback unit:** if separately authorized after proof and source-use closure, one isolated implementer worktree commit behind default-off `TLM_SQ_STEP0_FTERM_TRANSPORT`, containing the complete paired square/walk/replay change; rollback is unset flag/discard the single commit.

## Reproduction

```bash
# D1: approved temporary/output-only model, no cargo
PYTHONDONTWRITEBYTECODE=1 python3 -B <run>/artifacts/exp_arch07_miter.py \
  --source-root /Users/makimakiver/ecdsafail-challenge --term b_hi --first-step-only \
  --edge-residues 0,1,p-2,p-1 --emit <run>/artifacts/d1.json

# D2 only if D1 has no witness; replay each SAT model independently
PYTHONDONTWRITEBYTECODE=1 python3 -B <run>/artifacts/exp_arch07_smt.py --emit <run>/artifacts/d2.json
PYTHONDONTWRITEBYTECODE=1 python3 -B <run>/artifacts/exp_arch07_miter.py --replay <run>/artifacts/d2.json --emit <run>/artifacts/replay.json
```

This card preserves the round-2 `EXP-ARCH-07` mechanism, scope, invariants, falsifier, proof obligations, budget, and reopening trigger from `.pi/campaign-runs/20260816t232122Z/reports/round2-square-walk-reframe.json`.

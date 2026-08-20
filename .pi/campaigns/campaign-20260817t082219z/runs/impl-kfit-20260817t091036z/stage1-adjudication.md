# EXP-KFIT-01 stage-1 adjudication — cout k-fit harmonization

Worktree `/Users/makimakiver/ecdsafail-wt/kfit` @ a2067dcf3c991bcecfd51a4bf07cd7d1cc56c3a5 (detached, clean; only
untracked scratch `.pi-tmp/` removed after evidence copy). Zero source edits; env-only measurement.

## Gate results

| gate | result |
|---|---|
| 1 ownership/scope | PASS — `git status` clean; no `src/**` touched; no bake (no win) |
| 2 default identity | PASS — clean-env build reproduced compressed `6519bd01…a097` AND canonical `0a4a412c…ed0b0` (verify-default: emitted 8,958,690, Q 1,154) |
| anti-noop (instrumentation) | PASS — trace build (`TRACE_TLM_COUT/TOF/CCX/PROFILE=1`) left ops.bin byte-identical at `6519bd01…`; strip fully live (12,202 removed / 4,045 downgraded / 0 stale) |
| 3 candidate anti-noop | NOT REACHED — no stage-2 candidate produced (stage-1 gate for (a) not met) |
| 4 semantic obligation | ADJUDICATED — see below: `effective = min(selected, headroom)` pinned at every call; no accessor or call-count change possible or needed |
| 5 score-sign gate | NOT REACHED — no candidate; projected ΔT=0, ΔQ=0, ΔS=0 |
| 6 Q≤1154 | PASS observationally — `TLM_PROFILE peak_qubits=1154` inside `tlm_apply_inverse_mod_sub_register` |

## Stage-1 measurement (520 cout calls; table in `stage1-cout-table.tsv`)

Cout family = `next_cout_k()` → `take_cout_fit()` → `controlled_hybrid_add_cout_refs` (gidney.rs:1982) consuming
`APPLY_COUT_K` widened `BLOCKS_2`; call range 0–259 = inverse apply (`tlm_apply_inverse_mod_sub_register`),
262–521 = forward apply (`tlm_apply_forward_mod_add_register`), 260/261 skipped-tail adds.

- Anti-parallel k-profiles: inverse pass traverses wide→narrow (base 138→~22), forward pass narrow→wide (24→138).
- Emitted ops added: fwd 1,222,720 vs inv 1,239,326 (+16,606 pre-strip, all op kinds); executed-T asymmetry of the
  two register phases: 148,392.5 (inv) − 147,799.0 (fwd) = **+593.5 executed Toffoli** (TLM_TOF rows).
- Headroom saturation at EVERY call (`headroom = TLM_TARGET_Q − entry_active = 1154 − active`):
  - 240/260 calls per pass: `selected > headroom` → hard-capped,
  - 20/260 calls per pass: `selected == headroom` exactly,
  - **0/260 calls with true slack (`selected < headroom`)** on either side (corrected filter; an earlier
    filter that treated `eff==sel` as slack was wrong because `sel==headroom` re-caps any raise).

## Type (a)/(b) adjudication: TYPE (b) — intrinsic context, overrides cannot touch

`fit_schedule_value` (mod.rs:170) lets `TLM_COUT_K_CALL_OVERRIDES` replace only `selected`. The consumer
(gidney.rs:1985) computes `effective = selected.min(headroom)`; since effective is already at the headroom
ceiling everywhere it matters, any override is either (i) `k' ≥ headroom` → same effective → byte-identical
noop, or (ii) `k' < headroom` → strictly narrower than the mined optimum (wider k is cheaper at n=256:
3,949 ops at k_eff≈104 vs 6,938 at k_eff≈26 under similar context; the baked schedule is the mined optimum,
and EXP-BZ-03 empirically showed narrowing probes regress executed T +213 net with 1,772 orphaned strip keys
on a smaller perturbation). The divergence between mirrored passes is `entry_active`-driven (each pass runs in
a different live-qubit window of its walk), not schedule-chosen k. Corroboration: the global Q binding peak
(1,154) occurs inside `tlm_apply_inverse_mod_sub_register` itself — raising k on the expensive side would
breach Q=1154 even if the cap were lifted.

## Family attribution of the ledger's ~−7,838 potential

The EXP-REV-01 ledger asymmetry (69,549 vs 76,651 emitted / 73,468 executed) is in the WALK-BODY adds
(`controlled_add_active`, phases `tlm_*_gcd_*_body`), which consume `GCD_SUB_K`/`GCD_BRANCH`
(widened `BLOCKS_4`) — a different family with no per-call override (only `TLM_GCD_K_*` i-window adjusters that
hit all four passes symmetrically; `GCD_BRANCH` is a source schedule table). Its cap is the identical
`k.min(headroom)` at gidney.rs:1967. TLM_TOF walk-body rows measured here: mf 69,549.0 / mr 148,357.5 /
if 73,468.0 / ir 144,303.0 executed — consistent with the ledger.

## Verdict and reopening

- Predeclared falsifier first clause fired: **no divergent k that overrides can change at any mirrored pair** →
  type (b) intrinsic → scientific verdict **UNSUPPORTED** for this card's mechanism under this geometry.
  Not a global ceiling and not REFUTED-as-mechanism for schedule-table work.
- Stage 2/3 not entered (their entry condition was type (a)); no candidate stream, no strip re-key, no lambda
  exposure, no evaluation.
- Rollback: none needed (no source edit, no persistent env change; all probes were single-command env vars).
- Reopening triggers: (1) a certified stream-specific strip re-mine that makes schedule-table edits cheap — then
  harmonize `GCD_SUB_K` per-block fits as a NEW card (source edit in `schedule.rs` + paired walk/replay proof);
  (2) any change that frees live qubits in the walk/apply windows (raising headroom where the narrow fits sit);
  (3) a Q-window redesign moving the 1,154 binding peak out of the apply-register phases.

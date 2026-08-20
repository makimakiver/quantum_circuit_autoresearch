# EXP-REV-01 stage-1 attribution ledger — reverse replay premium

Basis: TRACE_TLM_REVERSE_STAGES=1 TRACE_TLM_CCX=1 TRACE_TLM_TOF=1 build at worktree commit
`eb63629` (base a2067dcf). ops.bin remained `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`
in BOTH the clean-env build and the trace build (non-perturbation proof; instrumentation emits no ops).
Pre-postpass executed model: emitted 1,352,434 / expected 1,299,076.0 (discount 3.95%); CCX total 1,347,642.
Scorer post-strip T=1,283,487.051 — the model is consistent with the scored basis.

## Matched-boundary per-stage table (emitted CCX / expected executed Toffoli)

Stage boundaries are symmetric: shift | comparator | cswap ladder | controlled_add_active | dialog codec/decode.
Forward comparator = decision compute (`rt_*f_cmp_compute`); reverse = `swap_decision_uncompute_vented`
(`rt_*r_cmp_uncompute`). Walks: mf = multiply forward (apply-free), mr = multiply reverse (replay+apply_fwd),
if = inverse forward (apply_inv), ir = inverse reverse (apply-free).

| stage | mf | mr | if | ir |
|---|---|---|---|---|
| shift / shift tail | 34,899 / 34,899 | 34,899 / 34,899 | 34,899 / 34,899 | 34,899 / 34,899 |
| comparator | 10,160 / 10,160 | 10,171 / 5,085.5 | 10,431 / 10,431 | 9,900 / 4,950.0 |
| cswap ladder | 34,905 / 34,905 | 34,905 / 34,905 | 34,905 / 34,905 | 34,905 / 34,905 |
| controlled_add_active | 69,549 / 69,549 | 76,651 / 73,468 | 76,651 / 73,468 | 69,549 / 69,549 |
| codec / decode | 1,379 / 1,379 | 1,466 / 1,466 | 1,379 / 1,379 | 1,466 / 1,466 |
| **walk total (executed)** | **150,892.0** | **149,823.5** | **155,082.0** | **145,769.0** |

Apply phases (`*_gcd_*_apply`) and `tlm_gcd_step_end` emit zero CCX/CCZ in all four walks (no TLM_TOF rows).

## Premium under matched boundaries

- multiply pair (mr − mf): **−1,068.5 executed** (reverse cheaper)
- inverse pair (ir − if): **−9,313.0 executed** (reverse cheaper)
- combined: **−10,381.5 executed** — there is NO positive reverse premium to recover.

Mutable buckets: add premium = (73,468−69,549) + (69,549−73,468) = **0 executed net** (a ±3,919 swap between
walk pairs from per-pass k-fit/GcdBit0Mode asymmetry, not a reverse excess); cswap premium = **0**;
comparator emitted: reverse is 520 CCX SMALLER (20,071 vs 20,591) and 10,555.5 executed cheaper
(the condition-stack 50% discount is already banked).

## Reconciliation of the claimed +59,242 (EXP-LTC-02 §3)

The prior ledger's buckets were boundary-asymmetric: "rev body" INCLUDED the reverse shift tail (34,899/traversal)
while "fwd body+compare" EXCLUDED the forward shift (34,899/traversal, counted under `*_gcd_forward_shift`), and
it crossed apply/no-apply walks. Decomposition of the apparent premium:

  2 × 34,899 (shift-tail boundary artifact) − 10,555.5 (comparator uncompute condition-stack discount) + 174
  (decode vs codec) = 59,416.5 ≈ 59,242 claimed (residual 174.5 = add-pairing/rounding, 0.29%).

Every prior number reconciles exactly under the new split: 69,549 (their fwd body) = rt_mf_add;
45,065 / 45,336 (their fwd compare) = cmp_compute + cswap; 144,303 (their rev body apply-free) = ir stages;
148,357.5 (their rev body with apply) = mr stages.

## Stage-2 trigger and falsifier adjudication

- Trigger "mutable add/cswap bucket holds ≥5,000 emitted CCX of premium": **NOT MET** (add 0 net, cswap 0,
  comparator −520 emitted). Stage 2 skipped per the card's own gate. No functional edit; no stream change.
- Predeclared falsifier (≥80% comparator-uncompute floor): fired in an amended form — the apparent premium is
  ~118% boundary artifact offset by the already-banked comparator discount; recoverable reverse-side share ≤ 0.
- Scientific verdict for the refit hypothesis: **UNSUPPORTED** (premise dissolves under matched boundaries).
  This is a property of THIS card's mechanism under THIS geometry — not a global ceiling on reverse-side work.

## New unresolved direction (recorded, not scope-crept)

Per-pass add k-fit asymmetry: mf/ir adds fit the cheap layout (69,549 executed) while mr/if adds fit the
expensive one (76,651 emitted / 73,468 executed) under the same modes in mirrored pairs. If the cheap layout
transfers to the two expensive passes, potential ≈ −7,838 executed (−2×3,919); requires a separate
exploitation card with paired k-fit proof (GCD_SUB_K / widen_sched_blocks anchoring) — reopened as a new
card candidate, not under EXP-REV-01.

## Score-sign projection (gate 6)

No functional change shipped: ΔT = 0, ΔQ = 0, ΔS = 0 vs frontier S=1,481,143,998. Stage-2 was not entered,
so no candidate stream, strip re-key, or lambda exposure exists. (For scale: had the claimed 59,242 been
recoverable, ΔS ≈ −68.4M at Q=1,154; the matched-boundary measurement shows the true recoverable share ≤ 0.)

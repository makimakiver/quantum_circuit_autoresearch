# Fleet round 1 — findings (2026-08-16)

Five-specialist scouting round + probe adjudication against frontier
`{T=1,284,916, Q=1,154, S=1,482,793,064, λ≈23.5, seed clean 9024/9024}` (results.tsv @ 892b03b).
**Zero cards accepted; frontier unchanged.** The durable value is the map below.

## 1. `rounds/dialog/` is DEAD CODE (build-verified)

`build()` (mod.rs:2007) → `trailmix_ludicrous::build_trailmix_ludicrous_ops()` (mod.rs:2302).
`build_builder()` (mod.rs:1469) and `configure_ecdsafail_submission_route()` (mod.rs:1201) have zero
callers; every `DIALOG_GCD_*` / `DIALOG_FUSE_*` consumer lives only under `src/point_add/rounds/dialog/`.
Proof: built with `DIALOG_GCD_APPLY_CLEAN_COMPARE_BITS=18`, `DIALOG_GCD_K5_RELEASE_DECODED_BLOCK_BITS=1`,
`DIALOG_GCD_FOLD_CARRY_TRUNC_W=17` all set → `ops.bin` md5 byte-identical (`b2855b2f0e6a…`). Playbook §3,
AGENTS.md, and the `.pi/agents/*` specs were corrected to the live `trailmix_ludicrous` map this round.
Four of five specialist first-pass reports were voided by this staleness — the anti-noop gate (§1.4)
caught all of them.

## 2. Any circuit change loses the ground seed (the λ-gate, observed directly)

Every probe that changed `ops.bin` without breaking the circuit landed at **10–30 classical / ~10 phase**
failures on the frontier seed — the intrinsic band, i.e. the current 0/0/0 seed is ground FOR the exact
shipped byte stream and does not transfer across any modification:

| probe | result | reading |
|---|---|---|
| `TLM_APPLY_FWD_CSWAP_SKIP_LAST=3` | 15 cl / 10 ph | seed lost, circuit plausibly intact |
| `TLM_APPLY_INV_CSWAP_SKIP_LAST=2` | 15 cl / 11 ph | seed lost |
| `TLM_APPLY_FWD_CSWAP_SKIP_LAST=0` (skip OFF) | 13 cl / 9 ph | seed lost — even *restoring* work loses it |
| `TLM_SQUARE_PEAK_CAP=1150 VENT_MARGIN=0` | 24 cl / 16 ph, Q still 1154 | seed lost, zero Q gain |

Consequence: the §6 accept gate ("4 checks green") is unreachable for ANY functional change without
re-grinding a nonce, at expected cost `e^λ ≈ e^23.5 ≈ 1.6e10` trials. This re-confirms, from live
experiments, that **λ is the binding constraint, not Toffoli count**. A T or Q win is only cashable
bundled with a grind budget or a λ reduction.

## 3. Structurally load-bearing skips (turning them off BREAKS the circuit, not just the seed)

These produced thousands of classical mismatches + saturated 141/141 phase — repointed/broken, not
intrinsic-band:

- `TLM_APPLY_ADD_SKIP_LASTK=0` → 8,436 cl / 141 ph. The last-step add-skip is structural: downstream
  is built assuming it fires. NOT a removable approximation.
- `TLM_APPLY_ADD_SKIP_FWD=2` → 2,031 cl / 141 ph. Dead direction.
- `TLM_APPLY_FWD_S2_ZERO_LAST=2` → 5,077 cl / 141 ph. Dead direction.
- `TLM_SQUARE_ADDSUB_SKIP_C=1` → 6,263 cl / 141 ph. Off-by-default for a reason; dead.

So the shipped skip-window configuration (`FWD_CSWAP=2, INV_CSWAP=1, S2_ZERO=1/1, ADD_SKIP_LASTK=1`)
sits at a local optimum bounded on BOTH sides by hard breakage or seed loss.

## 4. λ-metrology resolution limit

`TLM_DIRTY_SCAN_ROUNDS=12` yields 768 lanes ≈ **2 fault events** at λ≈23.5 → resolution ±~12 λ per
event. It read λ=23.50 identically for baseline and for probes the eval showed to be different. For 1σ
discrimination at ~10% you need ~100 events ≈ **600 rounds** (hours). Cheap dirty scans are triage-only;
never quote them as a λ comparison.

## 5. Cell status after round 1

| cell | status |
|---|---|
| B1 walk / ITERS | `ITERS=261` live (schedule.rs:4, const, no env knob); tail λ cashed (261→~0.48mm); 259/260 proven correctness-destroying (mod-3); 264 priced bad (~+11M S for ~0.44 λ) |
| B1 certificates | all ~20 `TLM_*_SKIP_STRUCTURAL_DEAD_*`/`_EXACT_*` flags are NO-OPS at 261 (mined at BAKED_ITERS=258 geometry; `drops_off_family` disables them) — superseded by `apply_deep_strip_identity` re-mined at 261; no lost value |
| B2 tape codec | EXHAUSTED: live tape = 609 q at ITERS=261 (`2+86·7+5`); `TLM_TAIL4_TOP32` IS the refuted 609→605/+8,038-CCX experiment; Pair encoder on the proven 6-CCX floor; only open lever = exact-eight-CCX SAT synthesis (~0.5% of T, solver-hard) |
| B3 apply | dead-knob hunt on live `TLM_*` block: clean (one benign duplicate, `LUD_EXTRA_FOLD_*`, live value already the cheaper one). Skip windows at bilateral local optimum (§3) |
| C1 square | from-zero sites: 2 of 3 masks exploited, 3rd proven 0-value; fast adders at the `MC ≥ n−1` floor; both probe cards rejected (§2, §3) |
| D pair-2 | direction-flavored skips already asymmetrically tuned (FWD=2 vs INV=1); all deeper/wider probes rejected |
| SCHED_J2 widening (slope card 2) | UNTESTED — requires paired GAP_J2 co-edit (8.36→4,646 mismatch coupling), high risk, needs the expensive λ metrology of §4 to adjudicate. The only open STRUCTURAL-λ thread from this round |

## 6. What a round 2 would need

1. A λ measurement budget: ~600-round dirty scans (or n≥12 full 9024-shot evals on paired nonces) per
   arm — hours each. Without it, no λ-class card can pass a 1σ gate.
2. A grind budget decision from the user: any accepted functional change requires re-grinding
   `DIALOG_TAIL_NONCE` (2^48 space) to find a new clean seed. Priced at `e^-23.5` per trial.
3. The one open thread: SCHED_J2+GAP_J2 co-widening at the early "tight magnitude bound" indices
   (05-qubit-reduction Step 5), aiming at the 2.80mm SCHED_J2 λ component — implementer edit, high
   risk, only worth attempting with 1+2 in place.

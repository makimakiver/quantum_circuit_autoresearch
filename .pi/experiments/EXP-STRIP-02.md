# EXP-STRIP-02 — enable the gated-off final CCX self-inverse cancel (2 CCX, exact)

Pinned to commit a2067dcf, ops.bin sha256 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097.

1. **experiment_id**: EXP-STRIP-02
2. **search_radius**: one env flag: `TLM_CCX_FINAL_CANCEL=1` (optionally `TLM_CCX_FINAL_STRADDLE=1`
   — measured to add nothing: straddle_extra=0, byte-identical result). No source edit required;
   the pass `constprop::ccx_final_cancel` (constprop.rs:1126) is fully implemented, runs after the
   first fanout fixpoint + M-60 + CCZ-cancel and before the deep strip, and is gated OFF by
   default (`TLM_CCX_FINAL_CANCEL` is set nowhere in src/ — verified by grep over the whole tree).
3. **mechanism**: `ccx_final_cancel` runs the shipped sound CCX self-inverse matcher
   (`find_inverse_pairs`, strict clean-support by default) over the final post-fanout stream;
   every returned pair is a proven identity (same controls/target, clean or net-restored between)
   → removal is bit-exact. Live probe on a2067dcf: `pairs=1, removed_ccx=2`. Each removed CCX is
   admission-charged (~0.96 avg admission) → ≈2 executed Toffoli. Downstream provenance verified:
   the deep-strip still fires 12,202/12,202 + 4,045/4,045 with **0 stale keys** (the cancelled
   pair's tuple is not keyed), post-strip fanout still 10 passes.
4. **predicted direction**: T: −2 executed Toffoli (ΔS ≈ −2,308 at Q=1154). Q: 0. lambda: 0
   (exact identity transform; no statistical content). Note: the ops.bin hash changes
   (6519bd01… → e6ae6b52…) which reseeds the Fiat-Shamir test inputs ⇒ SUB4_TAIL_NONCE=1337610097
   is invalidated and must be re-ground — the regrind, not the transform, is the cost. Bundle this
   flag with ANY other change that already forces a regrind (e.g. EXP-STRIP-01).
5. **cheapest_discriminator**: `TLM_CCX_FINAL_CANCEL=1 cargo run --release --bin build_circuit` —
   expect log `  [FINAL CCX cancel] straddle=false pairs=1 straddle_extra=0 removed_ccx=2 -> 8970900 ops`
   and sha256 e6ae6b52b7a655369df6f14a44b9c9e5a8d6c7a3a13b4c0a77930bb995524d57 ≠ baseline
   (knob-liveness proven this session).
6. **predeclared_falsifier**: pairs=0 on the then-current stream (the single pair is consumed by
   an upstream change), or enabling it flips the deep-strip log to `N stale keys skipped` with
   N>0 (ordinal interaction erasing more strip savings than the 2 CCX gained).
7. **evidence_debt**: the admission weight of the 2 removed gates is unmeasured (bounded ≤2);
   repaid implicitly by the promotion eval. Standalone promotion is NOT worth a regrind —
   evidence debt is repaid only when bundled with a regrind-forcing change.
8. **trial_budget / time_budget**: 1 build (15 s) + the shared regrind; effectively zero marginal.
9. **unresolved_directions**: does not decide whether a wire-equality generalisation of the
   matcher (memory: 388-pair ceiling on an older stream, needs analysis constprop already
   fixpoints on) is worth building — known-closed-adjacent, do not rerun same-tuple classes
   blindly on new streams without re-probing (this card IS the re-probe for the strict class).
10. **reopening_trigger**: any op-stream change upstream of the cancel — the probe is free
    (one build), re-run it every time the stream hash moves.
11. **reproduction_commands**:
    - `TLM_CCX_FINAL_CANCEL=1 cargo run --release --bin build_circuit 2>&1 | grep -E "FINAL CCX|deep-strip"` → pairs=1, removed_ccx=2, 0 stale; sha e6ae6b52….
    - `TLM_CCX_FINAL_CANCEL=1 TLM_CCX_FINAL_STRADDLE=1 …` → identical sha (straddle adds nothing).
    - Baseline: `cargo run --release --bin build_circuit` → 6519bd01….
12. **classification**: STRIP (exact, zero-lambda).

# EXP-SLOPE-03 — CMP_K comparator chunk-width retune (walk comparators)

Pinned: a2067dcf + ops.bin 6519bd01…; CMP_K live length 1028 (= 4 passes × 257, BLOCKS_4_SHORT).

1. experiment_id: EXP-SLOPE-03
2. search_radius: `CMP_K` (schedule.rs) consumed at comparator.rs:791 and :907 as `ck = next_cmp_k()+1` — the chunk width inside `compare_geq_chunked_middle_direct` for the walk's two `controlled_swap_decision_*_truncated` comparators. Positional addressing (traps §2): CMP_K[pass*257 + (i−1 even | 257−i odd)]. Sweep ck per call within ±3 of the baked value, holding the cmp_window (GAP) schedule fixed.
3. mechanism: the chunked comparator splits the k-bit window into chunks of width ck; gate count trades chunk width vs chunk count (carry propagation between chunks). The baked CMP_K was fitted pre-widen (BAKED_ITERS−1 anchors) and predates the shipped GAP δ2 narrowing (the window it chunks over changed underneath it). Slack is plausible but unmeasured.
4. predicted direction: T −0…−600 toffoli (unknown sign; the analogous COUT_K retune era found "zero slack" and GCD_SUB_K fully clamped — CMP_K may be equally tight). Q 0. λ 0 (chunking is exact — a pure re-partition of the same comparison; no window change). Residual risk: ordinal shifts tripwire strip keys (tax ≈ keys changed).
5. cheapest_discriminator: implementer adds a `TLM_CMP_K_DELTA` (or CALL_OVERRIDES) knob beside next_cmp_k() (mod.rs:265) — 5-line edit — then δ∈{−1,+1} build probes with op census. NO eval needed to rank.
6. predeclared_falsifier: ±1 sweeps both give ≥0 toffoli delta at ≥50 stale keys → the vector is exhausted; close without further search.
7. evidence_debt: none structural; λ-neutral claim rests on chunking exactness (verifiable by reading compare_geq_chunked_middle_direct — audit pending in implementation).
8. trial_budget / time_budget: 1 edit + 6 probes ≈ 30 min compute. Hard cap: half a day.
9. unresolved_directions: does not touch GAP/window widths (EXP-SLOPE-01), does not touch the apply-side comparators beyond the four walk passes.
10. reopening_trigger: EXP-SLOPE-02 landing (re-mine) makes any found slack fully bankable; re-run the sweep then.
11. reproduction_commands:
    ```
    # after implementer adds the knob:
    TLM_CMP_K_DELTA=1 CARGO_TARGET_DIR=/tmp/slope-scout-target cargo run --release --bin build_circuit 2>&1 | grep -E 'deep-strip|emitted ops'
    python3 /tmp/ops_census.py ops.bin
    ```
12. classification: TUNING.

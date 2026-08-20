# EXP-BZ-03 — Widened-tail schedule re-fit: APPLY_COUT_K / FOLD_SCHED / FFG_G fill entries (TUNING)

Pinned: commit a2067dcf, ops.bin 6519bd01…; schedule.rs:9 BAKED_ITERS=258 vs ITERS=261; widen_sched_blocks (tlm mod.rs:327-362).

1. **experiment_id**: EXP-BZ-03
2. **search_radius**: re-fit only the 3 fill entries per block created by `widen_sched_blocks` for the divsteps that did
   not exist in the baked search: `APPLY_COUT_K` calls {258,259} (fwd pass tail; call 260 is consumed but unused — the
   i=260 add is skipped) and {262,263} (rev pass, divsteps 259,258); `FOLD_SCHED` calls 257-259 and 260-262 (fill nv=53);
   `FFG_G` calls at the same tail indices (fill 53, capped 47 by TLM_FFG_MAX_G). Sweep via existing env hooks
   `TLM_COUT_K_CALL_OVERRIDES`, `TLM_FOLD_DELTA`/reserve tables, `TLM_FFG_DELTA` — no source edit needed to probe.
3. **mechanism**: the fill value for forward blocks is the block's last fitted entry (APPLY_COUT_K k=138 — the widest
   lookahead) and for reverse blocks the first. But divsteps 258-260 run at terminal walk width (SCHED_J2=12..9) where
   the interior fitted analogue (the mid-region dip, k=22-31 at APPLY_COUT_K[206..260]) suggests much smaller k is
   optimal. Smaller k selects a cheaper hybrid layout (`controlled_hybrid_add_cout_refs_impl` dispatch, gidney.rs:1982+).
   Similarly nv=53 (fully-clean fold) at the widened calls may exceed what Q-pressure actually requires.
4. **predicted direction**: T −300..−1,500 executed (4 adder calls x ~100-250 CCX layout delta + fold/ffg trims);
   Q unchanged or −0..1 (fold nv caps interact with TLM_TARGET_Q headroom); lambda unchanged (layout choice is exact —
   no probabilistic assumption touched).
5. **cheapest_discriminator**: `CARGO_TARGET_DIR=/tmp/<t> TLM_COUT_K_CALL_OVERRIDES='258:30,259:30,262:30,263:30' TRACE_TLM_TOF=1 cargo run --release --bin build_circuit`
   — read `tlm_apply_forward_mod_add_register` expected delta; require ops.bin sha != 6519bd01… (knob-liveness).
6. **predeclared_falsifier**: sweeping k over {22,25,31,45,60,90,138} at those four calls shows the register-phase
   expected-executed is flat within ±20 CCX (layout already optimal at fill) — kills the card before any source change.
7. **evidence_debt**: none structural; promotion gate is just four-checks-green + S strictly lower on a full run; the
   probe ladder (count-only → TRACE_TLM_TOF) already covers attribution.
8. **trial_budget / time_budget**: 30 min of env-sweep probes; implementer time only if a winning override set is found
   (then bake into schedule.rs via the existing fit tables).
9. **unresolved_directions**: does not re-mine the interior 516 fitted entries (that was the original search's job and
   is closed unless the objective function changes); does not touch CMP_K fills (walk-side, other cell).
10. **reopening_trigger**: any ITERS bump (261→262+) re-widens blocks and re-creates un-fitted fill entries — rerun the
    sweep; also reopening if the adder layout dispatch gains a new branch (shared-primitives change).
11. **reproduction_commands**: baseline as EXP-BZ-01; probe:
    `CARGO_TARGET_DIR=/tmp/bezout-scout-target TLM_COUT_K_CALL_OVERRIDES='258:30,259:30,262:30,263:30' TRACE_TLM_TOF=1 cargo run --release --bin build_circuit 2>&1 | grep -E 'TLM_TOF (phase=tlm_apply.*register|TOTAL)'`
12. **classification**: TUNING.

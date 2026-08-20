# Progress log — live entrypoint / prior-implementation audit

## Scope

This records the work completed in response to the request to identify the exact substantive improvements relative to the prior point-add implementation and write a benchmark note. No circuit source was modified.

## Completed work

1. Read the project evidence policy and CLI note workflow.
2. Verified the benchmark constructor call chain from source:

   ```text
   src/bin/build_circuit.rs:92
     -> point_add::build()
     -> src/point_add/mod.rs:2302
     -> trailmix_ludicrous::build_trailmix_ludicrous_ops()
     -> trailmix_ludicrous/ec_add.rs:275
     -> jump-2 forward/reverse GCD replay
   ```

3. Verified that the proposed legacy route is not called by the normal benchmark build:

   ```text
   src/point_add/mod.rs:1469   build_builder() definition
   src/point_add/mod.rs:1519   only source call to emit_dialog_gcd_raw_pa(...)
   src/point_add/rounds/dialog/mod.rs:2783  legacy emitter definition
   ```

   `build_circuit` calls `build()`, not `build_builder()`. `rounds/dialog` is compiled/re-exported legacy code. Therefore `DIALOG_GCD_*` experiments and edits isolated to that route are no-ops for normal `ops.bin` production.

4. Inspected Git history. The earliest local-history occurrence of the live TrailMix entrypoint is commit:

   ```text
   69d191fb34ed77cb4f210f59eb8058f312189c4f
   Submission af5abb17-1bf5-4ed2-8bfb-c653c043eb1f
   parent: 57eb64b000e728aa47e9ef9edca871e386f2f372
   ```

   That transition introduced the `trailmix_ludicrous` engine files and rewired the point-add implementation architecture.

5. Identified the three source-level implementation differences in the live route:

   - **Transcript encoding/replay:** the live GCD uses a direct `DialogCodec` tape with `Pair`, `Triple`, `Raw`, `Step0`, and `Tail4Top32` regions; it compresses during the forward walk and reverses/decompresses during replay. This replaces the older specialized sidecar/runway/block-layout architecture.
   - **Scheduled width lifetime:** `SCHED_J2` and `GAP_J2` are applied per divstep. Forward frees high `u`/`v` wires at the scheduled width; reverse allocates back to the same width before decoding/replay.
   - **Paired arithmetic and ancilla management:** the live forward/reverse Bezout update uses schedule-fitted controlled arithmetic, explicit known-bit parking/loaning, and paired fused fold operations. The `s2` condition remains required in both directions.

   The post-emission fanout/cancellation/deep-strip/tail-nonce stack in `src/point_add/mod.rs` is separately documented as an operation-stream optimization layer, not as a replacement of the core GCD algorithm.

6. Clarified the algorithmic terminology: both old dialog and live TrailMix are in the jump-2 binary extended-Euclid / Stein-Kaliski family. The switch is an implementation-architecture change, not a transition to Bernstein-Yang.

7. Published the full public benchmark-scoped standalone note:

   ```text
   ID:    05b51e5f-6797-409c-a97a-d26a1568bd82
   Title: Live entrypoint audit: dialog route is legacy, TrailMix is live
   Size:  12.7 KiB
   ```

   Publication command:

   ```bash
   ecdsafail notes add \
     --title 'Live entrypoint audit: dialog route is legacy, TrailMix is live' \
     --note-file /tmp/ecdsafail-live-entrypoint-audit.md
   ```

8. Stored the full note locally at:

   ```text
   .pi/notes/live-entrypoint-audit.md
   ```

## Evidence boundary

No `cargo run --release --bin build_circuit`, trusted evaluator run, `ops.bin` comparison, or `TLM_DIRTY_SCAN` was run for this audit. Accordingly this work establishes source reachability and architecture only; it does not prove a numeric TrailMix-versus-dialog T, Q, score, or lambda delta.

Any future performance claim must compare explicit route configurations, preserve both `ops.bin` artifacts/hashes, run the evaluator, and use paired dirty scans with at least 12 rounds for lambda.

## Local file inventory

```text
.pi/notes/live-entrypoint-audit.md           full published-note text
.pi/notes/progress-live-entrypoint-audit.md  this progress log
```

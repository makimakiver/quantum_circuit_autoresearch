# EXP-SM-01 tooling — scout census engine (PRESERVED from volatile /tmp)

Provenance: rebuilt by the stream-optimizer session of 2026-08-16 at `/tmp/streamopt/census`
(that path is VOLATILE — memory/05 §"Step 6" documents the v5 miner being lost exactly this way).
This copy is the durable record. `stream_census_main.rs` is the complete engine; `Cargo.toml`
uses the project-local relative dependency (`quantum_ecc = { path = "../../.." }`). Adjust that path only
after copying the manifest into an external scratch directory.

## What it is / is not
- IS: an instrumented mirror of `sim.rs::apply_iter` over a finished `ops.bin`
  (same op order, same per-op XOF consumption for Hmr/R). Per CCX/CCZ op index it
  accumulates: admission mask, fire mask, implication-violation masks
  (`viol_c1_implies_c2 = admit & q1 & !q2` etc.), admit population. Prints dead /
  downgrade candidate pools with admission histograms and an executed-T estimate.
- IS NOT: the v5 greedy miner (risk model, budget, key-table emission). That layer
  is EXP-SM-01's deliverable. Missing pieces: per-key EVENT COUNTS (fired_pop, not
  just the ever-fired mask), tuple/ordinal/occupancy keying identical to
  `apply_deep_strip_identity` (mod.rs:1705-1727), the greedy (cap 6.0 / floor 0.40 /
  budget B / order=rand2), and a `deep_strip_keys.rs`-format emitter.

## Rebuild recipe (scratch, never clobbers the repo artifact)
    mkdir -p /tmp/streamopt/census && cd /tmp/streamopt/census
    cp <repo>/.pi/experiments/EXP-SM-01-tooling/stream_census_main.rs .
    cp <repo>/.pi/experiments/EXP-SM-01-tooling/Cargo.toml .
    # Change quantum_ecc.path in the copied manifest to the absolute repository path.
    cargo build --release            # ~25 s
    ./target/release/stream_census <ops.bin> <threads> <batches/thread> <pool> [campaign-seed]
    # n_inputs = threads x batches x 64 lanes

## Validation (this session, 2026-08-16, live hardware)
- Baseline build reproduced 3x: sha 6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097 (13.6-19.2 s from /tmp/sqscout, CARGO_TARGET_DIR=/tmp/sqscout-target).
- Census n=65,536 (8x128x4096): avg executed T = 1,283,477.7 vs certified
  1,283,487.05 (0.0007% off) — mirror fidelity re-confirmed.
- Throughput measured: 65,536 inputs / 10.07 s wall (8 threads) = 6.5e3 inp/s
  (this box, now). Prior-session point: 1e6 / 442 s = 2.3e3 inp/s. Price long runs
  in the band 2.3e3-6.5e3 inp/s => n=3.2e8 is 14-39 h wall; n=1e8 is 4.3-12 h.
- SQ-01 variant stream (TLM_SQUARE_EARLY_SUM_UNBUILD=1, uncommitted square.rs edit):
  sha 486fa8945846a0fafbb9215d07a03f2eea1f9217f3705254c2ded42e61147dc6,
  6,799 stale keys, +5,844 emitted Toffoli, executed T 1,288,962.5 @ n=65,536
  (unpaired) = +5,485 vs baseline arm. Copy kept at /tmp/streamopt/ops.sq01_variant.bin (volatile).
- Strip-off substrate (SUB4_APPLY_STRIP=0): sha ed0d9cd306cc2d13683f8b45225c07005effc365f3f84b0911b2cabe52e6f555, 8,970,902 ops (reproduced; matches EXP-STRIP-01).

## Known engine caveats for the miner builder
- Point-pair sampling uses unbiased 64-bit indices across the full pool. The explicit campaign seed defaults
  to `20260803` (master XOF domain `stream-census-v2`), so renaming `ops.bin` no longer changes the inputs.
  Record the exact artifact hash separately; occupancy evidence never transfers across streams.
- Hmr/R consume the thread XOF in op order => census verdicts are stream-order-coupled:
  keys never transfer across op-stream changes (this is WHY the occupancy tripwire
  discards instead of re-addressing). A re-mine is always a fresh census.

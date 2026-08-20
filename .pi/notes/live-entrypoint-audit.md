# Live entrypoint audit: the supplied dialog comparison is a stale route, not the current circuit

**Model / harness:** GPT-5.6 Terra using pi coding-agent; read-only source and Git-history investigation.  
**Checkout examined:** `f735fd71200feced7defb9f9628130d7af153a40` (`Validate submission 92809b4b-f8a4-462b-a076-66e084b3d4a0`, 2026-08-16).  
**Classification:** architecture/provenance note only. No experiment flag was changed, no source was edited, and no build/evaluator or dirty-scan measurement was performed.

## Correction to the proposed entrypoint

The statement that the current entrypoint is

```text
build_builder() -> emit_dialog_gcd_raw_pa()
```

is false for the normal benchmark binary in this checkout. That pair is retained **compiled legacy code**, but it is unreachable from the harness circuit constructor.

The verified production path is:

```text
src/bin/build_circuit.rs:92
  point_add::build()
    src/point_add/mod.rs:2302
      trailmix_ludicrous::build_trailmix_ludicrous_ops()
        src/point_add/trailmix_ludicrous/mod.rs:453-476
          ec_add::ec_add(...)
            src/point_add/trailmix_ludicrous/ec_add.rs:275-310
              mod_mul_inverse_in_place(..., Direction::Inverse)
              mod_mul_inverse_in_place(..., Direction::Forward)
                forward_gcd_jump() / reverse_gcd_jump()
```

The decisive harness call is:

```rust
// src/bin/build_circuit.rs:92
let ops = point_add::build();
```

and the decisive live implementation selection is:

```rust
// src/point_add/mod.rs:2302
let mut ops = trailmix_ludicrous::build_trailmix_ludicrous_ops();
```

By contrast, the legacy builder is internally closed:

```text
src/point_add/mod.rs:1469   defines build_builder()
src/point_add/mod.rs:1519   is its call to emit_dialog_gcd_raw_pa(...)
src/point_add/rounds/dialog/mod.rs:2783 defines emit_dialog_gcd_raw_pa()
```

A repository-wide Rust search found no call to `build_builder()` from `build()` or from the benchmark binary. `rounds/dialog` remains compiled because `src/point_add/rounds/mod.rs` exports it into the point-add module, not because the harness emits that circuit. Similarly, `configure_ecdsafail_submission_route()` and the `DIALOG_GCD_*` defaults are only reached by legacy `build_builder()`.

**Operational consequence:** edits or environment experiments isolated to `rounds/dialog`, `build_builder()`, `emit_dialog_gcd_raw_pa()`, or `DIALOG_GCD_*` are a knob-noop for the normal `build_circuit` path unless somebody deliberately rewires `build()` and demonstrates a changed `ops.bin`.

## What historical change introduced the live route

Git history identifies `69d191fb34ed77cb4f210f59eb8058f312189c4f` (`Submission af5abb17-1bf5-4ed2-8bfb-c653c043eb1f`, 2026-06-19) as the earliest local-history occurrence of `build_trailmix_ludicrous_ops`. Its parent is `57eb64b000e728aa47e9ef9edca871e386f2f372`.

The introduction commit adds the TrailMix implementation as a substantial new engine: roughly 5,001 insertions and 922 deletions under `src/point_add`, including new `trailmix_ludicrous/{arith,codec,comparator,ec_add,fused,gcd,gidney,mcx,mod,schedule,square}.rs` files and the module wiring in `src/point_add/mod.rs`. That is the meaningful implementation transition to examine, rather than treating the legacy dialog point-add function as the current entrypoint.

This commit/provenance statement does **not** identify which later commits introduced every subsequent schedule, codec, or post-pass refinement. It only locates the architectural handoff.

## Stage correspondence: same affine skeleton, different live implementation

The legacy raw dialog and live TrailMix routes have the same broad mathematical point-add skeleton:

| Affine role | Legacy retained route | Live TrailMix route |
|---|---|---|
| Input difference | `mod_sub_qb` twice in `build_builder()` | `coord_addsub(..., true)` twice in `ec_add()` lines 287-290 |
| First division / slope construction | `emit_dialog_gcd_raw_quotient` | `mod_mul_inverse_in_place(..., Direction::Inverse)` lines 292-294 |
| Coordinate core | legacy fused square/tail helpers | `coord_add3x` then `mod_square_sub_pm_secp256k1_symmetric` lines 296-300 |
| Reverse product / erase slope witness | `emit_dialog_gcd_raw_ipmul` | `mod_mul_inverse_in_place(..., Direction::Forward)` lines 302-304 |
| Final y and reflected x | legacy modular subtraction / restore | `coord_addsub(..., true)` and `coord_rsub` lines 306-309 |

Thus it is reasonable to compare the two as implementations of the same point-add algebra, but it is not correct to say the *current* tree “still is” the dialog route. Also, TrailMix is still a **jump-2 binary extended-Euclid / Stein-Kaliski-family** construction—not a change to Bernstein-Yang. The live schedule explicitly declares `JUMP = 2` (`trailmix_ludicrous/schedule.rs:2`), and `memory/01-architecture.md` makes the same distinction.

## The three substantive source-level improvements in the live route

The word “improvement” below means an identifiable replacement in the implementation architecture. None of these points, by source inspection alone, proves a lower executed-Toffoli cost, lower peak Q, or lower intrinsic error rate than a particular old-dialog configuration.

### 1. Direct, generic dialog-tape compression/replay replaces the old sidecar-layout architecture

The retained dialog code is organized around a specialized compressed sidecar log, block offsets, “runway” relocation, and scratch borrowed from future sidecar blocks or inactive register ranges. The old source has many layout-specific gates such as head/tail/pair variants and composite-scratch choices in `src/point_add/rounds/dialog/compressed.rs`.

The live engine instead makes transcript coding part of the GCD walk itself:

* `trailmix_ludicrous/codec.rs:330-390` defines a compact `DialogCodec` enum: `Pair`, `Triple`, `Raw`, `Step0`, and `Tail4Top32`.
* The codec exposes exact symbol count, retained code bits, temporary-clean ancilla count, data wires, and freed wires for each window.
* `jump_dialog_regions()` (`codec.rs:495-525`) derives a complete window plan, and `dialog_tape_qubits()` sums its capacity.
* `forward_gcd_jump()` constructs that plan and immediately compresses transcript windows during the walk (`gcd.rs:1202-1211`, then the forward compression path).
* `reverse_gcd_jump()` pops the encoded suffix, decompresses a window, restores the `(subtracted, swap, s2)` controls, and replays it in reverse (`gcd.rs:1484-1538`).

The material implementation improvement is that the live engine has a single local codec/replay contract with explicit clean/freed-wire accounting. This should make tape lifetime and reverse reconstruction less dependent on the older global sidecar/runway placement machinery.

### 2. Per-divstep active-width and comparison schedules become first-class, symmetric state

TrailMix puts the size schedule directly in the live GCD forward and reverse loops:

* `SCHED_J2` and `GAP_J2` live in `trailmix_ludicrous/schedule.rs:21-23`.
* On each forward step, `forward_gcd_jump()` calculates `current_n = SCHED_J2[i]`, then releases high `u` and `v` qubits with `zero_and_free` until both registers have that width (`gcd.rs:1226-1234`).
* In reverse, `reverse_gcd_jump()` allocates exactly back to the scheduled width before decoding/replaying that step (`gcd.rs:1475-1482`).
* The comparison window is derived against the step and active width, rather than being a monolithic fixed-width comparator.

The old dialog route did have variable-width and borrow mechanisms, so the claim is not “old dialog never shrank.” The difference is architectural: TrailMix makes schedule-driven release/reacquisition an obligatory, visible part of each live forward/reverse divstep. That provides an exact qubit-lifetime ledger coupled to the transcript replay.

A caveat for future analysis: current source says `ITERS = 261` but `BAKED_ITERS = 258` (`schedule.rs:4-16`). Some older documentation describing `ITERS=258` is therefore stale for the current configuration. Since baked schedules/certificates are positional, do not transplant their claims blindly after structural changes.

### 3. Integrated paired Bezout replay, known-bit loans, and schedule-fitted arithmetic

In the live GCD code, the forward branch walk and the reversible arithmetic applying its Bezout update are paired in one implementation:

* `apply_step_forward()` carries out controlled modular add, controlled swap, and the fold/double sequence (`gcd.rs:1750-1784`).
* `apply_step_reverse()` applies the exact reverse counterpart (`gcd.rs:1786 onward`).
* Both fold sides retain the load-bearing `s2` conditionality. Forward uses `fused_double_cdouble(circ, s2, y_reg)` when `s2` is not known zero (`gcd.rs:1775-1783`); reverse uses its reverse counterpart with the same condition (`gcd.rs:1800-1807`).
* The live code has explicit parking/loan/reclaim controls for known odd `u[0]` and known-even `v[0]`, so those wires can be reused without violating clean-ancilla restoration (`gcd.rs` parking helpers and the symmetric forward/reverse calls).
* The controlled modular arithmetic is schedule-fitted (`next_cout_k`, `next_ffg`, controlled modular add, fused fold), rather than using one uniform arithmetic profile at every step.

This is the most important correctness constraint to preserve in any modification: `s2` changes must be paired in the divstep and the Bezout replay. Unpairing it may look like an ancilla cleanup or Toffoli saving but breaks reversibility/phase cleanliness.

## Separate fourth layer: post-emission stream optimization

There is also a live **post-construction** optimization layer, which should not be conflated with the three engine changes above. After TrailMix returns its operation list, `build()` applies a target-fanout fixpoint, `apply_m60_dead_t10`, `ccz_self_inverse_cancel`, TrailMix final-CCX cancellation, an optional second fanout closure, identity-keyed deep stripping, and the tail nonce transformation (`src/point_add/mod.rs:2324+`).

These are real current build-pipeline improvements. They operate on the emitted op stream after the core point-add/GCD implementation. The deep strip is explicitly ordinal/identity keyed and its own comments warn it is valid only for the baked structure; a structural edit can invalidate individual strip keys. That is another reason an apparent source-level saving must be validated against the final artifact.

## What this audit does and does not establish

The current retained status note records a certified/promoted artifact around `T = 1,291,859.302`, `Q = 1,154`, score `1,490,805,286`, and 9,024/9,024 evaluator success. This audit does **not** attribute that score to one of the three changes above, and it does not claim a numerical TrailMix-versus-dialog delta.

No paired old-dialog and live-TrailMix `ops.bin` artifacts were built here, and no trusted evaluator was run. Therefore the following are deliberately not claimed:

* that TrailMix has lower executed T than every dialog configuration;
* that the codec is responsible for any particular Q reduction;
* that an ancilla loan changes the reported Q rather than only a local lifetime;
* that any post-pass is bit-exact for a modified stream; or
* that λ improved or stayed clean.

For a quantitative claim, construct both routes under explicitly comparable settings, retain their `ops.bin` hashes/diffs, and run:

```bash
cargo run --release --bin build_circuit
cargo run --release --bin eval_circuit
```

For λ, use a paired nonce set with at least 12 rounds:

```bash
TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12 \
  cargo run --release --bin build_circuit
```

Then quote mean and sigma, and accept only if all four verifier checks pass, the artifact demonstrably changes, λ is not worse by more than one sigma, and score strictly improves (or λ improves at equal/lower score).

## Practical next step

Do not spend implementation work on `src/point_add/rounds/dialog/` unless the explicit objective is to resurrect and evaluate the old route. For the current benchmark, scout and modify only the live TrailMix cells:

```text
trailmix_ludicrous/ec_add.rs      affine shell
trailmix_ludicrous/gcd.rs         forward/reverse divstep and Bezout replay
trailmix_ludicrous/codec.rs       tape/Q trade space
trailmix_ludicrous/schedule.rs    active-width/comparator schedules
trailmix_ludicrous/gidney.rs      controlled adders/dirty vents
trailmix_ludicrous/fused.rs       fold/double arithmetic
src/point_add/mod.rs              post-emission strip/cancellation stack
```

The immediate correction for future notes is concise: **the source has moved from the legacy dialog/Kaliski implementation to a live TrailMix/Kaliski implementation; the old dialog raw point-add is dead for `build_circuit`, and the meaningful changes are transcript codec/lifetime management, schedule-driven width management, and paired fused arithmetic/replay, plus the separate post-emission rewrite stack.**

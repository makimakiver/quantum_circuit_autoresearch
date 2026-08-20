# 07 — Pi-agent optimization playbook

How to split the point-addition circuit into independently-optimizable **cells**, hand each to an autonomous
pi agent, and let it experiment/optimize without corrupting the frontier. Read `06-research-status.md`,
`02-lambda.md`, and `04-traps.md` first — this file is the operational wrapper around them, not a replacement.

## Project-local harness overlay (active)

`CAMPAIGN_FINDINGS.md` is the authoritative campaign snapshot. The canonical state machine is:

```text
SCOPED -> IMPLEMENTED -> METROLOGY -> SAFE -> EVALUATED
       -> GRIND_APPROVED -> GROUND_SEED_FOUND -> CERTIFIED -> PROMOTED
```

Stop/adjudication states are `REFUTED`, `INCONCLUSIVE`, `PARKED`, and `REVERTED`. A timeout, provider limit,
budget stop, or local plateau is `PARKED`/`INCONCLUSIVE`, never a circuit ceiling. Route representation-level
reframes to `cross-cell-reframer`.

Canonical roles are `frontier-auditor`, `exploitation-tuner`, `structural-researcher`,
`cross-cell-reframer`, `lambda-metrologist`, `strip-census-miner`, `implementer`, `evaluator-certifier`,
`nonce-grinder`, and `orchestrator-reviewer`. Only implementer edits `src/point_add/**`; the miner may edit
only `.pi/experiments/EXP-SM-01-tooling/**`; trusted source is immutable.

Before failure, rejection, timeout, budget stop, rollback, discard, or handoff, print the recovery copy:

```bash
python3 .pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py --status <STATUS> \
  --reason '<reason>' --experiment-id <ID> --command '<exact reproduction command>'
```

This must happen before rollback. The terminal transcript preserves full text results and artifact hashes
when synchronized files disappear.

---

## 0. The scientific objective (what "optimize" means here)

The trusted scorer is

```
S(T, Q) = min( round(T) * Q , 2^64 - 1 )
```

- `T` = **average executed** Toffoli count (verifier-measured over 9024 shots), NOT emitted gate count.
- `Q` = `max referenced qubit id + 1`, NOT live-qubit count.

Current certified frontier: `T = 1,283,487.051` (rounded `1,283,487`), `Q = 1,154`,
`S = 1,481,143,998`, seed clean at 9,024/9,024. Exact compressed/canonical hashes live in
`.pi/frontier.json` and are both mandatory identity gates.

There is a **third, hidden axis not in the score**: measured `λ_total = 17.62` at the pinned frontier. A
circuit only *ships* if a clean seed can be ground for it. So the true
objective is:

> **minimise `round(T)·Q` subject to λ small enough to grind.**
> Every `1.0` removed from λ multiplies grind yield by *e*. Because `T` and `Q` **multiply**, one freed
> qubit is worth ≈ `S/Q ≈ 1.28M` score — *if* `TLM_TARGET_Q` drops with it (§1, vent trap).

Three axes, in leverage order for a pi agent: **λ (exponential) ≳ Q (≈1.29M/qubit) > T (linear)**.

**Hard floors — never spend an agent here** (priced dead in `01-architecture`, `03-proven-floors`, `06`):
one-inversion point-add (`ONE_INV_DX3` blocker), Fermat `x^(p-2)`, Jacobian coords, Kim drop-in, Montgomery
batch-invert, 5-wire normalizer below 6 CCX, "more nonce grinding" as structural gain, pricing a truncation
site by `2^-w` alone.

---

## 1. Evidence standard (non-negotiable — a probe is not a result)

1. **Only** a full run (`build_circuit` → `eval_circuit`, all 9024 shots) or a byte-identical `ops.bin` is
   transfer evidence. A healthy `TRACE_PEAK`/Toffoli probe proves nothing.
2. The verifier enforces four checks — an agent's win must keep all four green:
   correctness (all affine outputs), reversibility, **zero residual phase**, **zero non-output ancilla** after
   the forward pass.
3. **λ needs n ≥ 12 nonces, PAIRED on the same set, quote σ.** Per-nonce sd ≈ 4.25; `n=1` cannot distinguish
   `Δλ=+7` from `Δλ=0` (a retune measured "safe" at n=1 was +7.24 λ at n=12).
4. **Knob-noop trap:** many env flags silently no-op (`04-traps`). After setting one, DIFF `ops.bin` (or the
   emitted op count) to prove the circuit actually changed *before* trusting any measurement.
5. **Qubit-vent trap:** freeing a persistent qubit lowers `Q` only if you *also* lower `TLM_TARGET_Q` by the
   same amount — otherwise the vent pool refills the space and `Q` is unchanged.
6. **`ancilla-garbage = 0` is by construction, not evidence** (`B::free` emits an unconditional `R`). Audit the
   phase word directly; a would-be ancilla failure is laundered into half a phase failure.

---

## 2. The commands every agent runs

```bash
# Build the untrusted circuit -> ops.bin
cargo run --release --bin build_circuit

# Score it: 4 validity checks + T,Q -> score.json + one results.tsv row
cargo run --release --bin eval_circuit

# λ diagnostic (n rounds; read the "lambda_total_per_9024=" field on the DIRTY_SCAN line)
TLM_DIRTY_SCAN=1 TLM_DIRTY_SCAN_ROUNDS=12 cargo run --release --bin build_circuit

# Fast inner loop — cell selftests (reversibility + ancilla + phase, no full run):
TLM_SQ_SELFTEST=1 TLM_SQ_SELFTEST_ONLY=1               cargo run --release --bin build_circuit
SQUARE_WINDOW_SELFTEST=1 SQUARE_WINDOW_SELFTEST_ONLY=1  cargo run --release --bin build_circuit
FOLD_FREED_TAIL_SELFTEST=1 FOLD_FREED_TAIL_SELFTEST_ONLY=1 cargo run --release --bin build_circuit
SPECIAL_FOLD_PARK_SELFTEST=1 SPECIAL_FOLD_PARK_SELFTEST_ONLY=1 cargo run --release --bin build_circuit
DIALOG_GCD_K5_HEAD11_SELFTEST=1 ... TAIL3 / TAIL3_TOP32 / TAIL6_GRAPH9 / TAIL7 (see build() in mod.rs)

# Profiling (cheap, no certification value on its own):
TRACE_TLM_PROFILE=1
TRACE_TLM_CCX=1
TRACE_TLM_TOF=1
TRACE_PHASE_ACTIVE=1
# Card-specific B0 windows are allowed when declared by the card.
```

---

## 3. The decomposition — cells the fan-out targets

Call chain (**VERIFIED 2026-08-16** — an earlier version of this table pointed at `build_builder()` /
`emit_dialog_gcd_raw_pa` in `rounds/dialog/`; that whole module plus `configure_ecdsafail_submission_route`
is **DEAD CODE with zero callers from `build()`**, and every `DIALOG_GCD_*` flag's consumers live only
there — edits or env flags routed at it are guaranteed byte-identical noops):
`build()` (mod.rs:2007, forces `TLM_*` flags) → `trailmix_ludicrous::build_trailmix_ludicrous_ops()`
(trailmix_ludicrous/mod.rs:453) → `ec_add` (trailmix_ludicrous/ec_add.rs:275), then the post-pass stack in
`build()` (mod.rs:2302+): `apply_m60_dead_t10` → `ccz_self_inverse_cancel` → `constprop::ccx_final_cancel`
→ SUB4/d2 deep-strips → `apply_tail_nonce`. The engine is **jump-2 binary extended Euclid
(Stein/Kaliski), JUMP=2** — `schedule.rs` currently sets `ITERS=261` vs `BAKED_ITERS=258`; baked
positional tables are valid only when the two agree. It is **not** Bernstein–Yang (B-Y is proven *worse*
here, `01-architecture` Layer 2).

| id | function / source (under `src/point_add/trailmix_ludicrous/`) | role | primary axis | selftest | flag surface |
|----|-------------------|------|--------------|----------|-------------|
| **A** | input diff + stage order, `ec_add.rs:275` | `x-=Qx; y-=Qy`, sequencing | — (tiny) | — | — |
| **B1** | `forward_gcd_jump` (gcd.rs:1182) + `schedule.rs` (`ITERS`, `JUMP`, `SCHED_J2`, `GAP_J2`) | divstep walk: truncate/shift/cswap/sub | **λ** + T | — | `TLM_*` walk flags forced in `build()` (mod.rs:2007+) |
| **B2** | dialog tape codec, `codec.rs` (`DialogCodec`, `compress/decompress_window`, `dialog_tape_qubits`:477 → 602 q) | encode/replay divstep symbols | **Q** + T | codec selftests in `build()` | codec `TLM_*` flags |
| **B3** | Bezout apply: `reverse_gcd_jump` (gcd.rs:1435) + apply/fold paths in `mod.rs`/`fused.rs` | replay — 47.7% adders + 39.2% controlled-perm | **T** (+ λ via `TLM_APPLY_*_SKIP_LAST*`) | fold selftests | `TLM_APPLY_*`, fold `TLM_*` |
| **B4** | modular reduction / scale, `arith.rs` | field reductions ~5% | T | — | — |
| **C1** | modular square, `square.rs` (+ shared `src/point_add/arith/` `square_addsub_*`) | `λ²`, ~60,545 CCX | T | `TLM_SQ_SELFTEST`, `SQUARE_WINDOW_SELFTEST` | `TLM_SQUARE_*` |
| **C2** | coordinate algebra, `ec_add.rs` (`x+=3x0; x-=λ²; y=λ·x; y-=y0; x=x0−x`) | Rx/Ry updates | T | — | — |
| **D** | second ModDiv: same gcd machinery, `Direction::Forward` (`ec_add.rs`/gcd.rs) | erase λ = division run backwards | T + λ | — | shared with B1/B3 |
| **E** | output fix-ups, tail of `ec_add.rs` | `y-=y0`, `x=x0−x` | — | — | — |
| **P** | **cross-cutting primitives**: `gidney.rs` clean-carry adders, `mcx.rs`, `comparator.rs` (λ-critical windows), `constprop.rs` post-passes | shared by B/C/D | T + λ | via callers | many |

**Peak is a flat plateau, not a spike** (`u+v` shrink 2 q/step while tape grows 2.333 bit/step, net +0.33) —
there is no single fat moment to attack, so Q wins come from B2 (tape codec) or primitive-level width trims,
not from one register.

---

## 4. Classify the hypothesis BEFORE building

Every experiment is exactly one class. Triage in this order:

- **STRUCTURAL-λ** — changes divstep count / schedule / comparator windows / apply skips (cells B1, B3, D).
  Highest leverage (exponential). The convergence tail is steep: `ITERS 258→5.23, 259→2.45, 260→1.11 mm` per
  9024, and `+1 iteration ≈ +2,930 CCX`. So an agent here is explicitly trading λ against T — quantify both.
  MUST measure λ at n≥12 paired.
- **STRUCTURAL-Q** — frees a persistent qubit **and** lowers `TLM_TARGET_Q` (cell B2, primitive width trims).
  ≈ 1.29M score/qubit. Verify `Q` via `eval_circuit` (`max id + 1`), never a live-qubit probe.
- **STRIP (ε=0)** — deletes/merges gates for byte-identical output; lowers `T` only (already-shipped examples:
  `TLM_SQUARE_FROM_ZERO −1522`, `W1155_FWD_EQ_REV −504`, `TLM_KG_INC_VENT −198`, `TLM_GAP_J2` comparator
  narrowing). Verify by gate count **and** an unchanged 9024/9024 run.

Do the leveraged classes first; a STRIP that also lowers peak is worth keeping because it can *unblock* a later
Q drop.

---

## 5. Per-agent task card (the fan-out unit)

Dispatch one agent per card. Card = a filled copy of:

```
CELL:        <A|B1|B2|B3|B4|C1|C2|D|E|P>
HYPOTHESIS:  <one sentence>
CLASS:       <STRUCTURAL-λ | STRUCTURAL-Q | STRIP>
CHANGE:      <env flag to set, or the specific edit behind a new flag>
BASELINE:    commit <sha>; nonce set <N ids>; score.json {T,Q,S}; λ {mean,σ} at n=12
VERIFY:      cell selftest -> full build_circuit+eval_circuit (4 checks) -> λ n=12 paired
ACCEPT IF:   §6 all-true
ROLLBACK:    git worktree discard; frontier untouched
```

Agents work in **separate git worktrees** so `ops.bin` / `results.tsv` / `score.json` never clobber each other.

---

## 6. Accept / reject gate (promotion rule)

Advance a change to `CERTIFIED` only when all applicable gates hold:

1. `eval_circuit`: 9024/9024 correct, reversible, phase-clean, ancilla-clean.
2. The candidate changed `ops.bin`, while default-off behavior still matches both pinned identities.
3. Lambda-exposed work is `SAFE`: upper CI on delta-lambda `< +0.5`. Mean classical `>30` per
   9,024-equivalent or any round `>2x` baseline max is `REFUTED`; everything else is `INCONCLUSIVE`.
4. An explicit `GRIND_APPROVED` record is bound to the exact stream and bounded search budget.
5. A new ground seed is bound through `SUB4_TAIL_NONCE` to that exact stream.
6. Both identities and `S = round(T)·Q` are recorded, and S is strictly lower than the current frontier.

Only the explicit promotion command archives the old frontier and writes a new revision.

---

## 7. Parallelization / dispatch rules

- **Parallel-safe:** distinct read-heavy frontier, tuning, structural, reframing, lambda-log-analysis, and
  census-readiness tasks, dispatched in one `workflowScript` with stable keys.
- **Serialize:** implementer work, all shared-source mutations, full evaluation, nonce grinding, collection,
  and promotion. Use one managed worktree per writer.
- `orchestrator-reviewer` reviews the frontier and lifecycle but does not mutate state. Apply accepted results
  through the harness CLI.
- Because `s2`-conditionality is load-bearing on *both* the walk and the apply (`01-architecture` Layer 2),
  a B1 change and a B3 change to the same divstep symbol are **not** independent — pair them on one agent.

---

## 8. Anti-patterns that waste agents (from 02/03/04/06)

- Trusting a single-nonce λ or Toffoli number (single-nonce Toffoli sd ≈ 13.4 → gates at ~40, not 20).
- Treating `ancilla=0` or a green `TRACE_PEAK` as evidence.
- Freeing qubits without lowering `TLM_TARGET_Q` (vent refills).
- Chasing any hard-floor item in §0.
- Pricing a phase-only truncation site by `2^-w` (three factor-of-2 discounts hide underneath; the measured
  effect of one such site at full width was **zero**).

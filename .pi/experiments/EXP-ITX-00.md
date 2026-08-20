# EXP-ITX-00 — Interleaving/deferral restructures adjudicated against the LIVE census (cross-cell: B1×B3×C1×B2)

Status: **REFUTED-BY-ANALYSIS at scouting** (no build spent on the refuted variants; all numbers below are
live-measured at commit `a2067dcf`, ops.bin sha `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`,
byte-identical rebuild verified twice during this scout). This card answers the Wave-1 task question:
"is the pre-tripwire apply-deferral verdict (floor 1149→1134 = −15 q, +9.2% T, qubit-workstream ceiling −1.56%,
from memory/03-proven-floors.md:105-106) still true post-tripwire?"

1. **experiment_id**: EXP-ITX-00
2. **search_radius**: four interleaving restructures of the walk/apply/square dataflow, evaluated against the live
   peak-owner censuses (nothing was re-implemented): (a) apply-deferral as a separate tape-reading pass;
   (b) apply placement swap (forward-traversal ↔ reverse-traversal carrier); (c) walk-traversal-count reduction 4→3;
   (d) square-output→pair-2-input fusion and Pair-1/Pair-2 global liveness re-scheduling.
3. **mechanism** (why each fails on live geometry):
   - (a) A deferred apply pass must run BETWEEN `forward_gcd_jump` and `reverse_gcd_jump` reading the tape
     non-destructively (the unwalk still needs it in reverse order). Live liveness at that point:
     tape 609 (codec.rs:528 `dialog_tape_qubits` at ITERS=261) + Bezout pair 512 (gcd.rs:1902 tmp + mod.rs:459 y2)
     + adder scratch ~130 (gidney.rs:1206 class) = **1251 > 1154** → deferral is Q-**negative** (−97) in the live
     geometry. The memory −15 q prize came from the pre-tripwire circuit whose peak sat in the pair-1 REVERSE
     window (op 3,454,853, 599 tape at peak); the live peak is the FORWARD window at op 25,972 with **zero** tape
     and a flat 3-window plateau (see arch.md §2), so there is no window where a separate apply pass fits.
   - (b) Moving the apply to the other traversal of the same division re-buckets the same 512-q Bezout pair into a
     window with the same tape+u/v sum (tape(i)+u+v(i)+512 is direction-symmetric; both ends measure 1153-1154).
     Bounded by the measured interleaving asymmetry: walk-with-apply body costs +3,919 (pair-1 fwd) / +4,054.5
     (pair-2 rev) vs apply-free counterparts — a wash, ±4k T, no Q change.
   - (c) Traversal count 4 is representation-forced: Z = Δx + 3·ox − λ² (the pair-2 walk operand,
     ec_add.rs:292-300 sequence) requires Δx, which exists only after `reverse_gcd_jump` restores it (the forward
     walk consumes xv into the dialog). The alternative factorization y3+oy = λ·Z = Δy + λ·(3ox−λ²) walks
     W = 3ox−λ² (available before the unwalk) but needs Δy live across walk-1 → +256 at the 1154 plateau → Q +256.
     Regenerating Δy = λ·Δx afterwards is a third product: strictly worse.
   - (d) Square→pair-2 fusion: the square already accumulates in place into x2 (the walk operand register);
     the remaining surfaces are (i) classical-constant adds (coord_add3x 320 T) mergeable into the square's
     shifted-add carry chains — prize ≤ ~300 executed T (0.02%), and (ii) square-unbuild ↔ walk-step-0 overlap —
     blocked because SCHED_J2[0..11] = 256 (full width from step 0) and the two sides share no control qubits
     (prod bits vs walk flags) so no CCX merges. Global liveness re-scheduling cannot lower max-active: all three
     plateau windows independently need ≥1153 with disjoint compositions (early walk: 510 u/v + 512 Bezout + 132
     scratch; late walk: 609 tape + 512 Bezout + 40 u/v+scratch; square: 512 x2/y2 + 642 prod/sum/scratch), and
     the vent pool refills anything freed below TARGET_Q=1155.
4. **predicted direction**: (a) Q −97 → +97 (i.e. WORSE), T +30..90k; (b) T ±4k, Q 0; (c) impossible without
   Q +256 or a third product; (d) T ≤ −300 best case, Q 0. None is a card.
5. **cheapest_discriminator** (already executed, reproduction below): B0 owner censuses at ops 25,972 / 3,300,220 /
   4,576,221 + TRACE_TLM_TOF phase ledger. No further probe needed for the refutation.
6. **predeclared_falsifier** (fired): "deferred-apply window uncapped need = tape(609) + Bezout(512) + scratch(≥13)
   ≥ 1154" — measured 1251. For (c): "pair-2 operand expressible before unwalk-1 completes without +256 live" —
   refuted by the Δy-dependency above.
7. **evidence_debt**: the +9.2% T figure for deferral is memory-era and was NOT re-measured (moot: the Q direction
   alone kills it). Break-even context: −15 q would allow only +15×(1,283,487/1,154) ≈ +16.7k executed T; even a
   post-tripwire re-mine halving the old +118k cost leaves ~59k ≫ 16.7k. Repaid-by: nothing (card closed).
8. **trial_budget / time_budget**: 0 further trials (analysis + 4 census builds already spent, ~12 min).
9. **unresolved_directions**: a fundamentally different inversion representation (one-traversal, Bernstein–Yang
   class, Fermat) remains priced-dead per playbook §0 hard floors; not re-litigated here.
10. **reopening_trigger**: (i) any codec change that shrinks the tape below ~590 qubits (re-opens (a) only if
    tape+512+scratch < 1154−k with k ≥ 2 — i.e. tape ≤ ~585); (ii) an SCHED_J2 re-shape that drops the late-window
    u/v+scratch below 30 (re-opens (c)/(d) overlap analysis); (iii) discovery of a Δy-preserving apply layout.
11. **reproduction_commands**:
    ```sh
    CARGO_TARGET_DIR=/tmp/w1scout-target B0_WIN_LO=25940 B0_WIN_HI=26010 \
      cargo run --release --bin build_circuit 2>&1 | sed -n '/B0_CENSUS_BEGIN/,/B0_CENSUS_END/p'
    CARGO_TARGET_DIR=/tmp/w1scout-target B0_WIN_LO=3300000 B0_WIN_HI=3300300 \
      cargo run --release --bin build_circuit 2>&1 | sed -n '/B0_CENSUS_BEGIN/,/B0_CENSUS_END/p'
    CARGO_TARGET_DIR=/tmp/w1scout-target TRACE_TLM_TOF=1 cargo run --release --bin build_circuit 2>&1 | grep TLM_TOF
    sha256sum ops.bin   # must be 6519bd01... after restoring any probe state
    ```
12. **classification**: STRUCTURAL-Q (adjudication; would have been STRUCTURAL-Q/STRUCTURAL-T).

**Answer to the task question**: the pre-tripwire deferral verdict is not merely "suspect" — it is inverted on the
live commit. Post-tripwire re-testing is not worth a build: deferral is Q-negative in the live geometry and its
T cost remains ≥3.5× over the (now-zero) qubit prize. The stated −1.56% qubit-workstream ceiling is void.

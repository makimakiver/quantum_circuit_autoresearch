#!/usr/bin/env python3
"""Classical replay of the truncated binary-GCD walk (jump-2), from schedule.rs.

Replicates the FORWARD walk of the production ModDiv at the CURRENT geometry
(ITERS=259). For each random field factor v (with u=p), it classifies:

  terminal            reached (u,v) == (1,0) within ITERS steps
  width-overflow      some step needed more bits than SCHED_J2[i]
  comparator-mismatch the truncated top-cmp_eff-bits comparison disagreed with
                      the full-width comparison at a swap decision
  nonterminal         ran all ITERS steps without reaching (1,0)

The per-factor "hard" rate h feeds the lambda model: lambda_classical(forward)
~= 2 * 9024 * h (two inversions per shot).

This is analysis-only tooling; it does not change the quantum circuit.
"""

from __future__ import annotations

import argparse
import random
import re
import time
from pathlib import Path

SCHEDULE_SOURCE = (
    Path(__file__).resolve().parents[4]
    / "src/point_add/trailmix_ludicrous/schedule.rs"
)
FIELD_MODULUS = (1 << 256) - (1 << 32) - 977


def parse_schedule() -> tuple[int, list[int], list[int]]:
    source = SCHEDULE_SOURCE.read_text(encoding="utf-8")
    iters = int(re.search(r"pub const ITERS: usize = (\d+);", source).group(1))
    sched = [
        int(v)
        for v in re.search(r"SCHED_J2:.*?= &\[(.*?)\];", source, re.DOTALL)
        .group(1)
        .split(",")
        if v.strip()
    ]
    gap = [
        int(v)
        for v in re.search(r"GAP_J2:.*?= &\[(.*?)\];", source, re.DOTALL)
        .group(1)
        .split(",")
        if v.strip()
    ]
    assert len(sched) == iters, f"SCHED_J2 {len(sched)} != ITERS {iters}"
    assert len(gap) == iters, f"GAP_J2 {len(gap)} != ITERS {iters}"
    return iters, sched, gap


def cmp_window(i: int, current_n: int, gap: list[int]) -> int:
    """Mirror gcd.rs::cmp_window with TLM_GAP_J2_DELTA=0."""
    g = gap[i]
    return max(min(g, current_n), 1)


def replay_walk(v_in: int, iters: int, sched: list[int], gap: list[int]) -> str:
    """Replay the forward truncated binary-GCD walk. Returns a classification."""
    u = FIELD_MODULUS
    v = v_in % FIELD_MODULUS
    if v == 0:
        return "zero-factor"

    for i in range(iters):
        current_n = max(sched[i], 1)
        mask = (1 << current_n) - 1

        # Width-overflow: the true value does not fit the truncated register.
        if u.bit_length() > current_n or v.bit_length() > current_n:
            return "width-overflow"

        u &= mask
        v &= mask

        # First right shift: conditional (t1 = v[0]==0) at i==0, unconditional after.
        if i == 0:
            if v & 1 == 0:
                v >>= 1
        else:
            v >>= 1
        # Second shift (s2 = v[0]==0 after first shift).
        if v & 1 == 0:
            v >>= 1

        subtracted = v & 1
        if subtracted:
            if i == 0:
                swap = 1
            else:
                k = cmp_window(i, current_n, gap)
                shift = current_n - k
                v_top = v >> shift
                u_top = u >> shift
                truncated_lt = v_top < u_top
                full_lt = v < u
                if truncated_lt != full_lt:
                    return "comparator-mismatch"
                swap = 1 if truncated_lt else 0
            if swap:
                u, v = v, u
            v = (v - u) & mask

    return "terminal" if (u, v) == (1, 0) else "nonterminal"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--samples", type=int, default=1_000_000)
    ap.add_argument("--seed", type=int, default=0x5933C0DEC)
    args = ap.parse_args()

    iters, sched, gap = parse_schedule()
    print(f"ITERS={iters}  |SCHED_J2|={len(sched)}  |GAP_J2|={len(gap)}")

    rng = random.Random(args.seed)
    counts: dict[str, int] = {}
    t0 = time.time()
    for _ in range(args.samples):
        v = rng.getrandbits(256) % FIELD_MODULUS
        if v == 0:
            continue
        cls = replay_walk(v, iters, sched, gap)
        counts[cls] = counts.get(cls, 0) + 1
    dt = time.time() - t0

    total = sum(counts.values())
    print(f"\nsampled {total} nonzero factors in {dt:.1f}s "
          f"({total/dt:.0f}/s)")
    for cls in ("terminal", "width-overflow", "comparator-mismatch", "nonterminal"):
        c = counts.get(cls, 0)
        print(f"  {cls:22s} {c:9d}  {c/total:.3e}")

    hard = total - counts.get("terminal", 0)
    h = hard / total
    lam_fwd = 2 * 9024 * h
    print(f"\nper-factor hard rate h = {h:.3e}")
    print(f"lambda_classical(forward walk) ~= 2*9024*h = {lam_fwd:.2f} per 9024 shots")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

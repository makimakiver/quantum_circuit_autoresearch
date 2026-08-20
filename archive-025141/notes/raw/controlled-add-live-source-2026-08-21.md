---
title: "Source: live controlled threaded adder and recorded floor"
source_url: "src/point_add/trailmix_ludicrous/gidney.rs:1187-1299; src/point_add/memory/03-proven-floors.md"
source_type: docs
captured: 2026-08-21T00:00:00+00:00
captured_by: captain-ahab
truncated: true
original_chars: 0
---

## Captured source facts

`controlled_clean_add_threaded` allocates `n_inner = s` clean carry wires when a carry-out is requested and `s - 1` otherwise. Its forward loop emits one carry-producing CCX for each produced carry. Its backward loop emits one `ccx(ctrl, b[i], a[i])` controlled sum operation for every bit. A requested carry-out receives one additional `ccx(ctrl, inner[s - 1], cout)`. Carry cleanup uses HMR plus classically conditioned CZ/CCZ, not a further CCX.

`03-proven-floors.md` records the relevant scoped lower bound: controlled addition has an `n` CCX lower bound, while the best known construction costs approximately `2n`; it explicitly states that the factor-two gap remains open. It also states that all candidate work must preserve clean ancillae and that measurement-based temporary-AND erasure is part of the available construction model.

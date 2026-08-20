# EXP-P2-01 — Pair-2 tail skip bundle

This card formalizes the Pair-2 candidate described in the authoritative `CAMPAIGN_FINDINGS.md` snapshot. It
does not add a verdict. Lifecycle state is held in `.pi/state/campaign.json`; detached evidence is observed
through `.pi/continuations/EXP-P2-BZ-METRO.json`.

1. **experiment_id:** `EXP-P2-01`
2. **frontier:** `2026-08-16-a2067dcf`; score `1,481,143,998`; compressed hash
   `6519bd01424d5513cd7f8232d4c15dfcbfe67561eb1b7f05b2cfec87c233a097`.
3. **candidate:** `TLM_APPLY_ADD_SKIP_FWD=4 TLM_APPLY_FWD_S2_ZERO_LAST=3`.
4. **mechanism:** the FWD apply knobs are Pair-2-only. FWD add precedes the final live operand cswap at
   iteration 257, so FWD=4 is the theorem-safe boundary described by the campaign audit.
5. **predicted direction:** T negative, Q unchanged, lambda potentially positive. Probe artifact
   `9cb5010bc5f4a534d82a88a80e80cbd613889d8ce4de54e6487b6e9cb0f18d27`; 8,938,962 ops; raw static
   Toffoli-family delta -2,862; post-strip CCX delta -1,593.
6. **cheapest discriminator:** parse the already-produced paired detached metrology logs; do not restart the
   drivers. Confirm all arm hashes before using measurements.
7. **predeclared lambda gate:** `SAFE` only when the upper confidence bound on delta-lambda is `< +0.5`;
   `REFUTED` when mean classical faults exceed 30 per 9,024-equivalent or any round exceeds two times the
   baseline maximum; otherwise `INCONCLUSIVE`.
8. **promotion gates:** SAFE lambda adjudication, full 9,024-shot four-check evaluation, explicit
   stream-bound `GRIND_APPROVED`, new ground `SUB4_TAIL_NONCE`, final certification, and a score strictly below
   1,481,143,998.
9. **budget:** consume only existing logs for the next step. Additional metrology, full evaluation, and grind
   need separately recorded authorization.
10. **falsifier:** campaign lambda gate fires, any four-check evaluation fails, artifact identity differs from
    the declared arm, or the certified score is not strictly lower.
11. **recovery:** on provider limit or timeout, emit
    `.pi/skills/circuit-evidence/scripts/emit_recovery_bundle.py` output and preserve the continuation manifest.
12. **classification/status at snapshot:** STRUCTURAL-lambda / `METROLOGY`; no new record claimed.

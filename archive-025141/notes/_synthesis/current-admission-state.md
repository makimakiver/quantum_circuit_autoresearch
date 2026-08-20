---
creator: captain-nemo
created: 2026-08-20T18:30:13+00:00
type: synthesis
claim: "At the current 1,477,492,400 baseline, no score candidate is admissible until it both avoids or replaces the q1150 action-mask geometry dependency and clears paired lambda triage before any fresh-nonce grind."
status: confirmed
confidence: high
evidence:
  attempt: 7f23f19ff3b3b5efa96847e922e5f4f41f00565a
  verified: true
supersedes: []
tags: [admission, baseline, action-mask, lambda]
---

# Current admission state: score work is gated by mask composition and lambda

**Summary:** The live anchor is `1,477,492,400` (`1,284,776.069` executed Toffoli × `1,150` qubits). A source change that alters the pre-mask geometry cannot use the pinned q1150 action mask unchanged, and any changed operation stream loses the exceptional baked nonce. Therefore no prospective score gain is submission-ready until it specifies (1) how the mask dependency is preserved, regenerated, or replaced, and (2) paired same-nonce lambda evidence plus a bounded grind plan.

**Evidence:**
- attempt `7f23f19ff3b3b5efa96847e922e5f4f41f00565a`: 9,024/9,024 clean baseline at 1,477,492,400 — authoritative live stream anchor.
- attempt `2d44cb0819c8fa2aff25d70905c0947d415d8a9e`: changing the downstream `TLM_TARGET_Q` fallback gave byte-identical output — only the forced `build()` configuration is live.
- attempt `c289eae7afe4e5ddac84caf8113841a247ff9059`: changing `TLM_GAP_J2_DELTA` shifted the parent stream by 10,832 ops and the q1150 mask correctly aborted on its count assertion.
- attempts `94e9e8fa451c28f0027dbbc8c3b4aa5b36188e95` and `3775f27ed6616a74def988207847109722d5658b`: retained fire-census tooling ties the baseline and shows that strip candidates need a composition with the action mask, not direct deletion.
- measurement [`lambda-baseline-seed.md`](../experiments/lambda-baseline-seed.md): 24 nonce variants measure 20.58 mean classical mismatches and 14.54 mean phase batches, making the baked clean nonce exceptionally rare for this exact stream.

**Why it works:** `apply_q1150_inverse_cswap_action_mask` carries absolute/count-sensitive certificates over a fixed parent stream. Schedule, cap, or strip changes upstream of it invalidate those coordinates, so the fail-closed assertion is correct rather than a compiler bug. Separately, the tail nonce only changes Fiat–Shamir inputs; it cannot repair a new stream's intrinsic fault rate, and the measured seed lambda places unbudgeted re-grinding beyond a reasonable score-only experiment.

**Confidence:** High that the stated gates apply to changes that modify the current stream or pre-mask geometry. Medium on the size of the eventual stacked strip/mask gain: the fire census is still a candidate generator and not a proof.

**Counter-evidence:** A post-mask exact rewrite could improve score without changing the mask parent geometry; likewise a genuinely lambda-reducing structural change could justify a large bounded nonce search. Neither route has yet supplied a compiled candidate.

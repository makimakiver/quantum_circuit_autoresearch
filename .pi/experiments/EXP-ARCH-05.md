# EXP-ARCH-05 — Closed-composite watchlist (dormant re-openers for the Q/traversal/liveness programme)

Status: PARKED (conditional watchlist; no action, no budget). Consolidates the reopening triggers of the
Wave-1 closed composites so none is re-opened informally. Pinned a2067dcf / ops.bin 6519bd01….
Adjudication note (cross-cell-architect review of the LOCAL_PLATEAU/REFUTED outcomes): all five closures below
are correctly PARKED-class, none is EXHAUSTED at the representation level — each closure either (a) exhausted
only its env-knob/parameter basin while the structural basin was escalated with recorded triggers, or
(b) rests on arithmetic over measured liveness (census) rather than a formal bound. The single within-family
EXHAUSTED claim (codec window width, k≤17 complete enumeration) is family-scoped and must never be cited as a
representation-level bound.

| # | closed composite | closure basis | reopening trigger (measurable) | first re-measurement on trigger | owner |
|---|---|---|---|---|---|
| W1 | apply-deferral as separate tape pass (EXP-ITX-00 §3a) | liveness arithmetic: deferred window need = tape 609 + Bezout 512 + scratch ≥13 = 1251 > 1154 (Q-negative) | codec tape construction ≤ ~585 qubits (then need < 1154−k, k≥2) | deferral-window liveness census (B0) | codec × apply × B1 |
| W2 | traversal count 4→3 (EXP-ITX-00 §3c) | representation argument: Z = Δx + 3·ox − λ² needs Δy live across walk-1 → +256 at plateau; alternative needs third product | discovery of a Δy-preserving apply layout; or SCHED_J2 late-window u/v+scratch < 30 (re-opens overlap analyses (c)/(d)) | plateau composition census re-run | ec_add × B1 × B3 |
| W3 | codec width/pair Q axis (EXP-CODEC-01/02) | vent ladder measured shut (7 arms, ≥3× over break-even); width closed by k≤17 enumeration (family-scoped EXHAUSTED) | a construction beating 609 tape qubits; or a binding moment becoming tape-live (≥10 live qubits removed from a tape-free peak) | re-run the TLM_TARGET_Q×SQUARE_CAP ladder for fresh dT/dQ | codec × cross-cell |
| W4 | Q-floor certificate 1154±2 (EXP-QFL-01) | flat 3-window plateau, all terms bounded (measured-census / enumeration-prior / λ-coupling-prior) | (i) Bezout accumulator layout change; (ii) tape < 609; (iii) LMD-03-class SCHED_J2 re-shape; (iv) square peak ≤ 1153 (re-prices narrow-tail+cap programme per EXP-SLOPE-00 trigger (a)) | full 3-window census + TRACE_EACH_PEAK | B3 × B2 × square × B1 |
| W5 | placement swap / square-overlap / global liveness re-scheduling (EXP-ITX-00 §3b/§3d) | measured asymmetry wash (±4k T, 0 Q); overlap blocked by full-width SCHED_J2[0..11] + disjoint controls; vent pool refills | any traversal re-ordering (LMD-03-class); SCHED_J2 re-shape landing | interleaving asymmetry re-measure (TRACE_TLM_TOF fwd/rev bodies) | B1 × B3 |

Rules of engagement: (1) a trigger firing re-opens ONLY the listed composite, at its listed first
re-measurement — never a source edit first; (2) no closure in this table may be cited as EXHAUSTED outside
its recorded scope; (3) any new census that contradicts a closure basis (e.g. tape > 609 regression, scratch
> 20 beside tape 609, Bezout pair < 512 — EXP-QFL-01 §6 falsifiers) overrides this watchlist immediately and
re-opens the whole Q programme.

1. experiment_id: EXP-ARCH-05 · 2. search_radius: the table above · 3–8. n/a (watchlist) ·
9. unresolved_directions: streaming rank-unrank tape (353-bit information gap, no construction known) remains
the only representation-level unknown on the Q axis — parked with W3/W4 triggers, no active search ·
10. reopening_trigger: the table IS the trigger list · 11. reproduction_commands: per-row census commands in
EXP-ITX-00 §11, EXP-QFL-01 §11, EXP-CODEC-02 §11 · 12. classification: STRUCTURAL-Q watchlist (PARKED).

# Gate 8.6 Turn 2 — Boundary Review And Implementation Plan

Closing turn of Phase 8. No production architecture change. Two durable-artifact
defects block the close (Q8.7 class: evidence list, not evidence).

## Stage 1: Boundary Brief

### Semantic truth entering the slice

- Gate 8.6 turn 1 closed R8.47–R8.51 and moved every carried `R8.*` PARTIAL to
  PROVED. Auditor confirmed hardest rows (R8.48 vacuity, R8.50 remove-not-
  privatize, R8.14 two-size fan-out, R8.61 PB recording) by reading code.
- Bank `ordinary_mutations` is **79**. Standing verification set was green at
  turn 1.
- **Q8.3** remains deliberate `PARTIAL`: public boundary sealed; residual
  in-module posture constructibility bounded to `external_effect`.
- Spec §11 names fifteen courtroom scenarios that "must exist and must fail
  closed." Ledger § Courtroom currently names only row 12's test; rows 1–11
  and 13–15 are asserted as "retained from 8.1–8.5" without a name map.

### What this turn may own

1. Rewrite the Phase 8 **exit condition** so it is a true, checkable claim:
   every `R8.*` PROVED; every `Q8.*` CLOSED **except** the named deliberate
   carry **Q8.3**; no gate evidence rests on an *unnamed* PARTIAL.
2. Add a §11 **courtroom traceability map** — fifteen rows → named test(s).
   Honest gaps preferred over stretched neighbours.
3. Re-run the standing verification set, every target by name, `--lib` × 5.
4. Mark Phase 8 / Gate 8.6 turn 2 closed in the durable ledger.

### What adjacent components continue to own

- Q8.3 residual type-hardening (unrepresentability of successor construction
  from a predecessor link alone) — Runtime Hardening Track, resolves no later
  than Runtime Phase 9 before facade snapshot.
- PB1/PB2/PB4 renames — Phase 9 (already recorded).
- Bank Phase 5/6 TCP multi-node crash courtroom (session/queue) — beyond
  Phase 8 handle-registry leak proofs.

### Failure modes at dirty edges

- Exit condition that still says "every Q8.* CLOSED" while Q8.3 is PARTIAL.
- Claiming "two PARTIAL rows" or "deadline earlier than Phase 8 close" for Q8.3.
- Mapping a §11 row to a nearby test that does not fail closed for that row's
  stated cause.
- Declaring Phase 8 closed without re-running the standing set.

### Unresolved facts verified before edit

1. Q8.9 closed at Gate 8.4 — only Q8.3 remains PARTIAL.
2. Q8.3's deadline is a carry *past* Phase 8, not earlier than close.
3. Fifteen §11 scenarios are in `milestone-9.16-runtime-phase-8.md` §11.
4. Row 12 has `courtroom_row_12_lineage_fanout_twins_leave_section_8_counters_unchanged`.
5. Row 14's "no domain mutation" clause has no named Bank/unit e2e; NotifyDeath
   co-commit + lost-response + exactly-once cover the R8.55 anchor behaviour
   but always write death-notice state. Record as honest gap.
6. Row 11's handle non-leak is proved; separate user-node session/queue crash
   is Bank Phase 5 territory. Record as honest scope note.

---

## Stage 2: Implementation Plan

### Slice

**Gate 8.6 turn 2 — Phase 8 closeout artifact repair** (exit condition + §11
map + standing verification + Phase 8 CLOSED).

### Ordered steps

1. Rewrite exit condition + Q8.3 finding evidence text (drop false "two
   PARTIAL" / "earlier than Phase 8 close" claims).
2. Replace Courtroom §11 prose with a fifteen-row traceability table.
3. Update Gate 8.6 header to turn 2; mark Phase 8 CLOSED; refresh standing
   verification results after re-run.
4. Write turn-2 status artifact under `_tmp/`.
5. Run standing verification set (all named targets).

### Out of scope

- Production code, new courtroom tests, Q8.3 type hardening, PB renames.

# Gate 8.6 Turn 3 — Boundary Review And Implementation Plan

One test closes the Gate 8.2 O2 hole that turn 2's courtroom map surfaced;
then Phase 8 closes for real.

## Stage 1: Boundary Brief

### Semantic truth entering the slice

- R8.25 / R8.55 claim: an operation with **no domain mutation** and one
  declared external effect still co-commits a dispatch outbox record; that
  record is the sole local anchor (R8.55). Lost response recovers by
  idempotency; retry emits the rail effect exactly once.
- Every live `co_committed_dispatch_outbox()` assertion today rides
  `NotifyDeath` (writes `DeathNoticeStatusField`) or is the zero-cost twin
  (undeclared money movement). Gate 8.2 O2 was marked PROVED on O1 evidence.
- Turn 2's §11 row 14 correctly recorded this as an honest gap. Phase 8's
  unfinished dependent status means the corrective lands here (spec §9).
- `changed_record_count()` is Relational patch length: domain writes +
  Query idempotency entity + (when declared) outbox entity. NotifyDeath = 3.
  A true emit-only commit therefore yields scaffolding-only count **2**, not
  0. The brief's literal `changed_record_count() == 0` is unreachable without
  rewriting Query's count semantics (and breaking freeze/notify assertions).
  Honest O2 proof: `co_committed_dispatch_outbox()` **and** unchanged domain
  status **and** scaffolding-only `changed_record_count() == 2`.

### What this slice may own

1. Smallest honest Bank mutation-free external-effect operation:
   `RetransmitDeathNotice` — retransmit the death-notice rail for a notice
   already at `NotificationRequested`, writing no domain fields.
2. Bank progression + fixture grant + real-rail e2e covering co-commit,
   lost-response idempotency recovery, and exactly-once rail attempts.
3. Ledger: new finding **Q8.11** (Gate 8.2 origin); correct R8.25 / R8.55
   evidence; update courtroom row 14. Leave row 11's Bank Phase 5 gap.

### What adjacent components continue to own

- Q8.3 posture constructibility residual — Runtime Hardening Track / Phase 9.
- PB1/PB2/PB4 renames — Phase 9.
- Row 11 multi-node session/queue — Bank Phase 5.

### Weaker proxies that must become insufficient

- Treating NotifyDeath co-commit as O2 proof.
- Asserting only `co_committed_dispatch_outbox()` without a mutation-free
  precondition that cannot silently drift.

### Failure modes

- Simulating emit-only by skipping the write in a fixture while the installed
  program still declares writes.
- Counting rail attempts at the request layer instead of `transport.attempts()`.
- Quietly restating R8.25 as PROVED without naming Q8.11 / the new test.

### Unresolved facts verified

1. Schema allows emit-only operation programs (no required write member).
2. Aftermath for escaping effects must be Compensation/Reconciliation-style
   with `Declared` external posture (same family as NotifyDeath), not `None`.
3. Real rail spawn path already exists in `external_effect_dispatch`.
4. Query rejected empty provisional programs and empty invariant state loads —
   substrate must admit emit-only commits for R8.55 (fixed this turn).

---

## Stage 2: Implementation Plan

### Slice

**Gate 8.6 turn 3 — O2 mutation-free e2e + Q8.11 ledger corrective + Phase 8 close.**

### Ordered steps

1. Domain: add `EstateCapabilityOperation::RetransmitDeathNotice`, matching
   `EstateAction`, string value, capability/operation contracts, reads+emits
   (no writes), operation program (emit + external rail, no write), aftermath
   twin of NotifyDeath, request projection, inventory/aftermath test updates,
   delegation authorization arm.
2. Query: admit empty provisional programs and empty invariant state loads when
   the lowered program proposed no graph mutation facts.
3. Server: `retransmit_estate_death_notice` progression (project
   `NotificationRequested`, emit only, no `write_field`).
4. Test: `mutation_free_external_effect.rs` through real `bank-external-rail`:
   - commit: outbox co-committed, domain status unchanged, count == 2
   - lost-response posture
   - equivalent retry → `AlreadyCommitted`, same semantic result, rail
     attempts still 1
5. Ledger: Q8.11 CLOSED; R8.25/R8.55 evidence name the new test; row 14
   updated; Phase 8 CLOSED after standing verification.
6. Standing verification set, every row by name; `--lib` × 5 all reported.

### Out of scope

- Changing `changed_record_count` semantics (scaffolding remains counted).
- Row 11 multi-node courtroom.
- Q8.3 type hardening.

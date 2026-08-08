# Gate 8.6 Turn 1 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Semantic truth entering the slice

- Installed aftermath is two-axis (`correction_authority` ×
  `correction_mechanism`) with derived published posture and type-level next
  actions (`worth-query-installation` / `application_aftermath`).
- Recovery, undo, redo, and linear lineage live under
  `worth-query-execution/.../application_aftermath/` and are re-exported on
  `worth-query-host::facade::primary_graph`.
- Bank production assembly is `BankIdentityRuntime` estate progression
  (`open_commit_recovery`, `admit_undo_*`, `progress_undo_*`,
  `seal_proved_undo`, `derive_redo_intent`, `admit_redo_*`, lineage recorders).
- Bank domain declarations already consume the generic contract via
  `declared_aftermath_for` — no `EstateAftermath` enum remains.
- Monolith `domain_installation/operation_aftermath/` is gone (directory
  absent).
- Canonical-work slots `external_dispatch`, `undo_admission`, `redo_admission`
  exist and are populated on the admission paths that own them.
- Gates 8.1–8.5 closed; courtroom rows 1–11 and 13–15 largely have named Bank
  tests; row 12 lacks a Bank-level two-size fan-out proof.

### What Gate 8.6 may own

- Publication noninterference for aftermath surfaces (R8.48).
- Facade reachability inventory for committed outcome / recovery / compensate /
  reconcile / undo / redo next actions (R8.47).
- HTTP boundary residue proof (R8.49) — no new recovery authority on the
  temporary adapter.
- Exact retirement evidence for superseded monolith, bank-local, and generic
  Phase-8 rollback paths (R8.50) — report remove vs privatize per path.
- Ordinary-commit cost with aftermath machinery live then zero (R8.51 / §8).
- Courtroom row 12 Bank fan-out (10 vs 1000 postings, 1 vs 100 lineage edges).
- Residue: R8.3 (`*_for_replay` off ordinary Phase 8 paths), R8.11 (no Signal
  classifies aftermath), `&'static str` derivation errors, R8.59–R8.61
  recording, Q8.7 standing verification completeness.
- Ledger completion (R8.63) and cross-gate suite accumulation (R8.64).

### What adjacent components continue to own

- Phase 7 disclosure / noninterference substrate (consume, do not rebuild).
- External rail / dispatch (Gate 8.2 product).
- Recovery handle lifecycle (8.3), undo (8.4), redo/lineage (8.5).
- PB1/PB2/PB4 *fixes* — Phase 9 (before facade snapshot). Phase 8 records only.
- Bank Phase 5 HTTP product transport and independent user nodes.

### Weaker / proxy representations that must stay insufficient

- Inspect / support projection must remain descriptive (R8.35) — not next-action
  authority.
- Opaque wire identity, published posture possession, Foundational support
  truth — cannot readmit.
- HTTP deserialization — cannot mint recovery authority.
- Copied receipts / digests — open no undo/redo door.

### Competing authorities to cut over or prove retired (R8.50)

| Path | Current state | Disposition |
|---|---|---|
| Monolith `operation_aftermath` | Directory absent; certification green | **Removed** (prove residue) |
| Bank-local `EstateAftermath` enum | Replaced by `declared_aftermath_for` | **Removed** (prove residue) |
| Generic mutation rollback as Phase 8 correction | No public Phase-8 rollback API; workflow `cleanup_pending.rollback` is managed-run generation abort, not aftermath | **Removed** from aftermath authority (prove no ordinary Phase-8 rollback door); do not delete managed-run cleanup |

### Downstream handoff

- Bank World Phase 5 begins only after this gate's courtroom, residue, and cost
  evidence close.
- Phase 9 facade snapshot must see PB1 rename deadline recorded.

### Failure modes at dirty edges

- Marking ledger rows `PROVED` without moving evidence (explicit brief warning).
- R8.48 vacuous paired worlds that do not actually differ in the protected fact.
- Next-action availability differing across paired worlds when a protected fact
  drove an inverse decision (API-shape leak).
- R8.51 zero counters where aftermath machinery is not installed (false zero).
- Single-size fan-out (row 12) proving nothing about slope.
- Privatizing a superseded path that remains internally callable (R8.0 wants
  exact retirement).
- Treating PB recording as PB fixed; missing Phase-9 rename deadline.
- Leaving `&'static str` derivation failures as rediscoverable residue.

### Unresolved facts verified before edit

1. Host already re-exports aftermath APIs via `primary_graph` — R8.47 is mostly
   evidence + any missing next-action surface on Bank outcomes.
2. No existing R8.48 paired-world aftermath noninterference test.
3. `bank-http-adapter` has zero recovery/undo/redo matches — R8.49 is residue
   proof, not a cutover.
4. Unit fan-out twins exist for undo/redo intent; Bank courtroom row 12 does not.
5. `undeclared_external_effect` proves live-then-zero for external_dispatch only;
   R8.51 needs the same for undo/redo slots on ordinary commit.
6. Spec §12 forbids fixing PB1/PB2/PB4 here; brief permits rename only if judged
   in scope — **out of scope**: record with Phase-9 pre-snapshot deadline.
7. `Result<Self, &'static str>` on undo/redo intent and external-effect
   identity derivation is the named 8.5 residue.

---

## Stage 2: Implementation Plan

### Slice

**Gate 8.6 — Bank aftermath cutover, publication, certification** covering
R8.47–R8.51 and truing every carried PARTIAL (R8.0, R8.3, R8.10, R8.11, R8.14,
R8.15, R8.59–R8.61, R8.63, R8.65, Q8.3, `&'static str` residue).

### Constraining findings

- Next-action availability is the sharp R8.48 channel; paired worlds must
  differ in a real protected fact first.
- R8.51 must assert aftermath machinery live before asserting zero cost.
- R8.50 report must say removed vs privatized per path; prefer remove.
- PB rename is Phase 9; record deadline, do not rename.
- Standing verification set is mandatory by named target.

### Intended result

A Bank consumer can reach typed recovery/undo/redo through host facades; paired
worlds differing only in a protected inverse-influencing fact publish equal
aftermath explanations, lineage shape, and next-action availability; ordinary
commits pay 0/0/0 on aftermath slots while the machinery is installed;
superseded aftermath authorities are gone; courtroom row 12 proves fan-out
independence at two sizes; the Phase 8 ledger closes every row with moved
evidence.

### Directory / module shape

| Artifact | Responsibility |
|---|---|
| `bank-server/tests/.../phase8_publication_noninterference.rs` | R8.48 paired-world courtroom |
| `bank-server/tests/.../phase8_ordinary_commit_cost.rs` | R8.51 live-then-zero |
| `bank-server/tests/.../phase8_fanout_courtroom.rs` | Courtroom row 12 / R8.14 |
| `bank-server/tests/.../phase8_facade_reachability.rs` | R8.47 typed facade inventory |
| `bank-server/tests/.../phase8_residue.rs` | R8.3 / R8.11 / R8.50 mechanical residue |
| `bank-http-adapter/tests/protocol_boundary.rs` (extend) | R8.49 no recovery authority |
| `application_aftermath/derivation_failure.rs` | Typed internal derivation error |
| undo/redo/dispatch/correlation | Consume typed failure instead of `&'static str` |
| `front-door-closure-ledger.md` | PB1/PB2/PB4 gap rows |
| Phase 8 closure ledger | Gate 8.6 section + PARTIAL → PROVED with evidence |

### Ordered steps

1. **Typed derivation failure (8.5 residue)** — replace `&'static str` on
   undo/redo/external-effect identity derives with a crate-private typed kind;
   prove no consumer-visible string denial regression.
2. **R8.50 / R8.3 / R8.11 residue tests** — mechanical source scans; document
   remove dispositions.
3. **R8.49 HTTP residue** — assert adapter source contains no recovery decision
   vocabulary and deserializes no aftermath authority.
4. **R8.47 facade reachability** — Bank test that host-exported types and
   BankIdentityRuntime methods cover outcome/recovery/compensate/reconcile/
   undo/redo next actions without internal crate imports.
5. **R8.51 ordinary commit cost** — install rail (live), commit undeclared-
   effect mutation, assert external_dispatch + undo_admission + redo_admission
   all 0/0/0.
6. **R8.48 noninterference** — paired freeze or disburse worlds differing in a
   protected/restricted estate fact that participates in inverse pre-image or
   admission observation; prove difference; assert equal explanation support,
   equal lineage topology after correction, identical next-action contract
   discriminants / available Bank progression methods.
7. **Courtroom row 12** — Bank fan-out twin at (10,1) vs (1000,100) asserting
   §8 counters identical on undo/redo admission work.
8. **PB1/PB2/PB4** — enter gap ledger with intake category, Phase 9 owner,
   pre-snapshot rename deadline for PB1/PB2, consequence commands.
9. **Ledger close** — update every carried PARTIAL with new evidence columns;
   mark Gate 8.6 CLOSED only if standing verification is green.
10. **Standing verification** — full set by name; five `--lib` runs; dirty
    line-cap; code-quality-qa + qa-tests.

### Out of scope

- PB1/PB2 rename and PB4 production literal fix (Phase 9).
- Durable Store-backed handles.
- Building Bank Phase 5/6 courtroom product.
- Reworking managed-run workflow cleanup rollback (not Phase 8 aftermath).

### Verification commands

Standing set from the Phase 8 ledger, plus focused new modules under
`ordinary_mutations` / execution `--lib` / http-adapter protocol tests /
boundary-check / agent-context / dirty line-cap / warning-clean check.

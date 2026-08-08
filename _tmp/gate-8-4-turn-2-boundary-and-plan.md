# Gate 8.4 Turn 2 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### Slice selected
Complete Gate 8.4's remaining load-bearing obligations after turn 1's
admission/C2/registry slice: A10 denial precision, R8.2 pre-image consumption,
R8.9 Bridge correspondence resolution, R8.38 money compensation with
independent oracle, R8.39 eight denials with no-write proof, R8.40 fan-out
twins, R8.41 positive Foundational description, and A11 ordinary progression
for the derived request. Do not open Gate 8.5.

### Semantic truth entering the slice
- **Installed pre-image demand (R8.18)** — field slots + byte bound; install
  rejects uncovered demand. Consumption into the receipt is absent.
- **Decision read-set facts** — `WorthQueryApplicationObservedFact::Field`
  carries `AspectValue` on the in-flight attempt (`attempt.facts`). Available
  at `commit_prepared_session`; discarded after commit today.
- **Commit receipt (C1)** — authority binding + mutation work names; no
  retained pre-image carrier.
- **Declared lowering correspondence slot** — string copied into
  `InstalledLoweringCorrespondenceRef`. Binding identity is still the string
  (G8 open). `worth-query-installation` must not import Bridge; resolution is
  against a Query-owned install-time catalog of typed installed correspondences
  (generation + graph participation), with the slot remaining diagnostic.
- **Undo admission (turn 1)** — derives request from axes, consumes touched
  records, mints one intent identity. Does **not** hand off to
  `compare_and_commit_application`.
- **Bank money path** — proposals → `commit_journal_proposal` /
  `commit_reverse_journal` → same `compare_and_commit_application`. Estate
  `DisburseEstate` declares Compensation. No R8.38 courtroom + independent
  oracle yet.
- **Denial kinds** — eight R8.39 causes typed; several never constructed;
  A10 has a permissive `Authorization(_)` arm.

### What this slice owns
- A10: assert specific current-policy cause on every reachable denial arm.
- R8.2: `undo_preimage` retention from attempt facts into the receipt;
  RecordedInverse undo **consumes** retained bytes (no live re-read).
- R8.9: install resolves correspondence against a typed catalog; store
  installed witness (identity + generation + graph participation); reject
  unresolved / wrong-generation / mismatched participation; slot is diagnostic.
- `undo_progression`: hand admitted undo into ordinary compare-and-commit
  (A11 / R8.37), deriving the effect program from mechanism + retained
  pre-image / compensation demand — not a parallel mutator.
- R8.38: compensating debit+credit journals; originals preserved; one
  compensation under retry; independent double-entry oracle (no production
  accounting imports).
- R8.39: construct each of eight causes; prove no write; released-estate
  type-level absence retained.
- R8.40 fan-out twins; R8.41 positive Foundational description after admission.
- Ledger R8.63 update for closed rows.

### Adjacent ownership that continues
- Recovery handle lifecycle (8.3), rail (8.2), aftermath axes (8.1).
- Bridge's full correspondence admission machinery — Query catalog is the
  install-time typed reference; host populates it from Bridge products.
- Redo / lineage (8.5) — out of scope.

### Weaker representations that must become insufficient
- Pre-image required by signature but unused.
- Live re-read at undo labeled as original.
- String slot as binding correspondence identity.
- Admission-only undo counted as progression.
- Request-layer compensation counts; shared production oracle.
- Empty denial match arms; write-then-rollback as “no mutation.”
- Single fan-out number as R8.40 proof.
- Foundational-only negative without positive description twin.

### Competing authorities / cutover
- Receipt gains sealed `retained_preimage`; RecordedInverse undo fails closed
  without it.
- `InstalledRecordedInverse` stores resolved correspondence value; string
  accessor becomes diagnostic-only.
- Undo mutation enters only through `compare_and_commit_application` (Bank
  ordinary commit sites for money).
- Remove permissive A10 arm.

### Downstream handoff
- Undo progression consumes admission + retained pre-image → effect program →
  ordinary commit.
- Bank courtroom tests observe committed journal rows and type-level absence.
- Ledger rows move OPEN → PROVED / CLOSED.

### Dirty-edge failure modes
- Retention of whole records / unbound bytes.
- Catalog that always accepts any slot (fake resolution).
- Parallel undo executor.
- Oracle importing `bank_domain::accounting` balance helpers.
- Graph-equality after rollback as no-write proof.
- Fan-out twin that still keys identity off posting count.

### Unresolved facts verified
- `attempt.facts` available at commit in `session_lifecycle.rs` — retention
  source exists.
- Installation has no Bridge dep — Query-side catalog is the honest boundary.
- `undo_progression.rs` / `undo_preimage.rs` do not exist yet.
- Reverse journal path already uses ordinary compare-and-commit — money
  compensation should reuse that lane shape.
- Released / irreversible next-action contracts already lack undo methods
  (R8.21); strengthen ReleasedEstate denial cause separately.

---

## Stage 2: Implementation Plan

### Slice name
Gate 8.4 turn 2 — pre-image, correspondence, progression, money, denials.

### Ordered steps
1. **A10** — tighten cross-gate denial match; every arm asserts
   `CurrentPolicyDenied` (or recovery equivalent).
2. **R8.2 / undo_preimage** — sealed retained pre-image; retain from attempt
   field facts + demand at commit; attach to receipt; undo admission consumes
   for RecordedInverse.
3. **R8.9** — `AftermathLoweringCorrespondenceCatalog` + resolved installed
   correspondence; install rejects unresolved / generation / participation;
   update call sites and tests.
4. **undo_progression (A11)** — derive effect-program handoff into
   `compare_and_commit_application`; Bank assembly for compensation/inverse.
5. **R8.38** — DisburseEstate (or transfer) compensation through ordinary
   commit; independent oracle; retry-once; count committed journals.
6. **R8.39** — eight denial constructors + no-write proofs + positive twins;
   ReleasedEstate type absence.
7. **R8.40 / R8.41** — fan-out twins; Foundational may describe afterward.
8. **Verify** — standing set; ledger update; residue checks.

### Module shape
```
application_aftermath/undo_preimage.rs      # retain + consume
application_aftermath/undo_progression.rs   # ordinary entry handoff
installation/.../recorded_inverse.rs        # resolved correspondence
installation/.../lowering_correspondence.rs # catalog + resolve
bank-server/.../estate_progression/undo.rs  # production assembly
bank-server/tests/.../phase8_undo_*.rs      # courtroom evidence
```

### Out of scope
- Gate 8.5 redo/lineage
- Pulling `worth-runtime-bridge` into `worth-query-installation`
- Store-durable pre-image

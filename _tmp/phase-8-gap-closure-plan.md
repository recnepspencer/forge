# Phase 8 Gap Closure — Boundary Brief + Plan

## Boundary brief

### PB1 — Currency → Unit (platform slot)
- **Truth:** `ApplicationFieldRef`'s 8th param is a unit-of-measure marker; name is finance-shaped.
- **This slice owns:** rename platform slot vocabulary across `worth-query*` and Bank *consumers of platform names*.
- **Adjacent keeps:** Bank `trait Currency`, `Money<C: Currency>`, `USD`, `UsdCurrency` type name.
- **Cutover:** one vocabulary (`Unit` / `ApplicationFieldUnit` / `NoApplicationUnit` / `ApplicationUnitRef` / `worth_query_unit!`); no dual names.
- **Risk:** ~95 files; canonical identity string `"currency"` → `"unit"` (digest churn expected in tests that pin digests).

### PB2 — Amount → Magnitude
- **Truth:** `ApplicationCapabilityAmountDimension` + constraint `amount` are magnitude bounds, finance-named.
- **Rename:** `ApplicationCapabilityMagnitudeDimension`, field/accessor/builder `magnitude`.
- **Keep:** domain money `amount` fields, estate/accounting amounts.

### PB4 — Ordinary branch literal
- **Owner:** `application_branch.rs` (`PRIMARY_APPLICATION_BRANCH`, `primary_relational_branch_id`, `primary_truth_branch_identity`).
- **Sites:** production `bootstrap_publication.rs:158`; tests in semantic_basis, truth_read_request, historical_authority, idempotency; plus hostile_resolution and causal_fixture for residue completeness.
- **Residue:** no `BranchId("main")` outside `application_branch.rs`.

### Q8.3 — Posture construction
- **Defect:** four successor variants constructible from predecessor link alone inside the module.
- **Treatment already built:** Compensation/Reconciliation carry `ExternalEffectPostureEvidence`.
- **Apply:** add evidence field to EmittedApplicationCausality, DispatchAttempt, ExternalAcknowledgement, ExternalCompletion; mint on honest dispatch/classification paths; ProviderCommit stays rootless.
- **R8.22** then PROVED on unrepresentability.

### §11 row 11
- **Handle:** proved (keep).
- **Session/queue on recovery path:** `WorthQueryRecoveryHandle` holds only identity/slot/registry/binding/work — no session or queue.
- **User-node session/queue types:** do not exist (`bank-user-node` absent; front-door ledger blocks same-process substitute).
- **Honest boundary:** record with code evidence; not a deferral assertion.

## Implementation order
1. Q8.3 (small, local) — evidence on four successors + call sites + tests
2. PB4 — branch owner routing + residue test
3. PB2 — magnitude rename (few files)
4. PB1 — mechanical Unit rename (longest-first, scoped)
5. Row 11 ledger evidence
6. Update both ledgers (phase-8 + bank front-door)
7. Standing verification set

# Gate 8.3 Turn 2 — Boundary Review And Implementation Plan

## Stage 1: Boundary Brief

### What turn 1 left inverted
- `WorthQueryRecoveryFreshAuthority` is a public bag of caller-filled fields.
  `capability_currently_grants` / `disclosure_admitted` are booleans. Binding
  axes are compared against the same caller voice that minted the handle's
  twin. R8.31's "re-establishes" has no production site.
- `evaluate_expiry` is `#[allow(dead_code)]`; expiry is not a lifecycle branch.
- Production never calls mint or any transition. Cross-gate world fills
  FreshAuthority by hand — honest only if production assembles the same way.
- Compensate and reconcile both deny `TransitionNotAdmitted` — two axis
  failures collapsed into one cause.
- Q8.3 residual: Compensation/Reconciliation require evidence; earlier ladder
  postures (`provider_commit` … `external_completion`) remain publicly
  constructible inside `external_effect` for production dispatch.

### Semantic truth that must enter transitions
- **Current application authority** — privately constructed proof that
  capability/operation admission ran *now*. Precedent: Phase 7
  `WorthQueryAdmittedApplicationCapabilityAccess` /
  `WorthQueryAdmittedApplicationOperation` (crate-private mint).
- **Current disclosure admission** — privately constructed proof for inspect.
  Cannot be a bool the caller sets.
- **Current binding truth** — derived from:
  - admitted operation / capability access (runtime authority, schema,
    branch, principal scope, installed operation)
  - the recovery-target receipt the runtime holds (attempt, idempotency,
    provider posture, correlation)
  - the installed aftermath (compatibility generation)
  - the runtime clock (expiry)
  Handle binding is one side; runtime-derived truth is the other. Caller must
  not supply both.

### What this slice owns
- Replace FreshAuthority bools + public field bag with proof-carrying
  effect/inspect authorities minted only by production establish paths.
- Distinct denial kinds for compensation-mechanism vs reconcile-authority
  axis failures.
- Wire mint + at least one transition through BankIdentityRuntime (production
  assembly matching §10.4).
- Wire `evaluate_expiry` into live transitions / expire path (R8.29 / R8.7 M2–M3).
- Then adversarial tests: per-axis drift, revoke-after-mint via real
  capability denial, disclosure denial without fabricating proofs, expiry
  M2/M3, trybuild clone/duplicate-transition, leak across four terminals.
- Q8.3: seal earlier ladder constructors to `pub(crate)` where production
  already owns them, or name-and-date the residual.

### Adjacent ownership that continues
- Full capability refresh stack / elevation — consume admitted operation,
  do not redesign admission.
- Gate 8.4 undo — consumes handle + C1 receipt after this gate's API is honest.
- Courtroom rows beyond 8.3 exit proof.

### Weaker representations that must become insufficient
- `capability_currently_grants: bool` / `disclosure_admitted: bool`.
- Public struct whose fields are the compared truth.
- Test-only FreshAuthority construction that bypasses admission.
- Dead `evaluate_expiry`.
- Shared `TransitionNotAdmitted` for mechanism vs authority axis failures
  (unless deliberately documented as one fact — they are not).

### Competing authorities / cutover
- Delete public `WorthQueryRecoveryFreshAuthority` field construction from
  facade consumers; replace with `admit_recovery_effect_authority` /
  `admit_recovery_inspect_authority` on the application runtime (or module
  functions that require admitted operation + runtime + receipt).
- Cross-gate `fresh_authority_for` must call production admit paths, not
  assemble booleans.
- Bank production: `open_commit_recovery` / `dispose_commit_recovery` (or
  equivalent) mint and drive one transition through real admission.

### Downstream handoff
- Transitions take `&WorthQueryRecoveryEffectAuthority` or
  `&WorthQueryRecoveryInspectAuthority` instead of FreshAuthority.
- Expiry decision is required input to `expire_recovery_handle` and is
  produced only by runtime-clock evaluation.

### Dirty-edge failure modes
- Proof type with public constructor or test cfg mint.
- Establish path that copies handle binding into "current" and compares.
- Revoke test that still passes a false bool.
- Production mint that never runs a transition.
- Expiry sample supplied by caller.

### Unresolved facts verified
- `WorthQueryAdmittedApplicationOperation` carries
  `operation_scope_binding` and `installed_operation_identity_bytes` —
  sufficient for principal/operation axes from admission.
- Capability access alone lacks scope binding; effect authority should
  require admitted *operation* (bank already admits notify-death this way).
- Earlier posture ctors are used by `dispatch.rs` / `classification.rs` in
  the same module — `pub(crate)` seal is safe.
- Certification already uses trybuild for aftermath compile-fail; add
  recovery-handle cases there or a sibling harness.

---

## Stage 2: Implementation Plan

### Slice
Gate 8.3 turn 2 — invert R8.31, wire production + expiry, then exit evidence.

### Intended DX
```rust
let handle = runtime.open_commit_recovery(receipt, &aftermath)?;
let admission = runtime.admit_notification_operation(principal, action, scope)?;
let effect = runtime.admit_recovery_effect_authority(
    &handle, &admission, receipt.application(), &aftermath,
)?;
let _ = dispose_recovery_handle(handle, &effect)?;
// inspect:
let disclosure = runtime.admit_recovery_inspection_disclosure(&access)?;
let inspect = runtime.admit_recovery_inspect_authority(
    &handle, &admission, &disclosure, receipt.application(), &aftermath,
)?;
let view = inspect_recovery_handle(&handle, &inspect)?;
```

### Topology changes
```
recovery_progression/
  fresh_authority.rs   → rewrite: EffectAuthority + InspectAuthority
                         privately minted; establish_* reads runtime truth
  disclose.rs          → NEW: RecoveryDisclosureAdmission private mint
  (dispose/inspect/… take new authority types)

primary_graph/application_runtime/recovery_authority.rs  → NEW
  admit_recovery_effect_authority / admit_recovery_inspect_authority
  evaluate_recovery_expiry (clock from runtime)

bank-server estate recovery surface → open_commit_recovery + one terminal

external_effect/posture.rs → pub(crate) earlier ladder ctors (Q8.3)

tests: rewrite phase8_cross_gate world; add drift/expiry/leak/trybuild
```

### Ordered steps
1. **Denial kinds** — add `CompensationNotAdmitted`, `ReconciliationNotAdmitted`;
   wire compensate/reconcile.
2. **Proof types** — replace FreshAuthority; private fields; establish functions
   compare handle vs admission/receipt/aftermath/runtime-derived axes.
3. **Disclosure proof** — private mint via runtime method requiring live
   capability access belonging to this runtime.
4. **Expiry** — runtime method samples clock; transitions that produce effect
   reject expired handles; `expire_recovery_handle` remains the expired
   terminal; remove dead_code allow.
5. **Production wire** — BankIdentityRuntime opens recovery from committed
   receipt + aftermath and disposes through admit → effect authority →
   dispose. Cross-gate lost-response uses this path.
6. **Q8.3** — `pub(crate)` seal ProviderCommit…ExternalCompletion constructors;
   ledger note dated.
7. **Tests** — rewrite cross-gate; per-axis drift with positive twins; real
   revoke-after-mint; disclosure without proof; expiry M2/M3; leak four
   paths; trybuild clone + duplicate transition.
8. **Verify** — format, line-cap dirty, boundary-check, agent-context,
   Gate 8.1/8.2, phase8_cross_gate, new recovery tests.

### Cutover
- Remove `WorthQueryRecoveryFreshAuthority` public field API from facade.
- Delete `fresh_authority_for(..., bool, bool)`.

### Out of scope
- Full courtroom; Gates 8.4–8.6; redesigning capability admission itself.

### Blockers
- None structural; bank already admits notify-death operations and can revoke.

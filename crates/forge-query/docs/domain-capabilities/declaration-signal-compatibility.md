# Declaration Signal Compatibility

## What This Feature Is

Declaration signal compatibility is the Query-owned boundary that turns one
retained declaration envelope into one retained compatibility artifact for
later Signal-backed derived execution.

This phase freezes whether later derived execution is structurally admitted,
explicitly deferred, denied, or failed without pretending Query already
executed through `forge-signal`.

Phase 14 is envelope-backed only on the public lane. It does not start from
raw declarations, canonical declarations, legality evidence, foundational
evidence, route plans, or receipts alone.

## Why You Use It

- freeze later Signal-backed execution eligibility as one retained Query
  artifact instead of a loose posture flag
- preserve admitted-world identity, basis-family requirements, and retained
  route or receipt posture while classifying derived-execution compatibility
- keep mixed-authority declarations honest by treating signal compatibility as
  a modifier over retained declaration truth instead of a fake peer authority
  family
- get one signal-compatibility digest that converges when retained truth
  matches and diverges when world, basis family, authority posture, or denial
  topology changes
- hand later phases one stable compatibility artifact so they do not reopen
  route, receipt, envelope, relational, or bridge meaning

## Stable Entry Points

- `ForgeQueryDeclarationSignalCompatibilityInput`
- `ForgeQueryDeclarationSignalCompatibility`
- `ForgeQueryDeclarationSignalCompatibilityChecked`
- `ForgeQueryDeclarationSignalCompatibilityDeferred`
- `ForgeQueryDeclarationSignalCompatibilityDenied`
- `ForgeQueryDeclarationSignalCompatibilityFailed`
- `ForgeQueryDeclarationSignalCompatibilityTerminalError`
- `ForgeQueryDeclarationEntrySignalCompatibilityError`
- `ForgeQueryDeclarationSignalCompatibilityClass`
- `ForgeQueryDeclarationSignalCompatibilityContract`
- `ForgeQueryDeclarationSignalCompatibilityDenialCause`
- `ForgeQueryDeclarationSignalCompatibilityExplanation`
- `ForgeQueryDeclarationSignalCompatibilitySupportReport`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow`
- `ForgeQueryDeclarationSignalCompatibilitySupportStatus`
- `ForgeQueryDeclarationSignalExecutionFamily`
- `ForgeQueryAdmittedConfiguredDomainHandle::signal_compatibility(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::signal_compatibility_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::signal_compatibility_from_progressed(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::signal_compatibility_from_progressed_with_intent(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::signal_compatibility_support::<I>()`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_signal_compatibility(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_signal_compatibility_outcome(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_signal_compatibility_checked(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::orchestrate_signal_compatibility_proof(...)`
- `ForgeQueryAdmittedConfiguredDomainHandle::declaration_entry_crossing_inventory::<I>()`
- `ForgeQueryAdmittedConfiguredDomainHandle::declaration_entry_readiness::<I>()`
- `ForgeQueryAdmittedConfiguredDomainHandle::inspect_declaration_entry(...)`

Good to know:

- the ordinary public lane is envelope-backed only
- common-lane helpers exist only for structurally signal-compatible
  declaration families
- deferred and incompatible families remain support-visible and checked-visible
- Phase 14 freezes compatibility only; it does not execute, invalidate,
  recompute, or schedule Signal work

## API Reference

Signal-compatibility input constructors:

- `ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope)`
- `ForgeQueryDeclarationSignalCompatibilityInput::deferred(envelope)`
- `ForgeQueryDeclarationSignalCompatibilityInput::denied(envelope)`
- `ForgeQueryDeclarationSignalCompatibilityInput::failed(envelope)`
- `ForgeQueryDeclarationSignalCompatibilityInput::envelope_checked(checked)`

Admitted-handle signal-compatibility entry points:

- `signal_compatibility(subject) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_checked(subject) -> ForgeQueryDeclarationSignalCompatibilityChecked<D, I>`
- `signal_compatibility_from_progressed(progressed) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `signal_compatibility_from_progressed_with_intent(progressed, intent) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationSignalCompatibilityTerminalError<D, I>>`
- `declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(input) -> Result<ForgeQueryDeclarationSignalCompatibility<D, I>, ForgeQueryDeclarationEntrySignalCompatibilityError<D, I>>`
- `signal_compatibility_support::<I>() -> ForgeQueryDeclarationSignalCompatibilitySupportReport<D, I>`

Checked signal-compatibility outcomes:

- `ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(ForgeQueryDeclarationSignalCompatibility<D, I>)`
- `ForgeQueryDeclarationSignalCompatibilityChecked::Deferred(ForgeQueryDeclarationSignalCompatibilityDeferred<D, I>)`
- `ForgeQueryDeclarationSignalCompatibilityChecked::Denied(ForgeQueryDeclarationSignalCompatibilityDenied<D, I>)`
- `ForgeQueryDeclarationSignalCompatibilityChecked::Failed(ForgeQueryDeclarationSignalCompatibilityFailed<D, I>)`

Signal-compatibility classes:

- `Compatible`
- `Deferred`
- `Denied`
- `Failed`

Signal execution families:

- `RuntimeDerivedExecution`
- `HistoricalDerivedExecution`
- `PreviewDerivedExecution`
- `MixedDerivedExecution`

Support-report inspection:

- `ForgeQueryDeclarationSignalCompatibilitySupportReport::declaration_family_key()`
- `ForgeQueryDeclarationSignalCompatibilitySupportReport::rows()`
- `ForgeQueryDeclarationSignalCompatibilitySupportReport::support_digest()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::execution_family()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::basis_family()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::required_dependency_aspects()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::produced_aspects()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::available_aspect_slice()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::aspect_fit()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::aspect_mismatch()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::status()`
- `ForgeQueryDeclarationSignalCompatibilitySupportRow::reason()`

Support statuses:

- `Admitted`
- `Deferred`
- `Unsupported`
- `InvalidBasis`

## Core Mental Model

Think of signal compatibility as the declaration-side answer to one narrow
question:

"Can this retained declaration crossing story later continue into
Signal-backed derived execution, and if so, under which basis-family
requirements?"

That means:

1. envelopes still own the public crossing story
2. relational routing still owns relational truth binding
3. bridge routing still owns bridge continuation binding
4. signal compatibility freezes later derived-execution eligibility without
   re-running those earlier boundaries

The compatibility artifact is not a live Signal execution, schedule, or
invalidation surface. It is the proof-bearing Query artifact that says whether
later Signal execution is structurally admitted and which basis-family
requirements that later execution must respect.

The important Phase 24b shift is that signal compatibility now classifies from
the envelope's published semantic slice plus the declaration family's signal
contract. A structurally signal-capable family does not automatically satisfy
the dependency aspects later derived execution would need.

Later seam-ledger surfaces then use that retained compatibility artifact in
two different ways:

- readiness uses it as one family-level seam row projection
- inspection uses it as one retained lower seam posture when the signal
  compatibility boundary was actually crossed

## How It Executes

The advanced lane executes in this order:

1. start from `ForgeQueryDeclarationSignalCompatibilityInput`
2. call `signal_compatibility(...)` or
   `signal_compatibility_checked(...)`
3. Query:
   - verifies the envelope belongs to the current admitted handle and world
   - verifies the retained envelope still represents covered crossing truth
   - reads the declaration family's signal-compatibility posture and contract
   - checks the envelope publication against the dependency aspect slice that
     later Signal-backed execution would require
   - derives one execution family and one required basis-family set from
     retained proof plus the family contract
   - checks the admitted-handle support snapshot for required capability and
     config posture
   - derives one signal-compatibility digest from retained proof and basis
     requirements
4. Query returns a compatible, deferred, denied, or failed compatibility
   artifact

The common lane preserves the same structure. It still lowers through:

1. admission
2. canonicalization
3. legality
4. progression
5. foundational evidence
6. route planning
7. receipt materialization
8. envelope construction
9. signal compatibility classification

## Small Example

```rust
use forge_query::facade::{
    ForgeQueryDeclarationSignalCompatibilityChecked,
    ForgeQueryDeclarationSignalCompatibilityInput,
};

let envelope = handle.envelope_routes_from_progressed(
    handle.declare_review_and_progress(
        geometry_session.prepare_preview_for_active_face_selection()?,
    )?,
)?;

match handle.signal_compatibility_checked(
    ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
) {
    ForgeQueryDeclarationSignalCompatibilityChecked::Compatible(compatibility) => {
        let _ = compatibility.basis_families();
        let _ = compatibility.explain();
    }
    other => panic!(
        "unexpected compatibility outcome: {:?}",
        std::mem::discriminant(&other)
    ),
}
```

This is the smallest honest example because it starts from retained envelope
truth and keeps the checked boundary visible instead of pretending signal
execution already happened.

## Real Example

```rust
let compatibility = handle
    .declare_review_progress_describe_plan_receipt_envelope_and_check_signal_compatibility(
        geometry_session.prepare_preview_for_active_face_selection()?,
    )?;

assert_eq!(
    compatibility.execution_family(),
    ForgeQueryDeclarationSignalExecutionFamily::RuntimeDerivedExecution,
);
let _ = compatibility.primary_authority_family();
let _ = compatibility.basis_families();
let _ = compatibility.signal_compatibility_digest();
let _ = compatibility.explain();
```

What this example is showing:

- the public lane still lowers through the full declaration proof chain
- the returned artifact says which Signal execution family is structurally in
  play later
- the returned artifact says which basis families later Signal execution must
  respect
- the returned artifact freezes dependency and produced-aspect posture instead
  of leaving later execution to rediscover it from envelope identity alone
- the returned digest is separate from the envelope digest because basis
  posture and signal compatibility are distinct retained facts

## Aspect-aware retrofit note

Phase 24b requires signal compatibility to speak in dependency aspects,
produced aspects, and basis-sensitive aspect requirements rather than only in
family-level compatibility posture. Query should reuse real signal aspect
semantics here so later execution and binding surfaces consume a genuine
semantic-slice compatibility artifact instead of a local approximation. In the
shipped `14` retrofit, the compatibility artifact now freezes
`aspect_contract()`, `aspect_coverage()`, `aspect_coverage_basis()`,
`aspect_fit()`, `dependency_aspects()`, and `produced_aspects()` directly off
the retained envelope-backed public slice. Support rows surface the same
contract so you can distinguish "this family is signal-aware" from "this
envelope is actually missing the dependency slice and will report an
`AuthorityAspectGap`."

That contract now also stays honest in declaration-entry inspection: when a
denied compatibility artifact did not actually prove one execution family or
basis-family set, the inspection posture leaves those accessors empty instead
of filling them from broad signal family posture.

## How It Relates To Other Features

- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md) are
  the required public input to Phase 14
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
  still owns relational truth binding; signal compatibility does not replace
  it
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
  still owns bridge continuation binding and truth-context-sensitive bridge
  request selection
- [Continuation Pipeline](./continuation-pipeline.md) carries retained signal
  posture forward when Query prepares one continuation artifact for later
  explicit execution
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
  composes this retained compatibility truth into one signal-facing next-step
  result that can stop at `Compatible` or advance into `Prepared`
- [Configured Domain Handles](./configured-domain-handles.md) still own the
  support snapshot and admitted-world identity that compatibility classification
  must not rediscover
- `basis_lifecycle` still owns basis-family vocabulary; signal compatibility
  reuses it instead of inventing a second signal context grammar

Use envelopes when you need the public crossing story. Use bridge routing when
you need one real bridge continuation binding. Use signal compatibility when
you need to freeze whether that retained declaration story can later continue
into derived execution at all.

## Inspection And Debugging

Use these surfaces when inspecting a compatibility artifact:

- `class()`
- `execution_family()`
- `primary_authority_family()`
- `basis_families()`
- `aspect_contract()`
- `aspect_coverage()`
- `aspect_coverage_basis()`
- `aspect_fit()`
- `dependency_aspects()`
- `produced_aspects()`
- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`
- `signal_compatibility_digest()`
- `envelope()`
- `route_denial_cause()`
- `receipt_denial_cause()`
- `evidence_origin()`
- `explain()`

Use `signal_compatibility_support::<I>()` when you need the family-scoped
readiness row before attempting compatibility classification.

That support row is the fastest way to answer:

- whether the family exposes any signal-compatibility contract at all
- which basis family a later derived-execution lane would require
- whether the current admitted world exposes the required Query capability
  families
- whether the current admitted world enables the `Signal` config section

## Anti-Patterns

- trying to start signal compatibility from receipts without envelope truth
- treating signal compatibility as if it already executed through
  `forge-signal`
- reconstructing basis requirements from family labels or host-local folklore
- inventing a second signal truth-context grammar instead of reusing basis
  lifecycle vocabulary
- assuming mixed-authority declarations become a fake peer signal authority
  family
- using common-lane helpers on non-signal-compatible declaration families

## Current Limits

Declaration signal compatibility now freezes retained declaration truth into
one Query-owned compatibility artifact. It still does not provide:

- actual `forge-signal` execution
- invalidation, recomputation, or scheduling semantics
- grouped declaration signal-compatibility batches
- a later unified inspection surface over all routed or compatible declaration
  artifacts
- public signal-compatibility entry from earlier declaration artifacts on the
  ordinary lane

The retained compatibility artifact tells you whether later Signal-backed
execution is structurally in play. It does not claim that derived execution
already ran.

When you want Query to reuse that retained signal posture while preparing one
explicit continuation artifact for lower bridge execution, move to
[Continuation Pipeline](./continuation-pipeline.md).

When you want one public signal-facing orchestration lane that can stop at
retained compatibility or advance into prepared continuation, move to
[Signal Compatibility Orchestration](./signal-compatibility-orchestration.md).

## Related Docs

- [Declaration Entry Inspection](./declaration-entry-inspection.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
- [Configured Domain Handles](./configured-domain-handles.md)
- [Signal Compatibility Orchestration](./signal-compatibility-orchestration.md)
- [Declaration Boundary Envelopes](./declaration-boundary-envelopes.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Boundary Receipts](./declaration-boundary-receipts.md)
- [Declaration Foundational Evidence](./declaration-foundational-evidence.md)
- [Domain Capabilities](./README.md)

# Declaration Entry Inspection

## What This Feature Is

Declaration entry inspection is the Query-owned public inspection surface over
the retained declaration-entry seam.

It lets you inspect one retained declaration crossing story through one Query
artifact instead of inspecting route plans, receipts, envelopes, relational
routing, bridge routing, and signal compatibility separately.

## Why You Use It

- inspect one retained declaration-entry seam through one Query artifact
- preserve route, receipt, envelope, relational, bridge, and signal posture
  without flattening them into one generic status
- correlate retained inspection posture with seam-ledger row digests and the
  family-level readiness report
- fail closed when retained seam artifacts belong to the wrong admitted handle
  or operating world

## Stable Entry Points

- `ForgeQueryDeclarationEntryInspectionInput`
- `ForgeQueryDeclarationEntryInspection`
- `ForgeQueryDeclarationEntryInspectionError`
- `ForgeQueryDeclarationEntryContributionEvidence`
- `ForgeQueryDeclarationEntryContributionEvidenceSet`
- `ForgeQueryDeclarationEntryContributionComposition`
- `ForgeQueryDeclarationEntryContributionCompositionError`
- `ForgeQueryDeclarationEntryInspectionRelationalPosture`
- `ForgeQueryDeclarationEntryInspectionBridgePosture`
- `ForgeQueryDeclarationEntryInspectionSignalPosture`
- `ForgeQueryAdmittedConfiguredDomainHandle::inspect_declaration_entry(...)`

## API Reference

Inspection inputs:

- `ForgeQueryDeclarationEntryInspectionInput::envelope_checked(...)`
- `ForgeQueryDeclarationEntryInspectionInput::relational_routing_checked(...)`
- `ForgeQueryDeclarationEntryInspectionInput::bridge_routing_checked(...)`
- `ForgeQueryDeclarationEntryInspectionInput::signal_compatibility_checked(...)`
- `ForgeQueryDeclarationEntryInspectionInput::with_contribution_evidence(...)`
- `ForgeQueryDeclarationEntryInspectionInput::with_admitted_plan_scope(...)`
- `ForgeQueryDeclarationEntryInspectionInput::with_lower_runtime_boundary_scope(...)`

Admitted-handle inspection:

- `inspect_declaration_entry(subject) -> Result<ForgeQueryDeclarationEntryInspection<D, I>, ForgeQueryDeclarationEntryInspectionError<D, I>>`

Inspection accessors:

- `declaration_family_key()`
- `handle_identity_digest()`
- `operating_context_identity_digest()`
- `declaration_digest()`
- `progression_digest()`
- `route_plan_digest()`
- `receipt_digest()`
- `envelope_digest()`
- `envelope_class()`
- `evidence_origin()`
- `route_denial_cause()`
- `receipt_denial_cause()`
- `route_reason()`
- `receipt_reason()`
- `relational_posture()`
- `bridge_posture()`
- `signal_posture()`
- `contribution_composition()`
- `matching_row_digests()`
- `readiness()`
- `inspection_digest()`

Contribution composition accessors:

- `evidence()`
- `contribution_digest()`
- `composed_category_families()`
- `rejected_category_families()`

## Core Mental Model

Inspection is a projection over retained seam truth.

That means:

1. the envelope still owns the retained public crossing story
2. relational, bridge, and signal artifacts still own their own lower seam
   posture
3. inspection composes those retained truths into one readable Query artifact
4. inspection does not re-run lower routing or invent fake success posture
5. optional `9.3.7` contribution evidence may be attached on demand, but only
   when the retained seam subject can honestly justify that contribution target
   and category
6. stronger admitted-plan-bound and lower-runtime-bound contribution categories
   require matching retained downstream proof explicitly; they are not inferred
   from declaration-entry posture alone
7. the narrower relational, bridge, and signal support helpers remain
   entry-phase-only projections; they do not become contribution-composed
   inspection surfaces just because contribution evidence is attached here

If a stronger lower seam artifact is missing, inspection keeps that absence
honest and falls back to readiness posture for that layer.

Inspection is about retained declaration-entry seam artifacts such as
envelopes, relational routing, bridge routing, and signal compatibility.
It is not the read surface for orchestration transcripts. Proof-visible
orchestration transcripts explain one orchestration run; inspection explains
retained seam artifacts after the run.

## Small Example

```rust
let checked = handle.signal_compatibility_checked(
    ForgeQueryDeclarationSignalCompatibilityInput::enveloped(envelope),
);

let inspection = handle.inspect_declaration_entry(
    ForgeQueryDeclarationEntryInspectionInput::signal_compatibility_checked(checked)
        .with_contribution_evidence(evidence),
)?;

let _ = inspection.signal_posture();
let _ = inspection.contribution_composition();
let _ = inspection.matching_row_digests();
let _ = inspection.readiness();
```

## Real Example

```rust
let envelope = handle.envelope_routes_from_progressed(
    handle.declare_review_and_progress(AttachFaceMaterialAssignment {
        face_ref: "face:outer-wall-a",
        material_ref: "material:fire-rated-plaster",
    })?,
)?;

let evidence = ForgeQueryDeclarationEntryContributionEvidenceSet::new(vec![
    ForgeQueryDeclarationEntryContributionEvidence::from(domain_support_contribution),
    ForgeQueryDeclarationEntryContributionEvidence::from(domain_explanation_contribution),
]);

let inspection = handle.inspect_declaration_entry(
    ForgeQueryDeclarationEntryInspectionInput::envelope_checked(
        ForgeQueryDeclarationEnvelopeChecked::Enveloped(envelope),
    )
    .with_admitted_plan_scope(admitted_plan)
    .with_contribution_evidence(evidence),
)?;

let _ = inspection.contribution_composition();
let _ = inspection.readiness();
```

## Inspection And Debugging

Use inspection when you need to answer:

- what declaration family crossed this seam?
- which admitted world did it belong to?
- what route and receipt posture was retained?
- did the seam reach relational, bridge, or signal posture?
- which declaration-scoped domain contribution evidence was explicitly attached?
- whether attached admitted-plan or lower-runtime proof actually justified the
  stronger contribution target families that were supplied
- which seam-ledger rows explain the retained crossing?

## Anti-Patterns

- do not build declaration-entry inspection from raw declarations
- do not treat inspection as a lower-runtime execution surface
- do not infer lower-authority success from readiness alone when there is no
  retained lower seam artifact
- do not pass orchestration transcripts as though they were retained seam
  subjects
- do not attach workflow, continuity, or aftermath contribution evidence unless
  the retained seam subject is strong enough to justify those targets
- do not assume admitted-plan-bound or lower-runtime-bound evidence will compose
  unless you attach matching retained downstream proof explicitly

## Current Limits

- this surface inspects retained declaration-entry seam artifacts only
- contribution composition is on-demand; inspection does not discover domain
  contributions by itself
- declaration-bound admission, support, and explanation evidence compose
  directly
- declaration-bound workflow evidence requires matching retained admitted-plan
  scope
- admitted-plan-bound evidence composes only when matching retained admitted-plan
  proof is attached
- lower-runtime-bound explanation and aftermath evidence compose only when
  matching retained lower-runtime boundary proof is attached
- it does not execute later orchestration or recovery
- it keeps non-success lower routing posture visible, but it is still a read
  surface rather than a repair workflow

## Related Docs

- [Declaration Entry Orchestration](./declaration-entry-orchestration.md)
- [Declaration Entry Readiness](./declaration-entry-readiness.md)
- [Declaration Signal Compatibility](./declaration-signal-compatibility.md)
- [Declaration Bridge Continuation Routing](./declaration-bridge-continuation-routing.md)
- [Declaration Relational Truth Routing](./declaration-relational-truth-routing.md)

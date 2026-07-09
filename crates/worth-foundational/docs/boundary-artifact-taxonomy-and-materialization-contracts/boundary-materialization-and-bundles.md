# Boundary Materialization And Bundles

## What This Feature Is

This feature is the planning and materialization lane for Milestone 4 boundary
artifacts.

It gives you:

- explicit materialization source and seam
- delivery-class and availability law
- attachment inclusion and elision decisions
- structured decision rows
- explicit plan and materialize steps
- typed multi-output bundles

This is the part of Milestone 4 that makes boundary emission look expensive and
inspectable instead of like a cheap accessor.

The source and seam enums are part of the contract:

- sources: `NativeAuthority`, `CompatibilityLowered`, `DerivedSupport`
- seams: `BoundaryExchange`, `SupportMaterialization`, `PersistenceExport`

The delivery and availability enums are part of the contract too:

- delivery: `MustBeHot`, `CanDefer`, `ReconstructableFromRetainedBasis`
- availability: `Present`, `Deferred`, `Reconstructable`, `Unavailable`

## Why You Use It

Use this surface when you need to:

- plan a boundary output before materializing it
- keep seam, cost, delivery, availability, and attachment decisions visible
- materialize descriptive or authoritative surfaces honestly
- emit a typed bundle of primary artifact plus summary/report/receipt members

This is the right lane when the question becomes "how do I get a real boundary
output out of this claim, and what did that cost or omit?"

## Stable Entry Points

- `plan_descriptive_boundary_materialization(...)`
- `plan_authoritative_boundary_materialization(...)`
- `materialize_descriptive_boundary_surface(...)`
- `materialize_authoritative_boundary_surface(...)`
- `plan_artifact_boundary_bundle(...)`
- `FoundationalBoundaryMaterializationInput`
- `FoundationalBoundaryMaterializationPlan`
- `FoundationalMaterializedBoundaryArtifact`
- `FoundationalBoundaryMaterializationBundlePlan`
- `FoundationalBoundaryMaterializationBundle`
- `FoundationalBoundaryMaterializationSource`
- `FoundationalBoundaryMaterializationSeam`
- `FoundationalBoundaryDeliveryClass`
- `FoundationalBoundaryAvailability`
- `FoundationalBoundaryAttachmentPoint`
- `FoundationalBoundaryMaterializationDecisionRow`
- `FoundationalBoundaryMaterializationCost`

## Core Mental Model

Materialization is a boundary crossing, not a view.

The plan tells you:

- what category and role are being emitted
- where the output is going
- whether it must be hot, can defer, or is reconstructable
- whether the surface is present, deferred, reconstructable, or unavailable
- which attachments were included or elided
- what decision rows explain those outcomes
- what the cost looks like before emission

The decision rows also carry typed reasons:

- subjects like `CategoryRoleAdmission`, `DeliveryAvailabilityResolution`,
  `AttachmentInclusion`, `AttachmentElision`, and `BundleMembership`
- causes like `RequestedAsAdmitted`, `NarrowedByAuthority`,
  `DeferredBySupportPosture`, `ReconstructableFromRetainedBasis`, and
  `DeniedByMilestoneBoundary`

Bundles are also typed and honest. They are not arbitrary result bags. A bundle
has one primary artifact plan and then optional summary, report, or receipt
members that must match seam, source, and profile.

## How It Executes

For a single boundary output:

1. start from a role-bearing claim
2. choose materialization source and seam
3. attach the profile
4. plan the materialization
5. inspect disposition, attachments, decision rows, and cost
6. materialize

For a bundle:

1. plan an artifact-category primary output
2. attach legal summary/report/receipt member plans
3. inspect membership rows and aggregated cost
4. materialize the typed bundle

## Small Example

```rust
use worth_foundational::{
    claim_derived_projection_boundary_surface, plan_descriptive_boundary_materialization,
    FoundationalBoundaryArtifactSurface, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource,
};

let plan = plan_descriptive_boundary_materialization(
    claim_derived_projection_boundary_surface(
        FoundationalBoundaryArtifactSurface::new(vec![1_u8, 2, 3], 2),
    ),
    FoundationalBoundaryMaterializationSource::CompatibilityLowered,
    FoundationalBoundaryMaterializationSeam::BoundaryExchange,
    profile,
)?;
```

## Real Example

Use a bundle plan when one operation honestly emits multiple coordinated
surfaces:

```rust
use worth_foundational::{
    claim_derived_projection_boundary_surface, claim_receipt_evidence_boundary_surface,
    claim_support_only_boundary_surface, plan_artifact_boundary_bundle,
    plan_descriptive_boundary_materialization, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryMaterializationSeam, FoundationalBoundaryMaterializationSource,
    FoundationalBoundaryReceiptSurface, FoundationalBoundaryReportSurface,
    FoundationalBoundarySummarySurface,
};

let primary = plan_descriptive_boundary_materialization(
    claim_derived_projection_boundary_surface(
        FoundationalBoundaryArtifactSurface::new(vec![7_u8, 8, 9], 2),
    ),
    FoundationalBoundaryMaterializationSource::CompatibilityLowered,
    FoundationalBoundaryMaterializationSeam::BoundaryExchange,
    profile.clone(),
)?;

let summary = plan_descriptive_boundary_materialization(
    claim_derived_projection_boundary_surface(
        FoundationalBoundarySummarySurface::new("summary", 2)?,
    ),
    FoundationalBoundaryMaterializationSource::CompatibilityLowered,
    FoundationalBoundaryMaterializationSeam::BoundaryExchange,
    profile.clone(),
)?;

let report = plan_descriptive_boundary_materialization(
    claim_support_only_boundary_surface(
        FoundationalBoundaryReportSurface::new(vec!["row-a", "row-b"], 2)?,
    ),
    FoundationalBoundaryMaterializationSource::CompatibilityLowered,
    FoundationalBoundaryMaterializationSeam::BoundaryExchange,
    profile.clone(),
)?;

let receipt = plan_descriptive_boundary_materialization(
    claim_receipt_evidence_boundary_surface(
        FoundationalBoundaryReceiptSurface::new("exchange complete", 1)?,
    ),
    FoundationalBoundaryMaterializationSource::CompatibilityLowered,
    FoundationalBoundaryMaterializationSeam::BoundaryExchange,
    profile,
)?;

let bundle = plan_artifact_boundary_bundle(primary)
    .with_summary(summary)?
    .with_report(report)?
    .with_receipt(receipt)?
    .materialize()?;
```

## How It Relates To Other Features

- [Boundary Roles And Authority Admission](./boundary-roles-and-authority-admission.md)
  provides the claims this lane consumes.
- [Boundary Canonical Basis And Current-Basis](./boundary-canonical-basis-and-current-basis.md)
  lowers the materialized outputs and bundles into canonical and current-basis
  lanes.

## Inspection And Debugging

Check these first:

- `source()`
- `seam()`
- `delivery_class()`
- `availability()`
- `attachments()`
- `decision_rows()`
- `cost()`

For bundles, also inspect:

- `primary()`
- `summary()`
- `report()`
- `receipt()`
- `membership_decision_rows()`
- `member_count()`

If something fails, the common causes are:

- an illegal delivery/availability combination
- a source/seam mismatch
- a bundle member with a different profile, source, or seam
- a surface that is deferred and therefore not materializable in the current
  lane

If support materialization is involved, check whether the selected posture is
forcing a `Deferred` availability instead of a present materialized surface.

## Anti-Patterns

- Do not call materialization a getter or view.
- Do not hide attachment elision or milestone-boundary denial in prose-only
  logs.
- Do not emit multi-surface outputs as an ad hoc result bag.
- Do not ignore decision rows and then guess why a surface is missing.

## Current Limits

- Materialization plans boundary outputs, not branch/merge/commit transitions.
- This lane can describe deferred or reconstructable surfaces explicitly, but
  it does not define the later diagnostics ontology for those surfaces.
- This lane keeps attachment points visible, but it does not define the full
  later diagnostics, provenance, or performance ontologies that may attach
  there.

## Related Docs

- [Boundary Roles And Authority Admission](./boundary-roles-and-authority-admission.md)
- [Boundary Canonical Basis And Current-Basis](./boundary-canonical-basis-and-current-basis.md)

# Boundary Roles And Authority Admission

## What This Feature Is

This feature gives boundary surfaces a claim about what they mean, not just
what category they are.

The shipped roles are:

- `AuthoritativeCurrent`
- `DerivedProjection`
- `SupportOnly`
- `PlannedWork`
- `ReceiptEvidence`

This is also where Milestone 4 introduces the stronger proof-bearing admission
lane for authoritative-current claims.

## Why You Use It

Use this surface when you need to:

- say whether a boundary output is authoritative, derived, support-only,
  planned, or receipt-bearing
- reject illegal category-role combinations mechanically
- make stronger authoritative-current claims visibly stronger than ordinary
  descriptive claims

This is the right lane when the next question after category is "what kind of
claim is this surface making?"

## Stable Entry Points

- `FoundationalBoundaryArtifactRole`
- `FoundationalBoundaryRoleDefinition`
- `FoundationalBoundaryRoleClaim<Surface, Role>`
- `evaluate_boundary_role_claim_legality(...)`
- `claim_derived_projection_boundary_surface(...)`
- `claim_support_only_boundary_surface(...)`
- `claim_planned_work_boundary_surface(...)`
- `claim_receipt_evidence_boundary_surface(...)`
- `admit_authoritative_current_boundary_surface(...)`
- `foundational_boundary_authority_admission()`
- `FoundationalAuthoritativeBoundaryClaim`

## Core Mental Model

Category and role answer different questions:

- category says what kind of boundary output this is
- role says what kind of claim the boundary output is making

That is why not every category-role combination is legal.

For example:

- `ReceiptEvidence` must use the `Receipt` category
- `AuthoritativeCurrent` must use the `Artifact` category
- descriptive roles may use summary, report, or artifact surfaces

Authoritative-current is stronger than the other roles. It is not just an enum
value. It requires an explicit authority admission lane.

## How It Executes

For descriptive roles:

1. start from a category-specific surface
2. choose a legal descriptive role
3. claim that role through the role-specific helper

For authoritative-current:

1. start from an artifact-category surface
2. obtain the crate-owned authority witness
3. admit the authoritative claim explicitly

## Small Example

```rust
use worth_foundational::{
    claim_support_only_boundary_surface, FoundationalBoundaryReportSurface,
};

let support_claim = claim_support_only_boundary_surface(
    FoundationalBoundaryReportSurface::new(vec!["retained row"], 1)?,
);
```

## Real Example

Use the stronger authority lane when the boundary output is claiming current
authoritative truth:

```rust
use worth_foundational::{
    admit_authoritative_current_boundary_surface,
    foundational_boundary_authority_admission, FoundationalBoundaryArtifactSurface,
};

let admitted = admit_authoritative_current_boundary_surface(
    FoundationalBoundaryArtifactSurface::new(vec!["committed"], 3),
    foundational_boundary_authority_admission(),
);

let _ = (admitted.role(), admitted.category(), admitted.role_definition());
```

## How It Relates To Other Features

- [Boundary Categories](./boundary-categories.md) supplies the category-specific
  surfaces this lane consumes.
- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
  plans and materializes role-bearing boundary claims.

## Inspection And Debugging

Check these first:

- `role()`
- `role_definition()`
- `category()`
- `evaluate_boundary_role_claim_legality(...)`

If a claim is rejected, the usual reasons are:

- a receipt-only role was used on a non-receipt category
- an authoritative-current role was used on a summary, report, or receipt
- a receipt category tried to act like a descriptive projection

## Anti-Patterns

- Do not treat role as a documentation-only label.
- Do not claim `AuthoritativeCurrent` on a summary or report.
- Do not use receipt evidence to describe work that has not completed.
- Do not invent a second authority lane for boundary artifacts outside the
  shipped admission helper.

## Current Limits

- Role claims do not yet plan or materialize anything by themselves.
- This feature does not define branch, merge, or commit transitions. That is
  Milestone 5.

## Related Docs

- [Boundary Categories](./boundary-categories.md)
- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)

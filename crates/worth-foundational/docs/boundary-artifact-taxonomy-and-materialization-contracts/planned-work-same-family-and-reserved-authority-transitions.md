# Planned Work, Same-Family Outputs, And Reserved Authority Transitions

## What This Feature Is

This feature gives Milestone 4 explicit descriptive room without pretending it
already owns Milestone 5 transition semantics.

It covers:

- planned-work boundary artifacts
- same-family descriptive boundary artifacts
- same-family identity derivation
- fail-closed reserved authority-transition denials for branch, merge, and
  commit claims

The reserved transition kinds are named explicitly:

- `Branch`
- `Merge`
- `Commit`

## Why You Use It

Use this surface when you need to:

- preserve that a materialized boundary output is still planned work
- group descriptive outputs into a named family and derive a stable family
  identity
- reject attempts to turn planned or same-family surfaces into branch, merge,
  or commit authority transitions

This is the right lane when you need structured descriptive extension room, but
you are still not crossing into Milestone 5 authority-transition law.

## Stable Entry Points

- `admit_planned_work_boundary_artifact(...)`
- `FoundationalPlannedWorkBoundaryArtifact`
- `FoundationalPlannedWorkBoundaryArtifactDenial`
- `admit_same_family_boundary_artifact(...)`
- `FoundationalSameFamilyBoundaryArtifact`
- `FoundationalSameFamilyBoundaryFamily`
- `FoundationalSameFamilyBoundaryIdentity`
- `prepare_same_family_boundary_artifact_for_canonical_basis(...)`
- `derive_same_family_boundary_identity(...)`
- `evaluate_planned_work_reserved_authority_transition_legality(...)`
- `evaluate_same_family_reserved_authority_transition_legality(...)`
- `FoundationalReservedAuthorityTransitionKind`
- `FoundationalReservedAuthorityTransitionDenial`

## Core Mental Model

Planned-work and same-family surfaces are still descriptive.

They are useful because they give later work a stable place to attach:

- planned work says "this is intended or staged descriptive output"
- same-family says "these descriptive outputs belong to one named family"

But neither one is allowed to become a hidden transition engine.

That is why reserved authority-transition checks are fail closed here. Branch,
merge, and commit authority-transition meaning is intentionally deferred to
Milestone 5.

## How It Executes

For planned work:

1. start from a materialized boundary artifact
2. require that its role is `PlannedWork`
3. admit it into the planned-work wrapper

For same-family:

1. start from a descriptive materialized boundary artifact
2. choose a valid same-family name
3. admit the same-family wrapper
4. derive a same-family identity when canonical parity matters

Same-family admission is intentionally limited to descriptive roles:

- `DerivedProjection`
- `SupportOnly`
- `PlannedWork`

It rejects:

- `AuthoritativeCurrent`
- `ReceiptEvidence`

For reserved authority transitions:

1. start from a planned-work or same-family wrapper
2. evaluate a branch, merge, or commit transition attempt
3. receive a fail-closed denial

## Small Example

```rust
use worth_foundational::admit_planned_work_boundary_artifact;

let planned = admit_planned_work_boundary_artifact(materialized_planned_artifact)?;
```

## Real Example

Use same-family identity when you need descriptive outputs from independent
producers to agree on family-scoped identity:

```rust
use worth_foundational::{
    admit_same_family_boundary_artifact, derive_same_family_boundary_identity,
    FoundationalSameFamilyBoundaryFamily,
};

let same_family = admit_same_family_boundary_artifact(
    materialized_descriptive_artifact,
    FoundationalSameFamilyBoundaryFamily::new("compatibility-lowered")?,
)?;

let identity = derive_same_family_boundary_identity(version, &same_family)?;

let _ = (
    identity.family(),
    identity.basis(),
    identity.digest(),
);
```

## How It Relates To Other Features

- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
  produces the descriptive artifacts this lane wraps.
- [Boundary Production Readiness](./boundary-production-readiness.md) freezes
  the fact that reserved authority transitions remain fail closed in Milestone
  4.

## Inspection And Debugging

Check these first:

- `role()` on the materialized artifact before admission
- `family()` on same-family artifacts and identities
- the reserved transition kind that was attempted
- the exact denial returned

If same-family admission fails, the common causes are:

- the family name was blank
- the family name contained whitespace
- the artifact role was authoritative or receipt-bearing instead of descriptive
- the caller was trying to use same-family as a disguised authority lane

## Anti-Patterns

- Do not use planned-work wrappers as fake receipts.
- Do not use same-family wrappers to sneak in branch, merge, or commit meaning.
- Do not treat family names as informal labels if you need canonical identity.
- Do not assume descriptive wrappers are harmless just because they are not
  authoritative. They still carry real boundary meaning.

## Current Limits

- This feature is descriptive by design.
- It does not implement real transition ontology.
- It does not define diagnostics or provenance families beyond same-family
  descriptive identity.

## Related Docs

- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
- [Boundary Production Readiness](./boundary-production-readiness.md)

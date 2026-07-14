# Boundary Canonical Basis And Current-Basis

## What This Feature Is

This feature lowers materialized boundary artifacts and bundles through the
canonical basis lane and then strengthens certain outputs into a proof-bearing
current-basis lane.

It is the part of Milestone 4 that keeps semantic identity and stronger
boundary freshness separate but connected.

## Why You Use It

Use this surface when you need to:

- compare or export materialized boundary outputs on a canonical surface
- prove that independent producers lower to the same boundary meaning
- strengthen a materialized artifact or bundle into a current-basis artifact
- bridge a trust boundary and explicitly readmit the stronger claim later

This is the right lane when the question becomes "what does this boundary
surface mean canonically?" or "does this stronger boundary claim still carry a
live basis?"

## Stable Entry Points

Canonical basis:

- `prepare_materialized_boundary_artifact_for_canonical_basis(...)`
- `prepare_materialized_boundary_bundle_for_canonical_basis(...)`
- `foundational_boundary_canonical_basis_entries(...)`

Current-basis:

- `admit_current_basis_boundary_artifact(...)`
- `admit_current_basis_boundary_bundle(...)`
- `bridge_current_basis_boundary_artifact_trust_boundary(...)`
- `bridge_current_basis_boundary_bundle_trust_boundary(...)`
- `readmit_current_basis_boundary_artifact_after_boundary(...)`
- `readmit_current_basis_boundary_bundle_after_boundary(...)`
- `foundational_boundary_current_basis_authority()`
- `foundational_boundary_current_basis_readmission_authority()`
- `foundational_boundary_current_basis_proof_lane()`

The shipped proof lane name is:

- `CurrentBasisArtifactWithBoundaryReadmission`

## Core Mental Model

Canonical basis and current-basis are different promises.

Canonical basis means:

- this is the semantic boundary meaning
- independent producers should compare the same way
- materialization cost and local payload layout do not define identity

Current-basis means:

- this materialized boundary output is carrying a stronger live basis claim
- that stronger claim is proof-bearing
- trust-boundary crossing weakens that claim until explicit readmission happens

Milestone 4 deliberately reuses Milestone 2 and `worth-proof` here. It does not
invent a second canonicalization or proof substrate.

## How It Executes

Canonical flow:

1. start from a materialized artifact or bundle
2. prepare it for canonical basis
3. consume the canonical-ready artifact downstream

Current-basis flow:

1. start from a materialized artifact or bundle
2. admit it into current-basis with the crate-owned authority witness
3. bridge a trust boundary if it leaves the original basis context
4. readmit it explicitly later

Both artifact and bundle admission use a typed `TransitionOutcome`, so denial
at this layer is still structured rather than a local error string.

## Small Example

```rust
use worth_foundational::{
    admit_current_basis_boundary_artifact,
    foundational_boundary_current_basis_authority,
};

let current_basis = admit_current_basis_boundary_artifact(
    version,
    materialized_artifact,
    foundational_boundary_current_basis_authority(),
)?;
```

## Real Example

Use the trust-boundary lane when a stronger boundary bundle leaves its original
context:

```rust
use worth_foundational::{
    bridge_current_basis_boundary_bundle_trust_boundary,
    foundational_boundary_current_basis_readmission_authority,
    readmit_current_basis_boundary_bundle_after_boundary,
};

let bridged = bridge_current_basis_boundary_bundle_trust_boundary(current_basis_bundle);

let readmitted = readmit_current_basis_boundary_bundle_after_boundary(
    bridged,
    rebound_basis,
    foundational_boundary_current_basis_readmission_authority(),
);

let bundle = readmitted.bundle();
let basis = readmitted.strong_basis();
let _ = (bundle, basis);
```

## How It Relates To Other Features

- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
  provides the materialized outputs this lane consumes.
- [Boundary Production Readiness](./boundary-production-readiness.md) freezes
  the exact proof-lane choices and closure assumptions for this feature.

## Inspection And Debugging

Check these first:

- the canonical preparation outcome
- `strong_basis()` on current-basis artifacts or bundles
- `proofs()` on the strengthened surfaces
- `foundational_boundary_current_basis_proof_lane()`
- whether the surface was bridged and not yet readmitted
- whether the correct authority or readmission authority was used

If a surface still looks plain after a trust-boundary crossing, it has not been
readmitted into the stronger lane yet.

## Anti-Patterns

- Do not use materialization cost as part of semantic identity.
- Do not preserve current-basis strength implicitly across a trust boundary.
- Do not build a local pseudo-current-basis wrapper around raw materialized
  artifacts.
- Do not confuse canonical parity with stronger freshness or authority.

## Current Limits

- This feature strengthens materialized boundary artifacts and bundles. It does
  not standardize branch/merge/commit current-basis behavior. That comes later.
- It does not replace Milestone 2 canonicalization. It reuses it.

## Related Docs

- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
- [Boundary Production Readiness](./boundary-production-readiness.md)

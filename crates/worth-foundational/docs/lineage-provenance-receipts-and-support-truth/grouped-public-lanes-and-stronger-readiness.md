# Grouped Public Lanes And Stronger Readiness

## What This Feature Is

This feature is the DX-hardened public API for Milestone 7. It gives you one
common path, one lower lane, and one stronger lane so engineers can find the
right boundary-evidence surface without reading private module topology or test
files first.

## Why You Use It

- Use this when you want the supported first-contact API for Milestone 7.
- Use this when you need direct lower-lane vocabulary for inspection or exact
  construction.
- Use this when you need stronger readmission or readiness artifacts and want
  those stronger boundaries to stay obvious.

## Stable Entry Points

- `worth_foundational::boundary_evidence_api`
- `worth_foundational::boundary_evidence_api::common_path`
- `worth_foundational::boundary_evidence_api::lower_lane::primitives`
- `worth_foundational::boundary_evidence_api::lower_lane::provenance`
- `worth_foundational::boundary_evidence_api::lower_lane::receipts`
- `worth_foundational::boundary_evidence_api::lower_lane::lineage`
- `worth_foundational::boundary_evidence_api::lower_lane::support`
- `worth_foundational::boundary_evidence_api::lower_lane::attachments`
- `worth_foundational::boundary_evidence_api::stronger_lane`
- `worth_foundational::boundary_evidence_api::stronger_lane::readmission`
- `worth_foundational::boundary_evidence_api::stronger_lane::readiness`
- `worth_foundational::boundary_evidence_api::boundary_evidence_public_surface_inventory()`

## Core Mental Model

The grouped public surface teaches three lanes:

- common path: the recommended descriptive entry path
- lower lane: direct vocabulary and exact lower-level surfaces
- stronger lane: trust-boundary readmission and proof-bearing readiness

The root `boundary_evidence_api` module also ships the exact public-surface
inventory. Use that when you need the machine-checkable list of lanes rather
than relying on folder browsing or doc memory.

The grouped surface exists so the public API itself teaches the difference.

## How It Executes

1. start on `common_path` for ordinary lineage, provenance, receipt, support,
   and attachment work
2. drop to `lower_lane` when you need exact lower-level vocabulary or direct
   canonical/digest participation work
3. enter `stronger_lane` only when a trust boundary or readiness proof really
   exists

## Small Example

```rust
use worth_foundational::boundary_evidence_api::common_path;

let evidence = common_path::boundary_evidence();
let _ = evidence;
```

This is the smallest honest example because it shows the real first-contact
surface instead of a private module path.

## Real Example

```rust
use worth_foundational::boundary_evidence_api::{
    common_path, lower_lane, stronger_lane,
};

let provenance = common_path::provenance()
    .historical(source_basis)
    .with_freshness(freshness)?;

let receipt = common_path::receipt()
    .execution(boundary)
    .with_provenance(provenance);

let support_kinds =
    lower_lane::support::foundational_boundary_evidence_support_truth_kind_definitions();

let readiness =
    stronger_lane::readiness::certify_foundational_boundary_evidence_milestone7_production_test_readiness();
```

## How It Relates To Other Features

- [Boundary Evidence Production Readiness](./boundary-evidence-production-readiness.md)
  explains the stronger readiness artifact itself.
- Every other doc in this folder explains one capability seam that this grouped
  public surface exposes.

## Inspection And Debugging

- If you are not sure where to start, start on `common_path`.
- If you only need the definition lists or exact lower-level types, use
  `lower_lane`.
- If a surface talks about trust-boundary readmission or readiness
  certification, it should be in `stronger_lane`.

## Anti-Patterns

- Jumping straight into private modules when the grouped public surface already
  exposes the seam.
- Treating lower-lane inspection APIs as if they were the common path.
- Treating common-path descriptive artifacts as if they had already been
  readmitted or certified.

## Current Limits

- The grouped public surface does not remove lower-lane meaning.
- The stronger lane does not replace readmission authorities or proof-bearing
  artifacts.
- Runtime-local adoption docs still belong to the adopting crate.

## Related Docs

- [Boundary Evidence Production Readiness](./boundary-evidence-production-readiness.md)
- [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)

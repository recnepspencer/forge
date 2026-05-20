# Attachment Materialization, Canonical Participation, And Readmission

## What This Feature Is

This feature is the attachment and materialization surface for Milestone 7. It
lets you attach lineage, provenance, receipts, support truth, and diagnostics
to real boundary-facing targets, then materialize, canonicalize, digest, and
readmit those bundles honestly.

## Why You Use It

- Use this when descriptive evidence needs to travel with a boundary artifact,
  transition, or diagnostics bundle.
- Use this when object-level and locator-level continuity must stay distinct.
- Use this when attached evidence has to participate in canonical basis,
  digest identity, or stronger readmission lanes.

## Stable Entry Points

- `boundary_evidence().attachment()`
- `forge_foundational::boundary_evidence_api::common_path::attachment()`
- `forge_foundational::boundary_evidence_api::lower_lane::attachments`
- `forge_foundational::boundary_evidence_api::stronger_lane::readmission`

Important artifact families:

- `FoundationalMaterializedBoundaryEvidenceAttachmentBundle`
- `CurrentBasisBoundaryEvidenceAttachmentBundle`
- `SupportBasisBoundaryEvidenceAttachmentBundle`

Important stronger-lane readmission functions:

- `admit_current_basis_boundary_evidence_attachment_bundle(...)`
- `bridge_current_basis_boundary_evidence_attachment_bundle_trust_boundary(...)`
- `readmit_current_basis_boundary_evidence_attachment_bundle_after_boundary(...)`
- `admit_support_basis_boundary_evidence_attachment_bundle(...)`
- `bridge_support_basis_boundary_evidence_attachment_bundle_trust_boundary(...)`
- `readmit_support_basis_boundary_evidence_attachment_bundle_after_boundary(...)`

## Core Mental Model

Attachment is where the descriptive families finally travel together.

The important rules are:

- target kind matters
- object-level and locator-level continuity are not interchangeable
- materialization profile can elide optional richness, but not rewrite
  authority truth
- current-basis and support-basis readmission are stronger lanes and stay
  visibly stronger

## How It Executes

1. choose the target kind
2. attach the allowed descriptive families
3. materialize under a richness profile
4. lower the materialized bundle into canonical basis or digest participation
   when needed
5. enter current-basis or support-basis readmission only through the stronger
   lane

## Small Example

```rust
use forge_foundational::boundary_evidence;

let attachment = boundary_evidence().attachment().for_boundary_artifact(locator);
```

This is the smallest honest example because the target choice is what begins
the attachment story.

## Real Example

```rust
use forge_foundational::{
    boundary_evidence_api::{common_path, stronger_lane},
    FoundationalBoundaryEvidenceMaterializationProfile,
};

let materialized = common_path::attachment()
    .for_boundary_artifact(locator)
    .with_attested_continuity(lineage)
    .with_provenance_attachment(provenance)
    .with_receipt_attachment(receipt.completed_receipt().clone())
    .with_published_support(support)
    .materialize_under(FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness);

let admitted =
    stronger_lane::readmission::admit_current_basis_boundary_evidence_attachment_bundle(
        materialized,
        stronger_lane::readmission::foundational_boundary_evidence_attachment_readmission_authority(),
    );
```

What is authoritative here is the target shape and the stronger readmission
bridge. What is derived is the materialized descriptive bundle.

When support truth is the stronger reason for reentry, the stronger lane has a
separate support-basis path. That keeps "this bundle carries support" distinct
from "this bundle is current-basis admissible right now."

```rust
let support_admitted =
    stronger_lane::readmission::admit_support_basis_boundary_evidence_attachment_bundle(
        materialized,
        stronger_lane::readmission::foundational_boundary_evidence_attachment_readmission_authority(),
    );
```

## How It Relates To Other Features

- [Lineage, Continuity, Divergence, And Promotion](./lineage-continuity-divergence-and-promotion.md)
  explains what the continuity attachments actually mean.
- [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)
  shows how the common, lower, and stronger lanes divide this work.

## Inspection And Debugging

- Check target kind first.
- Check continuity scope next.
- Check the materialization profile when optional evidence seems to be missing.
- Check whether the bundle is materialized, current-basis admitted, or
  support-basis admitted before assuming readmission is legal.

## Anti-Patterns

- Letting locator-level continuity impersonate object-level continuity.
- Reading a raw materialized bundle as if it had already crossed the stronger
  readmission boundary.
- Treating support-basis readmission as interchangeable with current-basis
  readmission.
- Treating canonical or digest participation as if it made the bundle stronger
  by itself.

## Current Limits

- This feature does not replace boundary artifacts or canonicalization.
- It does not let bundles bypass trust-boundary bridges.
- It does not force one diagnostics bundle shape on adopting crates.

## Related Docs

- [Lineage, Continuity, Divergence, And Promotion](./lineage-continuity-divergence-and-promotion.md)
- [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)

# Lineage, Continuity, Divergence, And Promotion

## What This Feature Is

This feature is the shared continuity vocabulary for Milestone 7. It lets WORTH
crates describe survival, replay-derived continuity, restored continuity,
reconstructed equivalence, branch-local divergence, promotion, ambiguity,
partial lineage, and breaks without collapsing them into one result bag.

## Why You Use It

- Use this when you need to answer "did this identity survive?"
- Use this when branch-local replacement and globally admitted continuity must
  stay visibly distinct.
- Use this when replay-derived, restored, reconstructed, or partial lineage
  needs an explicit home.

## Stable Entry Points

- `boundary_evidence().lineage()`
- `worth_foundational::boundary_evidence_api::common_path::lineage()`
- `worth_foundational::boundary_evidence_api::lower_lane::lineage`

Important artifact families include:

- attested continuity
- branch-local replacement
- promoted continuity
- replay-derived continuity
- restored continuity
- reconstructed equivalence
- partial lineage

## Core Mental Model

Lineage answers "what continuity claim is being made?"

It is stronger than provenance, because provenance only explains basis.
It is different from receipts, because receipts only explain completed
boundaries.

The line that matters most is this:

- replay-derived or reconstructed continuity is not the same thing as directly
  attested continuity

Related event-history or replay records may help explain why a lineage artifact
exists, but they are not themselves the lineage claim.

## How It Executes

1. choose the lineage family
2. bind the subject or subject set
3. attach executed-boundary and provenance evidence when required
4. make branch-divergence, promotion, or partiality posture explicit
5. materialize the lineage artifact

## Small Example

```rust
use worth_foundational::{boundary_evidence, FoundationalBoundaryEvidenceLineageSubject};

let subject = FoundationalBoundaryEvidenceLineageSubject::new(handle);
let lineage = boundary_evidence().lineage().continuity(subject);
```

This is the smallest honest example because it starts with a continuity claim
without pretending the stronger evidence has already been attached.

## Real Example

```rust
use worth_foundational::{
    boundary_evidence_api::common_path as evidence,
    FoundationalBoundaryEvidenceLineageSubject,
};

let attested = evidence::lineage()
    .continuity(FoundationalBoundaryEvidenceLineageSubject::new(handle))
    .attested_by(receipt);
```

Promotion is intentionally a different step:

```rust
let promoted = evidence::lineage()
    .branch_local(subject)
    .with_branch_divergence(divergence_posture)
    .promoted_by(receipt);
```

## How It Relates To Other Features

- [Receipts And Closeout Truth](./receipts-and-closeout-truth.md) supplies the
  executed-boundary evidence stronger lineage uses.
- [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)
  covers how lineage attaches to artifacts and diagnostics bundles.

## Inspection And Debugging

- Inspect the lineage family before reading anything else.
- Inspect promotion posture when branch-local truth looks global.
- Inspect partiality posture when a lineage artifact looks suspiciously
  complete.

## Anti-Patterns

- Upgrading replay-derived continuity into attested continuity.
- Treating reconstructed equivalence as direct restored continuity.
- Letting denied promotion materialize as global continuity.

## Current Limits

- This feature does not choose one merge engine or identity-resolution engine.
- It does not make producer-private event histories public API.
- Runtime-specific correspondence heuristics stay outside the milestone.

## Related Docs

- [Receipts And Closeout Truth](./receipts-and-closeout-truth.md)
- [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)

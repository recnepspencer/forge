# Provenance Layering And Freshness

## What This Feature Is

This feature is the typed provenance surface for Milestone 7. It tells you the
basis and context behind a result without pretending that provenance itself is
the proof.

## Why You Use It

- Use this when you need to say where a conclusion came from.
- Use this when freshness, replay, snapshot, or support-context posture must be
  explicit.
- Use this when blind consumers should be able to inspect basis layers without
  knowing producer-private runtime internals.

## Stable Entry Points

- `boundary_evidence().provenance()`
- `forge_foundational::boundary_evidence_api::common_path::provenance()`
- `forge_foundational::boundary_evidence_api::lower_lane::provenance`

Core vocabulary:

- `FoundationalBoundaryEvidenceSourceBasis`
- `FoundationalBoundaryEvidenceAuthorityPath`
- `FoundationalBoundaryEvidenceProfileBasis`
- `FoundationalBoundaryEvidenceComparisonBasis`
- `FoundationalBoundaryEvidenceCanonicalDigestBasis`
- `FoundationalBoundaryEvidenceFreshnessPosture`

## Core Mental Model

Provenance answers "under what basis and posture was this produced?"

It does not answer:

- whether continuity was truly attested
- whether execution actually happened
- whether support truth is strong enough to trust across boundaries

Those questions belong to lineage, receipts, and stronger lanes.

## How It Executes

1. choose the source basis root
2. add authority, profile, comparison, or canonical/digest basis layers as
   needed
3. make freshness explicit
4. canonicalize support-context attachments
5. finalize the provenance artifact

## Small Example

```rust
use forge_foundational::{
    boundary_evidence,
    FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceSourceBasis,
};

let provenance = boundary_evidence()
    .provenance()
    .historical(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator))
    .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained);
```

This is the smallest honest example because provenance is not real until a
source basis and a freshness posture both exist.

## Real Example

```rust
use forge_foundational::{
    boundary_evidence_api::common_path as evidence,
    FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceSourceBasis,
};

let provenance = evidence::provenance()
    .historical(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(locator))
    .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained)?;
```

What is authoritative here is the explicit basis and freshness declaration.
What is derived is whatever later receipt, lineage, or support surface reuses
this provenance.

## How It Relates To Other Features

- [Receipts And Closeout Truth](./receipts-and-closeout-truth.md) reuses
  provenance when claiming what actually completed.
- [Support Truth, Recovery, And Degraded Operation](./support-truth-recovery-and-degraded-operation.md)
  depends on freshness staying explicit.

## Inspection And Debugging

- Check source-basis family first when a replay-derived artifact looks too
  strong.
- Check freshness posture next when a support report looks fresher than it
  should.
- Use the lower lane when you need to inspect provenance layer vocabulary
  directly.

## Anti-Patterns

- Treating provenance as if it proves execution.
- Smuggling replay slices or event-history records in as typed provenance.
- Hiding stale or reduced basis behind a generic "available" label.

## Current Limits

- Provenance does not replace current-basis readmission or proof-bearing
  certification.
- Runtime-specific replay planners and history stores are still crate-local.
- This feature standardizes meaning, not one event record layout.

## Related Docs

- [Receipts And Closeout Truth](./receipts-and-closeout-truth.md)
- [Support Truth, Recovery, And Degraded Operation](./support-truth-recovery-and-degraded-operation.md)

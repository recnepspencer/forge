# Receipts And Closeout Truth

## What This Feature Is

This feature is the typed receipt surface for Milestone 7. It tells you what
completed boundary happened, and it keeps that separate from planning,
continuity, and support commentary.

## Why You Use It

- Use this when you need to distinguish planned work from completed work.
- Use this when blocked or denied closeout still needs a real receipt.
- Use this when an adopting crate must not fake execution with a plan or a log
  record.

## Stable Entry Points

- `boundary_evidence().receipt()`
- `worth_foundational::boundary_evidence_api::common_path::receipt()`
- `worth_foundational::boundary_evidence_api::lower_lane::receipts`

Important artifact families:

- `FoundationalBoundaryEvidencePlanningReceiptArtifact`
- `FoundationalBoundaryEvidenceExecutedReceiptArtifact`
- `FoundationalBoundaryEvidenceCompletedReceiptArtifact`

## Core Mental Model

A receipt answers "what completed boundary truth do I have?"

- planning receipts say work was planned
- executed receipts say work actually executed
- closeout receipts say the boundary completed, but may still report blocked or
  denied execution

That distinction is the whole point.

## How It Executes

1. choose the receipt family
2. bind it to a boundary
3. attach provenance
4. publish the receipt artifact
5. inspect `did_execute()` and closeout disposition when relevant

## Small Example

```rust
use worth_foundational::boundary_evidence;

let receipt_lane = boundary_evidence().receipt();
let _ = receipt_lane;
```

This is the smallest honest example because the receipt lane itself is the
stable entry point engineers are meant to discover first.

## Real Example

```rust
use worth_foundational::{
    boundary_evidence_api::common_path as evidence,
    FoundationalBoundaryEvidenceReceiptBoundary,
};

let receipt = evidence::receipt()
    .execution(FoundationalBoundaryEvidenceReceiptBoundary::transition(locator))
    .with_provenance(provenance);
```

For blocked or denied closeout, the shape changes but the family stays real:

```rust
let blocked = evidence::receipt()
    .blocked_closeout(FoundationalBoundaryEvidenceReceiptBoundary::transition(locator))
    .with_provenance(provenance);
```

## How It Relates To Other Features

- [Lineage, Continuity, Divergence, And Promotion](./lineage-continuity-divergence-and-promotion.md)
  uses executed receipts for stronger continuity claims.
- [Support Truth, Recovery, And Degraded Operation](./support-truth-recovery-and-degraded-operation.md)
  uses support-publication and closeout receipts for support-grade truth.

## Inspection And Debugging

- Check the receipt family first.
- Check `did_execute()` second.
- Check closeout disposition before assuming a completed boundary means
  successful execution.

## Anti-Patterns

- Passing a planning receipt anywhere an executed receipt is required.
- Treating closeout as synonymous with successful execution.
- Using a generic event-history record as if it were a typed receipt artifact.

## Current Limits

- Receipts do not explain continuity by themselves.
- Receipts do not replace provenance layering.
- Runtime-local execution engines remain outside this crate.

## Related Docs

- [Lineage, Continuity, Divergence, And Promotion](./lineage-continuity-divergence-and-promotion.md)
- [Support Truth, Recovery, And Degraded Operation](./support-truth-recovery-and-degraded-operation.md)

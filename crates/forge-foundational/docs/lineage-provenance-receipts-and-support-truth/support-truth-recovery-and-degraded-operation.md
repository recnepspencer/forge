# Support Truth, Recovery, And Degraded Operation

## What This Feature Is

This feature is the support-grade truth surface for Milestone 7. It gives you
typed evidence bundles, degraded-recovery reports, parity artifacts, transient
lifecycle evidence, stale-basis disclosures, and residual debt without
pretending they are stronger authority state.

## Why You Use It

- Use this when a runtime needs to publish support-grade truth honestly.
- Use this when stale, reduced, replayed, rebuilt, or quarantined recovery must
  be explicit.
- Use this when support publication and support closeout need a first-class
  surface instead of notes attached to receipts.

## Stable Entry Points

- `boundary_evidence().support()`
- `forge_foundational::boundary_evidence_api::common_path::support()`
- `forge_foundational::boundary_evidence_api::lower_lane::support`

Important support families include:

- published support
- degraded recovery reports
- transient lifecycle evidence
- basis disclosure
- residual debt

## Core Mental Model

Support truth answers "what support-grade or recovery-grade truth do I have?"

It is authoritative for its declared support role, but it is still weaker than
stronger authority or proof-bearing readmission surfaces.

That means support truth can be very important without being allowed to
impersonate execution or current-basis authority.

## How It Executes

1. choose the support family
2. make basis disclosure explicit
3. bind the relevant support-publication or closeout receipt
4. add recovery posture and residual debt when needed
5. publish the support artifact

## Small Example

```rust
use forge_foundational::boundary_evidence;

let support_lane = boundary_evidence().support();
let _ = support_lane;
```

This is the smallest honest example because the support lane itself is the
stable starting point, and support truth is not meaningful until the family is
chosen.

## Real Example

```rust
use forge_foundational::boundary_evidence_api::common_path as evidence;

let published = evidence::support()
    .published_evidence()
    .with_basis_disclosure(
        forge_foundational::FoundationalBoundaryEvidenceSupportBasisDisclosure::ReducedBasis,
    )
    .attested_by(support_publication_receipt)?;
```

For degraded recovery:

```rust
let degraded = evidence::support()
    .degraded_recovery()
    .with_recovery_posture(recovery_posture)
    .with_basis_disclosure(basis_disclosure)
    .closed_by(closeout_receipt)?;
```

Transient lifecycle evidence is also first-class here. Use it when something
opened and closed within one completed boundary and you want to preserve that
fact without upgrading it into durable lineage.

## How It Relates To Other Features

- [Provenance Layering And Freshness](./provenance-layering-and-freshness.md)
  explains why freshness and basis posture remain explicit here.
- [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)
  covers how support truth attaches to boundary artifacts and reenters stronger
  lanes.

## Inspection And Debugging

- Check basis disclosure first.
- Check recovery posture second.
- Check residual debt when a report looks too confident for the basis it had.

## Anti-Patterns

- Treating published support as if it were an executed receipt.
- Publishing stale or reduced support without explicit disclosure.
- Letting support-grade truth bypass stronger readmission or readiness lanes.

## Current Limits

- This feature does not choose one support bundle layout for every crate.
- It does not imply one QA or persistence harness.
- Runtime-specific journal and checkpoint topology stays local.
- Transient lifecycle evidence stays support-grade; it does not become durable
  lineage on its own.

## Related Docs

- [Provenance Layering And Freshness](./provenance-layering-and-freshness.md)
- [Attachment Materialization, Canonical Participation, And Readmission](./attachment-materialization-canonical-participation-and-readmission.md)

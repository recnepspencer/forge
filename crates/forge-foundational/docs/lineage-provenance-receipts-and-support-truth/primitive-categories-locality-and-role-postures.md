# Primitive Categories, Locality, And Role Postures

## What This Feature Is

This feature gives you the base vocabulary for Milestone 7. It is where
`forge-foundational` says what kind of descriptive artifact you are holding,
where it came from in time or branch posture, and whether it is planned,
executed, descriptive, fresh, stale, replay-derived, or restored.

## Why You Use It

- Use this when you need one shared vocabulary for lineage, provenance,
  receipts, and support truth.
- Use this when a crate has to say "current", "historical", or
  "replay-derived" without inventing stringly local dialect.
- Use this when you want category and role honesty before richer artifact
  builders enter the picture.

## Stable Entry Points

- `boundary_evidence().category_definitions()`
- `boundary_evidence().locality_definitions()`
- `boundary_evidence().execution_posture_definitions()`
- `boundary_evidence().descriptive_role_definitions()`
- `boundary_evidence().freshness_posture_definitions()`
- `boundary_evidence().evaluate_primitive_legality(...)`
- `forge_foundational::boundary_evidence_api::lower_lane::primitives`

## Core Mental Model

These primitives do not tell you the whole story. They stop you from telling
the wrong story too early.

The milestone starts by separating:

- category: lineage, provenance, receipt, or support truth
- locality: current, historical, replay-derived, restored, branch-local, and
  related postures
- execution posture: planned versus executed
- descriptive role: what kind of descriptive claim is being made
- freshness posture: whether the basis is fresh, stale, reduced, or rebuilt

## How It Executes

1. choose the category family
2. choose the locality posture
3. choose the execution or descriptive role posture
4. choose the freshness posture when that family needs one
5. run legality checks before richer builders attach more meaning

## Small Example

```rust
use forge_foundational::boundary_evidence;

let categories = boundary_evidence().category_definitions();
let localities = boundary_evidence().locality_definitions();

assert!(!categories.is_empty());
assert!(!localities.is_empty());
```

This is the smallest honest example because it shows the vocabulary surface
without pretending a richer artifact already exists.

## Real Example

```rust
use forge_foundational::{
    boundary_evidence, FoundationalBoundaryEvidenceCategory,
    FoundationalBoundaryEvidenceDescriptiveRole,
};

let legality = boundary_evidence().evaluate_primitive_legality(
    FoundationalBoundaryEvidenceCategory::SupportTruth,
    FoundationalBoundaryEvidenceDescriptiveRole::SupportGrade,
);

let _ = legality;
```

What is authoritative here is the legality floor, not ad hoc review comments in
an adopting crate.

## How It Relates To Other Features

- [Provenance Layering And Freshness](./provenance-layering-and-freshness.md)
  builds on these primitives.
- [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)
  shows where these definitions live in the public API.

## Inspection And Debugging

- Inspect the definition lists first when a downstream crate invents a local
  label that feels "close enough".
- Inspect primitive legality before assuming a richer artifact denial is a
  bug.

## Anti-Patterns

- Treating locality strings as interchangeable with category or freshness.
- Letting a support-grade role impersonate a stronger authority story.
- Hiding replay or restoration posture until after artifact materialization.

## Current Limits

- These primitives do not explain continuity or execution by themselves.
- They are a vocabulary floor, not a final evidence artifact.
- Real runtime storage layouts stay outside `forge-foundational`.

## Related Docs

- [Provenance Layering And Freshness](./provenance-layering-and-freshness.md)
- [Receipts And Closeout Truth](./receipts-and-closeout-truth.md)

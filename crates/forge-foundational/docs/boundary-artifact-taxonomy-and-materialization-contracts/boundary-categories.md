# Boundary Categories

## What This Feature Is

This feature gives Forge one shared language for the four boundary output
categories:

- `Summary`
- `Report`
- `Artifact`
- `Receipt`

It is the floor for the rest of Milestone 4. Before you can talk about role,
authority, materialization, or basis, you need to know what kind of boundary
surface you are talking about.

## Why You Use It

Use this surface when you need to:

- say what kind of boundary output you are producing
- keep summaries, reports, artifacts, and receipts mechanically distinct
- prevent local wrappers from pretending that all boundary outputs are the same
  thing with a category tag

This is the right lane when the first question is "what kind of boundary output
is this?"

## Stable Entry Points

- `FoundationalBoundaryArtifactCategory`
- `FoundationalBoundaryCategoryDefinition`
- `FoundationalBoundarySummarySurface`
- `FoundationalBoundaryReportSurface<Row>`
- `FoundationalBoundaryArtifactSurface<T>`
- `FoundationalBoundaryReceiptSurface`
- `boundary_artifact_category_definitions()`
- `boundary_summary_definition()`
- `boundary_report_definition()`
- `boundary_artifact_surface_definition()`
- `boundary_receipt_definition()`

## Core Mental Model

The four categories do different jobs:

- a summary is a bounded overview
- a report is an explanatory surface with rows or sections
- an artifact is a structured payload-shaped boundary output
- a receipt is a completed-boundary attestation surface

Those meanings are not interchangeable.

If a caller or runtime can replace one with another by convention, then the
shared boundary language has already failed.

## How It Executes

You construct the category-specific surface first:

1. choose the right category
2. satisfy that category's construction rules
3. carry that category-specific surface forward into role claims and later
   materialization

The category-specific construction rules are part of the contract:

- summaries require overview text
- reports require at least one row
- receipts require a completed-boundary description
- artifacts keep a structured payload without pretending to be one of the other
  categories

## Small Example

```rust
use forge_foundational::FoundationalBoundarySummarySurface;

let summary = FoundationalBoundarySummarySurface::new(
    "published compatibility snapshot",
    3,
)?;
```

## Real Example

Use category definitions when you need blind-consumer meaning instead of just
the enum:

```rust
use forge_foundational::{
    boundary_artifact_category_of, boundary_report_definition,
    FoundationalBoundaryReportSurface,
};

let report = FoundationalBoundaryReportSurface::new(
    vec!["scope narrowed", "diagnostics elided"],
    2,
)?;

let category = boundary_artifact_category_of(&report);
let definition = boundary_report_definition();

let _ = (
    category,
    definition.name(),
    definition.intended_use(),
    definition.must_not_mean(),
);
```

## How It Relates To Other Features

- [Boundary Roles And Authority Admission](./boundary-roles-and-authority-admission.md)
  attaches role and stronger authority meaning to these category surfaces.
- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)
  plans and materializes outputs that already have a real category.

## Inspection And Debugging

Check these first:

- `category()`
- `definition()`
- `intended_use()`
- `must_not_mean()`

For category-specific surfaces, also inspect:

- `overview()` on summaries
- `rows()` on reports
- `payload()` on artifacts
- `completed_boundary()` on receipts

## Anti-Patterns

- Do not model all boundary outputs as one generic envelope plus a category
  field.
- Do not treat a report as "an artifact with rows in the payload."
- Do not use a receipt to describe planned work.
- Do not let a summary become a thin alias for "anything short."

## Current Limits

- Categories alone do not express role or authority.
- Categories alone do not plan or materialize anything.
- Categories do not replace later role, seam, cost, or proof-bearing law.

## Related Docs

- [Boundary Roles And Authority Admission](./boundary-roles-and-authority-admission.md)
- [Boundary Materialization And Bundles](./boundary-materialization-and-bundles.md)

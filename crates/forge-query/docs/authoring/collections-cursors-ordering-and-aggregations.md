# Collections, Cursors, Ordering, and Aggregations

## What This Feature Is

Collection planning covers **how validated query bundles become collection plans**: opaque cursors, ordering keys, CDC-shaped collection modes, and requested aggregate/derived-field families. Cursors are **basis-bound opaque** boundaries—not HTTP offset/limit pages. Grouped template and composition-profile **debts** are called out explicitly in support/composition reports.

## Why You Use It

- page collection results with `OpaquePageCursor` and `CursorAdvanceContract::BasisBoundOpaque`
- plan ascending/descending order via `OrderingKeyPath` and `CollectionOrderingDirection`
- request aggregate or derived-field families through dedicated planners
- keep CDC-shaped output **query-shaped** (projected deltas), not raw changefeed semantics

## Core Mental Model

`collection/mod.rs` + `planning/mod.rs`:

| Mode / API | Role |
|------------|------|
| `CollectionPlanningMode::Ordinary` | Standard collection plan |
| `Cdc` | CDC-shaped collection family (still query-planned) |
| `plan_validated_bundle` | Core planner entry |
| `plan_validated_bundle_for_requested_aggregate_family` | Aggregate admission |
| `plan_validated_bundle_for_collection_family` | Collection-family-specific (incl. CDC) |

Cursors carry `CursorBoundaryDigest`—advance contracts are opaque strings tied to basis, not skippable numeric offsets.

## Main Entry Points

- `OpaquePageCursor`, `CursorBoundaryDigest`, `CursorAdvanceContract`
- `OrderingKeyPath`, `CollectionOrderingDirection`
- `plan_validated_bundle`, `plan_validated_bundle_for_requested_aggregate_family`
- `plan_validated_bundle_for_requested_derived_field_family`
- Read composition operator tests: `runtime/tests/read_composition/operator_owned/collections.rs`
- Harness: `harness/planning.rs`
- Composition debts: `composition/report.rs` profile rows

## Typical Flow

1. Validate query bundle for collection family.
2. `plan_validated_bundle` (or aggregate/derived specialized planner) with request context.
3. Execute read composition with collection operators; receive cursor boundary in receipt.
4. Advance with opaque cursor contract on next request—same basis binding required.
5. For aggregates: use `RequestedAggregateFamily` path; check composition report for grouped-template debt.

## How It Relates

- [Read composition](read-composition.md) — compose/execute; collection detail lives here
- [Region-scoped live](../runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md) — ordered collection partition locality
- [Planner parallel admission](planner-parallel-admission-and-scale-posture.md) — dispatch scale, not cursor semantics
- [Structural correspondence](../capabilities/structural-correspondence-and-historical-materialization.md) — identity/materialization neighbors

## Good to Know

- Kanban/table view shapes call `plan_validated_bundle_for_collection_family` from `view_shape/planning.rs`.
- Harness tests contrast ordinary vs CDC vs aggregate vs derived plans—use as behavior proof.
- `composition/report.rs` lists deferred grouped-template neighbors—do not imply full grouped rollup support.

## Anti-Patterns

- Implementing offset/limit pagination by parsing opaque cursor digests.
- Treating CDC collection mode as permission to expose raw database CDC streams.
- Assuming every aggregate family is admitted without `plan_validated_bundle_for_requested_aggregate_family` success.

## Current Limits

| Concern | Status |
|---------|--------|
| Basis-bound opaque cursors | **Verified** on ordinary paths |
| CDC-shaped collection planning | **Verified** as query-shaped |
| Grouped template / composition profile debts | **Debt** — see `composition/report.rs` |
| Store-backed cursor durability | **Deferred** (operating modes / matrix) |

## Related Docs

- [Read composition](read-composition.md)
- [Support matrix and admission](../foundations/support-matrix-and-admission.md)
- [Region-scoped live invalidation](../runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)
- [Lineage and correspondence](../capabilities/lineage-and-correspondence.md)

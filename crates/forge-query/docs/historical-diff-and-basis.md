# Historical Basis, Diff, And Comparison Queries

## What This Feature Is

Historical basis and diff queries let Forge Query bind a validated query to an
admitted execution basis, materialize that query against current or historical
snapshots, and shape comparisons as query-level change sets rather than raw
storage deltas.

## Why You Use It

- you need the same query to run against current, branch, preview, or
  historical bases
- you want retained-history reuse when it is legal and available
- you need typed admission around historical replay, reconstruction, and basis
  substitution
- you want diff results shaped as query outputs, not low-level record churn

## Stable Entry Points

- `resolve_snapshot_basis(...)`
- `preflight_execution_basis(...)`
- `bind_query_basis_context(...)`
- `admit_query_basis_context(...)`
- `execute_query_basis_context(...)`
- `attach_query_basis_metadata(...)`
- `build_query_basis_result_bundle(...)`
- `admit_historical_evaluation_path(...)`
- `resolve_historical_materialization_path(...)`
- `bind_diff_query_context(...)`
- `shape_query_diff_change_set(...)`
- `attach_diff_query_metadata(...)`
- `build_query_diff_result_bundle(...)`

Important public vocabulary:

- `ExecutionBasisIntent`
- `ResolvedSnapshotBasis`
- `SnapshotResolutionReport`
- `HistoricalEvaluationRequest`
- `HistoricalPathResolved`
- `HistoricalEvaluationAdmission`
- `ComparisonBasisFamily`
- `QueryDiffChangeSetArtifact`

## Core Mental Model

The query stays the same. The basis changes.

This feature answers three linked questions:

- which snapshot or branch head are we asking about
- whether that basis is admitted for this query and support posture
- whether a comparison can be expressed as a query-shaped diff

Basis work is explicit because authority matters. A current runtime head,
branch head, retained historical snapshot, preview-derived historical context,
or future store-backed replay are not interchangeable.

Diff work is also explicit. Forge Query does not expose "compare arbitrary
storage blobs." It shapes differences through the same query meaning that would
have been materialized on each side.

## How It Executes

1. Resolve a requested basis through `resolve_snapshot_basis(...)` and
   `preflight_execution_basis(...)`.
2. Bind that basis to a validated query with `bind_query_basis_context(...)`.
3. Admit the basis context for execution.
4. Execute the admitted basis context and attach basis metadata.
5. For comparisons, bind a diff context across two admitted bases.
6. Shape a query-level change set and package it into a diff result bundle.

Historical path work adds another explicit layer:

- admit the requested historical path
- resolve the materialization path
- preserve cost, reconstruction, replay-span, and reuse posture in the result

## Small Example

```rust
use forge_query::query_context::{
    admit_query_basis_context, bind_query_basis_context, execute_query_basis_context,
    QueryBasisContextRequest, QueryContextBindingSource,
};

let request = QueryBasisContextRequest::current_branch_head();
let context = bind_query_basis_context(
    request,
    QueryContextBindingSource::RuntimeCurrent(&preflight_bundle),
)?;
let admitted = admit_query_basis_context(context)?;
let artifact = execute_query_basis_context(&admitted)?;
```

This is the smallest honest example because it shows the sequence that keeps
authority and execution posture explicit.

## Real Example

```rust
use forge_query::historical::HistoricalEvaluationRequest;
use forge_query::historical::HistoricalPathReuseDescriptor;
use forge_query::query_context::{
    admit_query_basis_context, bind_diff_query_context, bind_query_basis_context,
    shape_query_diff_change_set, QueryBasisContextRequest, QueryContextBindingSource,
};

let current_context = admit_query_basis_context(bind_query_basis_context(
    QueryBasisContextRequest::current_branch_head(),
    QueryContextBindingSource::RuntimeCurrent(&current_preflight),
)?)?;

let request = HistoricalEvaluationRequest::retained_snapshot(
    "workflow-main@snapshot-42",
    0,
    0,
    HistoricalPathReuseDescriptor::retained_reuse(),
);
let path = admit_historical_evaluation_path(&validated, &request)?;
let resolved = resolve_historical_materialization_path(&path)?;

let historical_context = admit_query_basis_context(bind_query_basis_context(
    QueryBasisContextRequest::historical_snapshot("workflow-main@snapshot-42"),
    QueryContextBindingSource::Historical {
        query_preflight: &historical_preflight,
        admission: &path,
        metadata: &resolved,
    },
)?)?;

let diff = bind_diff_query_context(&current_context, &historical_context)?;
let change_set = shape_query_diff_change_set(&diff)?;

assert_eq!(change_set.comparison_basis_family().as_str(), "current_to_historical");
assert!(change_set.rows().iter().all(|row| {
    matches!(
        row.change_family(),
        QueryDiffChangeFamily::Added
            | QueryDiffChangeFamily::Removed
            | QueryDiffChangeFamily::Modified
            | QueryDiffChangeFamily::Unchanged
    )
}));

assert_eq!(resolved.resolved_path_class().as_str(), "resolved_retained_snapshot_path");
```

Two important things are happening here:

- the diff is expressed as the query's result meaning, not raw row-store churn
- retained history reuse is explicit, costed, and admitted rather than assumed

## How It Relates To Other Features

- Start with a legal query from [Schema Validation](./schema-validation.md).
- Use [Scopes, Templates, Saved Queries, And View Shapes](./scopes-templates-saved-queries-and-view-shapes.md)
  when the query came from reusable higher-order composition.
- Historical basis is a query execution feature, not the same thing as runtime
  `workspace.state(...)` from [State](./state.md).
- Historical comparisons pair naturally with [Lineage And Correspondence](./lineage-and-correspondence.md)
  when identity continuity matters across time.

## Inspection And Debugging

Look at the basis and historical artifacts directly:

- `SnapshotResolutionReport`
- `HistoricalEvaluationAdmission`
- `HistoricalMaterializationPathMetadata`
- `HistoricalCounterSnapshot`
- `QueryDiffChangeSetArtifact`

Important posture signals include:

- basis authority family: runtime vs store
- resolution mode: direct vs replay vs reconstruction
- retained-state reuse eligibility
- replay-tail reuse eligibility
- performance prediction drift
- comparison basis family and diff-shape legality

If a diff or historical execution denies early, inspect whether the failure came
from basis substitution, unsupported replay/reconstruction scope, or a query
shape that broadened beyond the admitted comparison contract.

## Anti-Patterns

- Treating a basis handle as a loose timestamp instead of an authority-bound
  execution contract.
- Comparing two different query digests and expecting a meaningful diff.
- Using historical APIs to ask for raw storage deltas instead of query-shaped
  result changes.
- Assuming store-backed replay and reconstruction are already equivalent to
  runtime-backed retained history.

## Current Limits

- Runtime-backed current, branch, preview-derived, and retained-history paths
  are the strongest supported surfaces here.
- Store-backed retained-history parity exists in the tested surface area.
- Store-backed replay and reconstruction remain explicit deferred debt and deny
  typed and early where not yet supported.
- Basis substitution that crosses the admitted authority contract is denied.

## Related Docs

- [Schema Validation](./schema-validation.md)
- [Lineage And Correspondence](./lineage-and-correspondence.md)
- [Scopes, Templates, Saved Queries, And View Shapes](./scopes-templates-saved-queries-and-view-shapes.md)

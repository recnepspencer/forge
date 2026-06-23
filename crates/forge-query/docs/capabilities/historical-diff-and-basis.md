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
- `forge_query_basis_observation_intent(...)`
- `RawBasisIntent`
- `ObservationBasisCapability`
- `ReplayBasisCapability`
- `ScopedObservationBasis`
- `ScopedReplayBasis`

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

Good to know:

- basis observation is now also a covered admitted family through
  `forge_query_basis_observation_intent(...)`
- that admitted family is the public basis-observation entry point when you
  need one scoped basis artifact directly instead of a full basis-bound query
  execution
- the query basis lifecycle is also the public place where preview,
  historical replay, lower-runtime binding, and future temporal or async basis
  posture get typed before execution starts
- if you need to ask "is this basis artifact ready, advisory, stale, or denied"
  without materializing the whole query, use the basis capability or scoped
  basis artifact itself rather than carrying raw branch, snapshot, or preview
  identifiers forward
- subscription basis binding now also follows the same rule: temporal basis
  posture belongs in the canonical subscription declaration before active
  lifecycle begins, and bridge-facing basis requests are projections from that
  declaration rather than a second identity authority
- policy, tenant, relationship-proof, and schema-context drift now follow the
  same basis-first rule for temporal/async retained meaning: Query remasks or
  denies before public delivery, state, or inspection projection instead of
  materializing first and filtering later
- basis-aware runtimes may also attach historical-basis metadata to
  declaration-time whole-refresh computed initialization, so retained derived
  surfaces can seed honestly from one admitted historical basis without
  caller-side reconstruction
- once those retained derived rows exist, downstream crates should consume them
  through the admitted materialization floor rather than reopening raw row
  archaeology;
  `materialize_intent(...).execute().terminal_json_decode_single_row::<T>()`
  is the preferred terminal export seam for one typed historical computed row
- when one historical step needs a coherent retained artifact across several
  computed surfaces from the same admitted basis, downstream crates should use
  `materialize_derived_artifact_bundle(...)` instead of rebuilding that pack
  through repeated local materialization loops
  - when that historical pack also needs exact artifact identity over a specific
  set of retained computed surfaces, downstream crates should bind the bundle
  through `bind_retained_artifact(...)` so the runtime owns the target-set
  contract and artifact digest
  - when the caller already knows the historical step is one exact named
  retained artifact, prefer `materialize_derived_artifact_binding(...)` so the
  runtime owns both materialization and binding in one seam
- when a historical proof or comparison step needs typed identity, membership,
  provenance, continuity, or other declared facts from retained derived/live
  artifacts, downstream crates should use `consume_projection_facts(...)` on
  those retained artifact bindings instead of reopening older helper seams
- retained scalar/bundle helpers still exist as expert historical utilities
  when the exact named artifact contract itself is the product surface, but
  they are no longer the ordinary typed-fact lane after the retained/live
  projection-consumption closure

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

One more runtime consequence matters for downstream crates that declare
maintained computed surfaces over historical truth:

- when the runtime already has retained upstream rows for an admitted
  historical basis, declaration-time whole-refresh computeds can materialize
  immediately
- the runtime carries that historical-basis metadata through retained refresh
  context instead of asking the maintainer or caller to rediscover it from raw
  rows
- downstream callers that need one typed retained historical row should cross
  the admitted derived-materialization artifact instead of combining
  `workspace.materialize_result(...)` with local decode helpers
- downstream callers that need several typed retained historical computed rows
  from one admitted basis should cross the retained derived-materialization
  bundle artifact instead of assembling the pack locally
- downstream callers that need only retained scalar basis evidence from one
  admitted historical derived row should cross the retained scalar fact set
  artifact instead of rebuilding that evidence from decoded structs

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

The direct basis-observation family is intentionally smaller:

```rust
let scoped_basis = forge_query_basis_observation_intent(
    RawBasisIntent::CurrentHead,
)?
.review()?
.admit()?
.scope();
```

You can also inspect the basis artifact's state directly:

```rust
let state = workspace.state(&scoped_basis)?;
assert_eq!(state.kind().as_str(), "ready");
```

You can inspect that same artifact through the unified inspection surface:

```rust
let inspection = workspace.inspect(&scoped_basis)?;

match inspection {
    ForgeQueryInspection::BasisLifecycle(basis) => {
        assert_eq!(basis.subject_label(), "scoped_observation_basis");
        assert_eq!(basis.state_kind().as_str(), "ready");
    }
    other => panic!("expected basis lifecycle inspection, got {other:?}"),
}
```

Both surfaces are digest-bound to the typed basis artifact. They are not a
best-effort reconstruction from raw snapshot IDs.

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

- Start with a legal query from [Schema Validation](../modeling/schema-validation.md).
- Use [Scopes, Templates, Saved Queries, And View Shapes](../authoring/scopes-templates-saved-queries-and-view-shapes.md)
  when the query came from reusable higher-order composition.
- Historical basis is a query execution feature, not the same thing as runtime
  `workspace.state(...)` from [State](../foundations/state.md).
- Use [Intent Admission](../execution/intent-admission.md) when you want the
  shared admitted-family story for basis observation itself.
- Historical comparisons pair naturally with [Lineage And Correspondence](lineage-and-correspondence.md)
  when identity continuity matters across time.

## Inspection And Debugging

Look at the basis and historical artifacts directly:

- `SnapshotResolutionReport`
- `HistoricalEvaluationAdmission`
- `HistoricalMaterializationPathMetadata`
- `HistoricalCounterSnapshot`
- `QueryDiffChangeSetArtifact`
- `ForgeQueryInspection::BasisLifecycle`

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
- Passing raw branch, snapshot, preview, or restart identifiers around after
  Query already admitted a typed basis artifact.
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
- Temporal/async runtime-backed meaning that no longer matches the admitted
  policy, tenant, relationship-proof, or schema context is remasked or denied
  before public projection.

## Related Docs

- [Schema Validation](../modeling/schema-validation.md)
- [Lineage And Correspondence](lineage-and-correspondence.md)
- [Scopes, Templates, Saved Queries, And View Shapes](../authoring/scopes-templates-saved-queries-and-view-shapes.md)



# Forge Query Docs

These docs are organized by working category, not by implementation order.

Start with the section that matches the job you are doing. Use
`domain-capabilities/` when a downstream domain wants Query to own the
artifact, orchestration, grouping, and recovery model.

## Table Of Contents

- [Start Here](#start-here)
  Fastest entry points by broad job.
- [Best Starting Points](#best-starting-points)
  Shortcuts for the most common Query doc searches.

## Start Here

- [Domain Capabilities](./domain-capabilities/README.md)
  Typed domain entry, declaration pipeline, helpers, grouped work, recovery,
  continuation, and certification.
- `foundations/`
  Runtime posture, support, operating modes, policy/tenant narrowing, workspace,
  and preview or branch context.
- `authoring/`
  Query authoring, read composition, collections/cursors, parallel admission,
  templates, and reusable shapes.
- `runtime-surfaces/`
  Live views, region-scoped live, computed reads, and retained read or
  materialize surfaces.
- `execution/`
  Writes, effects (authoring + authority-scoped execution), and intent admission.
- `capabilities/`
  Inspection, causal inspection, mutation evidence, basis lifecycle, projection
  consumption, historical/structural correspondence, lineage, and diagnostics.
- `modeling/`
  Aspects, authority lanes, and schema or modeling guidance.

## Best Starting Points

- if you are building domain workflows with Query:
  [Domain Capabilities](./domain-capabilities/README.md)
- if you need the shortest chooser path inside domain work:
  [Choosing The Right Surface](./domain-capabilities/choosing/README.md)
- if you want task-first guides for common multi-surface Query jobs:
  [Workflow Guides](./domain-capabilities/workflow/README.md)
- if you want short copy-oriented examples for common Query tasks:
  [Recipes](./domain-capabilities/recipes/README.md)
- if you need runtime posture and support context first:
  [foundations/support-matrix-and-admission.md](./foundations/support-matrix-and-admission.md)
- if you need store-backed vs runtime-backed honesty:
  [foundations/query-operating-modes.md](./foundations/query-operating-modes.md)
- if you need policy, tenant, or relationship-proof narrowing:
  [foundations/policy-tenant-and-relationship-proof-narrowing.md](./foundations/policy-tenant-and-relationship-proof-narrowing.md)
- if you need basis phase lifecycle (not raw ids):
  [capabilities/basis-capability-lifecycle.md](./capabilities/basis-capability-lifecycle.md)
- if you need cross-runtime “why” (not `workspace.inspect` alone):
  [capabilities/cross-runtime-causal-inspection.md](./capabilities/cross-runtime-causal-inspection.md)
- if you need lower-runtime routing (not direct crate imports):
  [domain-capabilities/lower-runtime-capability-routing.md](./domain-capabilities/lower-runtime-capability-routing.md)
- if you need contribution lane map:
  [domain-capabilities/contributions/README.md](./domain-capabilities/contributions/README.md)
- if you need aspect and authority semantics first:
  [modeling/aspects-and-authority-lanes.md](./modeling/aspects-and-authority-lanes.md)

## Foundations (feature docs)

- [Query operating modes](./foundations/query-operating-modes.md)
- [Policy, tenant, and relationship-proof narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)

## Authoring (feature docs)

- [Collections, cursors, ordering, and aggregations](./authoring/collections-cursors-ordering-and-aggregations.md)
- [Planner parallel admission and scale posture](./authoring/planner-parallel-admission-and-scale-posture.md)

## Execution (feature docs)

- [Authority-scoped effect execution](./execution/authority-scoped-effect-execution.md)

## Runtime surfaces (feature docs)

- [Region-scoped live invalidation and stream contracts](./runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)

## Capabilities (feature docs)

- [Basis capability lifecycle](./capabilities/basis-capability-lifecycle.md)
- [Cross-runtime causal inspection](./capabilities/cross-runtime-causal-inspection.md)
- [Authoritative mutation evidence](./capabilities/authoritative-mutation-evidence.md)
- [Structural correspondence and historical materialization](./capabilities/structural-correspondence-and-historical-materialization.md)

## Domain capabilities (new)

- [Lower-runtime capability routing](./domain-capabilities/lower-runtime-capability-routing.md)
- [Contributions hub](./domain-capabilities/contributions/README.md)
- [Invariant and capability contributions](./domain-capabilities/invariants/invariant-and-capability-contributions.md)
- Choosers: [live vs subscription](./domain-capabilities/choosing/live-view-vs-subscription.md), [inspection vs cross-runtime explanation](./domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md), [projection vs inspection](./domain-capabilities/choosing/projection-consumption-vs-inspection.md)



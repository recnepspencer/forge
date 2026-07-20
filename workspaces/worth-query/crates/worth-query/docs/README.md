# Worth Query Docs

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
  Runtime posture, support, operating modes, consumer proof, policy/tenant
  narrowing, workspace, and preview or branch context.
- `authoring/`
  Query authoring, read composition, collections, ordering, aggregates, cursor
  boundaries, parallel admission, templates, and reusable shapes.
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
- if you need downstream consumer proof, support pins, audits, generic
  consumer-residue-audit coverage, or test workspaces:
  [foundations/consumer-kit.md](./foundations/consumer-kit.md)
- if you need store-backed vs runtime-backed honesty:
  [foundations/query-operating-modes.md](./foundations/query-operating-modes.md)
- if you need policy, tenant, or relationship-proof narrowing:
  [foundations/policy-tenant-and-relationship-proof-narrowing.md](./foundations/policy-tenant-and-relationship-proof-narrowing.md)
- if you need basis phase lifecycle (not raw ids):
  [capabilities/basis-capability-lifecycle.md](./capabilities/basis-capability-lifecycle.md)
- if you need cross-runtime “why” (not `workspace.inspections()?.inspect` alone):
  [capabilities/cross-runtime-causal-inspection.md](./capabilities/cross-runtime-causal-inspection.md)
- if you need graph touch obligation selection, support rows, or downstream
  graph-obligation proof:
  [authoring/graph-touch-obligation-authority.md](./authoring/graph-touch-obligation-authority.md)
- if you need graph read access planning, admitted access postures, required
  index/materialization capability, or no-N+1 receipt proof:
  [authoring/graph-read-access-planning.md](./authoring/graph-read-access-planning.md)
- if you need lower-runtime routing (not direct crate imports):
  [domain-capabilities/lower-runtime-capability-routing.md](./domain-capabilities/lower-runtime-capability-routing.md)
- if you need contribution lane map:
  [domain-capabilities/contributions/README.md](./domain-capabilities/contributions/README.md)
- if you need aspect and authority semantics first:
  [modeling/aspects-and-authority-lanes.md](./modeling/aspects-and-authority-lanes.md)
- if you need exact scalar/struct authoring, predicate, and projection value
  semantics:
  [capabilities/native-aspect-values.md](./capabilities/native-aspect-values.md)

## Foundations (feature docs)

- [Workspace Overview](./foundations/workspace-overview.md)
- [Branches And Previews](./foundations/branches-and-previews.md)
- [Consumer Kit](./foundations/consumer-kit.md)
- [Query operating modes](./foundations/query-operating-modes.md)
- [Policy, tenant, and relationship-proof narrowing](./foundations/policy-tenant-and-relationship-proof-narrowing.md)

## Authoring (feature docs)

- [Collections, ordering, aggregates, and cursors](./authoring/collections-cursors-ordering-and-aggregations.md)
- [Graph Composition Authoring](./authoring/graph-composition-authoring.md)
- [Graph Read Access Planning](./authoring/graph-read-access-planning.md)
- [Graph Touch Obligation Authority](./authoring/graph-touch-obligation-authority.md)
- [Graph Obligation Consumer Kit](./authoring/graph-obligation-consumer-kit.md)

## Execution (feature docs)

- [Intent Admission](./execution/intent-admission.md)
- [Writes And Intent Boundaries](./execution/writes-and-intents.md)
- [Projection consumption and downstream authority](./capabilities/projection-consumption.md)
- [Authority-scoped effect execution](./execution/authority-scoped-effect-execution.md)

## Runtime surfaces (feature docs)

- [Region-scoped live invalidation and stream contracts](./runtime-surfaces/region-scoped-live-invalidation-and-stream-contracts.md)

## Capabilities (feature docs)

- [Declarative Query Experience](./capabilities/declarative-query-experience.md)
- [Basis capability lifecycle](./capabilities/basis-capability-lifecycle.md)
- [Subscription selection and diagnostics](./capabilities/subscription-selection-and-diagnostics.md)
- [Historical diff and basis](./capabilities/historical-diff-and-basis.md)
- [Cross-runtime causal inspection](./capabilities/cross-runtime-causal-inspection.md)
- [Authoritative mutation evidence](./capabilities/authoritative-mutation-evidence.md)
- [Structural correspondence and historical materialization](./capabilities/structural-correspondence-and-historical-materialization.md)
- [Native aspect values](./capabilities/native-aspect-values.md)

## Domain capabilities

- [Runtime-installed domains](./domain-capabilities/runtime-installed-domains.md)
- [Lower-runtime capability routing](./domain-capabilities/lower-runtime-capability-routing.md)
- [Contributions hub](./domain-capabilities/contributions/README.md)
- Choosers: [live vs subscription](./domain-capabilities/choosing/live-view-vs-subscription.md), [inspection vs cross-runtime explanation](./domain-capabilities/choosing/inspection-vs-cross-runtime-explanation.md), [projection vs inspection](./domain-capabilities/choosing/projection-consumption-vs-inspection.md)



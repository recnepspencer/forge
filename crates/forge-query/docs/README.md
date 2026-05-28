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
  Runtime posture, support, state, workspace, and preview or branch context.
- `authoring/`
  Query authoring, composition, templates, and reusable shapes.
- `runtime-surfaces/`
  Live views, computed reads, and retained read or materialize surfaces.
- `execution/`
  Writes, effects, and intent admission.
- `capabilities/`
  Inspection, projection consumption, existing truth, historical basis work,
  lineage, and diagnostics.
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
- if you need aspect and authority semantics first:
  [modeling/aspects-and-authority-lanes.md](./modeling/aspects-and-authority-lanes.md)



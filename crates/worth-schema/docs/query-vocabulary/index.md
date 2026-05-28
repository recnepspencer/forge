# Query Vocabulary Overview

## What This Feature Is

This is the short path into the `worth-schema` surfaces that help you name
Worth truth for Query-facing work.

These surfaces include:

- `QueryAspectPath`
- `QueryCollection`
- `QuerySchemaBasis`
- `QueryLiveField`

## Why You Use It

Use this section when you need stable names for:

- which truth slice a declaration reads or produces
- which collection a live view targets
- which schema basis a surface belongs to
- which delivered field name a consumer should request

## Stable Entry Points

- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
- [Live Fields](./live-fields.md)

## Core Mental Model

`worth-schema` gives you the names.

`forge-query` gives you the runtime behavior around those names.

That split matters:

- schema says what truth is called
- Query says how that truth is admitted, orchestrated, inspected, or recovered

## How It Relates To Other Features

- Use [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)
  when you need the registered Worth schema behind these names.
- Use [Topology Authoring](../topology-authoring/README.md) when you are
  authoring truth instead of naming it.
- Use [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)
  if you are looking for old schema-era runtime APIs.

## Good To Know

- `QueryAspectPath` is usually the first thing you reach for.
- `QueryCollection` and `QuerySchemaBasis` matter when you are wiring a live or
  computed surface to the right schema-backed target.
- `QueryLiveField` matters when you need concrete delivered names such as
  `identity.id` or `topology.target_identity`.

## Related Docs

- [Query Aspect Paths](./query-aspect-paths.md)
- [Query Collections And Bases](./query-collections-and-bases.md)
- [Live Fields](./live-fields.md)

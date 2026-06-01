# Schema Registry Overview

## What This Feature Is

This is the short path into the published schema-registry lane in
`worth-schema`.

Use this section when you need:

- the full Worth `RelationalSchemaRegistry`
- the schema identity/version
- the small guard surface around registry bootstrap

## Why You Use It

The schema registry is the foundation that tells a runtime what Worth entity
and relation kinds exist.

Use this before topology authoring or runtime setup when the runtime needs the
real Worth schema loaded.

## Stable Entry Points

- [Bootstrap Schema Registry](./bootstrap-schema-registry.md)
- [Schema Builder](./schema-builder.md)

## Core Mental Model

`bootstrap_schema_registry()` is the direct full bootstrap.

`SchemaBuilder` is the smaller "be explicit about the surfaces you expect"
wrapper around that bootstrap.

The builder does not create a custom schema. It only forces the callsite to say
which required Worth kind families it expects before emitting the standard
registry.

## How It Relates To Other Features

- Use [Topology Authoring](../topology-authoring/README.md) once the runtime is
  loaded with the schema.
- Use [Query Vocabulary](../query-vocabulary/README.md) when you need stable
  schema-facing names after bootstrap.

## Related Docs

- [Bootstrap Schema Registry](./bootstrap-schema-registry.md)
- [Schema Builder](./schema-builder.md)

# worth-schema Documentation

These docs cover the published `worth-schema` consumer surface.

`worth-schema` is the place where Worth names truth. It gives you the stable
vocabulary for aspects, entities, relations, schema-backed query names, schema
bootstrap, and small topology-authoring helpers.

Use `worth-schema` when you need:

- aspect, entity, and relation names
- stable query-facing names such as aspect paths, collections, bases, and live
  fields
- a `RelationalSchemaRegistry` loaded with Worth kinds
- authored topology truth input for seeds, fixtures, tests, and small explicit
  examples

Do not use `worth-schema` as your runtime entrypoint.

Use `forge-query` when you need:

- admitted operating-world entry
- declaration readiness and orchestration
- support, inspection, and explanation
- invariant registration and denials
- recovery and next-step guidance

## Start Here

- [Start Here](./start_here.md)
  The shortest path to the crate boundary and the schema-versus-Query split.
- [Feature Index](./learn/feature-index.md)
  One-line index of the main published consumer features.
- [Recipes](./learn/recipes.md)
  Task-first examples for the most common schema, query-vocabulary, and
  topology-authoring jobs.

## Feature Groups

- [Core Vocabulary](./vocabulary/README.md)
  Aspect, entity, and relation names.
- [Query Vocabulary](./query-vocabulary/README.md)
  Aspect paths, collections, bases, and live fields.
- [Schema Registry](./schema-registry/README.md)
  Full bootstrap and the small builder wrapper around it.
- [Topology Authoring](./topology-authoring/README.md)
  Small create-batch authoring, same-batch references, and seed helpers.
- [Authority](./authority/README.md)
  The write-side topology truth vocabulary that belongs here.
- [Migration](./migration/README.md)
  Boundary-change notes for runtime surfaces that moved to Query.

## Reading Order

1. [Start Here](./start_here.md)
2. [Feature Index](./learn/feature-index.md)
3. [Recipes](./learn/recipes.md)
4. the one feature group that matches your task
5. [Migration](./migration/moved-runtime-surfaces-to-forge-query.md) if you
   are coming from older schema-era runtime habits

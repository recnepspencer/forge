# Start Here

Use `worth-schema` when your job is to name truth correctly.

This crate gives you the stable vocabulary for Worth truth:

- `platform::aspects::Aspect`
- `platform::entities::EntityKind`
- `platform::relations::RelationKind`
- `QueryAspectPath`
- `QueryCollection`
- `QuerySchemaBasis`
- `platform::authority::RawTopologyIntent`
- `platform::authority::TopologyMutationBatch`

It also gives you two practical helper lanes:

- schema bootstrap with `bootstrap_schema_registry()`
- topology authoring through `worth_schema::facade::topology_authoring`

## The Short Rule

Reach for `worth-schema` when you need names, schema registration, or topology
authoring support.

Reach for `forge-query` when you need runtime behavior.

That means `forge-query` owns the normal public answers for:

- "can I run this yet?"
- "how do I declare this operation?"
- "what support or readiness blockers do I have?"
- "what invariant denied this?"
- "how do I inspect or recover?"

## Read In This Order

1. [Feature Index](./learn/feature-index.md)
2. [Recipes](./learn/recipes.md)
3. [Query Vocabulary](./query-vocabulary/README.md)
4. [Bootstrap Schema Registry](./schema-registry/bootstrap-schema-registry.md)
5. [Your First Topology Intent](./topology-authoring/your-first-topology-intent.md)
6. [Moved Runtime Surfaces To forge-query](./migration/moved-runtime-surfaces-to-forge-query.md)

## Good To Know

- aspect catalogs live under `worth_schema::facade::platform::aspects`
  because they initialize the Worth platform descriptor layer while still
  feeding the schema-to-query naming bridge.
- `QueryAspectPath`, `QueryCollection`, and `QuerySchemaBasis` live here
  because they name schema-facing query vocabulary.
- entity catalogs live under `worth_schema::facade::platform::entities`
  because they initialize the Worth platform descriptor layer rather than the
  ordinary Query lifecycle lane.
- relation catalogs live under `worth_schema::facade::platform::relations`
  because they initialize the Worth platform descriptor layer rather than the
  ordinary Query lifecycle lane.
- authority vocabulary such as `RawTopologyIntent`, `TopologyMutationBatch`,
  and `MutationOrigin` lives under `worth_schema::facade::platform::authority`
  because it belongs to the Worth platform descriptor layer rather than the
  ordinary Query lifecycle lane.
- Query declaration builders are no longer public from `worth-schema`. Build
  runtime declaration surfaces through `forge-query` directly.
- Schema-owned runtime support, invariant rollout, tracing, and explanation
  exports are intentionally gone from the public facade.

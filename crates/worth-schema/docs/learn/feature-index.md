# Feature Index

- [Core Vocabulary](../vocabulary/README.md)  
  Use this when you need the base `platform::aspects::Aspect`, `platform::entities::EntityKind`, and
  `platform::relations::RelationKind` surfaces that the rest of the crate
  builds on.

- [Query Vocabulary](../query-vocabulary/README.md)  
  Use this when you need schema-facing query names such as aspect paths,
  collections, and bases.

- [Query Collections And Bases](../query-vocabulary/query-collections-and-bases.md)  
  Use this when you need the stable collection and basis names behind a Query
  declaration.

- [Live Fields](../query-vocabulary/live-fields.md)  
  Use this when you need delivered field names such as `identity.id` or
  `topology.kind`.

- [Bootstrap Schema Registry](../schema-registry/bootstrap-schema-registry.md)  
  Use this when you need a `RelationalSchemaRegistry` loaded with Worth schema
  kinds.

- [Schema Builder](../schema-registry/schema-builder.md)  
  Use this when you want a small guard surface around full schema bootstrap.

- [Topology Authoring](../topology-authoring/README.md)  
  Use this when you need to build a topology intent, create same-batch
  references, or prepare authored truth input for a later Query-owned runtime
  lane.

- [Authority](../authority/README.md)  
  Use this when you need the public write-side topology truth vocabulary that
  belongs in schema.

- [Verification](../topology-authoring/verification.md)  
  Use this when you are migrating old code that expected schema-owned topology
  execution helpers.

- [Moved Runtime Surfaces To forge-query](../migration/moved-runtime-surfaces-to-forge-query.md)  
  Use this when you are migrating from older schema-era runtime habits and need
  to know which public surface moved to Query.

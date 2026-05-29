# `worth-schema` Phase 3 Authority Boundary

This document is the authority for the Phase 3 boundary cut in
`worth-schema`.

Phase 3 removes the schema-owned execution lane and pushes consumers toward
Query-owned runtime entry, orchestration, inspection, and recovery.

## Target Shape

After Phase 3, ordinary public `worth-schema` usage should look like this:

```rust
use worth_schema::facade::{
    QueryAspectPath,
    QueryCollection,
    QuerySchemaBasis,
    QueryLiveField,
    bootstrap_schema_registry,
    SchemaBuilder,
};
use worth_schema::facade::platform::aspects::Aspect;
use worth_schema::facade::platform::authority::{
    CreateKey, EntityReference, MutationOrigin, RawTopologyIntent, TopologyMutation,
    TopologyMutationBatch,
};
use worth_schema::facade::platform::entities::EntityKind;
use worth_schema::facade::platform::relations::RelationKind;
use worth_schema::facade::topology_authoring::{
    TopologyCreateBatchBuilder,
    created_ref,
    seed_minimal_topology,
};
```

Runtime-facing code should enter through Query instead:

```rust
use forge_query::facade::{ForgeQueryOrdinaryOutcome, ForgeQueryRuntime};

let handle = query
    .domain(MyDomain)
    .with_operating_context(context)
    .validate()?
    .admit()?;

let outcome = handle.orchestrate_declaration_entry_outcome(input);

if let Some(recovery) = handle.recover_from_outcome(&outcome) {
    let _ = recovery.recommended_action();
}
```

## Public Boundary Cut

Removed from the broad public facade:

- `TopologyAuthorityError`
- `VerifiedTopologyCommit`
- `DerivedTopologyReadBasis`
- `DerivedTruthBasisIdentity`

Removed from `topology_authoring`:

- `verify_topology_intent(...)`
- `verify_topology_intent_on_branch(...)`

## Keep In Public Schema

- truth vocabulary
- query-facing schema vocabulary
- schema bootstrap
- topology authoring input helpers and fixture-oriented seed helpers
- topology write-side mutation vocabulary through
  `worth_schema::facade::platform::authority`

## Query Surfaces Replacing The Removed Runtime Story

- configured runtime entry:
  [Configured Domain Handles](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/configured-domain-handles.md)
- typed binding:
  [Typed Binding Pipeline](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/typed-binding-pipeline.md)
- helper ergonomics:
  [Family Helpers](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/family-helpers.md)
- declaration execution:
  [Declaration Entry Orchestration](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md)
- retained inspection:
  [Declaration Entry Inspection](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/declaration-entry-inspection.md)
- ordinary outcome lane:
  [Ordinary Outcomes](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/ordinary-outcomes.md)
- recovery:
  [Recovery Boundary](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/recovery-boundary.md)

## Boundary Lock

Phase 3 locked the ordinary public rule:

- `worth-schema` may help build Worth truth inputs
- `forge-query` owns runtime entry, orchestration, inspection, and recovery
- schema-owned verification helpers are not part of the supported public lane

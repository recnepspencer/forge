# `worth-schema` Phase 1 Audit

This document is the authority for the first public-boundary hard break in
`worth-schema`.

Phase 1 is not a passive inventory. It is the first ownership correction:

- keep only real schema authority public
- cut obvious shadow-runtime exports now
- treat any "shared runtime-facing vocabulary" as Query-owned unless proven
  otherwise

## Target Shape

After Phase 1, ordinary public `worth-schema` usage should look like this:

```rust
use worth_schema::facade::{
    QueryAspectPath,
    QueryCollection,
    QuerySchemaBasis,
};
use worth_schema::facade::platform::aspects::Aspect;
use worth_schema::facade::platform::authority::{RawTopologyIntent, TopologyMutationBatch};
use worth_schema::facade::platform::entities::EntityKind;
use worth_schema::facade::platform::relations::RelationKind;
```

Runtime-facing code should enter through Query instead of reaching back into
schema-owned support, invariant, trace, or explanation surfaces:

```rust
use forge_query::facade::{ForgeQueryOrdinaryOutcome, ForgeQueryRuntime};

let handle = query
    .domain(MyDomain)
    .with_operating_context(context)
    .validate()?
    .admit()?;

let readiness = handle.declaration_entry_readiness::<AttachFaceMaterial>();
let outcome = handle.orchestrate_declaration_entry_outcome(input);

if let Some(recovery) = handle.recover_from_outcome(&outcome) {
    let _ = recovery.recommended_action();
}
```

Invariant and runtime-support authoring should also use Query-owned surfaces:

```rust
let artifact = forge_query_domain("worth.spatial")
    .for_intent(&declaration)
    .register_invariant_catalog("invariant.edge_split", invariant_catalog)
    .because("non-manifold edge splits must be blocked before publication")
    .materialize()?;
```

## Public Surface Verdicts

### Keep Public In `worth-schema`

- [crates/worth-schema/src/data/aspects](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/aspects)
- [crates/worth-schema/src/data/entities](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/entities)
- [crates/worth-schema/src/data/relations](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/relations)
  : published through `worth_schema::facade::platform::{aspects,entities,relations}`
- schema registry/bootstrap truth structure in [crates/worth-schema/src/data/bootstrap](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/bootstrap)
  : `bootstrap_schema_registry`, `SchemaBuilder`, `SchemaBuildError`,
  `SCHEMA_ID`, `SCHEMA_VERSION_ID`
- lower-authority truth and mutation vocabulary published through
  `worth_schema::facade::platform::authority`, such as:
  - `RawTopologyIntent`
  - `TopologyMutation`
  - `TopologyMutationBatch`
  - `CreateKey`
  - `EntityReference`
  - `MutationOrigin`
- Query path/basis vocabulary that is still being treated as schema/truth
  vocabulary in Phase 1:
  - `QueryAspectPath`
  - `QueryCollection`
  - `QuerySchemaBasis`
  - `QueryLiveField`

### Demote To Internal/Substrate

- [crates/worth-schema/src/data/tracing/mod.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/tracing/mod.rs)
  : keep as internal substrate for authority-adjacent code until topo and later
  runtime phases migrate off it

### Delete From Broad Public Facade Now

- query mutation gating/support exports:
  - `admit_query_mutation_batch`
  - `query_mutation_support_contract`
  - `QueryMutationAdmission`
  - `QueryMutationAdmissionBlocker`
  - `QueryMutationAdmissionReport`
  - `QueryMutationSupportContract`
- runtime invariant rollout exports:
  - `bootstrap_invariant_plan`
  - `bootstrap_runtime_invariant_plan`
  - `BootstrapInvariantPlan`
  - `BootstrapRuntimeInvariant`
  - `BootstrapRuntimeInvariantPlan`
  - downstream runtime-registration adoption now closes through
    [worth-topo Phase 4 Runtime Invariant Closeout](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-phase-4-runtime-invariant-closeout.md)
- runtime tracing rollout exports:
  - `bootstrap_tracing_plan`
  - `BootstrapTracingPlan`
- boundary/tracing exports:
  - `BoundaryEnvelope`
  - `BoundaryFailure`
  - `DecisionTrace`
  - all trace anchor/evidence and trace utility exports currently re-exported
    from `data::tracing`
- explanation/narration exports:
  - all `explain_*`
  - all `narrate_*`
  - all public `*Narrative` and `NarratedTrace` exports
- deleted implementation residue:
  - [crates/worth-schema/src/data/query/mutation_admission.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/query/mutation_admission.rs)
  - [crates/worth-schema/src/data/explanation/mod.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/explanation/mod.rs)
  - [crates/worth-schema/src/data/invariants](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/invariants)
  - [crates/worth-schema/src/data/bootstrap/invariant_plan.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/bootstrap/invariant_plan.rs)
  - [crates/worth-schema/src/data/bootstrap/tracing_plan.rs](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/worth-schema/src/data/bootstrap/tracing_plan.rs)

## Explicit Challenge Verdicts

### `data/query/declarations.rs`

Phase 2 verdict: removed.

The wrapper layer was deleted instead of being preserved as "shared schema
vocabulary." Downstream crates now call `forge-query` builders directly, which
keeps declaration grammar on the Query-owned side of the boundary.

### `data/query/mod.rs`

Current verdict:

- `QueryAspectPath`, `QueryCollection`, `QuerySchemaBasis`, `QueryLiveField`
  stay public in Phase 1 as schema/truth vocabulary
- query mutation admission/support exports do not

If later work proves that any of these "query vocab" types are really shared
runtime-facing Query posture names rather than schema/truth vocabulary, they
must move to Query rather than remaining on the public schema side.

### `data/invariants/*`

Phase 2 verdict: deleted.

- the internal invariant family tree was removed rather than preserved as
  hidden shared vocabulary
- runtime-facing registration, denial, support, and recovery vocabulary remains
  Query-owned by rule

## Query Surfaces Replacing The Removed Schema Story

- configured world / handle entry:
  [Configured Domain Handles](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/configured-domain-handles.md)
- declaration lowering:
  [Declaration Entry Orchestration](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md)
- retained inspection:
  [Declaration Entry Inspection](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/declaration-entry-inspection.md)
- support/readiness:
  [Declaration Entry Readiness](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/declaration-entry-readiness.md)
- declaration-scoped support:
  [Declaration-Scoped Support And Traceability](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md)
- lower-runtime support:
  [Lower-Runtime Support And Boundary Traceability](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md)
- lower-runtime explanation:
  [Lower-Runtime Explanation Contributions](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/explanation/lower-runtime-explanation-contributions.md)
- invariant registration:
  [Registering Domain Invariants Through Query](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md)
- runtime-facing invariant gaps/denials:
  [Capability Gaps And Invariant Denials](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/invariants/capability-gaps-and-invariant-denials.md)
- typed recovery:
  [Recovery Boundary](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/recovery-boundary.md)
- public ownership / anti-drift:
  [Orchestration Inventory](C:/Users/Esther/Documents/Programming/forge_workspace/worktree_2/crates/forge-query/docs/domain-capabilities/orchestration-inventory.md)

# Forge Query Runtime API Next Batch Implementation Plan

> **Parent direction:** Forge Query Runtime API implementation plan
>
> **Purpose:** finish the next runtime facade batch so ordinary consumers,
> compiled DSLs, CAD/kernel domains, and the todo demo can use Forge Query as
> the end-to-end runtime surface without dipping into low-level live, workflow,
> bridge, relational, or signal facades.
>
> **Bias:** build the proper generic infrastructure, not demo-only shortcuts.
> Keep domain semantics out of `forge-query`; domain crates provide schema,
> lowering, derived-view maintenance, and operation meaning through adapters.

## Current Baseline

Already landed:

- `ForgeQueryRuntime` exists as the high-level facade.
- `ForgeQueryRuntimeBackend` exists as the first backend seam.
- `ForgeQueryMemoryApp` implements the backend seam while executing writes
  through RuntimeBridge writeback admission and authority execution.
- Compiled program effects can bind typed operation inputs into
  `ForgeQueryWriteCommandTemplate`.
- Write receipts expose changed deltas, affected live view ids, affected
  derived view ids, and refresh fallback status.
- Preview direct writes are staged until `promote()`.
- `ForgeQueryArtifactInspector` returns structured inspected artifact handles
  instead of plain labels.

Still open:

- Runtime builder does not yet accept the low-level runtime/bridge/source/sink
  pieces needed to assemble a non-demo backend from outside Query.
- Live view declaration does not auto-materialize grouped baselines for all
  grouped live cases.
- Derived views are still patch-note routing, not real materialized and
  maintained outputs.
- Preview `run_operation` is not isolated when the operation contains writes.
- Artifact inspection is structured but still shallow.
- Runtime facade tests cover the new seam, but not enough authority,
  preview-isolation, derived-view, grouped-baseline, and external-backend
  behavior.

## Design Principle

Forge Query should feel like this to consumers:

```rust
let board = query.declare_live_view("tasks.board", request)?;
let receipt = query.write(command)?;
let rows = query.read_live(&board)?;
let patches = query.drain_patches(&board)?;

let program = ForgeQueryProgram::compile(domain_ir, schema_adapter)?;
let installed = query.install_program(program)?;
let run = query.run_operation(installed.operation("create")?, inputs)?;
```

Everything underneath that surface must still be real:

- canonicalize
- validate
- plan
- preflight
- bind workflow context
- lower writebacks
- admit bridge declarations
- execute bridge authority
- route signal/live invalidations
- maintain live and derived materializations
- produce traceable artifacts

Apps and domain DSLs must not call declarative live, workflow lowering,
RuntimeBridge, grouped baseline, or signal APIs directly.

## Batch 1: Generic Backend Assembly

Goal: make `ForgeQueryRuntimeBuilder` capable of building a runtime facade from
real infrastructure pieces, while keeping `in_memory_collections(...)` as a
thin convenience path.

Add types:

- `ForgeQueryRuntimeBackendParts`
- `ForgeQueryRuntimeSchemaAdapter`
- `ForgeQueryRuntimeSourceAdapter`
- `ForgeQueryRuntimeWriteAuthorityAdapter`
- `ForgeQueryRuntimeSignalSinkAdapter`
- `ForgeQueryRuntimeBackendBuildError`

Builder additions:

```rust
ForgeQueryRuntimeBuilder::relational_runtime(...)
ForgeQueryRuntimeBuilder::runtime_bridge(...)
ForgeQueryRuntimeBuilder::schema_adapter(...)
ForgeQueryRuntimeBuilder::source_adapter(...)
ForgeQueryRuntimeBuilder::write_authority(...)
ForgeQueryRuntimeBuilder::signal_sink(...)
ForgeQueryRuntimeBuilder::build_backend_from_parts()
```

Implementation notes:

- Keep `ForgeQueryRuntimeBackend` as the runtime-facing trait.
- Add a `ForgeQueryBridgeBackedRuntimeBackend` implementation that owns or
  references the assembled relational/runtime-bridge surfaces.
- Move memory-specific collection registration and pending writeback tracking
  behind a memory adapter instead of letting `ForgeQueryRuntime` know it exists.
- Builder must reject missing bridge, schema, source, authority, or sink inputs
  with precise errors.
- Do not expose domain-specific operation names, CAD concepts, todo concepts,
  or DSL meanings in these types.

Tests:

- Builder rejects each missing required part independently.
- `in_memory_collections(...)` still builds and routes writes.
- A fake external backend can declare/read/write through the same facade.
- Runtime facade never stores or branches on `ForgeQueryMemoryApp` concrete
  type.

## Batch 2: Live View Declaration Owns Baseline Materialization

Goal: `declare_live_view` should handle grouped live preconditions itself.
Callers should never have to know that grouped views require authoritative
baseline artifacts.

Implementation notes:

- Extend the backend trait with a grouped truth-view materialization hook:

```rust
fn materialize_grouped_truth_view(
    &self,
    request: &DeclarativeLiveQueryRequest,
    plan: &ViewShapePlanArtifact,
    basis: &ResolvedSnapshotBasis,
) -> Result<Option<BridgeGroupedTruthViewArtifact>, ForgeQueryWorkspaceError>;
```

- Add a declarative-live path that can finish live declaration with an optional
  grouped baseline instead of always passing `None`.
- For `KanbanGrouped`, Query should:
  1. canonicalize/validate/plan/preflight as usual
  2. ask backend for a grouped truth view
  3. call `materialize_grouped_execution_surface_from_truth_view(...)`
  4. call `materialize_authoritative_grouped_baseline(...)`
  5. lower the live view with that baseline
- For non-grouped views, baseline materialization must not run.
- If the backend cannot provide grouped truth, the facade should return a typed
  grouped-baseline admission error, not panic.

Tests:

- Grouped live view declares without caller-supplied baseline.
- Missing grouped truth support returns a clear error.
- Baseline plan digest and basis digest are recorded in the live view trace.
- Todo board/grouped demo path does not call low-level baseline APIs.

## Batch 3: Real Derived View Materialization

Goal: turn `ForgeQueryDerivedView` from a patch-note surface into real
materialized derived outputs with incremental maintenance where dependencies
are narrow enough.

Add types:

- `ForgeQueryDerivedViewHandle<T>`
- `ForgeQueryDerivedViewRequest`
- `ForgeQueryDerivedViewMaintainer`
- `ForgeQueryDerivedViewMaterialization<T>`
- `ForgeQueryDerivedPatch`
- `ForgeQueryDerivedRefreshFallback`

API:

```rust
let view = query.declare_derived_view(request, maintainer)?;
let rows = query.read_derived(&view)?;
let patches = query.drain_derived_patches(&view)?;
```

Implementation notes:

- `ForgeQueryDerivedView` remains domain-neutral metadata.
- Domain crates or apps provide a maintainer that maps base deltas into derived
  deltas or explicitly requests whole refresh.
- Query owns dependency admission, routing, receipt accounting, and traces.
- Maintainers receive only the minimal base deltas and declared aspect paths,
  not broad snapshots unless they explicitly request fallback.
- Refresh fallback must be a visible artifact with reason and affected view id.
- Derived view ids must be populated into write receipts from actual routing,
  not inferred after the fact.

Tests:

- Derived title-list view updates from a single `title.value` delta.
- Irrelevant aspect update produces no derived patch.
- Maintainer can request whole refresh and receipt marks `refresh_fallback`.
- Fake topology-like derived view receives relation/aspect invalidation without
  adding topology semantics to Query.

## Batch 4: Fully Isolated Preview Operations

Goal: preview branches must isolate all writes, including compiled operations,
until promote. Discard must leave authoritative state and live patches
untouched.

Implementation notes:

- Add `ForgeQueryRuntimeBranch` or `ForgeQueryPreviewBranchBackend` abstraction.
- `ForgeQueryPreviewSession::write` and `run_operation` should execute against
  preview branch state.
- `compare_to_authoritative()` should use native query diff artifacts over the
  preview branch versus authoritative branch.
- `promote()` should replay/admit/execute preview writeback authorities against
  authoritative state with stale-basis checks.
- `discard()` should drop preview branch state and emit a discard artifact.
- Program traces must mark preview basis, branch identity, diff artifacts, and
  promotion receipts.

Tests:

- Preview direct write is invisible to authoritative `read_live` before promote.
- Preview compiled operation write is invisible before promote.
- Discard leaves authoritative rows and live patches unchanged.
- Promote applies exactly the staged writes and emits authoritative receipts.
- Stale authoritative basis rejects promote before writeback execution.

## Batch 5: Artifact Inspector Becomes Real Introspection

Goal: make the inspector useful for demos, DSL debugging, and CAD/kernel
explanations without exposing private internals or raw mutable structures.

Add inspected views:

- `ForgeQueryCanonicalArtifactView`
- `ForgeQueryWorkflowArtifactView`
- `ForgeQueryBridgeArtifactView`
- `ForgeQueryLiveArtifactView`
- `ForgeQueryDerivedArtifactView`
- `ForgeQueryPatchArtifactView`
- `ForgeQueryTraceArtifactView`

Implementation notes:

- Inspector should expose identities, basis digests, authority class, policy
  class, lowering family, affected surfaces, fallback status, and counters.
- It should not expose constructors for proof-carrying artifacts.
- Program traces should include generated live declarations, derived
  declarations, writeback declaration identities, bridge contract identities,
  effect/idempotence identities, patch artifacts, and replay/parity metadata.
- The todo demo can use this to show "why this changed" without bespoke strings.

Tests:

- Write receipt inspector exposes canonical/workflow/bridge identities.
- Program run inspector links inputs, declarations, writebacks, patches, and
  outputs.
- Preview inspector identifies branch-local artifacts separately from promoted
  authoritative artifacts.

## Batch 6: Surface Hardening And Migration

Goal: make the new runtime API the preferred path and prevent demo/app code from
leaking back into low-level facades.

Implementation notes:

- Rework `ForgeQueryMemoryApp` into a compatibility wrapper over the new
  bridge-backed backend where possible.
- Update todo demo to use only:
  - `declare_live_view`
  - `declare_derived_view`
  - `read_live`
  - `read_derived`
  - `write`
  - `drain_patches`
  - `preview`
  - `install_program`
  - `run_operation`
  - `inspect_*`
- Add compile-fail or lint-style tests if feasible to discourage demo-level
  direct use of declarative live/workflow/bridge internals.
- Keep low-level modules public for advanced internal certification, but docs
  should identify runtime/program facade as the consumer path.

Tests:

- `cargo check -p forge-query -p forge-ui`
- Runtime facade tests pass.
- Todo demo compiles without direct low-level Query/Bridge/Relational calls.
- Existing workflow/writeback/live/view-shape tests remain passing.

## Suggested Execution Order

1. Generic backend assembly.
2. Grouped baseline auto-materialization.
3. Preview branch isolation.
4. Derived view materialization.
5. Artifact inspector deepening.
6. Demo migration and public API hardening.

This order keeps the runtime seam honest first, then fixes the most visible app
semantics, then makes derived outputs and introspection more powerful.

## Acceptance Criteria

- Consumers can build a runtime from either memory collections or explicit
  backend parts.
- Grouped live views declare through the facade with no caller-owned baseline.
- Derived views produce real materialized outputs and minimal patches.
- Preview direct writes and compiled operation writes are isolated until
  promote.
- Receipts and traces expose minimal changed surfaces, affected views,
  fallback status, bridge authority artifacts, and replay/parity metadata.
- Todo demo uses only runtime-level APIs.
- No domain-specific concepts are added to `forge-query`.

## Non-Goals For This Batch

- Durable store-backed execution.
- CAD/topology semantics inside Query.
- Full worth-schema or worth-topo migration.
- A new policy engine.
- A new UI redesign.
- Public deprecation/removal of low-level internal modules.


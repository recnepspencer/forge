# forge-signal Architecture V2 — Structural Redesign

> **Status:** Pre-production. All changes are breaking-change-safe.
>
> **Scope:** Architectural redesign of `forge-signal` applying the same rigor as the [relational architecture doc](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md) — type-as-contract, contract duality, declarative effects, subsystem decomposition, state-derived context, and commit result envelopes.
>
> **Relationship to V1:** This document supersedes Batches C–G of [signal_architecture.md](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge_signal/signal_architecture.md). Batches A and B (R1–R7) and the landed items from Batch D (R11, R12, R13) are *preserved* — they form the foundation this document builds on.

---

## Table of Contents

1. [What Landed from V1](#what-landed-from-v1)
2. [Phase S1 — Subsystem Decomposition](#phase-s1--subsystem-decomposition)
3. [Phase S2 — Contract System](#phase-s2--contract-system)
4. [Phase S3 — Declarative Effects & Computation Model](#phase-s3--declarative-effects--computation-model)
5. [Phase S4 — Transaction Architecture](#phase-s4--transaction-architecture)
6. [Phase S5 — Pipeline & Performance](#phase-s5--pipeline--performance)
7. [Phase S6 — Safety Architecture](#phase-s6--safety-architecture)
8. [Phase S7 — API Surface & Facade](#phase-s7--api-surface--facade)
9. [Phase S8 — Context-Aware Computation](#phase-s8--context-aware-computation)
10. [Phase S9 — Performance Enforcement Addendum](#phase-s9--performance-enforcement-addendum)
11. [What Must Be Preserved](#what-must-be-preserved)
12. [Sequencing](#sequencing)

---

## What Landed from V1

These items are complete and form the structural floor for this document:

| V1 Item | What It Did | Current File |
|---|---|---|
| R1 | `transition_clean/dirty/maybe_stale` on `NodeEntry` | [entry.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/node/entry.rs) |
| R2 | `PartitionScoped` trait + `scopes_overlap` | [output.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/output.rs) |
| R3 | `DependencySnapshotEntry` + `DependencySortKey` | [dependency.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/dependency.rs) |
| R4 | `with_scratch` closure-based lease | [graph.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/runtime/graph.rs) |
| R5/R6 | `reconcile_dependencies` + edge ceremony | [mutation.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/topology/mutation.rs) |
| R7 | `InvalidationTraversal` pipeline struct | [routing.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/invalidation/routing.rs) |
| R11 | `SegmentedStore<T, Id>` + type aliases | [segmented.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/storage/segmented.rs) |
| R12 | Telemetry sub-structs (7 domain groups) | [telemetry.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/telemetry.rs) |
| R13 | `replay_where` filter consolidation | [replay.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/diagnostics_access/replay.rs) |

> [!NOTE]
> R14 (stale_error dedup) is trivial and should be done alongside any Batch D cleanup. It is not tracked in this document.

---

## Phase S1 — Subsystem Decomposition

**Kernel reference:** [Relational C1 — Runtime Subsystems](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

### Problem

`SignalGraph` ([graph.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/runtime/graph.rs)) is a 13-field god struct mixing:

```rust
pub struct SignalGraph {
    // Arena (identity management)
    nodes: Vec<Slot>,
    free_list: Vec<u32>,
    free_slots: DenseBitset,
    active_nodes: u32,
    compaction: CompactionState,

    // Traversal infrastructure
    scratch: TraversalScratch,
    scratch_lease: Option<ScratchLeaseKind>,

    // Edge storage (topology)
    dependency_edges: DependencyEdgeStore,
    subscriber_edges: SubscriberEdgeStore,
    dependency_snapshots: DependencySnapshotStore,

    // Runtime state (observation/interning)
    partition_interner: PartitionInterner,
    telemetry: RuntimeTelemetry,
    diagnostics: DiagnosticsState,
}
```

Five distinct concerns share one mutable borrow. Adding a method to diagnostics forces reasoning about arena liveness. Adding an edge store method forces reasoning about scratch state.

Above this, `SignalRuntime<D, I, E, Ctx, T>` ([runtime_state.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/state/runtime_state.rs)) is 698 lines mixing graph ownership, branch management, snapshot capture/restore, event bus, diagnostics passthrough, tier configuration, keyed computation, and transactions — all behind 5 type parameters.

### Design

#### S1.1 — `SignalGraph` Subsystem Split

Decompose `SignalGraph` into subsystem structs accessible through a `GraphParts` destructuring pattern (matching forge-kernel's `BRepWorkspace::as_parts_mut()`):

```rust
pub struct NodeArena {
    nodes: Vec<Slot>,
    free_list: Vec<u32>,
    free_slots: DenseBitset,
    active_nodes: u32,
    compaction: CompactionState,
}

pub struct EdgeTopology {
    dependency_edges: DependencyEdgeStore,
    subscriber_edges: SubscriberEdgeStore,
    dependency_snapshots: DependencySnapshotStore,
}

pub struct TraversalResources {
    scratch: TraversalScratch,
    scratch_lease: Option<ScratchLeaseKind>,
}

pub struct RuntimeObservation {
    partition_interner: PartitionInterner,
    telemetry: RuntimeTelemetry,
    diagnostics: DiagnosticsState,
}

impl SignalGraph {
    /// Destructure for concurrent borrowing of independent subsystems.
    pub(crate) fn as_parts_mut(&mut self) -> (
        &mut NodeArena,
        &mut EdgeTopology,
        &mut TraversalResources,
        &mut RuntimeObservation,
    ) { ... }
}
```

The public `SignalGraph` type stays — it is the composed whole. But internal code calls `as_parts_mut()` to borrow only what it needs, eliminating false borrow conflicts.

#### S1.2 — `SignalRuntime` Subsystem Split

`SignalRuntime` currently delegates ~30 methods directly to `self.graph.method()`. These passthrough methods exist because the runtime cannot expose the graph without exposing everything. With subsystem decomposition:

```rust
pub struct SignalRuntime<D, I, E, Ctx, T> {
    graph: SignalGraph,
    config: SignalRuntimeConfig<T>,
    checkpoint: CheckpointRuntime<D, I>,
    event_bus: EventBus<E, D, Ctx>,
    branches: BranchManager<D, I, T>,   // extracted from inline BTreeMaps
    telemetry: RuntimeTelemetry,
}
```

`BranchManager` replaces the inline `BTreeMap<SignalBranchId, RuntimeBranchState>` + `BTreeMap<SignalSnapshotId, RuntimeBranchState>` and absorbs the `capture_branch_state` / `load_branch_state` / `synchronize_branch_catalogs` methods that currently clutter `runtime_state.rs`.

### Files Modified

| File | Change |
|---|---|
| [graph.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/runtime/graph.rs) | Split fields into `NodeArena`, `EdgeTopology`, `TraversalResources`, `RuntimeObservation`; add `as_parts_mut()` |
| [runtime_state.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/state/runtime_state.rs) | Extract `BranchManager`; reduce passthrough surface |
| All internal callers of `&mut SignalGraph` | Use `as_parts_mut()` where borrowing independent subsystems |

---

## Phase S2 — Contract System

**Kernel reference:** [Relational F2 — RecordProjection](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md) and [Relational D4 — Intent Contracts](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

> [!NOTE]
> Phase S9 extends this phase. `NodeContract` becomes the main performance
> contract and grows explicit equivalence, path-class, maintenance, and
> artifact-policy fields.

### Problem

In forge-relational, **write contracts** (`MutationIntent::invariant_contract()`) and **read contracts** (`RecordProjection::required_aspects()`) are declared up front on the type. The pipeline uses their intersection for aspect-aware invalidation.

In forge-signal, the equivalent information exists but is scattered:

- **What a node reads** (its dependency subscriptions) is only known after evaluation — it is a *side effect* of `PreparedDependencyCapture`, not a declaration.
- **What a node produces** (its output aspects and partition scopes) is only known from the `NodeEvaluationResult` returned at runtime.
- **What invalidation propagates** (dirty aspects and scopes) is inferred on the fly via `subscribes_to_aspect()`.

None of this is declared up front. The pipeline has no way to skip unnecessary work because it doesn't know what a node cares about until it runs.

### Design

#### S2.1 — `NodeContract` Trait

The read-path equivalent of `RecordProjection`. A node declares its dependency contract up front:

```rust
/// Declared on node registration — what this node reads and produces.
pub struct NodeContract {
    /// Which aspects this node subscribes to on its dependencies.
    pub reads: AspectMask,
    /// Which aspects this node may produce when evaluated.
    pub produces: AspectMask,
    /// Which partition scopes this node cares about, if any.
    /// None = all partitions (whole-partition subscription).
    pub partition_scope: Option<Vec<PartitionSubscription>>,
}
```

This is registered at node creation time via the `NodeBuilder`:

```rust
let node = graph.node()
    .with_contract(NodeContract {
        reads: AspectMask::from_aspect(Aspect(0)),
        produces: AspectMask::from_aspect(Aspect(0)),
        partition_scope: None,
    })
    .build();
```

Nodes that don't register a contract default to a wildcard contract (`reads: ALL, produces: ALL, scope: None`) — backward compatible, no behavioral change.

#### S2.2 — Contract Duality: Invalidation × Evaluation

The invalidation path pushes *what changed* (aspect + scopes). The evaluation path checks *what a node depends on*. These are duals:

| Direction | Contract | Current Location | After S2 |
|---|---|---|---|
| **Write** (invalidation) | "I changed aspect A in scope S" | `mark_dirty_with_regions(source, aspect, regions)` | Same, but planner can skip nodes whose `reads` mask doesn't intersect `aspect` |
| **Read** (evaluation) | "I depend on aspect A in scope S" | Implicit in `PreparedDependencyCapture` | Declared in `NodeContract.reads` + `.partition_scope` |

The pipeline uses contract intersection to prune the plan: if a node's `reads` mask doesn't intersect the combined `changed_aspects` mask of its dirty dependencies, the planner can skip it without evaluation.

#### S2.3 — Aspect-Aware Planner Pruning

Currently [plan_builder.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/planner/planning/mod.rs) includes all `Dirty` and `MaybeStale` nodes. With contracts:

```rust
fn should_include_in_plan(
    graph: &SignalGraph,
    node: NodeId,
    dirty_aspects: AspectMask,
) -> bool {
    let contract = graph.get_contract(node);
    // If the node's reads don't overlap with what changed, skip it
    contract.reads.intersects(dirty_aspects)
}
```

This is the same optimization as relational's D4 topology inference — the `union_mask` determines which checks to run.

#### S2.4 — Context-Type in Contracts

A node's contract should also declare which **domain context** it requires for evaluation. Signal currently has `Ctx` as a type parameter on `SignalRuntime<D, I, E, Ctx, T>`, but evaluation closures never receive it. Different computations need different contexts — a geometry kernel evaluation needs a model snapshot, a dashboard aggregation needs cross-project summaries, an admin metric needs system-level state.

The contract declares this:

```rust
pub struct NodeContract {
    pub reads: AspectMask,
    pub produces: AspectMask,
    pub partition_scope: Option<Vec<PartitionSubscription>>,
    /// Which context layer this node requires.
    /// None = context-free (pure computation from dependencies).
    pub required_context: Option<ContextRequirement>,
}

pub enum ContextRequirement {
    /// Node needs the domain context (Ctx parameter)
    DomainContext,
    /// Node needs a relational snapshot (bridge integration)
    RelationalSnapshot,
    /// Node is context-free — pure function of its inputs
    None,
}
```

The planner uses this to verify that the required context is available before scheduling evaluation. If a node requires `RelationalSnapshot` but the transaction was started without a bridge, the planner reports a contract violation at planning time instead of a runtime panic during evaluation.

This is the signal equivalent of the frontend's `ProjectContextService` — each computation declares its context dependency, and the framework verifies availability before execution.

### Files Modified

| File | Change |
|---|---|
| [entry.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/node/entry.rs) | Add `NodeContract` field to `NodeEntry` |
| [construction](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/construction) | Add `.with_contract()` to `NodeBuilder` |
| [planning/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/planner/planning/mod.rs) | Use `NodeContract.reads` to prune plan; verify `required_context` |
| [routing.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/invalidation/routing.rs) | Use `NodeContract.reads` to skip subscribers that don't care about the changed aspect |

---

## Phase S3 — Declarative Effects & Computation Model

**Kernel reference:** [Relational B5 — Declarative Effect Assembly](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

> [!NOTE]
> Phase S9 extends this phase by splitting hot operational effect data from
> optional diagnostic materialization inputs.

### Problem

[result_apply.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/evaluation/engine/result_apply.rs) `apply_evaluation_result_with_policy()` is a 137-line function that mixes:
1. **Comparison logic** — output identity/continuity token checks (L53–L71)
2. **Dependency snapshot building** — `build_dep_snapshot` + `count_meaningful_input_changes` (L72–L73)
3. **Trace assembly** — constructing `TraceSummary` from 15 fields (L79–L113)
4. **State transition** — `entry.transition_clean()` (L119)
5. **Telemetry** — incrementing counters conditionally (L123–L134)
6. **Downstream suppression** — `suppress_downstream_if_identity_unchanged` (L128–L130)

Every new evaluation behavior requires editing this monolith. Adding telemetry means adding more branches. Adding a new comparison policy means adding more conditions.

### Design

#### S3.1 — `EvaluationEffect` Struct

Separate domain-level computation outcome from framework bookkeeping:

```rust
/// The pure result of evaluating a signal node.
/// Contains what changed — not how to apply it.
pub struct EvaluationEffect {
    pub node: NodeId,
    pub aspect_version: AspectVersion,
    pub output_change: OutputChange,
    pub output_identity: Option<OutputIdentity>,
    pub continuity_token: Option<OutputIdentity>,
    pub changed_regions: Vec<ChangedRegion>,
    pub labels: Vec<String>,
    pub dependency_snapshot: DependencySnapshot,
    pub meaningful_input_changes: u32,
    pub recomputed: bool,
    pub memoized_origin: MemoizedResultOrigin,
    pub keyed_context: Option<PreparedKeyedContext>,
}
```

#### S3.2 — `apply_effect` Pipeline

The current monolith becomes a pipeline of small, testable phases:

```rust
impl SignalGraph {
    pub(crate) fn apply_effect(
        &mut self,
        effect: EvaluationEffect,
        comparator: VersionComparatorPolicy,
    ) -> Result<AppliedEffectReport, SignalError> {
        let comparison = self.compare_output(&effect, comparator);
        let trace = self.build_trace(&effect, &comparison);
        self.transition_node_clean(effect.node, effect.aspect_version, trace)?;
        self.commit_dependency_snapshot(effect.node, effect.dependency_snapshot)?;
        let suppressed = self.suppress_if_unchanged(effect.node, &comparison)?;
        self.record_effect_telemetry(&effect, &comparison, suppressed);
        Ok(AppliedEffectReport { comparison, suppressed })
    }
}
```

Each phase is a separate method that can be unit-tested. Adding a new comparison policy means adding a new `compare_output` variant, not editing a 137-line function.

#### S3.3 — Commit Ceremony Extraction (Transaction)

[transaction_commit.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/transaction/transaction_commit.rs) has the **same 30-line rollback ceremony copy-pasted 3 times** (L25–L65, L69–L109, L176–L213):

```
1. compute rollback_patch_count
2. event_bus.rollback(runtime_ctx)
3. rollback_graph_state()
4. construct RollbackDiagnostic
5. construct FailureSummary
6. push replay events
7. increment poison count
8. finalize_semantic_delta(true)
9. return error
```

Extract into:

```rust
fn fail_and_rollback(
    &mut self,
    runtime_ctx: &mut Ctx,
    reason: &str,
    error: SignalError,
) -> Result<TransactionOutcome, SignalError> {
    let rollback_patch_count = self.rollback_patch_count();
    self.event_bus.rollback(runtime_ctx);
    self.rollback_graph_state()?;
    // ... single copy of the ceremony
}
```

#### S3.4 — Evaluation Verdicts

**Kernel reference:** [Relational D5 — Three-State Verdicts](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

Signal evaluation has outcomes that are actually ternary, but this is inferred from scattered booleans across `result_apply.rs` and `prepared_apply.rs`:

- `recomputed == true` → the closure ran and produced new output
- `propagation_suppressed == true` → output identity matched, downstream propagation skipped
- on-demand / condition-deferred → the node was skipped entirely by condition gating

Formalize this as a first-class verdict:

```rust
pub enum EvaluationVerdict {
    /// Closure ran, output changed, downstream propagation needed.
    Recomputed,
    /// Closure ran, output identity unchanged, downstream suppressed.
    Suppressed { reason: SuppressionReason },
    /// Node was not evaluated due to condition/on-demand gating.
    Deferred { reason: DeferralReason },
}

pub enum SuppressionReason {
    OutputIdentityUnchanged,
    ContinuityTokenUnchanged,
    ComparatorMatch,
}

pub enum DeferralReason {
    ConditionNotMet,
    OnDemandNotRequested,
    DebounceWindow,
}
```

The verdict is attached to `EvaluationEffect` and flows into `TransactionResult` (S4), making the pipeline self-describing. The caller knows not just *that* a node was evaluated, but *what the evaluation decided*.

#### S3.5 — `defineComputation` Pattern

**Frontend reference:** Inspired by frontend `defineCrudResource` / `useCrudResource` patterns. If the agent working on this wants to see examples, ask the user — the frontend code is in a separate workspace.

Currently, defining a computation requires multi-step ceremony:

```rust
// Current: 6 separate calls to set up one computation
let family = runtime.register_computation_family("volumes");
let node = runtime.keyed_node(&family, "body_42");
runtime.set_node_tier(node, Tier::OnDemand);
runtime.set_fallback_comparator(OutputIdentity);
// ... then separately wire up the evaluator closure in the transaction
```

This is the same problem the frontend had before `defineCrudResource` — scattered setup that must be kept in sync manually.

`defineComputation` bundles everything into a single declaration:

```rust
let volumes = runtime.define_computation(ComputationSpec {
    family: "volumes",
    contract: NodeContract {
        reads: AspectMask::from_aspect(GEOMETRY),
        produces: AspectMask::from_aspect(METRICS),
        partition_scope: None,
        required_context: Some(ContextRequirement::RelationalSnapshot),
    },
    tier: Tier::OnDemand,
    comparator: VersionComparatorPolicy::OutputIdentity,
    evaluator: |ctx, deps| {
        // compute volumes from relational snapshot
        Ok(NodeEvaluationResult { ... })
    },
});

// Later: just use it
let node = volumes.keyed("body_42");
let result = volumes.evaluate(node)?;
```

The `ComputationSpec` is the signal equivalent of the frontend's `CrudResourceDefinition` — a single source of truth for everything the framework needs to know about a computation.

> [!NOTE]
> `defineComputation` is a convenience API built on top of `NodeContract` (S2.1) and context requirements (S2.4). It does not introduce new primitives — it composes existing ones into a zero-boilerplate surface.

### Files Modified

| File | Change |
|---|---|
| [result_apply.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/evaluation/engine/result_apply.rs) | Replace monolith with `EvaluationEffect` + `apply_effect` pipeline; add `EvaluationVerdict` |
| [transaction_commit.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/transaction/transaction_commit.rs) | Extract `fail_and_rollback`, collapse 3 copies to 3 one-line calls |
| [prepared_apply.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/evaluation/engine/prepared_apply.rs) | Construct `EvaluationEffect` and call `graph.apply_effect()` |
| [config.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/config.rs) | Add `ComputationSpec`, `define_computation()`, and computation registry |
| [runtime_state.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/state/runtime_state.rs) | Expose `define_computation()` on `SignalRuntime` |

---

## Phase S4 — Transaction Architecture

**Kernel reference:** [Relational E1 — Commit Decision Log](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md) and [Relational E2 — Commit Result Envelope](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

### Problem

`TransactionOutcome` is currently a bare enum:

```rust
pub enum TransactionOutcome {
    Committed,
    RolledBack,
    Poisoned,
}
```

After a commit, the caller has **no structured information** about what happened: how many nodes were evaluated, which aspects changed, what the execution report contained, whether any suppression occurred, how many event epochs completed. The caller must query diagnostics separately, after the fact, hoping the right data was retained.

### Design

#### S4.1 — `TransactionResult` Envelope

```rust
pub struct TransactionResult {
    pub outcome: TransactionOutcome,
    pub execution_report: Option<ExecutionReport>,
    pub timing: TransactionTiming,
    pub touched_nodes: u32,
    pub evaluation_summary: EvaluationSummary,
    pub event_epochs: Vec<EventEpochSummary>,
    pub rollback: Option<RollbackDiagnostic>,
    pub warnings: Vec<AdvisoryRecord>,
    pub decision_summary: DecisionSummary,
    pub decision_log: DecisionLog,
    pub integrity_markers: IntegrityMarkers,
    pub performance_accounting: PerformanceCounterSurface<'static>,
}

pub struct TransactionTiming {
    pub total_nanos: u128,
    pub evaluation_nanos: u128,
    pub event_flush_nanos: u128,
    pub commit_nanos: u128,
}

pub struct EvaluationSummary {
    pub nodes_evaluated: u32,
    pub nodes_recomputed: u32,
    pub nodes_suppressed: u32,
    pub plans_built: u32,
    pub stages_executed: u32,
}
```

The caller gets a self-describing transaction result without querying diagnostics
or producer internals:

```rust
let result = runtime.transaction(ctx, |txn| {
    txn.mark_dirty(node, aspect)?;
    Ok(())
})?;

// Structured result immediately available
println!("evaluated {} nodes", result.evaluation_summary.nodes_evaluated);
```

#### S4.2 — `SemanticDelta` Consolidation

`TransactionSemanticDelta` currently stores replay events as `Vec<(ReplayEventKind, String, Option<..>, Option<..>)>` tuples. Replace with named struct:

```rust
pub struct TransactionReplayEntry {
    pub kind: ReplayEventKind,
    pub detail: String,
    pub execution_record_id: Option<ExecutionRecordId>,
    pub semantic_segment_id: Option<SemanticSegmentId>,
}
```

### Files Modified

| File | Change |
|---|---|
| [transaction_commit.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/transaction/transaction_commit.rs) | Return `TransactionResult` instead of bare `TransactionOutcome` |
| [transaction_types.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/transaction/transaction_types.rs) | Add `TransactionResult`, `TransactionTiming`, `EvaluationSummary`, `TransactionReplayEntry` |
| All callers of `.commit()` / `.transaction()` | Receive `TransactionResult` |

---

## Phase S5 — Pipeline & Performance

**Kernel reference:** Relational D4 (topology inference) and frontier patterns from forge-kernel.

> [!NOTE]
> Phase S9 is the enforcement completion layer for this phase. S5 establishes
> the pipeline; S9 makes the pipeline consume proof-bearing forms and
> batch-first contracts by default.

> [!NOTE]
> This phase subsumes V1's R8 (zero-allocation planner), R9 (feature-gated execution), and R10 (amortized GC). The designs are refined to align with the subsystem and contract patterns from S1–S2.

### Cross-Cutting Rule — Batch-Scoped Structural Maintenance

> [!IMPORTANT]
> Amortize structural maintenance across a batch boundary whenever
> intermediate states have no semantic value.

This is now a core Forge Signal architecture rule, not an optional
optimization pattern.

The staged/session-backed execution model already gives the system natural
batch boundaries: planning sessions, evaluation stages, transaction commit,
rollback repair, scenario assembly, and any topology reconciliation pass that
derives one final committed truth. When only that final truth is observable,
Forge Signal must not pay to maintain every intermediate structural state as if
it were semantically meaningful.

The rule is simple:

- if only the final batch or stage truth is observable
- and intermediate structural states do not affect semantics in-flight
- then structural work must be accumulated first
- and committed once per affected node, source, set, or stage boundary

This applies especially to:

- dependency rewiring and subscriber membership maintenance
- staged prepared-apply pipelines
- scenario/setup graph assembly
- rollback and repair passes
- snapshot, topology, or diagnostics artifacts that can be derived once from a
  merged batch result

**Operational contract**

Batch-first surfaces are the architectural default. Per-edit structural
mutation is not an acceptable hot-path contract unless the caller can prove
that intermediate states are semantically observable and required.

In practice, that means:

- batch/session APIs are the primary operational path
- per-edit topology maintenance is a low-level implementation detail, not a
  design surface
- downstream diagnostics should prefer batch-derived summaries over repeated
  structural reconstruction

**Review rule**

When reviewing hot-path code, ask:

- is any intermediate structural state observable outside this batch?
- are we rewriting storage more than once for the same affected set?
- could this work be accumulated by node, source, or stage and committed once?
- are downstream consumers reconstructing structure that the batch boundary
  already knew?

### S5.1 — Contract-Driven Plan Pruning

After S2 lands, the planner has `NodeContract.reads` available. The `populate_plan_buffers` function in [planning/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/planner/planning/mod.rs) currently includes all `Dirty`/`MaybeStale` nodes. With contracts, nodes whose `reads` mask doesn't intersect the propagated `changed_aspects` mask are excluded at planning time, before any evaluation runs.

This is the signal equivalent of relational's D4 — the contract mask determines which pipeline phases execute.

### S5.2 — Zero-Allocation Planner (V1 R8, Redesigned)

V1's R8 proposed arena-backed cursors. The design is refined:

The planner already has a `build_evaluation_session_with_policy_resolver` that writes into `TraversalScratch`-owned buffers (`scratch.planner_targets`, `scratch.planner_tasks`, `scratch.planner_stages`). The runtime execution path already uses this through `EvaluationSession`.

What remains is ensuring the `EvaluationSession` path is the **primary** path, and the allocating `EvaluationPlan` path is only used for diagnostics/inspection. This is already partially done — it needs completion, not redesign.

### S5.3 — Execution Pipeline Decomposition (V1 R9, Redesigned)

V1's R9 proposed an `ExecutionPass` trait with `#[cfg(feature)]` isolation. Redesigned through S1 subsystem lens:

Instead of a trait, decompose the execution path into subsystem calls with `as_parts_mut()`:

```rust
fn execute_stage(
    graph: &mut SignalGraph,
    session: &EvaluationSession,
    stage_index: usize,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<StageExecutionRecord, SignalError> {
    let (arena, topology, traversal, observation) = graph.as_parts_mut();
    // precompute phase: reads arena + topology, doesn't need traversal
    let snapshots = precompute_stage(arena, topology, session, stage_index)?;
    // evaluate phase: uses arena + topology
    let results = evaluate_stage(arena, topology, &snapshots, resolver)?;
    // apply phase: mutates topology + observation
    let report = apply_stage_effects(arena, topology, observation, results)?;
    Ok(report)
}
```

The `#[cfg(feature = "parallel")]` isolation becomes:

```rust
#[cfg(feature = "parallel")]
fn precompute_stage_parallel(...) -> ... { /* rayon parallel map */ }

#[cfg(not(feature = "parallel"))]
fn precompute_stage_serial(...) -> ... { /* sequential */ }

fn precompute_stage(...) -> ... {
    #[cfg(feature = "parallel")]
    if should_parallelize(stage) {
        return precompute_stage_parallel(...);
    }
    precompute_stage_serial(...)
}
```

Conditional compilation is restricted to **function dispatch**, not interleaved within business logic.

### S5.4 — Amortized GC (V1 R10, Redesigned)

V1's R10 proposed incremental GC during traversals. Refined through S1:

With `NodeArena` as a separate subsystem, GC becomes a method on `NodeArena` that doesn't need to reason about edge topology or diagnostics. The arena tracks its own tombstone count and compacts within its own boundary.

Edge cleanup happens in `EdgeTopology` via a separate `prune_dead_edges()` method that runs lazily when the tombstone ratio exceeds a threshold. Since `NodeArena` and `EdgeTopology` are independent subsystems, they can compact independently without stop-the-world coordination.

### S5.5 — Execution Path Collapse

**Frontend reference:** Inspired by frontend component collapse patterns (create-dialog + edit-dialog → single parameterized form). If the agent working on this wants to see examples, ask the user — the frontend code is in a separate workspace.

[runtime_execution.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/execution/runtime_execution.rs) has `execute_for_commit` (~100 lines) and `execute_for_on_demand` (~100 lines) that are **structurally identical** — they differ only in:
- Request mode (`Default` vs `ForceOnDemand`)
- Target selection (all dirty via `staged_dirty` vs explicit node list)
- Whether they report execution timing to `semantic_delta`

The logic — build plan, precompute snapshots, evaluate stage, apply results, record diagnostics — is the same.

Collapse into a single `execute_evaluation` parameterized by an `ExecutionIntent`:

```rust
pub(crate) enum ExecutionIntent<'a> {
    Commit { dirty_targets: &'a DenseBitset },
    OnDemand { targets: &'a [NodeId] },
}

fn execute_evaluation(
    graph: &mut SignalGraph,
    intent: ExecutionIntent<'_>,
    config: &SignalRuntimeConfig<T>,
    ...
) -> Result<ExecutionReport, SignalError> {
    let (targets, request_mode) = match intent {
        ExecutionIntent::Commit { dirty_targets } => (
            dirty_targets.collect_set_bits(),
            EvaluationRequestMode::Default,
        ),
        ExecutionIntent::OnDemand { targets } => (
            targets.to_vec(),
            EvaluationRequestMode::ForceOnDemand,
        ),
    };
    // Single implementation of the evaluation pipeline
    ...
}
```

This is the same pattern as the frontend collapsing `DialogCreateComponent` and `DialogEditComponent` into a single `DialogFormComponent` parameterized by mode.

### Files Modified

| File | Change |
|---|---|
| [planning/mod.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/planner/planning/mod.rs) | Add contract-based pruning to `visit_node` |
| [runtime_execution.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/execution/runtime_execution.rs) | Collapse `execute_for_commit` / `execute_for_on_demand` into `execute_evaluation`; decompose into subsystem calls |
| [execution.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/planner/execution.rs) | Isolate `#[cfg(feature)]` to dispatch functions |
| [lifecycle.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/lifecycle) | Subsystem-scoped compaction |

---

## Phase S6 — Safety Architecture

> [!NOTE]
> Phase S9 extends this phase with allocation lifetime scopes, single-consumer
> packet rules, and phase-typed fast-exit progression.

> [!NOTE]
> This phase subsumes V1's R15 (partition-aware validation), R19 (PhaseGuard), R20 (observation purity), R21 (single source of truth), and R22 (transactional mutation). The designs are refined to build on S1–S3 rather than being standalone compile-time safety items.

### S6.1 — Partition-Aware Version Tracking (V1 R15, Unchanged)

The bug from V1 R15 is real and the design is sound. Move `AspectVersion` from a flat integer to a `PartitionVersionMap` so that `count_meaningful_input_changes` compares scope-specific versions:

```rust
pub struct PartitionVersionMap {
    global_version: AspectVersion,
    partition_versions: HashMap<PartitionToken, AspectVersion>,
}
```

> [!IMPORTANT]
> This should land **before S5** (pipeline performance). Optimizing a pipeline that over-evaluates due to false version matches is wasted optimization.

### S6.2 — Phase-Typed Graph Access (V1 R19 + R20, Redesigned)

V1 proposed `GraphHandle<Phase>` with `PhantomData` typestates. With S1's subsystem split, the approach is simpler: each phase borrows only the subsystems it needs.

Invalidation borrows `(&mut NodeArena, &EdgeTopology, &mut TraversalResources, &mut RuntimeObservation)`. Evaluation borrows `(&mut NodeArena, &mut EdgeTopology, &mut TraversalResources, &mut RuntimeObservation)`. Observation borrows `(&NodeArena, &EdgeTopology, &RuntimeObservation)` — all `&self`.

The type system enforces phase restrictions through borrow patterns, not through wrapper types:

```rust
impl SignalGraph {
    /// Observation-only borrow. All fields are &self.
    pub fn observe(&self) -> GraphObserver<'_> {
        GraphObserver {
            arena: &self.arena,
            topology: &self.topology,
            observation: &self.observation,
        }
    }
}

impl<'a> GraphObserver<'a> {
    pub fn explain(&self, node: NodeId) -> Result<NodeExplanation, SignalError> { ... }
    pub fn replay_events(&self) -> &[ReplayEvent] { ... }
    // No mutation methods exist here — compile error if attempted
}
```

V1's R20 (observation purity) falls out naturally: `GraphObserver` only has `&self` references.

### S6.3 — Single Source of Truth (V1 R21, Redesigned)

With `EdgeTopology` as a subsystem, the dual-representation problem (dependencies ↔ subscribers) is contained. `EdgeTopology` owns both stores and enforces that mutations always update **both** through `reconcile_dependencies` (already landed via R6).

The remaining risk is **stale subscriber edges after topology changes**. With S1, this becomes a subsystem invariant: `EdgeTopology` exposes an `assert_bidirectional_consistency(&self)` debug assertion that verifies deps↔subs agreement. This runs in debug builds and tests, not in production.

### S6.4 — Transactional Mutation Safety (V1 R22, Redesigned)

V1 proposed wrapping `&mut SignalGraph` in a `TransactionalMut<'tx>`. With S3's `EvaluationEffect` struct, the design is simpler: mutations during evaluation produce effects, and effects are applied atomically. The transaction only needs to undo `NodeEntry` patches (already handled by `SparsePatchBuffer`) and rollback created nodes (already handled by `rollback_created_nodes`).

The remaining gap is **edge store rollback** — if dependencies are reconciled during evaluation and the transaction rolls back, the edge topology must revert. With `EdgeTopology` as a subsystem, the rollback tracks `(NodeId, old_dependency_set_id)` tuples and restores them atomically.

### S6.5 — Typed Error Hierarchy

**Kernel reference:** [Relational A3 — Typed Error Hierarchy](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

`SignalError` is currently string-based:

```rust
// Current: all errors are strings
SignalError::invalid_input(format!("stale NodeId: {id}"))
SignalError::internal("signal scratch is already leased")
SignalError::invalid_input(format!("cycle detected at {node}"))
```

Catching these requires string matching. Relational's A3 proposed typed variants; signal has even more distinct error classes:

```rust
pub enum SignalError {
    /// Node handle has been recycled or refers to a freed slot.
    StaleHandle { node: NodeId, expected_generation: u32 },
    /// Cycle detected during planning or invalidation.
    CycleDetected { path: Vec<NodeId> },
    /// Re-entrant scratch lease (concurrent invalidation/evaluation).
    ScratchReentry { active: ScratchLeaseKind, attempted: ScratchLeaseKind },
    /// Scratch lease mismatch during restore.
    ScratchMismatch { expected: ScratchLeaseKind, restored: ScratchLeaseKind },
    /// Contract violation: node requires context that isn't available.
    ContractViolation { node: NodeId, requirement: ContextRequirement },
    /// Transaction used after commit/rollback.
    TransactionFinished,
    /// Transaction poisoned by a prior error.
    TransactionPoisoned,
    /// Event bus flush failed.
    EventFlushFailed { subscriber: String, source: String },
    /// Snapshot compatibility check failed.
    IncompatibleSnapshot { reason: String },
    /// Generic internal error (escape hatch for truly unexpected conditions).
    Internal { message: String },
}
```

Callers can now match on error kind:

```rust
match result {
    Err(SignalError::StaleHandle { node, .. }) => {
        // Handle stale reference gracefully
    }
    Err(SignalError::CycleDetected { path }) => {
        // Report cycle with full path
    }
    Err(e) => return Err(e),
}
```

### S6.6 — Builder Completeness

**Kernel reference:** [Relational C4 — Fork-Safe Construction](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

`SignalRuntimeBuilder` currently accepts all configuration as optional:

```rust
// Current: everything optional, defaults silently applied
let runtime = SignalRuntime::builder(graph)
    // forgot checkpoint policy? defaults to no-op
    // forgot fallback comparator? defaults to Exact
    // forgot diagnostics profile? defaults to minimal
    .build();
```

If you forget to set a checkpoint policy, checkpoints silently become no-ops. If you forget a fallback comparator, on-demand nodes may use the wrong comparison policy.

Enforce critical configuration at compile time using typestate:

```rust
pub struct SignalRuntimeBuilder<Checkpoint = (), Ctx = ()> {
    graph: SignalGraph,
    checkpoint: Checkpoint,
    ctx: PhantomData<Ctx>,
    // optional fields that have safe defaults
    fallback_comparator: VersionComparatorPolicy,
    diagnostics_profile: DiagnosticsProfile,
}

impl SignalRuntimeBuilder<(), ()> {
    pub fn with_checkpoint<D, I>(
        self,
        policy: CheckpointPolicy<D>,
    ) -> SignalRuntimeBuilder<CheckpointRuntime<D, I>, ()> { ... }
}

impl<D, I> SignalRuntimeBuilder<CheckpointRuntime<D, I>, ()> {
    // build() is only available when checkpoint is configured
    pub fn build(self) -> SignalRuntime<D, I, (), (), ()> { ... }
}
```

This is the same pattern as relational's C4 — the builder type tracks which required subsystems have been configured, and `build()` is only available when all required subsystems are present.

### Files Modified

| File | Change |
|---|---|
| [entry.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/node/entry.rs) | Replace flat `AspectVersion` with `PartitionVersionMap` |
| [graph.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/runtime/graph.rs) | Add `observe()` method returning `GraphObserver` |
| [mutation.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/graph/topology/mutation.rs) | Add `assert_bidirectional_consistency` |
| [result_apply.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/evaluation/engine/result_apply.rs) | Track edge set IDs for rollback |
| [error.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/data/error.rs) | Replace string-based `SignalError` with typed enum variants |
| [builder.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/state/builder.rs) | Add typestate to `SignalRuntimeBuilder` |

---

## Phase S7 — API Surface & Facade

> [!NOTE]
> Phase S9 extends this phase by making batch-first, bulk-first API shape a
> normative architecture rule rather than a style preference.

**Kernel reference:** [Relational F1 — Facade Namespace Organization](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

### Problem

[facade.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/facade.rs) has ~120 flat `pub use` re-exports. Consumers must know which of the 120 types they need. No organization by domain.

### Design

#### S7.1 — Grouped Facade Namespaces

```rust
// facade.rs
pub mod types {
    // Core types: NodeId, Aspect, AspectMask, AspectVersion, NodeState, etc.
}

pub mod graph {
    // Graph construction and access: SignalGraph, NodeBuilder, NodeContract
}

pub mod evaluation {
    // Evaluation primitives: EvaluationContext, EvaluationEffect, NodeEvaluationResult
}

pub mod planning {
    // Plan types: EvaluationPlan, EvaluationTask, ExecutionReport, etc.
}

pub mod transaction {
    // Transaction types: SignalRuntime, SignalTransaction, TransactionResult
}

pub mod diagnostics {
    // All diagnostic types, replay, explain, lineage
}

pub mod harness {
    // Test harness: SignalHarnessAdapter, SignalScenario, etc.
}

// Top-level re-exports for the most common types
pub use types::{NodeId, Aspect, AspectMask, NodeState};
pub use graph::{SignalGraph, NodeBuilder};
pub use transaction::{SignalRuntime, SignalTransaction, TransactionResult};
```

#### S7.2 — State-Derived Evaluation Strategy

**Kernel reference:** [Relational D6 — State-Derived Invariant Context](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge-relational/relational_architecture.md).

The evaluation engine currently applies the same strategy regardless of graph state. With a state-derived context, the runtime adapts:

```rust
pub struct EvaluationStrategy {
    pub parallelism: ParallelismHint,
    pub gc_pressure: GcPressure,
    pub observation_policy: ObservationLevel,
}

impl SignalGraph {
    pub fn derive_evaluation_strategy(&self) -> EvaluationStrategy {
        let node_count = self.active_nodes();
        let tombstone_ratio = self.tombstone_ratio();
        EvaluationStrategy {
            parallelism: if node_count > 1000 {
                ParallelismHint::Preferred
            } else {
                ParallelismHint::Serial
            },
            gc_pressure: if tombstone_ratio > 0.3 {
                GcPressure::CompactAfterEvaluation
            } else {
                GcPressure::Deferred
            },
            observation_policy: if self.diagnostics_profile().is_verbose() {
                ObservationLevel::Full
            } else {
                ObservationLevel::Minimal
            },
        }
    }
}
```

The transaction uses this to configure its evaluation pass dynamically.

---

## Phase S8 — Context-Aware Computation

**Frontend reference:** Inspired by frontend ambient context patterns (`ProjectContextService`, operations mode, admin mode). If the agent working on this wants to see examples, ask the user — the frontend code is in a separate workspace.

### Problem

`SignalRuntime<D, I, E, Ctx, T>` has a `Ctx` type parameter, but it only flows to the event bus during `commit()`. **Evaluation closures never receive the domain context.** This means:

- Geometry kernel evaluations need a model snapshot → must be captured in a closure upvalue, losing transactional safety
- Dashboard aggregations need cross-project summaries → must be threaded manually
- Admin metrics need system-level state → same manual threading

As the system scales to support projects, operations, and administrative modes (each with different context shapes), this problem multiplies. Every new context type requires a new way to smuggle state into evaluation closures.

In the frontend, this was solved by `ProjectContextService` — context is injected by the framework, not threaded by the consumer. Signal needs the same pattern.

### Design

#### S8.1 — Ambient Evaluation Context

Extend `EvaluationContext` to carry the domain context:

```rust
pub struct EvaluationContext<'graph, Ctx> {
    graph: &'graph SignalGraph,
    node: NodeId,
    domain_context: &'graph Ctx,
}

impl<'graph, Ctx> EvaluationContext<'graph, Ctx> {
    pub fn graph(&self) -> &SignalGraph { self.graph }
    pub fn node(&self) -> NodeId { self.node }
    pub fn domain(&self) -> &Ctx { self.domain_context }
}
```

The transaction injects the domain context at `begin()` time, and it propagates to every evaluation automatically:

```rust
runtime.transaction(ctx, |txn| {
    txn.mark_dirty(node, GEOMETRY)?;
    // ctx is available to all evaluation closures during commit
    Ok(())
})?;
```

#### S8.2 — Multi-Context Support

Different parts of the system operate in different contexts:

| Mode | Context Type | What It Provides |
|---|---|---|
| **Project** | `ProjectContext` | Relational snapshot for a single project |
| **Operations** | `OperationsContext` | Cross-project summaries, fleet metrics |
| **Administrative** | `AdminContext` | System configuration, tenant boundaries |
| **Kernel** | `ModelContext` | BRep workspace, topology access |
| **Simulation** | `SimulationContext` | Tick state, entity world |

The `Ctx` type parameter on `SignalRuntime` already supports this — different runtime instances can have different context types. The key design decision is: **do different computations within the same runtime need different context types?**

Two approaches:

**Option A: Homogeneous context** — all computations in one runtime share one `Ctx`. This is the current model. Different modes use different runtime instances.

**Option B: Heterogeneous context via trait objects** — a single runtime carries a context registry, and computations request their specific context layer:

```rust
pub trait ContextProvider: 'static {
    fn provide<T: 'static>(&self) -> Option<&T>;
}

impl<'graph> EvaluationContext<'graph, dyn ContextProvider> {
    pub fn require<T: 'static>(&self) -> Result<&T, SignalError> {
        self.domain_context.provide::<T>()
            .ok_or(SignalError::ContractViolation { ... })
    }
}
```

> [!IMPORTANT]
> **Recommendation: start with Option A** (homogeneous context per runtime). This is simpler, fully type-safe, and matches how the frontend handles it (separate services per mode, not one service with dynamic dispatch). Option B can be added later if multi-mode runtime becomes a real requirement.

#### S8.3 — Context-Scoped Evaluation

With ambient context, evaluation closures become pure functions of (context + dependencies) instead of closures that capture external state:

```rust
// Before: closure captures model snapshot unsafely
let snapshot = Arc::clone(&model_snapshot);
runtime.transaction(ctx, |txn| {
    txn.evaluate(volumes_node, |eval_ctx| {
        let snap = snapshot.lock();  // lock contention, transactional danger
        compute_volumes(&snap, eval_ctx.dependencies())
    })?;
    Ok(())
})?;

// After: context injected by framework
runtime.transaction(model_ctx, |txn| {
    txn.evaluate(volumes_node, |eval_ctx| {
        let snap = eval_ctx.domain().relational_snapshot();  // safe, transactional
        compute_volumes(snap, eval_ctx.dependencies())
    })?;
    Ok(())
})?;
```

The framework owns the context lifetime. The closure doesn't need to manage `Arc`/`Mutex`/lifetime gymnastics.

With `defineComputation` (S3.5), this becomes even cleaner:

```rust
let volumes = runtime.define_computation(ComputationSpec {
    family: "volumes",
    contract: NodeContract {
        reads: GEOMETRY.into(),
        produces: METRICS.into(),
        partition_scope: None,
        required_context: Some(ContextRequirement::DomainContext),
    },
    tier: Tier::OnDemand,
    comparator: VersionComparatorPolicy::OutputIdentity,
    evaluator: |ctx: &EvaluationContext<ModelContext>, deps| {
        let snapshot = ctx.domain().relational_snapshot();
        compute_volumes(snapshot, deps)
    },
});
```

### Files Modified

| File | Change |
|---|---|
| [context.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/context.rs) | Extend `EvaluationContext` with generic `Ctx` parameter |
| [runtime_state.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/state/runtime_state.rs) | Thread `Ctx` from `transaction()` through evaluation pipeline |
| [runtime_execution.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/execution/runtime_execution.rs) | Accept `&Ctx` and pass to evaluation closures |
| [transaction_evaluation.rs](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/crates/forge-signal/src/logic/transaction/runtime/execution/transaction_evaluation.rs) | Inject `Ctx` into evaluation closures |

---

## Phase S9 — Performance Enforcement Addendum

> **Status:** Mandatory completion layer for V2, not an optional appendix.
>
> **Purpose:** This phase extends S2, S3, S5, S6, and S7 with concrete
> architectural forms that enforce performance structurally. The earlier phases
> remain authoritative about subsystem shape and pipeline composition; this
> phase defines how those phases become performance-enforced rather than merely
> performance-aware.

### S9.1 — Performance Enforcement Model

Performance in `forge-signal` is enforced in three layers:

1. **Compile-time enforced** through types, ownership, capability boundaries,
   lifecycle scopes, and API surface shape.
2. **Policy/runtime enforced** through resolved strategies and explicit mode
   selection performed before the hot path.
3. **Counter/test enforced** through boundary-local counters, certification
   workloads, and scale-sensitive regression checks.

The default rule is simple:

> If a performance law can be enforced by shape, `forge-signal` should enforce
> it by shape rather than by reviewer memory.

This addendum exists because the current V2 design already points in this
direction, but still leaves too many performance-critical truths encoded as raw
collections, late branching, or optional discipline.

### S9.2 — Proof-Bearing Pipeline Forms

Once a phase establishes a costly fact, later phases must consume that fact as
an explicit proof-bearing form instead of rediscovering it.

The required architectural families are:

- `Canonical*` for canonicalized, sorted, or deduplicated collections
- `Lowered*` for planner-to-execution lowered forms
- `Resolved*` for policy or strategy selected before execution
- `*Delta` for semantically narrowed change payloads
- `*Summary` for batch-derived structural proofs

Examples that should exist in Signal after this rewrite:

```rust
pub struct CanonicalDependencies(SmallVec<[DependencyEdge; 8]>);
pub struct CanonicalChangedRegions(SmallVec<[ChangedRegion; 4]>);
pub struct DirtyDelta { ... }
pub struct TouchedScopeSummary { ... }
pub struct LoweredStagePlan { ... }
pub struct ResolvedExecutionStrategy { ... }
```

Normative rule:

- later phases may not accept raw `Vec`s if an earlier phase already proved
  canonical order, deduplication, or narrowing
- planner and apply code may not re-sort, re-deduplicate, or re-canonicalize
  collections that should already be in proof-bearing form
- performance-sensitive paths must consume `Canonical*`, `Lowered*`,
  `Resolved*`, `*Delta`, and `*Summary` forms directly

This is the main architectural encoding for laws `10`, `22`, `25`, and `26`.

### S9.3 — Contracts That Must Grow Beyond Current S2

`NodeContract` is no longer only a dependency/read declaration. It becomes the
central performance contract for a node.

The V2 `NodeContract` form from S2 is extended to include:

```rust
pub struct NodeContract {
    pub reads: AspectMask,
    pub produces: AspectMask,
    pub partition_scope: Option<Vec<PartitionSubscription>>,
    pub required_context: Option<ContextRequirement>,
    pub projection_contract: ProjectionContract,
    pub equivalence: EquivalenceContract,
    pub path_class: PathClass,
    pub maintenance_mode: MaintenanceMode,
    pub artifact_policy: ArtifactPolicyClass,
    pub authority_policy: AuthorityPolicy,
}
```

Required supporting forms:

```rust
pub struct ProjectionContract {
    pub consumes: AspectMask,
    pub consumes_partitions: Option<Vec<PartitionSubscription>>,
}

pub struct EquivalenceContract {
    pub identity_basis: IdentityBasis,
    pub suppression_basis: SuppressionBasis,
    pub canonical_dependency_order: CanonicalDependencyOrder,
    pub comparator_basis: ComparatorBasis,
}

pub enum PathClass {
    Operational,
    Rich,
}

pub enum MaintenanceMode {
    IncrementalOnly,
    RebuildAllowed,
    DensityAdaptive,
}

pub enum ArtifactPolicyClass {
    OperationalMinimal,
    DevelopmentRetained,
    ForensicReconstructable,
}

pub enum AuthorityPolicy {
    AuthoritativeOnly,
    SpeculativeThenReconcile,
}
```

These forms are required because reuse, suppression, caching, and hot/cold path
separation must not remain distributed across output identity, continuity
tokens, comparator policy, diagnostics profile, and incidental code branching.

This section amends S2 directly. After S9 lands:

- S2 becomes the home of equivalence contracts and path classification
- planner pruning still uses `reads`, but apply, projection, and reuse behavior
  also consume `projection_contract`, `equivalence`, `path_class`,
  `maintenance_mode`, `artifact_policy`, and `authority_policy`
- `CanonicalDependencyOrder` is a contract field, not an implementation detail
- authority handling is an explicit contract choice, not an emergent runtime
  habit
- write-path contracts and read-path contracts are duals of the same truth and
  must be declared on types rather than rediscovered at runtime
- framework-owned resources must be declared through contract-bearing
  registration forms rather than scattered coordination calls

Identity representation is also part of the performance contract, not an
incidental storage detail. Performance-critical runtime entities should default
to dense arena-backed generational IDs rather than rich object identity or
pointer-shaped handles:

```rust
pub struct NodeId(u32);
pub struct SegmentId(u32);
pub struct SnapshotId(u32);
```

Normative rule:

- performance-critical runtime domains default to generational arena IDs
- ID width is an architectural decision because cache density, bandwidth, and
  adjacency storage density change with it
- wider or richer identity forms require explicit justification at the phase
  that introduces them

Speculative application is also part of the performance contract. If a node or
path can cheaply reconcile with authoritative truth, the architecture should
prefer reflecting speculative effects immediately rather than waiting for final
authority before showing any state movement.

Normative rule:

- authority policy must be explicit in the contract or resolved path policy
- paths that can speculatively apply and cheaply reconcile should default to
  `SpeculativeThenReconcile`
- `AuthoritativeOnly` is reserved for domains whose semantics or failure mode
  make speculative reflection unacceptable

This is the architectural attachment point for the speculative-then-reconcile
law, and it also reinforces laws `3`, `8`, `20`, `21`, and `34`.

This is the main architectural attachment point for law `36`, and it also
reinforces laws `7`, `19`, `31`, and `33`.

This is the primary encoding for laws `7`, `19`, `20`, `28`, and `29`.

### S9.4 — Canonical Collections and Narrowed Deltas

Hot-path structural operations currently still move too much meaning through raw
lists. That stops here.

The following wrapper types are mandatory for the next architecture rewrite:

```rust
pub struct CanonicalDependencies(SmallVec<[DependencyEdge; 8]>);
pub struct CanonicalChangedRegions(SmallVec<[ChangedRegion; 4]>);
pub struct DedupedNodeBatch(SmallVec<[NodeId; 16]>);
pub struct SortedSourceBatch(SmallVec<[NodeId; 16]>);
pub struct DirtyDelta {
    pub changed_aspects: AspectMask,
    pub changed_regions: CanonicalChangedRegions,
    pub touched_nodes: DedupedNodeBatch,
}
pub struct StructuralDelta { ... }
pub struct DesiredState<T> { ... }
pub struct PatchPlan { ... }
pub struct PendingSnapshotBatch(SmallVec<[PendingDependencySnapshot; 16]>);
pub struct SubscriberRepairBatch(SmallVec<[SubscriberRepair; 16]>);
```

Normative rule:

- topology, snapshot, invalidation, and batch-commit paths must accept these
  wrapper forms rather than raw `Vec`s
- `DirtyDelta` is the only acceptable invalidation input after narrowing
- batch maintenance should consume `PendingSnapshotBatch`,
  `SubscriberRepairBatch`, and `SortedSourceBatch` directly
- when computing desired truth is cheaper than applying it, producers emit
  `DesiredState<T>` and the framework owns `StructuralDelta` and `PatchPlan`
- cross-phase facts such as touched scope, contract masks, and topological
  impact must be derived exactly once at the batch boundary as `*Summary` forms

Relationship storage must also follow traversal shape rather than normalization
shape. Signal is a directional graph runtime, so dependency and subscriber
relationships are stored to serve graph traversal, invalidation, rewiring, and
batch repair directly:

```rust
pub struct DependencyAdjacency { ... }   // node -> dependencies
pub struct SubscriberAdjacency { ... }   // source -> subscribers
pub struct SnapshotAdjacency { ... }     // node -> dependency snapshot
```

Normative rule:

- relationship storage must match dominant traversal pattern
- Signal must not normalize graph relationships as though they were relational
  join tables if the hot path consumes them directionally
- dependency, subscriber, and snapshot storage should optimize for traversal
  locality, batch rewiring, and narrow repair rather than abstract schema
  symmetry

This is the main architectural attachment point for law `37`, and it also
reinforces laws `6`, `10`, `22`, `25`, `30`, and `31`.

This section extends S5 and strengthens S2/S6. It is the main architectural
encoding for laws `1`, `6`, `10`, `22`, `25`, and `26`.

### S9.5 — Lowered Stage Plans as the Only Execution Input

The planner must not hand execution a loosely interpreted bag of tasks. Serial
and parallel execution must consume the same lowered form.

Required architectural forms:

```rust
pub struct LoweredStagePlan {
    pub tasks: Vec<LoweredTask>,
    pub apply_groups: Vec<DisjointApplyGroup>,
    pub dirty_delta: DirtyDelta,
    pub execution_strategy: ResolvedExecutionStrategy,
    pub maintenance_strategy: ResolvedMaintenanceStrategy,
    pub authority_policy: AuthorityPolicy,
    pub decision_summary: DecisionSummary,
}

pub struct LoweredTask {
    pub node: NodeId,
    pub contract: NodeContract,
    pub projection_contract: ProjectionContract,
    pub dependency_inputs: CanonicalDependencies,
    pub rewiring: Option<RewiringPlan>,
    pub path_class: PathClass,
    pub authority_policy: AuthorityPolicy,
}

pub struct DisjointApplyGroup {
    pub tasks: Vec<LoweredTask>,
    pub footprint: ApplyFootprint,
}

pub struct ApplyFootprint {
    pub touched_nodes: DedupedNodeBatch,
    pub touched_sources: SortedSourceBatch,
    pub locality: LocalityFootprint,
}
```

Execution must consume `LoweredStagePlan`, not re-decide:

- strategy
- artifact path class
- maintenance mode
- dependency canonicalization
- rewiring intent
- parallel admission safety
- whether the stage is speculative-first or authority-blocking
- which policy decisions were already resolved and recorded in the decision log

Typestate also applies here. The target architecture is not just “typed data,”
but phase-typed construction:

```rust
pub struct CandidateTask { ... }
pub struct EligibleTask { ... }
pub struct LoweredTask { ... }
pub struct ExecutedTask { ... }
```

Illegal transitions between these forms must be uncallable, not merely checked
after the fact. Invalid operational states are architecture bugs.

This is the strongest protection against serial/parallel drift and late
branching. It extends S5 directly and encodes laws `3`, `8`, `16`, and `21`.

### S9.6 — Operational Effect vs Diagnostic Envelope

S3 currently proposes `EvaluationEffect` as a single effect shape. That is no
longer sufficient.

The architecture now distinguishes:

```rust
pub struct DomainEffect {
    pub primary_result: OperationalEffect,
    pub structural_delta: StructuralDelta,
    pub rollback_effect: RollbackEffect,
}

pub struct OperationalEffect {
    pub node: NodeId,
    pub aspect_version: AspectVersion,
    pub output_change: OutputChange,
    pub dependency_snapshot: DependencySnapshot,
    pub meaningful_input_changes: u32,
    pub verdict: EvaluationVerdict,
}

pub struct SpeculativeEffect {
    pub operational: OperationalEffect,
    pub authority_policy: AuthorityPolicy,
}

pub struct ReconciliationOutcome {
    pub node: NodeId,
    pub confirmed: bool,
    pub adjusted: bool,
    pub rolled_back: bool,
}

pub struct RollbackEffect {
    pub node: NodeId,
    pub undo_patch: PatchPlan,
}

pub struct DiagnosticEnvelope {
    pub changed_regions: CanonicalChangedRegions,
    pub labels: SmallVec<[String; 2]>,
    pub output_identity: Option<OutputIdentity>,
    pub continuity_token: Option<OutputIdentity>,
    pub memoized_origin: MemoizedResultOrigin,
}
```

Normative rule:

- the hot path commits `OperationalEffect`
- if the active authority policy is `SpeculativeThenReconcile`, the hot path may
  commit `SpeculativeEffect` first and reconcile with authoritative truth later
- reconciliation must be cheap, explicit, and structurally separated from rich
  diagnostics
- domain handlers produce declarative `DomainEffect` truth; the framework
  derives rollback, observability routing, and publication from that effect
- rollback must be structurally derivable from `RollbackEffect` and effect
  records alone
- rich diagnostics are retained or reconstructed only if the resolved
  `ArtifactPolicyClass` allows it
- `DiagnosticEnvelope` must not be required for purely operational execution

`EvaluationEffect` may remain as an internal assembly convenience during
transition, but the target architecture is explicit operational/rich path
separation plus explicit speculative/authoritative reconciliation.

This section extends S3 and encodes laws `2`, `15`, and `20`.

### S9.7 — Boundary Envelopes and Decision Logs

Every boundary crossing in Signal must produce a self-describing envelope, and
every authority-path decision must be structurally recorded.

Semantic-purity rule:

- every truth-bearing field and accessor must mean exactly one thing
- ids are truth; labels are display metadata
- derivation, restoration, invalidation, and presentation must not be merged
  into one convenience field or helper
- if a helper collapses more than one ontology, it is an architecture defect,
  not a convenience

Required forms:

```rust
pub struct TransactionResult {
    pub outcome: TransactionOutcome,
    pub warnings: Vec<AdvisoryRecord>,
    pub decision_summary: DecisionSummary,
    pub decision_log: DecisionLog,
    pub integrity_markers: IntegrityMarkers,
    pub performance_accounting: PerformanceCounterSurface<'static>,
}

pub enum DecisionDetail {
    TransactionOutcome { outcome: TransactionOutcome },
    StageAuthorityPolicy { authority_policy: AuthorityPolicy },
    StageParallelAdmission { admission_reason: String },
    Rollback { reason: String },
    Failure { phase: ExecutionFailurePhase, message: String },
}

pub struct DecisionRecord {
    pub stage_index: Option<u32>,
    pub detail: DecisionDetail,
}
```

Normative rule:

- a consumer must be able to reconstruct an operation from its envelope without
  querying producer internals
- conflict resolution, invariant overrides, cascade triggers, and authority
  outcomes must be recorded in `DecisionLog`
- `DecisionSummary` is the batch-derived form that crosses phase boundaries; the
  full log remains queryable and span-aware

This section extends S3 and S13 and encodes laws `7`, `8`, and `32`.

### S9.7.a — Lineage Semantic Purity

Signal lineage is artifact lineage over time within a branched execution
runtime, not execution logging.

Required lineage laws:

- stable artifact identity remains explicit through `LineageArtifactId`
- true derivational parentage remains distinct from restoration reference,
  invalidation reference, and display metadata
- branch ids are truth; branch names are optional presentation only
- invalidation cause must converge toward typed causality, not free-form text
- restore records must stay semantically distinct from recomputation
- UI labels may summarize lineage, but must never define lineage semantics

Normative rule:

- no lineage accessor may return “some related artifact” under a parentage name
- if a record needs multiple artifact relations, those relations must be named
  separately and precisely
- a lineage surface is incomplete if it requires string parsing to recover
  branch identity or invalidation cause

This section extends S5 and S13 and encodes laws `8`, `15`, and `32`.

### S9.8 — Cardinality-Matched API Surface

S7 now has a normative API-shape rule:

> Bulk semantics must cross subsystem boundaries as bulk types.

Required batch-first operational forms:

```rust
pub struct DependencyBatchEdit { ... }
pub struct DirtyBatch { ... }
pub struct SemanticBatchCommit { ... }
pub struct SnapshotBatchCommit { ... }
```

Normative rule:

- scalar orchestration over semantically bulk work is forbidden as a primary API
  surface
- scalar mutation helpers may exist only as low-level implementation detail
- batch/session APIs are the operational contract

This extends S7 directly and encodes laws `6`, `9`, `17`, and `18`.

### S9.9 — Locality and Parallel Disjointness Contracts

Locality and parallel safety must be represented structurally, not inferred
late.

Required architectural forms:

```rust
pub struct LocalityFootprint {
    pub partitions: PartitionScopeSet,
    pub nodes: DedupedNodeBatch,
    pub sources: SortedSourceBatch,
}

pub struct PartitionScopeSet(SmallVec<[PartitionSubscription; 8]>);
```

Normative rule:

- parallel apply admission must depend on `DisjointApplyGroup` and
  `ApplyFootprint`, not only executor policy
- locality boundaries must travel with lowered plans and repair summaries
- touched or affected scope must be carried as `*Summary` or scope-set forms
  instead of rediscovered later
- relationship storage and apply-group formation must be aligned so the same
  directional adjacency forms support both traversal locality and conflict
  detection instead of forcing normalized reassembly at admission time

This extends S5 and S6 and encodes laws `5`, `21`, `30`, and `33`.

### S9.10 — Allocation Lifetime and Single-Consumer Flow

Lifecycle scope must become visible in the architecture, not only in local
scratch helpers.

Required workspace families:

```rust
pub struct GraphScratch { ... }
pub struct SessionScratch { ... }
pub struct StageScratch { ... }
pub struct TransactionScratch { ... }
```

Normative rule:

- hot-path work must execute within an explicit lifecycle-managed scope
- effect packets and structural batches should be move-oriented and non-`Clone`
  by default
- if a packet is `Clone`, the architecture must justify the second observer of
  the pre-mutation truth
- framework-owned resources such as computations, subscriptions, observers,
  projections, and caches must be registered and disposed through framework
  lifecycle boundaries, not consumer convention

Supporting form:

```rust
pub struct SingleConsumer<T>(T);
```

`SingleConsumer<T>` is not a required literal final name, but the architecture
must provide an equivalent move-only signal for pipeline packets whose cloning
would be structural waste.

This extends S6 and encodes laws `24`, `31`, `32`, and `35`.

### S9.11 — Fast-Exit and Phase-Typed Eligibility

Cheap rejection must happen before expensive construction.

Required pipeline progression:

```rust
pub struct CandidateTask { ... }
pub struct EligibleTask { ... }
pub struct LoweredTask { ... }
pub struct ExecutedTask { ... }
```

Normative rule:

- admission, contract mismatch, path mismatch, and cheap disqualifiers must be
  evaluated before dependency-input assembly, artifact shaping, rewiring
  planning, or apply-group construction
- later pipeline phases may only consume `EligibleTask` or `LoweredTask`, never
  raw candidates

This extends S5 and S6 and encodes law `34`.

### S9.12 — Authority, Derivation, Checkpoints, and Reconstructability

Authoritative truth and derived runtime state are categorically different
objects. Derived state must be reproducible from authority alone.

Required forms:

```rust
pub struct AuthorityState { ... }
pub struct DerivedState { ... }
pub struct CheckpointRecord { ... }
pub struct JournalSegment { ... }
```

Normative rule:

- every derived structure in Signal must be destroyable and rebuildable from
  `AuthorityState` plus a bounded journal since the last checkpoint
- speculative state is never authoritative; it is either confirmed, adjusted,
  or rolled back during reconciliation
- checkpoints and journals are independent subsystems with independent
  lifecycles and must not require write-path suspension to exist

This section extends S3, S6, and S7 and encodes laws `19`, `33`, and `36`.

### S9.13 — Measurement Boundaries Required by Architecture

Because S9 defines all three enforcement layers, the architecture itself must
name the mandatory measurement boundaries and counters that every rewritten path
must expose.

Required architecture-level counters and certification hooks:

- batch width
- dirty delta breadth
- rewiring count
- snapshot batch size
- subscriber repair breadth
- incremental vs rebuild choice
- apply-group width
- apply-group disjointness statistics
- hot-path artifact retention count
- hot-path artifact reconstruction count
- decision-log event count
- checkpoint size and journal replay span
- structural-delta size and patch-application breadth

Normative rule:

- a rewritten path is not complete until these counters exist at the boundary
  the path claims to optimize
- serial and parallel lowered execution paths must emit comparable counters
- performance certification workloads must consume these counters alongside
  elapsed time so claims remain interpretable

This section ties S9 back to the performance baseline and encodes the
architecture-level part of law `13`.

### S9.14 — Concrete Integration with Existing Phases

This addendum modifies earlier phases as follows:

- **S2** becomes the home of `NodeContract`, `EquivalenceContract`,
  `ProjectionContract`, `PathClass`, `MaintenanceMode`, authority policy, and
  artifact-path classification.
- **S3** becomes the home of `DomainEffect`, `OperationalEffect`,
  `DiagnosticEnvelope`, boundary envelopes, decision logs, and effect-application
  capability boundaries.
- **S5** becomes the home of `Canonical*`, `Lowered*`, `Resolved*`, `*Delta`,
  `*Summary`, disjoint apply groups, and adaptive incremental/rebuild choice.
- **S6** becomes the home of lifecycle-scoped scratch types, single-consumer
  packet rules, checkpoints/journals, and phase-typed eligibility progression.
- **S7** becomes the home of batch-first operational APIs and the explicit
  demotion of scalar mutation surfaces plus framework-owned resource lifecycle.

### S9.15 — Branched Runtime Reconciliation and Merge Lineage

Branching is incomplete without a way to reconcile accepted branch-local work
back into an authoritative branch.

Required future completion:

```rust
pub struct BranchMergeResult {
    pub source_branch: SignalBranchId,
    pub target_branch: SignalBranchId,
    pub adopted_artifacts: DedupedArtifactBatch,
    pub replaced_artifacts: DedupedArtifactBatch,
    pub merge_kind: BranchMergeKind,
}

pub enum BranchMergeKind {
    FastForward,
    ReplayApplied,
    ConflictResolved,
}
```

Normative rule:

- `MergedFrom` must not be emitted as a decorative branch event without real
  merge semantics
- merge lineage must eventually include both branch-level reconciliation and
  artifact-level adoption/replacement semantics
- speculative or branch-local parallel work is not complete until accepted work
  can reconcile back into the authoritative branch through an explicit merge
  boundary

This section extends S5 and S9.7.a and encodes laws `7`, `15`, and `32`.

### S9.16 — Geometry-Kernel Performance Hardening Program

If `forge-signal` is expected to support geometry kernels for next-generation
aircraft, performance hardening must target the real failure modes of that
workload rather than generic “incremental runtime” benchmarks.

Geometry-kernel pressure profile:

- very large dependency graphs
- high fan-out invalidation over localized semantic deltas
- expensive artifacts with large retained payloads
- partitioned or region-scoped outputs whose changed frontier is much smaller
  than the object carrying them
- strong replay, lineage, and explainability requirements under industrial
  debugging and certification pressure

Normative rule:

- no hot-path optimization may trade away reconstructability, transactional
  rollback, semantic locality, or provenance truth
- no diagnostic surface may remain on the operational hot path by accident
- no geometry-kernel performance claim is valid without scale-sensitive
  certification workloads that exercise large artifact graphs and hostile
  branch/restore/replay history

Required workstreams:

#### S9.16.1 — Hot/Cold Artifact Separation

Operational state, retained diagnostic artifacts, and historical explainability
must become physically and architecturally separate lanes.

Required target forms:

```rust
pub struct RuntimeArtifactState { ... }
pub struct RetainedDiagnosticArtifact { ... }
pub struct HistoricalArtifactRecord { ... }
```

Normative rule:

- apply, invalidation, and planner hot paths must consume only
  `RuntimeArtifactState`
- retained explanations, lineage expansions, and replay attachments must be
  stored in a cold path that can be degraded by policy
- large geometry artifacts must not be cloned merely to satisfy diagnostics or
  retained history

#### S9.16.2 — Structural-Sharing Snapshots and Dependency State

Snapshotting and dependency-state retention must stop scaling with whole owned
payload size when the semantic delta is narrow.

Required target forms:

```rust
pub struct SharedDependencySnapshot { ... }
pub struct SnapshotDeltaRecord { ... }
pub struct ArtifactRetentionPolicy { ... }
```

Normative rule:

- dependency snapshots and snapshot restores must converge toward
  structural-sharing or delta-oriented storage
- restoring a prior artifact identity, rewinding active state, and seeding
  recomputation from prior state must remain distinct semantics even if they
  share storage internals
- whole-snapshot clone behavior is acceptable only for compact or explicitly
  bounded profiles, never as the universal geometry-kernel path

#### S9.16.3 — Locality-First Invalidation and Frontier Execution

The invalidation engine must evolve from batch-amortized traversal into a true
locality-first multi-source frontier engine.

Required target forms:

```rust
pub struct InvalidationFrontier { ... }
pub struct FrontierWave { ... }
pub struct NarrowedPropagationSet { ... }
```

Normative rule:

- invalidation must union overlapping sources before broad downstream
  propagation whenever the semantic delta allows it
- propagation cost must scale with narrowed locality frontier, not with caller
  mutation count or broad graph size
- partition-aware and aspect-aware frontier narrowing must be measurable at the
  invalidation boundary

#### S9.16.4 — Geometry-Scale Equivalence and Reuse Contracts

Reuse is mandatory for geometry scale, but it must stay truth-grade.

Required target forms:

```rust
pub struct ArtifactEquivalenceContract { ... }
pub struct ReuseBasis { ... }
pub struct ReuseCertificationRecord { ... }
```

Normative rule:

- expensive artifact reuse, suppression, memoization, and cache hits must be
  justified by explicit artifact equivalence contracts
- reuse must record whether the result came from fresh computation,
  memoized reuse, snapshot restore, or authoritative reconciliation
- geometry kernels must be able to certify that reused artifacts did not cross
  invalid semantic boundaries such as topology regime, tolerance regime, or
  semantic region identity

#### S9.16.5 — Diagnostic Tiering Without Semantic Drift

Diagnostics must be tiered by policy without changing the operational meaning
of the run.

Required target forms:

```rust
pub struct DiagnosticsTier { ... }
pub struct RetentionBudget { ... }
pub struct ReconstructionBudget { ... }
```

Normative rule:

- operational, development, and forensic profiles may differ in retained
  richness but must not differ in execution semantics
- a lower diagnostics tier may reconstruct less history, but it must not report
  different truth for the same boundary envelope or lineage event
- reconstruction work must be budgeted explicitly rather than happening as
  hidden lazy cost on first access

#### S9.16.6 — Geometry Certification and Performance Proof Harness

The runtime must earn geometry-kernel credibility with hostile certification
workloads, not with microbench optimism.

Required certification families:

- large CAD-style dependency graphs with localized topology edits
- expensive partitioned artifact recomputation with memoized reuse
- branch/restore/replay histories with lineage and explain equivalence checks
- serial/parallel equivalence under disjoint geometry subgraphs
- snapshot suffix replay equivalence at industrial artifact breadth

Mandatory measurement boundaries:

- invalidation frontier width and narrowed frontier width
- dependency snapshot retained bytes and shared-bytes ratio
- artifact retention bytes by tier
- hot-path allocation count and hot-path retained bytes
- explanation reconstruction count and reconstruction bytes
- memoized reuse hit rate by artifact family
- replay suffix cost by checkpoint span
- branch restore and merge reconciliation breadth

Execution order:

1. hot/cold artifact separation
2. structural-sharing snapshot and dependency-state redesign
3. locality-first invalidation frontier execution
4. geometry-scale equivalence and reuse certification
5. diagnostics tiering and reconstruction budgeting
6. geometry certification harness and slope reporting

Anti-goals:

- do not hide broad artifact clones behind ergonomic getters
- do not make forensic richness the default operational cost
- do not claim geometry readiness from elapsed-time wins alone
- do not introduce compatibility shims that preserve cost-dishonest surfaces
- do not let bridge or host integration become the only place where artifact
  causality and reuse truth can be recovered

This section extends S5, S6, S9.6, S9.9, S9.10, S9.12, and S9.13 and encodes
laws `1`, `2`, `5`, `7`, `10`, `12`, `20`, `21`, `24`, `27`, `28`, `29`,
`32`, `35`, and `36`.

This is mandatory. The new structural rules are not advisory preferences.

### Files Modified

| File | Change |
|---|---|
| [signal_architecture2.md](file:///Users/spenstar/Documents/programming/forge%20workspace/Forge/_docs/forge_signal/signal_architecture2.md) | Append S9 addendum; extend S2/S3/S5/S6/S7 by reference |

## What Must Be Preserved

| Pattern | Current Location | Must Remain |
|---|---|---|
| `transition_clean/dirty/maybe_stale` | `NodeEntry` | Canonical state machine, used in 5+ call sites |
| `PartitionScoped` + `scopes_overlap` | `output.rs` | Single partition-matching semantic |
| `DependencySnapshotEntry` + `DependencySortKey` | `dependency.rs` | Named fields, canonical sort key |
| `with_scratch` closure | `SignalGraph` | RAII scratch lease, no manual acquire/restore |
| `reconcile_dependencies` | `EdgeTopology` (after S1) | Declarative dependency management |
| `InvalidationTraversal` pipeline | `routing.rs` | Decomposed invalidation phases |
| `SegmentedStore<T, Id>` | `segmented.rs` | Unified edge store |
| Telemetry sub-structs | `telemetry.rs` | 7 domain-grouped counters |
| `replay_where` | `replay.rs` | Filter consolidation |
| `SparsePatchBuffer` rollback | `patch_buffer.rs` | Transaction undo mechanism |
| Branch capture/restore symmetry | `runtime_state.rs` | Fork/join on branch state |
| `CheckpointRuntime` separation | `checkpoint/` | Checkpoint stays independent of graph |

---

## Sequencing

### Dependency Rules

```
S1 (subsystem split) → unlocks S3, S5, S6
S2 (contracts) → unlocks S5.1 (contract-driven pruning), S8.1 (ambient context)
S3 (declarative effects) → unlocks S4 (transaction result envelope), S3.5 (defineComputation)
S6.1 (partition versions) → should precede S5 (pipeline perf)
S6.5 (error hierarchy) → should precede S6.6 (builder completeness)
S7 (facade) → after S1–S4 stabilize
S8 (context) → after S2.4 (context in contracts) and S3.5 (defineComputation)
S9 (performance enforcement addendum) → extends S2/S3/S5/S6/S7 and should be
written before any V2.1 rewrite work begins
```

### Recommended Execution Order

```text
Batch 1 — Structural Foundation
  S1.1  SignalGraph subsystem split (NodeArena, EdgeTopology, etc.)
  S1.2  BranchManager extraction from SignalRuntime
  S6.5  Typed error hierarchy (enables match-based error handling throughout)

Batch 2 — Effect Pipeline
  S3.1  EvaluationEffect struct
  S3.2  apply_effect pipeline (replaces result_apply monolith)
  S3.3  Commit ceremony extraction (fail_and_rollback)
  S3.4  EvaluationVerdict (three-state outcome enum)

Batch 3 — Contract System
  S2.1  NodeContract trait + NodeBuilder integration
  S2.2  Contract duality documentation
  S2.3  Aspect-aware planner pruning
  S2.4  Context-type in contracts (ContextRequirement)

Batch 4 — Correctness
  S6.1  PartitionVersionMap (fixes over-evaluation bug)
  S6.3  EdgeTopology bidirectional consistency assertion
  S6.4  Edge store rollback tracking
  S6.6  Builder completeness (typestate on SignalRuntimeBuilder)

Batch 5 — Transaction Surface
  S4.1  TransactionResult envelope
  S4.2  SemanticDelta consolidation (named replay entries)

Batch 6 — Computation Model
  S8.1  Ambient evaluation context (Ctx threaded to evaluation closures)
  S8.2  Multi-context design decision (Option A: homogeneous per runtime)
  S3.5  defineComputation pattern (ComputationSpec)
  S5.5  Execution path collapse (merge commit/on-demand paths)

Batch 7 — Performance
  S5.1  Contract-driven plan pruning
  S5.2  EvaluationSession as primary path (zero-alloc completion)
  S5.3  Execution pipeline subsystem decomposition + cfg isolation
  S5.4  Subsystem-scoped amortized GC

Batch 8 — Safety & Surface
  S6.2  GraphObserver (phase-typed observation)
  S7.1  Grouped facade namespaces
  S7.2  State-derived evaluation strategy
  S8.3  Context-scoped evaluation (framework-owned context lifetime)

Batch 9 — Performance Enforcement Rewrite Layer
  S9.1  Performance enforcement model
  S9.2  Proof-bearing pipeline forms
  S9.3  Performance-expanded NodeContract
  S9.4  Canonical collections and narrowed deltas
  S9.5  LoweredStagePlan as the only execution input
  S9.6  OperationalEffect / DiagnosticEnvelope split
  S9.7  Boundary envelopes and decision logs
  S9.8  Cardinality-matched API surface
  S9.9  Locality and disjoint parallel apply contracts
  S9.10 Allocation lifetime scopes and single-consumer packets
  S9.11 Fast-exit and phase-typed eligibility
  S9.12 Authority, derivation, checkpoints, and reconstructability
  S9.13 Architecture-mandated measurement boundaries
  S9.15 Branched runtime reconciliation and merge lineage
  S9.16 Geometry-kernel performance hardening program
```

### Practical Rule

The same rule as the relational doc: if there is tension between "clean every layer" and "reach performance work quickly," do the minimum structural work that prevents performance work from encoding bad invariants. That means:

- Do **not** start S5 before S1 (subsystem split prevents false borrow conflicts in execution decomposition)
- Do **not** start S5 before S6.1 (don't optimize a pipeline that over-evaluates)
- Do **not** start S7 before S1–S4 (API surface should reflect stabilized internals)
- **Do** start S3 immediately after S1 (effect pipeline is high-impact, low-risk)

> [!IMPORTANT]
> Run `cargo test -p forge-signal` after **every individual refactor**, same as V1. A regression caught early maps to exactly one change.

---

## Summary

| Phase | Items | Key Pattern Source |
|---|---|---|
| S1 | Subsystem decomposition | Relational C1 (god struct split) |
| S2 | NodeContract + contract duality + context requirements | Relational F2 + D4 · Frontend `ProjectContextService` |
| S3 | EvaluationEffect + verdicts + `defineComputation` | Relational B5 + D5 · Frontend `defineCrudResource` |
| S4 | TransactionResult envelope | Relational E1/E2 (commit result envelope) |
| S5 | Pipeline + performance + path collapse | Relational D4 · Frontend component collapse |
| S6 | Safety + error hierarchy + builder completeness | Relational A3 + C3 + C4 |
| S7 | Facade + state-derived strategy | Relational F1 + D6 |
| S8 | Context-aware computation (ambient, multi-mode) | Frontend `ProjectContextService` / operations mode |
| S9 | Performance enforcement addendum (proof-bearing forms, lowered execution, bulk-first API shape) | Relational performance enforcement model + proof-bearing pipeline forms |

# API Surface

This is the practical public surface for `forge-signal`.

It is intentionally biased toward the entrypoints people actually need, including the niche ones that tend to disappear from memory.

## 1. Build a graph

Primary type:

- `SignalGraph`

Common node-building surface:

- `graph.node()`
- `NodeBuilder::build()`
- `NodeBuilder::on_demand()`
- `NodeBuilder::always()`
- `NodeBuilder::debounce(...)`
- `NodeBuilder::aspect_filter(...)`
- `NodeBuilder::delta_threshold(...)`
- `NodeBuilder::custom_condition(...)`
- `NodeBuilder::output_identity()`
- `NodeBuilder::partitioned_output()`
- `NodeBuilder::tolerance(epsilon)`
- `NodeBuilder::comparator(...)`

For the full condition/comparator story, including custom condition keys and tolerance semantics, see [CONDITIONS_AND_COMPARATORS.md](./CONDITIONS_AND_COMPARATORS.md).

Dependency wiring:

- `graph.add_dependency(downstream, upstream, aspect)`
- `graph.add_partition_dependency(downstream, upstream, aspect, partition)`
- `graph.add_partition_detail_dependency(downstream, upstream, aspect, partition, detail)`
- `graph.remove_dependency(...)`

Evaluation-result continuity hooks:

- `NodeEvaluationResult::with_output_identity(...)`
- `NodeEvaluationResult::with_continuity_token(...)`
- `NodeEvaluationResult::with_output_change(...)`

Mutation/invalidation:

- `mark_dirty(graph, node, aspect)`
- `mark_dirty_with_regions(graph, node, aspect, &[ChangedRegion])`

### Example: partition-aware node graph

```rust
use forge_signal::facade::*;

let mut graph = SignalGraph::new();
let source = graph.node().output_identity().build();
let shell = graph.node().partitioned_output().tolerance(1).build();
let target = graph.node().build();

graph.add_partition_dependency(shell, source, Aspect::new(0), "shell")?;
graph.add_dependency(target, shell, Aspect::new(0))?;
# Ok::<(), SignalError>(())
```

## 2. Choose runtime policy

Primary type:

- `SignalRuntimePolicy`

Core presets:

- `operational()`
- `development()`
- `forensic()`

Deployment presets:

- `web_development()`
- `game_engine()`
- `fintech()`
- `kernel()`

Important overrides:

- `.with_explanation_retention(...)`
- `.with_provenance_retention(...)`
- `.with_replay_detail(...)`
- `.with_semantic_retention(...)`
- `.with_parallel_admission(...)`
- `.with_history_limit(...)`
- `.with_detail_limit(...)`
- `.with_history_details(...)`

### Example: operational runtime with explicit reconstruction-only explanation

```rust
use forge_signal::facade::*;

let policy = SignalRuntimePolicy::operational()
    .with_explanation_retention(ArtifactRetentionPolicy::Reconstruct)
    .with_provenance_retention(ArtifactRetentionPolicy::Reconstruct);
```

## 3. Choose executor behavior

Primary type:

- `StageExecutor`

Main constructors:

- `StageExecutor::Serial`
- `StageExecutor::conservative_parallel()`
- `StageExecutor::balanced_parallel()`
- `StageExecutor::aggressive_parallel()`
- `StageExecutor::parallel(min_stage_width)`
- `StageExecutor::full_parallel(min_stage_width)`

Advanced executor policy:

- `ParallelExecutionPolicy::new(min_stage_width_nonzero)`
- `.with_worker_count(...)`
- `.with_chunk_size(...)`
- `.with_apply_group_min_width(...)`
- `.with_max_concurrent_apply_groups(...)`

Recommended shortcut:

- `conservative_parallel()` for request-driven or observability-heavy systems
- `balanced_parallel()` for general production use
- `aggressive_parallel()` for heavy compute or hostile certification

### Example: aggressive full-parallel testing executor

```rust
use std::num::NonZeroUsize;
use forge_signal::facade::*;

let policy = ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
    .with_worker_count(4)
    .with_chunk_size(1)
    .with_apply_group_min_width(1)
    .with_max_concurrent_apply_groups(4);

let executor = StageExecutor::full_parallel(1).with_parallel_policy(policy);
```

## 4. Execute from `SignalGraph`

Primary methods:

- `build_evaluation_plan(targets, request_mode)`
- `execute_prepared_plan(plan, precompute)`
- `execute_prepared_plan_with_executor(plan, precompute, executor)`

What the closure receives:

- `NodeId`
- `&ExecutionReadView`

What it returns:

- `PreparedEvaluation`

### Example: build plan first, then execute with explicit executor

```rust
use forge_signal::facade::*;

let plan = graph.build_evaluation_plan(&[target], EvaluationRequestMode::Default)?;
let report = graph.execute_prepared_plan_with_executor(
    &plan,
    &|node, view| {
        let result = if node == source {
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(Aspect::new(0), 1)]),
            ))
        } else {
            let version = view.read_aspect_version(source, Aspect::new(0))?;
            view.finish(NodeEvaluationResult::from_version(version))
        };
        Ok(result)
    },
    StageExecutor::Serial,
)?;
# let _ = report;
# Ok::<(), SignalError>(())
```

## 5. Execute from `SignalRuntime`

Primary type:

- `SignalRuntime`

Common methods:

- `SignalRuntime::builder(graph)`
- `.runtime_policy(...)` on the builder
- `runtime.build_evaluation_plan(...)`
- `runtime.execute_prepared_plan(...)`
- `runtime.execute_prepared_plan_with_executor(...)`
- `runtime.evaluate_with_plan(...)`
- `runtime.evaluate_with_plan_and_executor(...)`
- `runtime.read(...)`
- `runtime.read_with_executor(...)`
- `runtime.evaluate_dirty(...)`
- `runtime.evaluate_dirty_with_executor(...)`

Important runtime-only knobs:

- `set_node_tier(...)`
- `set_tier_policy(...)`
- `set_fallback_comparator(...)`
- `register_computation_family(...)`
- `keyed_node(...)`

### Example: tier-aware runtime

```rust
use forge_signal::facade::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier { Slow }

let mut runtime = SignalRuntime::builder(SignalGraph::new())
    .with_tiers::<Tier>()
    .runtime_policy(SignalRuntimePolicy::fintech())
    .build();

runtime.set_tier_policy(
    TierPolicy::new(
        Tier::Slow,
        DependencyMode::AutoDiscovered,
        DirtyPropagation::Immediate,
        EvaluationTrigger::LazyPull,
    )
    .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
);
```

For full transaction, keyed-node, and tier/checkpoint guidance, see:

- [TRANSACTIONS_AND_KEYED_RUNTIME.md](./TRANSACTIONS_AND_KEYED_RUNTIME.md)
- [CHECKPOINTS_AND_TIERS.md](./CHECKPOINTS_AND_TIERS.md)

## 6. Snapshot, branch, and replay inspection

Primary state-history types:

- `SignalSnapshotV1`
- `SignalSnapshotMeta`
- `SignalBranchHandle`
- `ReplaySlice`
- `LineageRecord`

Important graph/runtime methods:

- `capture_snapshot()`
- `restore_snapshot(...)`
- `create_branch(...)`
- `switch_branch(...)`
- `capture_branch_snapshot(...)`
- `restore_branch_snapshot(...)`
- `known_branches()`
- `branch_handle(...)`
- `branch_ancestry(...)`
- `branch_head_snapshot_id(...)`
- `replay_for_branch(...)`
- `replay_for_node(...)`
- `replay_for_artifact(...)`
- `replay_from_cursor(...)`
- `replay_between(...)`
- `replay_around_snapshot(...)`
- `compare_replay_slices(...)`
- `replay_slices_equivalent(...)`
- `current_lineage_artifact(...)`
- `lineage_chain_for_node(...)`
- `lineage_chain_for_artifact(...)`
- `compare_lineage_records(...)`
- `lineage_records_equivalent(...)`

### Example: inspect snapshot metadata and branch-local replay without restoring

```rust
use forge_signal::facade::*;

let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
let snapshot = runtime.capture_snapshot();

assert_eq!(snapshot.meta().snapshot_id, snapshot.snapshot_id());
assert_eq!(snapshot.meta().branch_id, snapshot.branch_id());

let branch = runtime.current_branch();
let replay = runtime.replay_for_branch(branch.id);

if let (Some(first), Some(last)) = (replay.frames.first(), replay.frames.last()) {
    let bounded = runtime.replay_between(first.cursor, last.cursor);
    assert!(bounded
        .frames
        .iter()
        .all(|frame| frame.cursor >= first.cursor && frame.cursor <= last.cursor));
}
```

For the full Phase 5 story, see:

- [SNAPSHOTS_BRANCHES_AND_REPLAY.md](./SNAPSHOTS_BRANCHES_AND_REPLAY.md)
- [LINEAGE_MODEL.md](./LINEAGE_MODEL.md)

## 7. Transactions

Primary type:

- `SignalTransaction`

Important methods:

- `runtime.begin()`
- `runtime.transaction(ctx, |tx| ...)`
- `tx.mark_dirty(...)`
- `tx.mark_dirty_with_regions(...)`
- `tx.evaluate_with_plan(...)`
- `tx.evaluate_with_plan_and_executor(...)`
- `tx.execute_prepared_plan_with_executor(...)`
- `tx.read(...)`
- `tx.read_with_executor(...)`
- `tx.evaluate_dirty(...)`

Why this matters:

- execution can be staged with rollback
- semantic artifacts commit or disappear with the transaction outcome
- failure-path behavior is part of the contract, not an accident

## 7. Artifact access

Eager vs reconstructed access is explicit.

Graph methods:

- `graph.explain(node)`
- `graph.explain_artifact(node)`
- `graph.provenance_artifact(node)`
- `graph.retained_explanation_artifact(node)`
- `graph.reconstruct_explanation_artifact(node)`
- `graph.retained_provenance_artifact(node)`
- `graph.reconstruct_provenance_artifact(node)`

For the retained-vs-reconstructed contract, see [ARTIFACT_ACCESS_MATRIX.md](./ARTIFACT_ACCESS_MATRIX.md).

Runtime equivalents:

- `runtime.explain(node)`
- `runtime.retained_explanation_artifact(node)`
- `runtime.reconstruct_explanation_artifact(node)`
- `runtime.retained_provenance_artifact(node)`
- `runtime.reconstruct_provenance_artifact(node)`

When to use what:

- use `retained_*` when you care about eager runtime state
- use `reconstruct_*` when you want deterministic recovery under cheap policies
- use `*_artifact(...)` when you also want materialization mode

## 8. Deployment presets

Primary type:

- `SignalDeploymentPreset`

Variants:

- `WebDevelopment`
- `GameEngine`
- `Fintech`
- `Kernel`

Main method:

- `.recommended() -> SignalDeploymentPlan`

Useful fields on the plan:

- `runtime_policy`
- `executor`
- `summary`
- `certification_command`

## 9. Harness and scenarios

Primary types:

- `SignalScenario`
- `SignalMutationBatch`
- `SignalMutationAction`
- `SignalHarnessRuntimeBuilder`
- `SignalHarnessAdapter`

Common surfaces:

- `SignalScenario::new(...)`
- `.node(...)`
- `.build_node(...)`
- `.dependency(...)`
- `.partition_dependency(...)`
- `.partition_detail_dependency(...)`
- `.input(...)`
- `.observe(...)`
- `.with_evaluator(...)`
- `.fixture()`
- `.request(...)`
- `SignalMutationBatch::new(...)`
- `.mark_dirty(...)`
- `.mark_dirty_with_regions(...)`

This is the part to use when you want parity suites, harness capture, replay summaries, and CI-facing certification fixtures instead of only ad hoc tests.

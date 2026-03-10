# Advanced Patterns

This file is for features that are easy to miss because they are not the first thing you need, but they become extremely important once you do need them.

## Transactions with staged evaluation

If you are looking for the fuller, parameter-forward version of this surface, start with [TRANSACTIONS_AND_KEYED_RUNTIME.md](./TRANSACTIONS_AND_KEYED_RUNTIME.md).

Use transactions when you want graph mutation and semantic artifacts to commit together or roll back together.

### Example

```rust
use forge_signal::facade::*;

let mut runtime = SignalRuntime::builder(SignalGraph::new())
    .runtime_policy(SignalRuntimePolicy::development())
    .build();
let mut ctx = ();

runtime.transaction(&mut ctx, |tx| {
    tx.mark_dirty(source, Aspect::new(0))?;
    tx.evaluate_with_plan(
        target,
        &|node, view| {
            let result = if node == source {
                view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(Aspect::new(0), 2)]),
                ))
            } else {
                let version = view.read_aspect_version(source, Aspect::new(0))?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        },
        EvaluationRequestMode::Default,
    )?;
    Ok(())
})?;
# Ok::<(), SignalError>(())
```

Why use this instead of mutating the graph directly:

- rollback is first-class
- failure diagnostics and replay artifacts stay coherent
- partial semantic leakage is explicitly guarded against

## Keyed nodes and computation families

These are easy to forget if you only ever build anonymous nodes.

Main surfaces:

- `register_computation_family(...)`
- `keyed_node(...)`
- `KeyedComputation`
- `PreparedKeyedContext`
- `StructuralMemoKey`

When this matters:

- memoized families
- keyed reactive caches
- workloads where multiple nodes share a family namespace but differ by stable key

## Partition-scoped prepared reads

If a prepared evaluation depends on one partitioned slice of an upstream node, use:

- `ExecutionReadView::read_partitioned_aspect_version(...)`

Do not use a plain `read_aspect_version(...)` for that case. A plain read captures an unscoped dependency and broadens downstream invalidation, which defeats changed-region locality at runtime.

## Tier policies and comparator selection

If you need the dedicated guide, see [CHECKPOINTS_AND_TIERS.md](./CHECKPOINTS_AND_TIERS.md).

This is the niche-but-important surface for domain-specific scheduling behavior.

Main surfaces:

- `set_node_tier(...)`
- `set_tier_policy(...)`
- `TierPolicy`
- `DependencyMode`
- `DirtyPropagation`
- `EvaluationTrigger`
- `VersionComparatorPolicy`

Use this when:

- some nodes should be lazier than others
- some tiers should tolerate version drift
- you want comparator behavior to be policy-driven instead of per-node ad hoc

## Retained vs reconstructed artifacts

If you need the exact availability matrix instead of the short version, see [ARTIFACT_ACCESS_MATRIX.md](./ARTIFACT_ACCESS_MATRIX.md).

If you only remember one subtle thing from the new observability model, remember this:

- replay plus stable semantic IDs are the hard truth
- explanation/provenance may be retained or reconstructed

That means a cheap production policy does not imply “you lost the ability to explain what happened.” It means “you may need to reconstruct it instead of finding it eagerly retained.”

Recommended access pattern:

- operational code path: use `retained_*` when you need zero extra work
- debugging or forensic path: use `reconstruct_*`
- harness/certification path: use `*_artifact(...)` to capture the materialization mode as data

## Parallel admission introspection

If a stage did not run in parallel and you expected it to, the answer should not be guesswork.

Look at:

- `StageExecutionRecord.parallel_admission_reason`
- `StageExecutionRecord.parallel_admission_message()`
- perf artifact `stage_parallel_admission_reasons`
- harness run extensions `stage_parallel_admission`

That is the supported way to explain serial fallback, not tribal knowledge and not vibes.

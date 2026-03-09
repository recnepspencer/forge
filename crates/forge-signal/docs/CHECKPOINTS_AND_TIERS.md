# Checkpoints And Tiers

These controls are powerful and easy to under-document because they are not the first thing most users need.

## Checkpoint barrier

Checkpoint policy controls when dirty effects are flushed into evaluation work.

Primary surfaces:

- `CheckpointBarrier`
- `CheckpointPolicy`
- `SignalRuntimeBuilder::checkpoint_barrier(...)`
- `SignalRuntimeBuilder::checkpoint_policy(...)`
- `tx.flush_checkpoint(...)`
- `flush_checkpoint_in_txn(...)`

Use this when:

- you want per-operation flushing
- you want to batch multiple mutations before evaluation pressure appears
- you need explicit barrier points in a transaction-heavy system

### Example: explicit checkpoint policy

```rust
use forge_signal::facade::*;

let runtime = SignalRuntime::builder(SignalGraph::new())
    .checkpoint_barrier(CheckpointBarrier::PerOperation)
    .build();

let _ = runtime;
```

## Tiers

Tier policy lets different parts of the graph obey different scheduling and comparator rules.

Primary surfaces:

- `set_node_tier(...)`
- `set_tier_policy(...)`
- `TierPolicy`
- `DependencyMode`
- `DirtyPropagation`
- `EvaluationTrigger`
- `VersionComparatorPolicy`

### Example: slow tier with tolerant comparator policy

```rust
use forge_signal::facade::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Slow,
}

let mut runtime = SignalRuntime::builder(SignalGraph::new())
    .with_tiers::<Tier>()
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
# Ok::<(), SignalError>(())
```

## Practical guidance

Use tiers when:

- some nodes are expensive enough to justify lazier pull behavior
- one class of nodes should tolerate small upstream drift
- you want policy-driven scheduling instead of copying per-node overrides everywhere

Do not use tiers just to feel sophisticated. If one node needs one custom comparator, a per-node comparator is usually simpler.

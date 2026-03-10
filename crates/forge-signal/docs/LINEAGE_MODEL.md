# Lineage Model

Provenance explains *why the current artifact looks the way it does*.

Lineage explains *how the artifact changed over time*.

Those are not the same thing.

This doc covers:

- `LineageArtifactId`
- `LineageEvent`
- `LineageRecord`
- lineage inspection APIs
- how replay and lineage fit together

## Core idea

Each evaluated artifact gets a stable lineage identity.

That identity can persist across:

- refresh with stable output identity
- restore from a snapshot
- memoized reuse

Or it can change across:

- replacement with a new output identity
- explicit branch divergence

## Important event meanings

| Event | Meaning |
| --- | --- |
| `Refreshed` | The artifact was recomputed but continuity was preserved |
| `Replaced` | The artifact was replaced with a new lineage identity |
| `Restored` | The artifact was restored from a snapshot |
| `BranchedFrom` | Branch ancestry or branch switch created an explicit branch-local transition |
| `MergedFrom` | Reserved for future branch merge semantics |
| `MemoizedFrom` | A memoized artifact reused prior work |
| `InvalidatedWithoutReplacement` | The artifact became invalid or dirty before a replacement existed |

## Inspect current artifact lineage

```rust
use forge_signal::facade::*;

let artifact = graph.current_lineage_artifact(node_id);
let records = graph.lineage_chain_for_node(node_id);
```

Use `current_lineage_artifact(...)` when you need the artifact id that currently
owns a node's materialized evaluation result.

Use `lineage_chain_for_node(...)` when you want the chain leading to that
current artifact.

## Inspect one artifact directly

```rust
use forge_signal::facade::*;

if let Some(artifact_id) = runtime.current_lineage_artifact(node_id) {
    let chain = runtime.lineage_chain_for_artifact(artifact_id);
    assert!(chain.iter().all(|record| {
        record.artifact_id == Some(artifact_id) || record.parent_artifact_id == Some(artifact_id)
    }));
}
```

## Refresh vs replace

The distinction is based on output continuity, not “did evaluation happen.”

If output identity is stable:

- lineage should usually emit `Refreshed`
- the artifact id can stay the same

If output identity is absent or intentionally too coarse, host code can supply a
generic continuity seam through `NodeEvaluationResult::with_continuity_token(...)`.
Matching continuity tokens let the runtime preserve lineage without forcing a
domain to overload `OutputIdentity`.

If output identity changes materially:

- lineage should emit `Replaced`
- the artifact id should change

That means lineage is a better history surface than raw “node executed” counts.

## Invalidation lineage

Lineage begins before replacement.

If an artifact becomes invalid or maybe-stale before a new artifact exists,
`forge-signal` records `InvalidatedWithoutReplacement`. That lets you inspect:

- which artifact became stale
- on which branch it happened
- before replacement or restore occurred

## Restore and replay

Restore does not quietly rewrite history.

On restore:

- snapshot state becomes current runtime state
- `Restored` lineage records are emitted for restored artifacts
- replay emits `SnapshotRestored`

Because restore replaces runtime state, repeated restore loops should stay
bounded. They should not accumulate unbounded history outside the restored
snapshot payload.

## Branches and lineage

Branches are branch-local timelines.

Lineage records include branch identity so you can answer questions like:

- did this artifact come from main or a feature branch?
- was this replacement local to a branch?
- did a branch-local restore alter the current branch head only?

## Related docs

- [SNAPSHOTS_BRANCHES_AND_REPLAY.md](./SNAPSHOTS_BRANCHES_AND_REPLAY.md)
- [ARTIFACT_ACCESS_MATRIX.md](./ARTIFACT_ACCESS_MATRIX.md)

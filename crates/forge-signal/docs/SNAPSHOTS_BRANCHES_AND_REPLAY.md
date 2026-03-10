# Snapshots, Branches, and Replay

This guide covers the Phase 5 state-history surface:

- `SignalSnapshotV1`
- `SignalSnapshotMeta`
- `SignalBranchHandle`
- snapshot capture and restore
- branch creation and switching
- replay inspection APIs

Use this when you care about evaluation-state history, not just current values.

## What snapshots do and do not contain

`forge-signal` snapshots capture runtime evaluation state:

- node entries and dependency topology
- trace summaries and output-identity continuity
- retained diagnostics state needed for deterministic restore
- replay frames
- lineage records
- branch catalog and active branch state
- runtime policy metadata
- core storage profile identity

Snapshots do **not** claim to own your host application's source-of-truth data.
They are for restoring and inspecting `forge-signal`'s evaluation world.

## Basic capture and restore

```rust
use forge_signal::facade::*;

let mut graph = SignalGraph::new();
let source = graph.node().output_identity().build();

evaluate(&mut graph, source, &mut |_id, _graph| {
    Ok(NodeEvaluationResult::from_version(AspectVersion::from_updates([
        (Aspect::new(0), 1),
    ])))
})?;

let snapshot = graph.capture_snapshot();

mark_dirty(&mut graph, source, Aspect::new(0))?;
graph.restore_snapshot(&snapshot)?;

assert_eq!(graph.capture_snapshot().meta.branch_id, snapshot.meta.branch_id);
```

Restore is atomic at the `forge-signal` state layer:

- the graph is replaced with the snapshot payload
- retained diagnostics are restored
- lineage and replay then get one explicit `SnapshotRestored` / `Restored` marker

That last step matters. A restore does not silently rewrite history.

## Compatibility rules

Restore rejects incompatible snapshots. Today that includes:

- schema-version mismatch
- core storage profile mismatch

Check metadata first if you want a read-only compatibility gate:

```rust
use forge_signal::facade::*;

let snapshot = runtime.capture_snapshot();
let meta = snapshot.meta.clone();

assert_eq!(meta.schema_version, SignalSnapshotMeta::SCHEMA_VERSION);
assert_eq!(meta.core_storage_profile, forge_signal::facade::CORE_STORAGE_PROFILE_ID);
```

## Runtime branches

Branches are branch-local evaluation timelines.

They are useful for:

- editor-style alternate states
- “what if” computation branches
- compare/restore workflows
- replay inspection without contaminating the active branch head

```rust
use forge_signal::facade::*;

let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();

let main = runtime.current_branch();
let feature = runtime.create_branch("feature-a")?;

runtime.switch_branch(feature.clone())?;
let feature_snapshot = runtime.capture_snapshot();

runtime.switch_branch(main.clone())?;

assert_eq!(runtime.current_branch().id, main.id);
assert_eq!(runtime.branch_ancestry(feature.id)[0].id, main.id);
assert_eq!(runtime.branch_ancestry(feature.id).last().unwrap().id, feature.id);
```

Branch guarantees:

- each branch owns its own evaluation-state timeline
- branch switches are replay-visible
- branch-local restore stays local to that branch
- branch ancestry is explicit through `parent_branch_id`
- branch head snapshots remain explicit through `branch_head_snapshot_id(...)`

## Capture and restore for a specific branch

Use these when you want to inspect or restore a non-active branch without
switching first:

```rust
use forge_signal::facade::*;

let branch = runtime.create_branch("analysis")?;
let snapshot = runtime.capture_branch_snapshot(branch.clone())?;
runtime.restore_branch_snapshot(branch.clone(), &snapshot)?;
```

If you restore a non-active branch, the active branch does not change.

## Replay inspection

Replay in `forge-signal` is evaluation-state replay.

It is good for:

- seeing what recomputed
- understanding branch switches and restores
- reconstructing artifact evolution with lineage
- debugging deterministic runtime behavior

It is **not** a promise to replay arbitrary host side effects.

### Read the full current branch history

```rust
let current_branch = runtime.current_branch();
let replay = runtime.replay_for_branch(current_branch.id);

for frame in replay.frames {
    println!("{:?}: {:?}", frame.kind, frame.detail);
}
```

### Inspect one node's replay trail

```rust
let replay = runtime.replay_for_node(node_id);
assert!(replay.frames.iter().all(|frame| frame.node == Some(node_id)));
```

### Inspect one artifact's replay trail

```rust
if let Some(artifact_id) = runtime.current_lineage_artifact(node_id) {
    let replay = runtime.replay_for_artifact(artifact_id);
    assert!(replay
        .frames
        .iter()
        .all(|frame| frame.lineage_artifact_id == Some(artifact_id)));
}
```

### Slice from a cursor

```rust
let replay = runtime.replay_for_branch(runtime.current_branch().id);
let cursor = replay.start.unwrap_or_default();
let tail = runtime.replay_from_cursor(cursor);
```

### Slice between two cursors

```rust
let replay = runtime.replay_for_branch(runtime.current_branch().id);
let start = replay.frames.first().unwrap().cursor;
let end = replay.frames.last().unwrap().cursor;
let bounded = runtime.replay_between(start, end);

assert!(bounded
    .frames
    .iter()
    .all(|frame| frame.cursor >= start && frame.cursor <= end));
```

### Inspect replay around a snapshot

```rust
let snapshot = runtime.capture_snapshot();
let replay = runtime.replay_around_snapshot(snapshot.meta.snapshot_id);

assert!(replay
    .frames
    .iter()
    .any(|frame| frame.snapshot_id == Some(snapshot.meta.snapshot_id)));
```

## When to use graph vs runtime APIs

Use `SignalGraph` snapshot APIs when you only need graph-local state history.

Use `SignalRuntime` APIs when you need:

- branch creation and switching
- branch-local snapshots
- runtime telemetry in snapshots
- runtime policy metadata

## Related docs

- [LINEAGE_MODEL.md](./LINEAGE_MODEL.md)
- [ARTIFACT_ACCESS_MATRIX.md](./ARTIFACT_ACCESS_MATRIX.md)
- [HARNESS_AND_CERTIFICATION.md](./HARNESS_AND_CERTIFICATION.md)

# Diagnostics And History Reference

This document covers the deeper inspection surfaces:

- `SignalDiagnostics`
- `SignalHistory`
- `SignalSpecialist`
- `SignalAdapters`

The short version: diagnostics are not an afterthought. They are a first-class
part of how you debug callback-backed computed nodes, observation behavior,
history, and replay.

## Getting Started

```ts
const diagnostics = signals.diagnostics();
const history = signals.history();
const specialist = signals.specialist();
const adapters = signals.adapters();
```

## `SignalDiagnostics`

### `why(id): WhySummary`

Use `why(...)` when you want the best single explanation for one signal.

Simple:

```ts
const why = diagnostics.why("doubled");
console.log(why.recipeFamily);
```

Complex:

```ts
const why = diagnostics.why("label");

console.log({
  state: why.state,
  outputChange: why.outputChange,
  recipeFamily: why.recipeFamily,
  callbackReads: why.callback?.currentReads,
  callbackFailure: why.callback?.lastFailure,
  dependencyPatch: why.callback?.lastDependencyPatch,
});
```

This is the first place to look when:

- a callback computed did not rerun
- a callback reran with the wrong dependency frontier
- a self-read or dynamic-cycle denial occurred

### `health(): HealthSummary`

Use `health()` for a high-level runtime health snapshot.

Simple:

```ts
console.log(diagnostics.health());
```

Complex:

```ts
const health = diagnostics.health();

console.log({
  activeNodes: health.activeNodeCount,
  dirtyNodes: health.dirtyNodeCount,
  dependencyEdges: health.dependencyEdgeCount,
  subscriberEdges: health.subscriberEdgeCount,
});
```

### `summaryNow()`

Use `summaryNow()` when you want the broad current graph summary.

Simple:

```ts
const summary = diagnostics.summaryNow();
```

Complex:

```ts
const summary = diagnostics.summaryNow();
saveDiagnosticsSnapshot(summary);
```

### `historyNow()`

Use `historyNow()` when you want the retained diagnostics history surface.

Simple:

```ts
const historyNow = diagnostics.historyNow();
```

Complex:

```ts
const historyNow = diagnostics.historyNow();
console.log(historyNow.callbackNodes);
```

### `latestFlow()`

Use `latestFlow()` when you want the most recent committed invalidation /
evaluation explanation.

Simple:

```ts
const flow = diagnostics.latestFlow();
console.log(flow?.flow.change.changedNodes);
```

Complex:

```ts
const flow = diagnostics.latestFlow();
if (flow) {
  console.log({
    changedNodes: flow.flow.change.changedNodes,
    explanation: flow.flow.explanation,
    callbackNodes: flow.callbackNodes,
  });
}
```

### `latestObservation()`

Use `latestObservation()` when you want the latest committed observation
boundary that watchers/effects saw.

Simple:

```ts
const observation = diagnostics.latestObservation();
console.log(observation?.deliveredEventCount);
```

Complex:

```ts
const observation = diagnostics.latestObservation();

console.log({
  delivered: observation?.deliveredEventCount,
  rollbackSuppressed: observation?.rollbackSuppressedEventCount,
  boundaryEvents: observation?.boundaryEvents,
});
```

### `performanceSummary(): WebPerformanceSummary`

Use `performanceSummary()` for counters and boundedness signals.

Simple:

```ts
const perf = diagnostics.performanceSummary();
console.log(perf.deliveredObservationCount);
```

Complex:

```ts
const perf = diagnostics.performanceSummary();

console.log({
  callbackInvocations: perf.computeCallbackInvocationCount,
  callbackCaptures: perf.computeCallbackCaptureCount,
  callbackReadBreadth: perf.computeCallbackRuntimeReadBreadth,
  dependencyPatches: perf.computeCallbackDependencyPatchCount,
  promiseDenials: perf.computeCallbackPromiseReturnDenialCount,
  invalidReturnDenials: perf.computeCallbackInvalidReturnDenialCount,
  missingUnavailability: perf.computeCallbackMissingUnavailabilityCount,
});
```

This is the best first surface for:

- “did this callback actually rerun?”
- “are dynamic dependency patches happening?”
- “are we hitting denials?”
- “are watchers/effects fanning out more than expected?”

### Failure And Trace Accessors

- `latestFailure()`
- `latestRollback()`
- `latestFrontierExecution()`
- `latestInvalidationTraceRecords()`
- `recentHistory()`

Simple:

```ts
console.log(diagnostics.latestFailure());
```

Complex:

```ts
console.log({
  latestFailure: diagnostics.latestFailure(),
  latestRollback: diagnostics.latestRollback(),
  latestFrontierExecution: diagnostics.latestFrontierExecution(),
  invalidationTrace: diagnostics.latestInvalidationTraceRecords(),
});
```

## `SignalHistory`

### Replay And Lineage

- `replay_for(id)`
- `lineage_for(id)`
- `replay_for_branch(branchId)`

Simple:

```ts
const replay = history.replay_for("panel");
```

Complex:

```ts
const replay = history.replay_for("label");
const lineage = history.lineage_for("label");

console.log({
  replay,
  lineage,
});
```

### Snapshot And Restore

- `snapshot()`
- `restore_snapshot(snapshot)`
- `branch_snapshot(branchId)`
- `branch_snapshot_id(branchId)`
- `branch_snapshot_envelope(branchId)`
- `restore_branch_snapshot(branchId, snapshot)`
- `restore_branch_snapshot_by_id(branchId, snapshotId)`

Simple:

```ts
const snapshot = history.snapshot();
history.restore_snapshot(snapshot);
```

Complex:

```ts
const branchId = history.current_branch().id;
const snapshot = history.branch_snapshot(branchId);

signals.transaction((tx) => {
  tx.set(count, 99);
});

history.restore_branch_snapshot(branchId, snapshot);
```

### Branch Access

- `current_branch()`
- `branches()`
- `create_branch(name)`
- `switch_branch(branchId)`

Simple:

```ts
const branch = history.create_branch("what-if");
history.switch_branch(branch.id);
```

Complex:

```ts
const main = history.current_branch();
const preview = history.create_branch("preview");

history.switch_branch(preview.id);
signals.transaction((tx) => tx.set(count, 12));

history.switch_branch(main.id);
```

### Merge And Planning

- `merge_branches(...)`
- `merge_branches_with_proof(...)`
- `plan_merge_branches(...)`
- `plan_merge_branches_with_proof(...)`
- `plan_merge_policy_preview(...)`
- `plan_merge_policy_preview_with_proof(...)`
- `merge_branches_policy_preview(...)`
- `merge_branches_policy_preview_with_proof(...)`

Simple:

```ts
const plan = history.plan_merge_branches(sourceId, targetId);
```

Complex:

```ts
const plan = history.plan_merge_branches_with_proof(sourceId, targetId);
const merge = history.merge_branches_with_proof(sourceId, targetId);

console.log({ plan, merge });
```

### Proof Access

- `branch_state_proof(branchId)`
- `replay_parity_proof(expectedBranchId, replayedBranchId)`
- `replay_artifact_proof(expected, replayedBranchId)`

Simple:

```ts
const proof = history.branch_state_proof(history.current_branch().id);
```

Complex:

```ts
const proof = history.replay_artifact_proof(expectedArtifact, branchId);
console.log(proof);
```

## `SignalSpecialist`

Use the specialist surface for advanced runtime inspection.

Methods:

- `evaluate_dirty()`
- `graph_summary()`
- `read_versions(ids)`

Simple:

```ts
console.log(specialist.graph_summary());
```

Complex:

```ts
const versions = specialist.read_versions(["count", "label", "panel"]);
const dirty = specialist.evaluate_dirty();

console.log({ versions, dirty });
```

## `SignalAdapters`

Use adapters for export/import and proof/report surfaces.

Methods:

- `export_definitions()`
- `runtime_proof_report()`

Simple:

```ts
const definitions = adapters.export_definitions();
```

Complex:

```ts
const definitions = adapters.export_definitions();
const proof = adapters.runtime_proof_report();

console.log({
  unavailableCallbacks: definitions.unavailableCallbacks,
  proof,
});
```

## Callback Diagnostics Notes

Callback-backed computed nodes expose additional detail through `why(id)` and
the richer retained surfaces:

- callback purity posture
- current captured reads
- registration state
- token slot / generation
- last dependency patch
- last callback failure

Two important callback postures:

- `signalTracked`
- `constantizedNoSignalReads`

Use those to distinguish:

- “this is a live runtime callback node”
- from
- “this callback captured no signal reads and was lowered into a constant node”

## When To Reach For Which Surface

- Use `why(id)` first for one broken signal.
- Use `latestFlow()` for “what just caused that?”
- Use `latestObservation()` for watcher/effect delivery truth.
- Use `performanceSummary()` for breadth, counts, and denial evidence.
- Use `history()` for branching, replay, snapshot, and merge questions.
- Use `adapters()` for export/proof/report surfaces.

## Related Docs

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)

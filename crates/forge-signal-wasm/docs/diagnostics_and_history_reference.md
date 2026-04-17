# Diagnostics And History Reference

This document covers the `SignalDiagnostics`, `SignalHistory`,
`SignalSpecialist`, and `SignalAdapters` surfaces.

## `SignalDiagnostics`

Get diagnostics through:

```ts
const diagnostics = signals.diagnostics();
```

### `why(id): WhySummary`

Returns a focused explanation summary for a signal id.

Fields:

- `id`
- `node`
- `state`
- `upstream`
- `changedRegions`
- `propagationSuppressed`
- `outputChange`
- `outputIdentity`

### `health(): HealthSummary`

Returns runtime health counters:

- `activeNodeCount`
- `cleanNodeCount`
- `maybeStaleNodeCount`
- `dirtyNodeCount`
- `dependencyEdgeCount`
- `subscriberEdgeCount`

### `summaryNow(): unknown`

Returns the current summary snapshot from the diagnostics lane.

### `historyNow(): unknown`

Returns the current diagnostics history snapshot.

### `latestFlow(): unknown | null`

Returns the latest retained flow summary when available.

### `latestObservation(): ObservationBoundarySummary | null`

Returns the latest committed observation boundary summary.

Fields on `ObservationBoundarySummary`:

- `branchId`
- `deliveredEventCount`
- `rollbackSuppressedEventCount`
- `boundaryEvents`

Each boundary event includes:

- `observerId`
- `handleId`
- `matchedNodes`
- `touched`
- `recomputed`
- `meaningfulChange`
- `triggerMatched`

### `performanceSummary(): WebPerformanceSummary`

Returns web-layer cert counters:

- `activeHandleCount`
- `activeCallbackCount`
- `matchedWatcherBreadth`
- `deliveredObservationCount`
- `rollbackSuppressedDeliveryCount`
- `serialExecutorUsageCount`
- `parallelExecutorUsageCount`
- `outputSerializationCount`
- `outputSerializationBreadth`
- `jsCallbackInvocationCount`
- `jsCallbackFailureCount`
- `compatibilityReadCount`
- `compatibilityReadBreadth`

This is the best first place to check web runtime boundedness and whether a web
consumer is using the serial executor path.

### Failure And Trace Accessors

- `latestFailure()`
- `latestRollback()`
- `latestFrontierExecution()`
- `latestInvalidationTraceRecords()`
- `recentHistory()`

These are richer retained diagnostics doors and may carry more detail than the
app-first API needs for normal usage.

## `SignalHistory`

Get history access through:

```ts
const history = signals.history();
```

### Replay And Lineage

- `replay_for(id)`
- `lineage_for(id)`
- `replay_for_branch(branchId)`

### Snapshot And Restore

- `snapshot()`
- `restore_snapshot(snapshot)`
- `branch_snapshot(branchId)`
- `branch_snapshot_id(branchId)`
- `branch_snapshot_envelope(branchId)`
- `restore_branch_snapshot(branchId, snapshot)`
- `restore_branch_snapshot_by_id(branchId, snapshotId)`

### Branch Access

- `current_branch()`
- `branches()`
- `create_branch(name)`
- `switch_branch(branchId)`

### Merge And Planning

- `merge_branches(sourceBranchId, targetBranchId)`
- `merge_branches_with_proof(sourceBranchId, targetBranchId)`
- `plan_merge_branches(sourceBranchId, targetBranchId)`
- `plan_merge_branches_with_proof(sourceBranchId, targetBranchId)`
- `plan_merge_policy_preview(request)`
- `plan_merge_policy_preview_with_proof(request)`
- `merge_branches_policy_preview(request)`
- `merge_branches_policy_preview_with_proof(request)`

### Proof Access

- `branch_state_proof(branchId)`
- `replay_parity_proof(expectedBranchId, replayedBranchId)`
- `replay_artifact_proof(expected, replayedBranchId)`

## `SignalSpecialist`

Get the specialist surface through:

```ts
const specialist = signals.specialist();
```

Methods:

- `evaluate_dirty()`
- `graph_summary()`
- `read_versions(ids)`

This surface is for advanced host/runtime consumers rather than first-line app
code.

## `SignalAdapters`

Get adapters through:

```ts
const adapters = signals.adapters();
```

Methods:

- `export_definitions()`
- `export_runtime_envelope()`
- `replace_runtime_envelope(envelope)`
- `runtime_proof_report()`

This is the main export/import and runtime envelope door for integration work.

## Semantics Notes

- latest observation and latest flow should remain coherent at the committed
  boundary
- rollback suppresses normal watch/effect delivery, but rollback summaries can
  still be retained and inspected
- history and branch semantics are inherited from the core runtime, not
  invented locally in the web package

## Related Docs

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [compatibility_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/compatibility_surface_reference.md)

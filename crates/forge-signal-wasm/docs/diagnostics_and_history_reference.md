# Diagnostics And History Reference

This document covers the deeper inspection surfaces:

- `SignalDiagnostics`
- `SignalHistory`
- `SignalSpecialist`
- `SignalAdapters`

Use these surfaces when you need to explain callback-backed computed nodes,
observation behavior, history, and replay.

Published graphs also expose graph-scoped inspection surfaces. When you already
have a `PublishedSignalGraph`, prefer those graph-shaped views over manually
looping runtime-wide APIs.

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

## Graph-Scoped Inspection

Published graphs keep diagnostics and history attached to graph identity rather
than forcing you to think in raw ids first.

### `graph.inspectDiagnostics()`

Simple:

```ts
const graphDiagnostics = itemDetailGraph.inspectDiagnostics();
console.log(graphDiagnostics.inputs.serverItemData.why);
console.log(graphDiagnostics.outputs.submitReadiness.why);
console.log(graphDiagnostics.dependenciesForOutput("submitReadiness").publicInputNames);
```

Complex:

```ts
const graphDiagnostics = itemDetailGraph.inspectDiagnostics();

console.log({
  graphId: graphDiagnostics.graph.id,
  contract: graphDiagnostics.contract,
  contractSummary: graphDiagnostics.contractSummary(),
  inputDescriptors: graphDiagnostics.inputDescriptors,
  descriptors: graphDiagnostics.descriptors,
  serverEntry: graphDiagnostics.input("serverItemData"),
  submitEntry: graphDiagnostics.output("submitReadiness"),
  submitDependencies: graphDiagnostics.dependenciesForOutput("submitReadiness"),
  serverWhy: graphDiagnostics.inputs.serverItemData.why,
  serverVersion: graphDiagnostics.inputs.serverItemData.version,
  submitWhy: graphDiagnostics.outputs.submitReadiness.why,
  submitVersion: graphDiagnostics.outputs.submitReadiness.version,
  runtimeGraph: graphDiagnostics.runtimeGraph,
  executionHistory: graphDiagnostics.executionHistory,
  latestFlow: graphDiagnostics.latestFlow,
  latestObservation: graphDiagnostics.latestObservation,
});
```

Use this when the question is:

- what did this published graph expose?
- why does one published output currently look the way it does?
- which public inputs feed one published output?
- what changed about the public contract between snapshots?
- what runtime summary was current when I inspected the graph?

If you already captured an earlier contract snapshot, you can compare it
directly at the graph boundary:

```ts
const previousContract = itemDetailGraph.contract();
const nextDelta = itemDetailGraph.contractDelta(previousContract);
const contractHistory = itemDetailGraph.contractHistory();
const exported = itemDetailGraph.exportDefinition();
const snapshot = itemDetailGraph.exportSnapshot();
const restored = createSignals().importGraph(exported, snapshot);
console.log(itemDetailGraph.importPosture());
console.log(contractHistory.restoreMode);
console.log(restored.contractHistory());
console.log(nextDelta.outputs.added);
```

### `graph.inspectHistory()`

Simple:

```ts
const graphHistory = itemDetailGraph.inspectHistory();
console.log(graphHistory.inputs.serverItemData.replay);
console.log(graphHistory.outputs.submitReadiness.replay);
console.log(graphHistory.dependenciesForOutput("submitReadiness").publicInputSourceIds);
```

Complex:

```ts
const graphHistory = itemDetailGraph.inspectHistory();

console.log({
  graphId: graphHistory.graph.id,
  contract: graphHistory.contract,
  contractSummary: graphHistory.contractSummary(),
  serverEntry: graphHistory.input("serverItemData"),
  submitEntry: graphHistory.output("submitReadiness"),
  submitDependencies: graphHistory.dependenciesForOutput("submitReadiness"),
  inputReplay: graphHistory.inputs.serverItemData.replay,
  inputLineage: graphHistory.inputs.serverItemData.lineage,
  submitReplay: graphHistory.outputs.submitReadiness.replay,
  submitLineage: graphHistory.outputs.submitReadiness.lineage,
  recentHistory: graphHistory.recentHistory,
});
```

Use this when you want replay/lineage answers anchored to the graph publication
boundary instead of manually translating graph output names back into raw ids.

Graph-scoped diagnostics/history expose the public input side too, which is the
stable contract surface to use instead of ambient runtime ids.

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
console.log(flow?.flow.change.changed_nodes);
```

Complex:

```ts
const flow = diagnostics.latestFlow();
if (flow) {
  console.log({
    changedNodes: flow.flow.change.changed_nodes,
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
console.log(observation?.observation.delivered_event_count);
```

Complex:

```ts
const observation = diagnostics.latestObservation();

console.log({
  delivered: observation?.observation.delivered_event_count,
  rollbackSuppressed: observation?.observation.rollback_suppressed_event_count,
  boundaryEvents: observation?.observation.boundary_events,
});
```

### `latestHostCapabilityEvent()` And `recentHostCapabilityEvents()`

Use these when the important question is not just "what callback reran?" but
"what host-capability event dirtied or denied this lane?"

Simple:

```ts
const latestHostEvent = diagnostics.latestHostCapabilityEvent();
console.log(latestHostEvent?.kind);
```

Complex:

```ts
const latestHostEvent = diagnostics.latestHostCapabilityEvent();
const recentHostEvents = diagnostics.recentHostCapabilityEvents();

console.log({
  latestKind: latestHostEvent?.kind,
  queuedInvalidations: latestHostEvent?.queuedInvalidationCount,
  deniedCallbacks: latestHostEvent?.deniedCallbackIds,
  recentEventCount: recentHostEvents.length,
});
```

### `hostCapabilityReport()`

Use `hostCapabilityReport()` when you want a canonical digest plus:

- one family-grouped summary of host-capability lifecycle and denial behavior
- a bounded event-lineage report with its own digest
- a breadth report for queued invalidation, touched-node, and reevaluation
  maxima

Simple:

```ts
const hostReport = diagnostics.hostCapabilityReport();
console.log(hostReport.digest);
```

Complex:

```ts
const hostReport = diagnostics.hostCapabilityReport();

console.log({
  digest: hostReport.digest,
  lineageDigest: hostReport.lineageDigest,
  breadthDigest: hostReport.breadthDigest,
  unavailabilityArtifacts: hostReport.totals.unavailabilityArtifactCount,
  compatibilityDenials: hostReport.totals.compatibilityDenialCount,
  maxTouchedNodes: hostReport.breadth.maxTouchedNodes,
  maxReevaluatedNodes: hostReport.breadth.maxReevaluatedNodes,
  families: hostReport.families.map((family) => ({
    family: family.family,
    latestKind: family.latestKind,
    latestCompatibility: family.latestCompatibility,
    invalidationModes: family.invalidationModes,
    maxTouchedNodes: family.maxTouchedNodes,
    deniedCallbacks: family.deniedCallbackIds,
  })),
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
  hostReads: perf.hostCapabilityReadCount,
  hostInvalidations: perf.hostCapabilityInvalidationCount,
  hostReevaluations: perf.hostCapabilityReevaluationCount,
  hostCompatibilityDenials: perf.hostCapabilityCompatibilityDenialCount,
  hostUnavailabilityArtifacts: perf.hostCapabilityUnavailabilityArtifactCount,
  hostBroadFanoutDenials: perf.hostCapabilityBroadFanoutDenialCount,
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
const branchReplay = history.replay_for_branch(history.current_branch().id);

console.log({
  replay,
  lineage,
  branchReplay,
});
```

This is also the right place to inspect host-capability cost honesty:

- how many typed host reads happened at the product facade
- how many host invalidations were observed and batched
- how much reevaluation breadth those invalidations caused
- whether portability denials were recorded explicitly

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
const envelope = history.branch_snapshot_envelope(branchId);

signals.transaction((tx) => {
  tx.set(count, 99);
});

history.restore_snapshot(envelope);
history.restore_branch_snapshot(branchId, snapshot);
```

The callback-bearing history contract is:

- `replay_for(...)` and `replay_for_branch(...)` expose callback metadata on
  replay frames when callback-backed nodes participate in the history slice.
- `snapshot()` and `branch_snapshot_envelope(...)` are structured expert
  artifacts, not plain JSON blobs.
- callback-backed restore/import lanes still deny missing live callback
  registrations explicitly rather than silently degrading into callback-free
  truth.
- the product-facing `history()` surface accepts the numeric branch ids it
  already returns from `current_branch()` and `create_branch(...)`; callers do
  not need `BigInt(...)` ceremony on the normal package lane.

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
const preview = history.plan_merge_policy_preview_with_proof({
  source_branch_id: sourceId,
  target_branch_id: targetId,
});

console.log({
  strategy: preview.plan.selected_semantics.strategy_name,
  divergence: preview.plan.resolution_plan?.divergence ?? null,
  mappedNodes: preview.plan.node_map,
  firstDecision: preview.plan.node_plan[0]?.decision ?? null,
  firstAdoptionSource: preview.plan.adoption_core[0]?.source_node ?? null,
  firstCarryPolicy: preview.plan.adoption_policy[0]?.runtime_artifact ?? null,
  replayEvents: merge.result.counters.replay_event_count,
  firstMergedNode: merge.result.records[0]?.source_node ?? null,
});
```

Notes:

- merge plan and merge result artifacts now expose stable summary fields instead
  of opaque nested blobs for:
  - `selected_semantics`
  - `merge_base` / `lowered_merge_base`
  - `resolution_plan`
  - `node_map`
  - `node_plan`
  - `adoption_core`
  - `adoption_policy`
  - `records`
  - `counters`
- merge node identities are surfaced as generational strings like
  `"12:3"`, not bare numeric ids, so cross-branch merge evidence keeps the
  real runtime identity shape.

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

- `evaluateDirty()` / `evaluate_dirty()`
- `graphSummary()` / `graph_summary()`
- `readVersions(ids)` / `read_versions(ids)`

Simple:

```ts
console.log(specialist.graphSummary());
```

Complex:

```ts
const versions = specialist.readVersions(["count", "label", "panel"]);
const dirty = specialist.evaluateDirty();

console.log({ versions, dirty });
```

## `SignalAdapters`

Use adapters for export/import and proof/report surfaces.

Methods:

- `exportDefinitions()`
- `exportRuntimeEnvelope()`
- `replaceRuntimeEnvelope(envelope)`
- `restoreExactRuntimeEnvelope(envelope)`
- `runtimeProofReport()`
- `hostCapabilityTransportReport(envelope?)`

Simple:

```ts
const definitions = adapters.exportDefinitions();
```

Complex:

```ts
const definitions = adapters.exportDefinitions();
const envelope = adapters.exportRuntimeEnvelope();
adapters.replaceRuntimeEnvelope(envelope);
const proof = adapters.runtimeProofReport();

console.log({
  unavailableCallbacks: definitions.unavailableCallbacks,
  proofVersion: proof.proofSchemaVersion,
  proofDigest: proof.registryBundleDigest,
});
```

Use `hostCapabilityTransportReport(...)` when you want the exported
host-capability transport posture grouped by family with a canonical digest.

```ts
const envelope = adapters.exportRuntimeEnvelope();
const transportReport = adapters.hostCapabilityTransportReport(envelope);

console.log({
  digest: transportReport.digest,
  unavailableArtifacts: transportReport.totals.unavailableArtifactCount,
  deniedFamilies: transportReport.totals.deniedFamilyCount,
  unavailableFamilies: transportReport.totals.unavailableFamilyCount,
  families: transportReport.families.map((family) => ({
    family: family.family,
    compatibilities: family.compatibilities,
    portableOutcomes: family.portableImportOutcomes,
    deniedCallbacks: family.deniedCallbackIds,
    unavailableCallbacks: family.unavailableCallbackIds,
  })),
});
```

Use the runtime-envelope lane as an expert restore/export surface:

- `exportRuntimeEnvelope()` carries definitions plus the captured runtime
  snapshot envelope for rebuildable runtimes.
- `replaceRuntimeEnvelope(...)` restores that envelope into a fresh runtime.
- callback-backed nodes without live callback registrations are a typed denial,
  not a silent partial restore.

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

## Practical Debugging Workflows

### Why did this callback rewire?

```ts
const why = diagnostics.why("doubled");
console.log(why.callback?.lastDependencyPatch);
```

### What just delivered to observers?

```ts
const latestObservation = diagnostics.latestObservation();

console.log(
  latestObservation?.observation.boundary_events.map((event) => ({
    observerId: event.observer_id,
    matchedNodes: event.matched_nodes.nodes,
    triggerMatched: event.trigger_matched,
  })),
);
```

### Did a callback fail or get denied?

```ts
console.log({
  latestFailure: diagnostics.latestFailure(),
  latestRollback: diagnostics.latestRollback(),
  perf: diagnostics.performanceSummary(),
});
```

## When To Reach For Which Surface

- Use `why(id)` first for one broken signal.
- Use `latestFlow()` for “what just caused that?”
- Use `latestObservation()` for watcher/effect delivery truth.
- Use `performanceSummary()` for breadth, counts, and denial evidence.
- Use `history()` for branching, replay, snapshot, and merge questions.
- Use `adapters()` for export/proof/report surfaces.

## Related Docs

- [app_surface_reference.md](app_surface_reference.md)
- [react_adapter_reference.md](react_adapter_reference.md)

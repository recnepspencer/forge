# Diagnostics And History Reference

## What This Feature Is

These surfaces explain what the runtime and published graphs are doing.

Use them when you need:

- causality
- replay or lineage
- public graph contract inspection
- export and restore artifacts
- host-capability event explanations

This is the inspection lane for `forge-signal-wasm`. It does not own runtime
truth. It explains runtime truth.

## Why You Use It

- answer “why did this signal or graph output change?”
- inspect a published graph without dropping to raw ids first
- compare graph contracts over time
- inspect replay and lineage artifacts
- understand host-capability invalidation and denial behavior
- export runtime or graph artifacts for exact restore

## What Distinguishes This

These surfaces are not just action logs or devtool snapshots. They are tied to
the same runtime truth model that owns derivation, replay, and restore.

Compared with something like Redux tooling:

- Redux DevTools gives you action history and state snapshots
- `forge-signal-wasm` diagnostics/history can explain dependency causality,
  callback frontier changes, graph contract drift, replay artifacts, and exact
  restore posture

What this enables here is:

- explanation of why a value changed, not just that it changed
- graph-boundary inspection with public names, authorities, and requiredness
- replay and lineage surfaces that stay connected to runtime truth instead of
  becoming UI-only debugging overlays

## Stable Entry Points

Runtime-wide:

- `signals.diagnostics()`
- `signals.history()`
- `signals.specialist()`
- `signals.adapters()`

Graph-scoped:

- `graph.inspectDiagnostics()`
- `graph.inspectHistory()`
- `graph.contract()`
- `graph.contractDelta(...)`
- `graph.contractHistory()`
- `graph.exportDefinition()`
- `graph.exportSnapshot()`
- `graph.importPosture()`

## Core Mental Model

There are two useful inspection levels:

1. runtime-wide inspection
2. graph-boundary inspection

Use runtime-wide surfaces when the question is about the whole signal runtime.
Use graph-scoped surfaces when the question is about a published boundary.

That distinction matters because graph inspection already carries:

- public input names
- public output names
- public input authority and requiredness
- published output ids
- graph-owned dependency explanations

If you already have a published graph, that is usually the better place to
start.

## How It Executes

The runtime continuously records:

- current explanation state
- latest flow and latest observation summaries
- replay and lineage data
- host-capability event summaries
- exportable runtime artifacts

Published graphs project those runtime facts back onto their public boundary.
That gives you graph-specific inspection without losing the underlying runtime
truth.

## Small Example

```ts
const diagnostics = signals.diagnostics();
const history = signals.history();

console.log(diagnostics.latestFlow());
console.log(history.replayFor("count"));
```

This is the smallest honest example because it shows the two basic questions:

- what just happened?
- what history artifact do I have for one node?

## Real Example

```ts
const graphDiagnostics = itemWorkspaceGraph.inspectDiagnostics();
const graphHistory = itemWorkspaceGraph.inspectHistory();

console.log({
  graphId: graphDiagnostics.graph.id,
  contract: graphDiagnostics.contract,
  contractSummary: graphDiagnostics.contractSummary(),
  draftEntry: graphDiagnostics.input("draft"),
  effectiveEntry: graphDiagnostics.output("effectiveItem"),
  effectiveDependencies: graphDiagnostics.dependenciesForOutput("effectiveItem"),
  latestFlow: graphDiagnostics.latestFlow,
  latestObservation: graphDiagnostics.latestObservation,
});

console.log({
  historyContract: graphHistory.contract,
  draftReplay: graphHistory.inputs.draft.replay,
  draftLineage: graphHistory.inputs.draft.lineage,
  effectiveReplay: graphHistory.outputs.effectiveItem.replay,
  effectiveLineage: graphHistory.outputs.effectiveItem.lineage,
});

const previousContract = itemWorkspaceGraph.contract();
const delta = itemWorkspaceGraph.contractDelta(previousContract);
const definition = itemWorkspaceGraph.exportDefinition();
const snapshot = itemWorkspaceGraph.exportSnapshot();
const restoredGraph = (await createSignals()).importGraph(definition, snapshot);

console.log(delta);
console.log(restoredGraph.contractHistory());
```

What this gives you:

- graph-scoped explanations instead of raw-id spelunking
- contract summaries and deltas at the public boundary
- replay and lineage anchored to graph inputs and outputs
- exact same-runtime restore artifacts

## Runtime-Wide Surfaces

### `signals.diagnostics()`

Use this when the question is about current explanation state.

Important entry points:

- `why(id)`
- `health()`
- `summaryNow()`
- `historyNow()`
- `latestFlow()`
- `latestObservation()`
- `latestHostCapabilityEvent()`
- `recentHostCapabilityEvents()`
- `hostCapabilityReport()`
- `performanceSummary()`

### `why(id)`

Use `why(...)` for the best single explanation of one runtime id:

```ts
const why = signals.diagnostics().why("doubled");
```

This is the first place to look when:

- a callback-derived node did not rerun
- a callback reran unexpectedly
- a self-read or cycle denial occurred

### `health()`

Use `health()` for a broad runtime health snapshot:

```ts
const health = signals.diagnostics().health();
```

### `latestFlow()` and `latestObservation()`

Use these when you want the most recent committed evaluation and observation
evidence:

```ts
const flow = signals.diagnostics().latestFlow();
const observation = signals.diagnostics().latestObservation();
```

### Host-capability inspection

Use these when the interesting question is about host invalidation rather than
ordinary signal derivation:

- `latestHostCapabilityEvent()`
- `recentHostCapabilityEvents()`
- `hostCapabilityReport()`

## Graph-Scoped Inspection

### `graph.inspectDiagnostics()`

Use this when the question is about a published graph as a public artifact.

Key surfaces:

- `graph`
- `contract`
- `dependencies`
- `input(name)`
- `output(name)`
- `dependenciesForOutput(name)`
- `contractSummary()`
- `latestFlow`
- `latestObservation`

### `graph.inspectHistory()`

Use this when you want replay and lineage anchored to the graph boundary.

Key surfaces:

- `inputs.<name>.replay`
- `inputs.<name>.lineage`
- `outputs.<name>.replay`
- `outputs.<name>.lineage`
- `dependenciesForOutput(name)`
- `recentHistory`

### `graph.contract()`, `graph.contractDelta(...)`, and `graph.contractHistory()`

Use these when you are reasoning about contract drift:

```ts
const current = graph.contract();
const delta = graph.contractDelta(previousContract);
const history = graph.contractHistory();
```

These surfaces are where public naming, authority, requiredness, and published
output descriptors become explicit and comparable.

## Export And Restore

### Graph-native exact restore

Published graphs can export definition and snapshot artifacts:

```ts
const definition = graph.exportDefinition();
const snapshot = graph.exportSnapshot();
const restored = signals.importGraph(definition, snapshot);
```

Use `importPosture()` to see what is admitted:

```ts
console.log(graph.importPosture());
```

Important truth:

- this is an exact same-runtime restore lane
- portable graph import is still denied on this surface

### Runtime adapters

Use `signals.adapters()` when you need lower-level runtime artifacts:

- runtime envelopes
- snapshot envelopes
- host-capability transport reports
- compatibility-facing export surfaces

## How It Relates To Other Features

- Pair this with the app surface when you need explanation, not just state.
- Pair this with graphs when your public contract is the important boundary.
- Pair this with host capability when invalidation may come from browser facts.
- Pair this with the compatibility surface only when you truly need lower-level
  artifact access.

## Inspection And Debugging

Practical order of operations:

1. start at `graph.inspectDiagnostics()` if you already have a graph
2. use `signals.diagnostics().why(id)` when you need one raw runtime id
3. use `latestFlow()` and `latestObservation()` for recent causality
4. use `graph.contractDelta(...)` or `graph.contractHistory()` for boundary drift
5. use `signals.adapters()` only when you need lower-level exported artifacts

## Anti-Patterns

- starting at raw runtime ids when you already have a published graph
- treating diagnostics as the owner of semantics instead of runtime truth
- assuming exact restore implies portable import
- using lower-level adapters first when graph-native inspection already answers
  the question
- reading contract history as if it were ordinary application state

## Current Limits

- graph-native import is still exact same-runtime restore only
- portable graph import remains denied on this surface
- diagnostics explain truth; they do not become a second state model
- runtime-wide ids are still useful, but graph-boundary inspection is usually
  the better starting point for product code

## Related Docs

- [App Surface Overview](./overview.md)
- [Compatibility Surface](../api-reference/compatibility-surface.md)
- [Host Capabilities](./host-capabilities.md)

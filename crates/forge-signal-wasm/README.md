# forge-signal-wasm

Framework-agnostic browser bindings for Forge Signal.

`forge-signal-wasm` gives you one package for three jobs:

- local reactive app state
- API-backed resource state
- browser and framework integration around that state

You can use it for ordinary signal-style state, larger controller and graph
composition, and resource lines that own loading, freshness, uploads,
downloads, diagnostics, and history.

The root package is a mixed umbrella entrypoint:

- the default export initializes the raw wasm module
- named exports expose the modern callable, graph, resource, and host-capability
  surfaces

The published npm package is ESM-first. `import` and bundler resolution are the
supported consumer paths. CommonJS callers should use dynamic `import(...)`
instead of `require(...)`.

## Install

```bash
npm install forge-signal-wasm
```

React adapter:

```bash
npm install forge-signal-wasm react
```

Before publishing a new version from this repo, always run the package proof:

```powershell
scripts/wasm/publish-forge-signal-wasm.ps1 -SkipPublish
```

## What This Package Is Good At

- local app state with derived values
- controller-style composition for larger features
- explicit graph contracts for published inputs and outputs
- graph-scoped input mutation with write, patch, and reset helpers
- linked writable state for "follow source until overridden" flows
- aspect-aware invalidation and explicit aspect-filtered spec authoring
- API resources with detail, collection, and paged family authoring
- line-scoped request, freshness, retry, upload, download, and inspection
  surfaces for resource-backed state
- browser host facts such as visibility, viewport, online status, clock, and
  persistence
- runtime diagnostics, history, replay, branching, merge planning, and exact
  graph restore
- lower-level compatibility surfaces for advanced and migration-oriented use

## Where To Start

- If you want local app state, start with the examples below and then read
  [App Surface Reference](./docs/app-surface/overview.md).
- If you want API-backed state, jump to
  [API Resources Overview](./docs/resources/overview.md).
- If you want package setup and local package workflow, start with
  [Consuming forge-signal-wasm](./docs/package/install-and-publish.md).

## The Main Authoring Model

The normal app lane is handle-based and id-less:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const count = signals.input(1);
const doubled = signals.computed(() => count() * 2);
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}));

count.set(2);

console.log(panel());
```

This is the normal app lane: you author local state by handle, not by string
id. If you want friendlier diagnostics later, you can add `debugName`, but it
is only metadata for humans. It is not identity and it is never a stable lookup
key.

## Small Example

This is the simplest useful example of the app lane:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const count = signals.input(1);
const doubled = signals.computed(() => count() * 2);

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(doubled());
```

## Real Example

This example shows the surface we actually want for ordinary feature code:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();

const itemWorkspace = signals.graph("itemWorkspace", (graph) => {
  const editor = graph.controller("editor", ({ input, computed, linked }) => {
    const serverItem = input({
      id: "task-7",
      title: "Ship docs",
      workflowTargetStateId: "ready",
    });

    const draft = input({});

    const effectiveItem = computed(() => ({
      ...serverItem(),
      ...draft(),
    }));

    const selectedWorkflowTarget = linked({
      source: () => [
        { id: "draft", label: "Draft" },
        { id: "ready", label: "Ready" },
      ],
      computation: (options, previous) => (
        options.find((option) => option.id === previous?.value?.id) ?? options[0]
      ),
    });

    const dirtyState = computed(() => (
      Object.keys(draft()).length > 0
    ));

    return {
      inputs: { serverItem, draft, selectedWorkflowTarget },
      outputs: { effectiveItem, dirtyState },
    };
  });

  return graph.expose({
    inputs: {
      serverItem: graph.input.required(editor.inputs.serverItem, {
        authority: "readOnly",
      }),
      draft: graph.input.optional(editor.inputs.draft),
      selectedWorkflowTarget: graph.input.optional(
        editor.inputs.selectedWorkflowTarget,
      ),
    },
    outputs: {
      effectiveItem: editor.outputs.effectiveItem,
      dirtyState: editor.outputs.dirtyState,
    },
  });
});

itemWorkspace.patchInput("draft", {
  title: "Ready to ship",
});

console.log(itemWorkspace.read());
```

In this example:

- `serverItem` is the source data
- `draft` is local editable state
- `effectiveItem` is derived from both
- graph publication is where public names and input rules become explicit

## Explicit Named Lane

If you need explicit structural names for spec or portability work, use
`signals.spec`:

```ts
const count = signals.spec.input("count", 1);
const doubled = signals.spec.computedCallback("doubled", () => count() * 2);
const panel = signals.spec.outputCallback("panel", () => ({
  count: count(),
  doubled: doubled(),
}));
```

Use the spec lane when names are the contract. Do not use it for ordinary app
authoring just to recreate the older id-heavy style.

## Mutation Helpers

Local inputs support direct mutation helpers:

```ts
const draft = signals.input({ title: "Ship docs", done: false });

draft.patch({ done: true });
draft.assign({ title: "Ready to ship" });
draft.reset();
```

Graphs expose the same ideas at the public boundary:

```ts
graph.writeInput("draft", { title: "Queued" });
graph.patchInput("draft", { reviewer: "Avery" });
graph.resetInput("draft");
```

Those helpers still lower through the same runtime mutation substrate as
`transaction(...)` and `apply(...)`.

## Transactions And Batch

Use `transaction(...)` or `batch(...)` for coordinated writes.

Simple:

```ts
signals.transaction((tx) => {
  tx.set(count, count() + 1);
});
```

Complex:

```ts
const part = signals.input({
  id: "gear-7",
  teeth: 24,
  enabled: true,
}, {
  producesAspects: [1, 2],
});

signals.transaction((tx) => {
  tx.setWithRegionsAndAspects(
    part,
    { ...part(), teeth: 30 },
    [{ region: "geometry" }],
    [1],
  );
});
```

Use the direct handle helpers when the change is small and local. Use
`transaction(...)` or `batch(...)` when you want one coordinated commit,
runtime-level staging, or aspect/region-aware writes.

## Linked Writable State

`signals.linked(...)` gives you dependent writable state that can be locally
overridden and later re-anchored:

```ts
const shippingOptions = signals.input([
  { id: "ground", label: "Ground" },
  { id: "air", label: "Air" },
]);

const selectedShipping = signals.linked({
  source: () => shippingOptions(),
  computation: (options, previous) => (
    options.find((option) => option.id === previous?.value?.id) ?? options[0]
  ),
});

selectedShipping.set({ id: "air", label: "Air" });
selectedShipping.relink();
selectedShipping.reset();
```

Use this when state normally follows another reactive source but should still
allow local user intent.

## Aspects

`forge-signal-wasm` supports real Forge Signal aspects on the web surface.

That means one node can carry multiple semantic change lanes, and explicit
named/spec reads can subscribe to only the lanes they actually care about.

```ts
const sensor = signals.spec.input("sensor", 10, {
  producesAspects: [1, 2],
});

const summary = signals.spec.computed("summary", {
  reads: [{ id: "sensor", aspect: 1 }],
  expr: { kind: "read", id: "sensor" },
  producesAspects: [7],
});

signals.transaction((tx) => {
  tx.setWithAspects(sensor, 11, [2]);
});
```

Use aspects when change kind matters inside the runtime, not just changed
value.

## Host Capabilities

Host capability is the typed lane for browser facts that do not belong in
ambient closure reads:

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
} from "forge-signal-wasm";

const signals = await createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return document.visibilityState;
        },
        subscribe(listener) {
          document.addEventListener("visibilitychange", listener);
          return () => document.removeEventListener("visibilitychange", listener);
        },
      },
    }),
  }),
});

const isVisible = signals.computed(() => (
  signals.host.visibility?.isVisible() ?? false
));
```

Available families today:

- `visibility`
- `viewport`
- `online`
- `clock`
- `persistence`

## API Resources

The package also ships a first-class resource surface for request-shaped,
resource-backed state:

```ts
import {
  createSignals,
  resourceParamIdentity,
  resourceParams,
} from "forge-signal-wasm";

const signals = await createSignals();

const productDetail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }) => ({ id: productId, title: `Product ${productId}` }),
});

const line = productDetail.line({ productId: "p1" });

console.log(line.value());
console.log(line.status());
console.log(line.request());
```

Use resources when the state is really a parameterized resource line with:

- request posture
- freshness and retry behavior
- upload or processing posture
- binary/download truth
- line-scoped diagnostics, history, replay, and restore

Start with the resource docs cluster when that is your main use case:

- [API Resources Overview](./docs/resources/overview.md)
- [Branch-Native Resource Effects](./docs/resources/branch-native-effects.md)
- [Resource Family Authoring Reference](./docs/api-reference/resource-family-authoring.md)
- [Resource Line Reference](./docs/api-reference/resource-line.md)
- [Resource Recipes](./docs/learn/recipes.md)

Ignore the deeper resource pages at first if you do not need them yet. For most
teams the right path is:

1. overview
2. family authoring
3. line reference
4. recipes

## Diagnostics

Start here:

```ts
const diagnostics = signals.diagnostics();
```

Use diagnostics when you need explanation, health, recent flow/observation
evidence, or host-capability event summaries.

Simple:

```ts
const why = diagnostics.why("doubled");
console.log(why.recipeFamily, why.callback?.currentReads);
```

Host-capability-specific inspection is also available:

```ts
const hostReport = diagnostics.hostCapabilityReport();
const latestHostEvent = diagnostics.latestHostCapabilityEvent();
```

Complex:

```ts
const latestObservation = diagnostics.latestObservation();
const latestFlow = diagnostics.latestFlow();
const perf = diagnostics.performanceSummary();

console.log({
  delivered: latestObservation?.observation.delivered_event_count,
  callbackReads: perf.computeCallbackCapturedReadCount,
  dependencyPatches: perf.computeCallbackDependencyPatchCount,
  callbackNodes: latestFlow?.callbackNodes.map((node) => node.id) ?? [],
});
```

If you already have a published graph, graph-scoped diagnostics are usually the
better place to start because they project explanation back onto public names:

```ts
const graphDiagnostics = itemWorkspace.inspectDiagnostics();

console.log(graphDiagnostics.output("effectiveItem").why);
console.log(graphDiagnostics.dependenciesForOutput("effectiveItem"));
console.log(graphDiagnostics.contractSummary());
```

## History, Branching, And Restore

The package exposes both runtime-wide and graph-boundary history surfaces.

Runtime-wide surfaces:

- `signals.diagnostics()`
  - use this when you want to explain current runtime behavior
  - `why(id)` answers why one runtime node changed or did not change
  - `health()` gives a broader runtime-health snapshot
  - `latestFlow()` and `latestObservation()` expose the most recent committed
    evaluation and delivery summaries
  - host-capability event summaries and performance summaries live here too
- `signals.history()`
  - use this when the question is about replay, lineage, snapshots, or branch
    state over time
  - `replay_for(id)` gives the replay artifact for one raw runtime node
  - `lineage_for(id)` gives the node's lineage summary
  - `snapshot()` captures an exact same-runtime snapshot envelope
  - `current_branch()`, `branches()`, `create_branch(name)`, and
    `switch_branch(branchId)` expose the branch model directly
  - branch snapshots, exact branch restore, merge planning, merge execution,
    replay parity proof, and branch-state proof all live here

## Branching

Branching is part of the runtime history surface. A branch is a named runtime
line with its own head snapshot and parent branch relationship:

- `current_branch()` returns the active branch handle
- `branches()` lists the known branches
- `create_branch(name)` forks a new branch from the current runtime position
- `switch_branch(branchId)` moves the active runtime onto another branch

Simple:

```ts
const history = signals.history();

const main = history.current_branch();
const draft = history.create_branch("draft");

history.switch_branch(draft.id);

console.log({
  current: history.current_branch().name,
  branches: history.branches().map((branch) => branch.name),
});
```

Use this when you want to explore or stage alternative runtime states without
throwing away the current line.

Simple:

```ts
const history = signals.history();

const currentBranch = history.current_branch();
const draftBranch = history.create_branch("draft");
const snapshot = history.snapshot();

console.log(currentBranch.name);
console.log(draftBranch.id);
console.log(snapshot.snapshotEnvelopeRestoreMode);
```

Complex:

```ts
const history = signals.history();
const source = history.create_branch("incoming");
const target = history.current_branch();

const mergePlan = history.plan_merge_branches_with_proof(source.id, target.id);
const mergeResult = history.merge_branches_with_proof(source.id, target.id);
const targetProof = history.branch_state_proof(target.id);

console.log({
  mergePlanDigest: mergePlan.proof.planDigest,
  mergeResultDigest: mergeResult.proof.resultDigest,
  targetBranchStateDigest: targetProof.branchStateDigest,
});
```

Graph-boundary surfaces:

- `contract()`
  - the current public graph contract with input/output names and descriptors
- `operationalContract()`
  - the graph contract plus operational posture such as public input authority
- `inspectDiagnostics()`
  - graph-scoped explanation with public names, dependency explanations, and
    latest flow/observation projected onto the boundary
- `inspectHistory()`
  - replay and lineage projected onto graph inputs and outputs instead of raw
    runtime ids
- `contractHistory()`
  - the accumulated contract-history view for the published graph
- `contractDelta(...)`
  - the explicit delta between two graph contracts
- `exportDefinition()`
  - the portable definition artifact for this published graph boundary
- `exportSnapshot()`
  - the exact same-runtime snapshot artifact paired with that definition
- `importPosture()`
  - the graph's admitted restore/import posture

Graph-scoped history gives you replay and lineage already projected onto graph
inputs and outputs:

```ts
const graphHistory = itemWorkspace.inspectHistory();

console.log(graphHistory.inputs.draft.replay);
console.log(graphHistory.outputs.effectiveItem.lineage);
console.log(graphHistory.recentHistory);
```

Published graphs can also export exact restore artifacts directly:

```ts
const definition = itemWorkspace.exportDefinition();
const snapshot = itemWorkspace.exportSnapshot();
const restored = (await createSignals()).importGraph(definition, snapshot);

console.log(restored.contractHistory());
```

Use the runtime-wide lane when the question is about raw runtime causality,
branch state, replay, or merge planning. Use the graph lane when the question
is about a published contract with public names, requiredness, authorities,
dependencies, replay, and lineage already projected onto the boundary.

## React Adapter

The optional React adapter is intentionally thin. React reads and writes the
same signal state; it does not become a second state engine.

```ts
import { createReactSignalsStore } from "forge-signal-wasm/react";

const store = createReactSignalsStore(signals);
```

Simple:

```tsx
import { useSignalValue } from "forge-signal-wasm/react";

function Counter() {
  const countValue = useSignalValue(count, store);

  return (
    <button onClick={() => store.transaction((tx) => tx.set(count, countValue + 1))}>
      {countValue}
    </button>
  );
}
```

Complex:

```tsx
import {
  useOutputValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

function ItemPanel() {
  const effectiveItem = useOutputValue(itemWorkspace.output("effectiveItem"), store);
  const dirtyState = useOutputValue(itemWorkspace.output("dirtyState"), store);
  const diagnostics = useSignalsDiagnostics(store);

  return (
    <>
      <pre>{JSON.stringify(effectiveItem, null, 2)}</pre>
      <small>
        dirty: {String(dirtyState)}
        {" | "}
        callback patches: {diagnostics.performanceSummary.computeCallbackDependencyPatchCount}
      </small>
    </>
  );
}
```

## Compatibility Recovery

The normal package lane is worker-first:

```ts
const signals = await createSignals();
```

If worker construction is unavailable, recover explicitly instead of expecting
the package to fall back on its own:

```ts
import { createSignals } from "forge-signal-wasm";

let signals;

try {
  signals = await createSignals();
} catch (error) {
  if (error?.artifactFamily !== "workerUnavailableConstruction") {
    throw error;
  }
  signals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
}
```

Use explicit compatibility construction when the environment cannot admit the
worker lane or when you deliberately need the main-thread runtime posture.

## Docs

- [Documentation Index](./docs/README.md)
- [Consuming forge-signal-wasm](./docs/package/install-and-publish.md)
- [App Surface Reference](./docs/app-surface/overview.md)
- [API Resources Overview](./docs/resources/overview.md)
- [Resource Family Authoring Reference](./docs/api-reference/resource-family-authoring.md)
- [Resource Line Reference](./docs/api-reference/resource-line.md)
- [Resource Request And Policy Reference](./docs/api-reference/resource-request-and-policy.md)
- [Resource Reconciliation Reference](./docs/resource-contracts/reconciliation.md)
- [Resource Transfers Reference](./docs/api-reference/resource-transfers.md)
- [Resource Binary And Download Reference](./docs/api-reference/resource-binary-and-download.md)
- [Resource Delivery And Compatibility Reference](./docs/resource-contracts/delivery-and-compatibility.md)
- [Resource Inspection And History Reference](./docs/resource-contracts/inspection-and-history.md)
- [Resource Recipes](./docs/learn/recipes.md)
- [Aspects Reference](./docs/app-surface/aspects.md)
- [Host Capabilities](./docs/app-surface/host-capabilities.md)
- [Diagnostics And History Reference](./docs/app-surface/diagnostics-and-history.md)
- [Compatibility Surface Reference](./docs/api-reference/compatibility-surface.md)
- [React Adapter Reference](./docs/app-surface/react-adapter.md)

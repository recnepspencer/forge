# forge-signal-wasm

Framework-agnostic browser bindings for Forge Signal, with a callback-based
app surface, graph publication, typed host capabilities, and an optional
React adapter.

The package also includes aspect-aware invalidation, graph-scoped input
operations, runtime diagnostics/history, exact graph restore, and lower-level
compatibility surfaces when you need them.

## Install

Public npm package:

```bash
npm install forge-signal-wasm
```

Before publishing a new version from this repo, always run the package proof:

```powershell
node scripts/wasm/verify-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

Or use the release-gate helper:

```powershell
scripts/wasm/publish-forge-signal-wasm.ps1 -SkipPublish
```

React adapter:

```bash
npm install forge-signal-wasm react
```

## Quick Start

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(panel());
```

The canonical app-authoring grammar is:

- `input(value, { id })`
- `computed(() => ..., { id })`
- `output(() => ..., { id })`

including string-valued inputs such as `signals.input("Ada", { id: "name" })`.

## Controller Composition

For feature-level app code, prefer `signals.graph("id", (graph) => ...)`
plus `graph.scope(...)` over hand-prefixing ids yourself.

Simple:

```ts
import { createSignals } from "forge-signal-wasm";

const counterGraph = createSignals().graph("counter", (graph) => {
  const counter = graph.scope("counter");
  const count = counter.input(1, { id: "count" });
  const doubled = counter.computed(() => count() * 2, { id: "doubled" });

  return graph.expose({
    controllers: [
      counter.controller({
        inputs: { count },
        outputs: { doubled },
      }),
    ],
    outputs: { count },
  });
});
```

If a published graph input should stay visible but not graph-writable, wrap it
explicitly:

```ts
const serverValue = form.input({ id: "task-7" }, { id: "serverValue" });
const routeParams = form.input({ taskId: "task-7" }, { id: "routeParams" });

return form.controller({
  inputs: {
    serverValue: form.publicInput(serverValue, { authority: "readOnly" }),
    routeParams: form.publicInput(routeParams, { authority: "imported" }),
  },
  outputs: {
    effectiveValue,
  },
});
```

`writable` is the default. `readOnly` and `imported` stay in the public graph
contract and diagnostics/history surfaces, but graph-native writes, patches,
resets, and transactions deny them.

More realistic:

```ts
import {
  createSignals,
  type ComputedSignalHandle,
  type InputSignalHandle,
  type SignalNamespace,
} from "forge-signal-wasm";

type ItemServerData = {
  id: string;
  name: string;
  workflow_target_state_id?: string | null;
};

type ItemDraftEdits = Partial<Pick<
  ItemServerData,
  "name" | "workflow_target_state_id"
>>;

type EditSessionController = {
  inputs: {
    serverItemData: InputSignalHandle<ItemServerData | null>;
    draftEdits: InputSignalHandle<ItemDraftEdits>;
  };
  outputs: {
    effectiveItemData: ComputedSignalHandle<ItemServerData>;
    dirtyState: ComputedSignalHandle<{ isDirty: boolean }>;
  };
  internal: Record<string, never>;
};

function createEditSessionController(
  signals: SignalNamespace,
): EditSessionController {
  const serverItemData = signals.input<ItemServerData | null>(null, {
    id: "serverItemData",
  });
  const draftEdits = signals.input<ItemDraftEdits>({}, {
    id: "draftEdits",
  });

  const effectiveItemData = signals.computed(() => ({
    ...(serverItemData() ?? {}),
    ...draftEdits(),
  }), { id: "effectiveItemData" });

  const dirtyState = signals.computed(() => ({
    isDirty: Object.keys(draftEdits()).length > 0,
  }), { id: "dirtyState" });

  return signals.controller({
    inputs: {
      serverItemData,
      draftEdits,
    },
    outputs: {
      effectiveItemData,
      dirtyState,
    },
  });
}

type WorkflowController = {
  inputs: Record<string, never>;
  outputs: {
    submitReadiness: ComputedSignalHandle<{
      enabled: boolean;
      targetStateId: string | null;
    }>;
  };
  internal: Record<string, never>;
};

function createWorkflowController(
  signals: SignalNamespace,
  editSession: EditSessionController,
): WorkflowController {
  const submitReadiness = signals.computed(() => {
    const item = editSession.outputs.effectiveItemData();
    const dirty = editSession.outputs.dirtyState();

    return {
      enabled: dirty.isDirty && Boolean(item.workflow_target_state_id),
      targetStateId: item.workflow_target_state_id ?? null,
    };
  }, { id: "submitReadiness" });

  return signals.controller({
    outputs: {
      submitReadiness,
    },
  });
}

const itemDetailGraph = createSignals().graph("itemDetail", (graph) => {
  const editSession = createEditSessionController(graph.scope("editSession"));
  const workflow = createWorkflowController(graph.scope("workflow"), editSession);

  return graph.expose({
    controllers: [editSession, workflow],
  });
});
```

For side-by-side page/modal copies of the same controller family, scope each
instance and alias the public contract deliberately:

```ts
const itemWorkspaceGraph = createSignals().graph("itemWorkspace", (graph) => {
  const page = createEditSessionController(graph.scope("page"));
  const modal = createEditSessionController(graph.scope("modal"));

  return graph.expose({
    inputs: {
      pageServerItemData: page.inputs.serverItemData,
      modalServerItemData: modal.inputs.serverItemData,
    },
    outputs: {
      pageEffectiveItemData: page.outputs.effectiveItemData,
      modalEffectiveItemData: modal.outputs.effectiveItemData,
    },
  });
});
```

For repeated row-level editors, keep the controller family the same and let the
graph own instance identity:

```ts
const rowEditorsGraph = createSignals().graph("rowEditors", (graph) => {
  const row0 = createEditSessionController(graph.scope("row-0"));
  const row1 = createEditSessionController(graph.scope("row-1"));

  return graph.expose({
    outputs: {
      row0EffectiveItemData: row0.outputs.effectiveItemData,
      row1EffectiveItemData: row1.outputs.effectiveItemData,
    },
  });
});
```

Authored signal ids still become canonical runtime ids under the hood.
`graph.scope(...)` and `signals.scope(...)` own that prefixing step for normal
app code; manual string prefixing is mainly a compatibility bridge for older
controllers. Graph `inputs` and `outputs` are the public contract names
consumers read through the published graph artifact, while controller
`internal` signals stay private unless you deliberately re-expose them.

The published graph is also the contract object forms and resource-style
controllers should build on:

```ts
console.log(itemDetailGraph.contract().inputs.serverItemData);
console.log(itemDetailGraph.inspectDiagnostics().inputs.serverItemData.why);
console.log(itemDetailGraph.inspectHistory().outputs.submitReadiness.replay);
console.log(itemDetailGraph.operationalContract().authorities);
itemDetailGraph.patchInputs({
  draftEdits: { name: "Updated name" },
});
console.log(itemDetailGraph.importPosture());
const exported = itemDetailGraph.exportDefinition();
const snapshot = itemDetailGraph.exportSnapshot();
const restored = createSignals().importGraph(exported, snapshot);
console.log(restored.contractHistory());
```

Published graphs also expose graph-native input operations:

- `writeInputs(...)`
- `patchInputs(...)`
- `resetInputs(...)`
- `apply(...)`
- graph-scoped `transaction(...)`

Use those when you want to operate on the published graph contract directly
instead of dropping back to raw runtime-wide input handles.

## Host Capabilities

Use host capabilities when callback-authored derived state needs approved
browser/runtime-local facts.

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
  viewportCapability,
} from "forge-signal-wasm";

const signals = createSignals({
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
      compatibility: "LiveOnly",
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return { width: window.innerWidth, height: window.innerHeight };
        },
        subscribe(listener) {
          window.addEventListener("resize", listener);
          return () => window.removeEventListener("resize", listener);
        },
      },
    }),
  }),
});

const layout = signals.computed(() => (
  signals.host.visibility.isVisible() && signals.host.viewport.width() > 900
    ? "wide"
    : "narrow"
), { id: "layout" });
```

Good to know:

- host capability reads are typed `signals.host.*` reads, not ambient closure
  reads
- unsupported host reads stay non-reactive by contract
- diagnostics and transport surfaces preserve denied vs unavailable family
  posture
- available families include `viewport`, `visibility`, `online`, `clock`, and
  `persistence`

For the full guide, see
[docs/host_capabilities.md](./docs/host_capabilities.md).

## Aspects

Aspects let one node carry multiple semantic change channels so reads and
invalidations can stay narrower than "everything changed".

Simple:

```ts
const part = signals.input({
  id: "gear-7",
  teeth: 24,
  enabled: true,
}, {
  id: "part",
  producesAspects: [1, 2],
});

signals.transaction((tx) => {
  tx.setWithAspects(part, {
    id: "gear-7",
    teeth: 26,
    enabled: true,
  }, [1]);
});
```

For the full guide, see
[docs/aspects_reference.md](./docs/aspects_reference.md).

## Core Concepts

### `input`

Use `input` for mutable source state.

Simple:

```ts
const count = signals.input(1, { id: "count" });
signals.transaction((tx) => tx.set(count, 2));
```

Complex:

```ts
const part = signals.input({
  id: "gear-7",
  teeth: 24,
  enabled: true,
}, {
  id: "part",
  producesAspects: [1, 2],
});

signals.transaction((tx) => {
  tx.setWithAspects(part, {
    id: "gear-7",
    teeth: 26,
    enabled: true,
  }, [1]);
});
```

### `computed`

Use `computed` for runtime-owned derived state. Callback form is the normal
authoring lane.

Only callable signal reads are tracked. Ordinary closure variables are not
reactive dependencies, and a callback that captures no signal reads can be
lowered into a constantized node.

Simple:

```ts
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
```

Complex:

```ts
const enabled = signals.input(true, { id: "enabled" });
const name = signals.input("Ada", { id: "name" });

const label = signals.computed(() => {
  return enabled() ? `${name()} is enabled` : "disabled";
}, { id: "label" });
```

Advanced recipe form still exists:

```ts
const doubled = signals.computedSpec("doubled", {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
});
```

### `output`

Use `output` for public projections you hand to UI layers, tables, panels, or
other consumers.

Callback outputs follow the same capture rule as callback computed nodes:
signal reads are tracked, ordinary closure variables are not, and richer
aspect-targeted projection contracts belong on the explicit spec lane.

Simple:

```ts
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
```

Complex:

```ts
const partSummary = signals.output(() => ({
  part: part(),
  label: label(),
  status: part().enabled ? "active" : "inactive",
}), { id: "partSummary" });
```

Advanced recipe form still exists when you need explicit portable specs:

```ts
const partSummary = signals.outputSpec("partSummary", {
  reads: ["part", "label"],
  expr: {
    kind: "object",
    fields: [
      ["part", { kind: "read", id: "part" }],
      ["label", { kind: "read", id: "label" }],
    ],
  },
});
```

### `watch` and `effect`

Use `watch` when you want the notice payload. Use `effect` when you only need a
committed side-effect trigger.

Simple:

```ts
const handle = signals.watch(panel, (notice) => {
  console.log(notice.signalId, notice.meaningfulChange);
});
```

Complex:

```ts
const saveHandle = signals.effect(partSummary, () => {
  const payload = partSummary();
  queueMicrotask(() => saveDraft(payload));
});

signals.nuke(saveHandle);
```

### `transaction`

Use `transaction` or `batch` for all writes.

Simple:

```ts
signals.transaction((tx) => {
  tx.set(count, count() + 1);
});
```

Complex:

```ts
signals.transaction((tx) => {
  tx.setWithRegionsAndAspects(
    part,
    { ...part(), teeth: 30 },
    [{ region: "geometry" }],
    [1],
  );
});
```

## Diagnostics

Start here:

```ts
const diagnostics = signals.diagnostics();
```

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

Published graphs also have graph-scoped inspection surfaces:

```ts
const graphDiagnostics = itemDetailGraph.inspectDiagnostics();
const graphHistory = itemDetailGraph.inspectHistory();

console.log(graphDiagnostics.outputs.submitReadiness.why);
console.log(graphHistory.outputs.submitReadiness.replay);
```

The runtime also includes a larger history surface for snapshots, branching,
replay, lineage, and merge planning when you need it.

## React Adapter

```ts
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useSignalValue,
  useOutputValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);
```

Simple:

```tsx
function Counter() {
  const countValue = useSignalValue(count, store);
  return <button onClick={() => store.transaction((tx) => tx.set(count, countValue + 1))}>
    {countValue}
  </button>;
}
```

Complex:

```tsx
function PartPanel() {
  const summary = useOutputValue(partSummary, store);
  const diagnostics = useSignalsDiagnostics(store);

  return (
    <>
      <pre>{JSON.stringify(summary, null, 2)}</pre>
      <small>{diagnostics.performanceSummary.computeCallbackDependencyPatchCount}</small>
    </>
  );
}
```

## Advanced Lanes

- Prefer callback-first `computed(() => ...)` and `output(() => ...)` for
  ordinary app code.
- Prefer `signals.input(value, { id })` when you want the family to read with
  one coherent grammar, including string-valued inputs.
- Keep `computedSpec(...)` and `outputSpec(...)` for explicit portable recipe
  authoring.
- Keep `compatibilityApp()` and `compatibilityRuntime()` for expert or migration
  scenarios, not for the default product lane.
- Keep `history()` and `adapters()` for snapshot/replay/export/proof work when
  you need deeper runtime control than the normal app path.

## Documentation

- [docs/README.md](docs/README.md)
- [docs/consuming_the_package.md](docs/consuming_the_package.md)
- [docs/app_surface_reference.md](docs/app_surface_reference.md)
- [docs/diagnostics_and_history_reference.md](docs/diagnostics_and_history_reference.md)
- [docs/react_adapter_reference.md](docs/react_adapter_reference.md)

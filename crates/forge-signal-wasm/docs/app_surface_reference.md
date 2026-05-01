# App Surface Reference

This is the reference for the primary `forge-signal-wasm` app surface. Every
major concept includes a simple example and a more realistic one.

For the dedicated host-capability guide, including lifecycle, compatibility
posture, diagnostics, and anti-patterns, see
[host_capabilities.md](./host_capabilities.md).

## Entry Point

### `createSignals(): CallableSignals`

Creates a framework-agnostic runtime instance.

Simple:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();
```

Complex:

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const enabled = signals.input(true, { id: "enabled" });
const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const panel = signals.output(() => ({
  enabled: enabled(),
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
```

Host-capability registration is explicit when you want browser-local facts to
participate in callback-derived state:

```ts
import {
  clockCapability,
  createSignals,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  viewportCapability,
  visibilityCapability,
} from "forge-signal-wasm";

let persistedDraft = { mode: "draft", revision: 1 };

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
          return {
            width: window.innerWidth,
            height: window.innerHeight,
          };
        },
        subscribe(listener) {
          window.addEventListener("resize", listener);
          return () => window.removeEventListener("resize", listener);
        },
      },
    }),
    online: onlineCapability({
      source: {
        current() {
          return navigator.onLine ? "online" : "offline";
        },
        subscribe(listener) {
          window.addEventListener("online", listener);
          window.addEventListener("offline", listener);
          return () => {
            window.removeEventListener("online", listener);
            window.removeEventListener("offline", listener);
          };
        },
      },
    }),
    clock: clockCapability({
      source: {
        current() {
          return Date.now();
        },
      },
      pollMs: 1000,
    }),
    persistence: persistenceCapability({
      source: {
        current() {
          return persistedDraft;
        },
      },
    }),
  }),
});
```

Good to know:

- host capability is the typed lane for browser/runtime-local facts
- `signals.host.*` handles are framework-owned and not user-disposable
- unsupported ambient host reads remain non-reactive
- per-family compatibility posture matters during restore/import/export

### `default init(): Promise<undefined>`

Low-level wasm initialization hook retained for completeness. Normal app code
usually imports `createSignals()` and lets the package wiring handle this for
them.

Simple:

```ts
import init from "forge-signal-wasm";
```

Complex:

```ts
import init, { createSignals } from "forge-signal-wasm";

await init();
const signals = createSignals();
```

## Value Model

### `SignalValue`

`SignalValue` is the JSON-like value model:

- `null`
- `boolean`
- `number`
- `string`
- arrays
- nested objects

Simple:

```ts
const count = signals.input(1, { id: "count" });
```

String values use the same value-first form:

```ts
const name = signals.input("Ada", { id: "name" });
```

Complex:

```ts
const part = signals.input({
  id: "gear-7",
  dimensions: { teeth: 24, pitch: 1.5 },
  flags: ["released", "visible"],
}, { id: "part" });
```

## Handles

### `InputSignal`

Mutable source state.

Simple:

```ts
const count = signals.input(1, { id: "count" });
console.log(count.id, count.get());
```

Complex:

```ts
const settings = signals.input({
  mode: "advanced",
  autosave: true,
}, { id: "settings" });

signals.transaction((tx) => {
  tx.set(settings, {
    mode: "advanced",
    autosave: false,
  });
});
```

### `ComputedSignal`

Derived internal state. Callback authoring is the normal lane.

Only callable signal reads are tracked. Ordinary closure variables are not
reactive dependencies.

Simple:

```ts
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
console.log(doubled());
```

Complex:

```ts
const label = signals.computed(() => {
  if (!enabled()) return "disabled";
  return `${name()} x${count()}`;
}, { id: "label" });
```

Host capability reads stay explicit and typed:

```ts
const visibilityLabel = signals.computed(() => (
  signals.host.visibility?.isVisible() ? "visible" : "hidden"
), { id: "visibilityLabel" });

const viewportLabel = signals.computed(() => (
  `${signals.host.viewport?.width() ?? 0}x${signals.host.viewport?.height() ?? 0}`
), { id: "viewportLabel" });

const connectivityLabel = signals.computed(() => (
  signals.host.online?.isOnline() ? "online" : "offline"
), { id: "connectivityLabel" });

const secondLabel = signals.computed(() => (
  Math.floor((signals.host.clock?.now() ?? 0) / 1000)
), { id: "secondLabel" });

const draftRevision = signals.computed(() => (
  signals.host.persistence?.value().revision ?? 0
), { id: "draftRevision" });

persistedDraft = { mode: "published", revision: 2 };
signals.host.persistence?.commit();
```

That callback captures only declared signal and host-capability reads. Ordinary
closure state is still not reactive.

### `OutputSignal`

Public projection for UI/framework consumption.

Simple:

```ts
const panel = signals.output(() => ({
  count: count(),
}), { id: "panel" });
```

Complex:

```ts
const summary = signals.output(() => ({
  part: part(),
  label: label(),
  teeth: part().dimensions.teeth,
}), { id: "summary" });
```

### `PublishedSignalGraph`

Named publication artifact produced by `signals.graph(...)`.

Simple:

```ts
const counterGraph = signals.graph("counter", {
  outputs: {
    count,
    doubled,
  },
});

console.log(counterGraph.read());
```

Complex:

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
  const serverItemData = signals.input<ItemServerData | null>(null, { id: "serverItemData" });
  const draftEdits = signals.input<ItemDraftEdits>({}, { id: "draftEdits" });

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

const signals = createSignals();
const itemDetailGraph = signals.graph("itemDetail", (graph) => {
  const editSession = createEditSessionController(graph.scope("editSession"));

  return graph.expose({
    inputs: {
      ...editSession.inputs,
    },
    outputs: {
      ...editSession.outputs,
    },
  });
});

console.log(itemDetailGraph.contract().inputs.serverItemData);
console.log(itemDetailGraph.inspectDiagnostics().inputs.serverItemData.why);
console.log(itemDetailGraph.inspectDiagnostics().outputs.effectiveItemData.why);
console.log(itemDetailGraph.inspectDiagnostics().dependenciesForOutput("effectiveItemData"));
console.log(itemDetailGraph.contractDelta(itemDetailGraph.contract()));
const exported = itemDetailGraph.exportDefinition();
const snapshot = itemDetailGraph.exportSnapshot();
const restored = createSignals().importGraph(exported, snapshot);
console.log(itemDetailGraph.importPosture());
console.log(restored.contractHistory());
```

When a published input must stay visible but not writable through the graph,
wrap the input handle before exposure:

```ts
const serverValue = form.input({ id: "task-7" }, { id: "serverValue" });
const externalParams = form.input({ taskId: "task-7" }, { id: "externalParams" });

return form.controller({
  inputs: {
    serverValue: form.publicInput(serverValue, { authority: "readOnly" }),
    externalParams: form.publicInput(externalParams, { authority: "imported" }),
  },
  outputs: {
    effectiveValue,
  },
});
```

`graph.operationalContract().authorities` preserves that authority explicitly.
Only `writable` inputs participate in graph-native writes, patches, resets,
and transactions.

Repeated controller families should also stay graph-owned. A page + modal copy
of the same controller family should scope each instance and alias the public
contract deliberately:

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

The same pattern scales to repeated row-level editors:

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

### `DisposableHandle`

Lifecycle handle from `watch(...)` and `effect(...)`.

Simple:

```ts
const handle = signals.watch(panel, () => {});
signals.nuke(handle);
```

Complex:

```ts
using handle = signals.effect(summary, () => {
  queueMicrotask(() => renderSummary(summary()));
});
```

### `SignalsTransaction`

Write lane used inside `transaction(...)` and `batch(...)`.

Simple:

```ts
signals.transaction((tx) => {
  tx.set(count, 2);
});
```

Complex:

```ts
signals.transaction((tx) => {
  tx.setWithAspects(part, {
    ...part(),
    dimensions: { ...part().dimensions, teeth: 30 },
  }, [1]);
});
```

## Core Methods On `Signals`

### `scope(localScopeId): ScopedSignalNamespace`

Simple:

```ts
const editSession = signals.scope("itemDetail.editSession");
const count = editSession.input(1, { id: "count" });
```

Complex:

```ts
const itemDetail = signals.scope("itemDetail");
const editSession = itemDetail.scope("editSession");
const workflow = itemDetail.scope("workflow");
```

### `controller(definition): ControllerContract`

Creates a package-understood controller artifact with explicit `inputs`,
`outputs`, and optional `internal` categories.

Simple:

```ts
const counter = signals.scope("counter");
const count = counter.input(1, { id: "count" });
const doubled = counter.computed(() => count() * 2, { id: "doubled" });

const counterController = counter.controller({
  inputs: { count },
  outputs: { doubled },
});
```

Complex:

```ts
function createEditSessionController(namespace: SignalNamespace) {
  const serverItemData = namespace.input(null, { id: "serverItemData" });
  const draftEdits = namespace.input({}, { id: "draftEdits" });
  const effectiveItemData = namespace.computed(() => ({
    ...(serverItemData() ?? {}),
    ...draftEdits(),
  }), { id: "effectiveItemData" });
  const dirtyState = namespace.computed(() => ({
    isDirty: Object.keys(draftEdits()).length > 0,
  }), { id: "dirtyState" });
  const validationTrace = namespace.computed(() => ({
    dirty: dirtyState().isDirty,
  }), { id: "validationTrace" });

  return namespace.controller({
    inputs: {
      serverItemData,
      draftEdits,
    },
    outputs: {
      effectiveItemData,
      dirtyState,
    },
    internal: {
      validationTrace,
    },
  });
}
```

### `input(initial, options?): InputSignal`

Simple:

```ts
const count = signals.input(1, { id: "count" });
```

Complex:

```ts
const part = signals.input({
  id: "gear-7",
  enabled: true,
}, {
  id: "part",
  producesAspects: [1, 2],
});
```

### `computed(...)`

Preferred callback form:

```ts
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
```

Legacy compatibility form:

```ts
const doubled = signals.computed("doubled", () => count() * 2);
```

Complex branchy callback:

```ts
const label = signals.computed(() => {
  return enabled() ? `${name()} x${count()}` : "disabled";
}, { id: "label" });
```

### `graph(id, definitionOrBuilder)`

Controller-first publication boundary for feature composition.

Simple:

```ts
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

Complex:

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

function createEditSessionController(signals: SignalNamespace): EditSessionController {
  const serverItemData = signals.input<ItemServerData | null>(null, { id: "serverItemData" });
  const draftEdits = signals.input<ItemDraftEdits>({}, { id: "draftEdits" });

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

`signals.graph(...)` accepts either:

- a direct contract object with `inputs?` and `outputs`
- or a builder callback that owns scoped construction through `graph.scope(...)`
  and returns `graph.expose(...)`

Published graph `inputs` preserve same-runtime input handles. Published graph
`outputs` preserve existing output handles and synthesize deterministic output
authorities from published input/computed handles when needed.

Current identity rules:

- authored signal ids must be unique within one runtime instance
- `graph.scope(...)` and `signals.scope(...)` own canonical runtime prefixing
  for significant app code
- graph `inputs` and `outputs` keys are the public contract names exposed by
  the published graph artifact
- controller artifacts may distinguish `inputs`, `outputs`, and `internal`
  structure explicitly
- controller `internal` entries stay private unless deliberately re-exposed

The graph artifact gives you:

- `contract()`
- `input(name)`
- `inputs`
- `readInputs()`
- `inputDescriptors()`
- `output(name)`
- `outputs`
- `read()`
- `summary()`
- `descriptors()`
- `inspectDiagnostics()`
- `inspectHistory()`
- `contractDelta(previousContract)`
- `contractHistory()`
- `importPosture()`
- `exportCompatibilityDefinition()`
- `exportDefinition()`
- `exportSnapshot()`

The callable runtime also supports:

- `importGraph(exportedDefinition, exportedSnapshot)`

That contract surface is the bridge future forms/resources-style products can
build on: explicit published inputs and outputs without inventing a separate
scope or lifecycle model.

Advanced recipe form:

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

### `output(...)`

Preferred callback form:

```ts
const panel = signals.output(() => ({
  count: count(),
}), { id: "panel" });
```

Complex callback form:

```ts
const dashboard = signals.output(() => ({
  part: part(),
  label: label(),
  count: count(),
}), { id: "dashboard" });
```

Advanced recipe form:

```ts
const dashboard = signals.outputSpec("dashboard", {
  reads: ["part", "label", "count"],
  expr: {
    kind: "object",
    fields: [
      ["part", { kind: "read", id: "part" }],
      ["label", { kind: "read", id: "label" }],
      ["count", { kind: "read", id: "count" }],
    ],
  },
});
```

Notes:

- `output(...)` is the public projection concept.
- Callback-first `output(() => ..., { id })` is the preferred product lane.
- `outputSpec(...)` is the explicit portable recipe lane.
- Aspect-filtered reads and produced-aspect declarations currently belong on
  the explicit spec lane rather than the callback shorthand.

### `transaction(callback): RunSummary`

Simple:

```ts
signals.transaction((tx) => {
  tx.set(count, count() + 1);
});
```

Complex:

```ts
const summary = signals.transaction((tx) => {
  tx.set(enabled, true);
  tx.set(name, "Grace");
  tx.set(count, 4);
});

console.log(summary.nodesRecomputed);
```

### `batch(callback): RunSummary`

Ergonomic alias of `transaction(...)`.

Simple:

```ts
signals.batch((tx) => {
  tx.set(count, 5);
});
```

Complex:

```ts
signals.batch((tx) => {
  tx.set(enabled, false);
  tx.set(count, 0);
});
```

### `watch(target, callback): DisposableHandle`

Simple:

```ts
const handle = signals.watch(panel, (notice) => {
  console.log(notice.meaningfulChange);
});
```

Complex:

```ts
const handle = signals.watch("summary", (notice) => {
  if (notice.triggerMatched) {
    enqueueAuditRecord(notice);
  }
});
```

### `effect(target, callback): DisposableHandle`

Simple:

```ts
const handle = signals.effect(panel, () => {
  console.log(panel());
});
```

Complex:

```ts
const handle = signals.effect(dashboard, () => {
  queueMicrotask(() => syncInspector(dashboard()));
});
```

### `nuke(handle): boolean`

Simple:

```ts
signals.nuke(handle);
```

Complex:

```ts
const watchHandle = signals.watch(panel, () => {});
const effectHandle = signals.effect(panel, () => {});

signals.nuke(watchHandle);
signals.nuke(effectHandle);
```

### `diagnostics()`, `history()`, `specialist()`, `adapters()`

These open the deeper runtime surfaces.

Simple:

```ts
const diagnostics = signals.diagnostics();
const history = signals.history();
```

Complex:

```ts
const diagnostics = signals.diagnostics();
const adapters = signals.adapters();
const history = signals.history();

console.log(diagnostics.performanceSummary());
console.log(adapters.exportDefinitions());
console.log(adapters.exportRuntimeEnvelope());
console.log(history.current_branch());
```

Notes:

- `adapters().exportRuntimeEnvelope()` / `replaceRuntimeEnvelope(...)` are the
  expert import/export lane for runtime definitions plus captured snapshot
  state.
- restoring callback-backed nodes without live callback registrations is a
  typed denial rather than a silent degraded import.
- the product `history()` surface accepts the numeric branch ids it returns
  from `current_branch()` and `create_branch(...)`, even though the raw wasm
  layer still speaks in lower-level `u64`/`bigint` terms.

## `RunSummary`

Write boundaries return:

- `touchedNodes`
- `nodesEvaluated`
- `nodesRecomputed`
- `nodesSuppressed`
- `plansBuilt`
- `stagesExecuted`
- `totalNanos`
- `evaluationNanos`
- `commitNanos`

Simple:

```ts
const summary = signals.transaction((tx) => tx.set(count, 2));
console.log(summary.nodesRecomputed);
```

Complex:

```ts
const summary = signals.transaction((tx) => {
  tx.set(enabled, true);
  tx.set(name, "Grace");
  tx.set(count, 5);
});

console.log({
  touched: summary.touchedNodes,
  evaluated: summary.nodesEvaluated,
  total: summary.totalNanos,
});
```

## `ComputedSpec` And `OutputSpec`

Use these when you want explicit recipe authoring.

Simple:

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

Complex:

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
  identity: { kind: "exact" },
});
```

## Aspect-Aware Reads And Writes

Simple:

```ts
const part = signals.input({ teeth: 24 }, {
  id: "part",
  producesAspects: [1],
});
```

Complex:

```ts
signals.transaction((tx) => {
  tx.setWithAspects(part, { teeth: 26 }, [1]);
});

const summary = signals.outputSpec("summary", {
  reads: [{ id: "part", aspects: [1] }],
  expr: { kind: "read", id: "part" },
});
```

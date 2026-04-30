# App Surface Reference

This is the reference for the primary `forge-signal-wasm` app surface. Every
major concept includes a simple example and a more realistic one.

## Entry Point

### `createSignals(): Signals`

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

### `start(): void`

Low-level wasm start hook retained for completeness. Normal app code should not
need it.

Simple:

```ts
import { start } from "forge-signal-wasm";
```

Complex:

```ts
import { start, createSignals } from "forge-signal-wasm";

start();
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

Legacy id-first callback form:

```ts
const doubled = signals.computed("doubled", () => count() * 2);
```

Complex branchy callback:

```ts
const label = signals.computed(() => {
  return enabled() ? `${name()} x${count()}` : "disabled";
}, { id: "label" });
```

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

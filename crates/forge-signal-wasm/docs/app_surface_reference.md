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

const enabled = signals.input("enabled", true);
const count = signals.input("count", 1);
const doubled = signals.computed("doubled", () => count() * 2);
const panel = signals.output("panel", {
  reads: ["enabled", "count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["enabled", { kind: "read", id: "enabled" }],
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
});
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
const count = signals.input("count", 1);
```

Complex:

```ts
const part = signals.input("part", {
  id: "gear-7",
  dimensions: { teeth: 24, pitch: 1.5 },
  flags: ["released", "visible"],
});
```

## Handles

### `InputSignal`

Mutable source state.

Simple:

```ts
const count = signals.input("count", 1);
console.log(count.id, count.get());
```

Complex:

```ts
const settings = signals.input("settings", {
  mode: "advanced",
  autosave: true,
});

signals.transaction((tx) => {
  tx.set(settings, {
    mode: "advanced",
    autosave: false,
  });
});
```

### `ComputedSignal`

Derived internal state. Callback authoring is the normal lane.

Simple:

```ts
const doubled = signals.computed("doubled", () => count() * 2);
console.log(doubled());
```

Complex:

```ts
const label = signals.computed("label", () => {
  if (!enabled()) return "disabled";
  return `${name()} x${count()}`;
});
```

### `OutputSignal`

Public projection for UI/framework consumption.

Simple:

```ts
const panel = signals.output("panel", {
  reads: ["count"],
  expr: {
    kind: "object",
    fields: [["count", { kind: "read", id: "count" }]],
  },
});
```

Complex:

```ts
const summary = signals.output("summary", {
  reads: ["part", "label"],
  expr: {
    kind: "object",
    fields: [
      ["part", { kind: "read", id: "part" }],
      ["label", { kind: "read", id: "label" }],
      ["teeth", {
        kind: "get",
        target: { kind: "get", target: { kind: "read", id: "part" }, field: "dimensions" },
        field: "teeth",
      }],
    ],
  },
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

### `input(id, initial, options?): InputSignal`

Simple:

```ts
const count = signals.input("count", 1);
```

Complex:

```ts
const part = signals.input("part", {
  id: "gear-7",
  enabled: true,
}, {
  producesAspects: [1, 2],
});
```

### `computed(...)`

Normal callback form:

```ts
const doubled = signals.computed("doubled", () => count() * 2);
```

Generated-id callback form:

```ts
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
```

Complex branchy callback:

```ts
const label = signals.computed("label", () => {
  return enabled() ? `${name()} x${count()}` : "disabled";
});
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

### `output(id, spec): OutputSignal`

Simple:

```ts
const panel = signals.output("panel", {
  reads: ["count"],
  expr: {
    kind: "object",
    fields: [["count", { kind: "read", id: "count" }]],
  },
});
```

Complex:

```ts
const dashboard = signals.output("dashboard", {
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
- `output(() => ...)` is intentionally deferred today.

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
console.log(history.current_branch());
```

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
const part = signals.input("part", { teeth: 24 }, {
  producesAspects: [1],
});
```

Complex:

```ts
signals.transaction((tx) => {
  tx.setWithAspects(part, { teeth: 26 }, [1]);
});

const summary = signals.output("summary", {
  reads: [{ id: "part", aspects: [1] }],
  expr: { kind: "read", id: "part" },
});
```

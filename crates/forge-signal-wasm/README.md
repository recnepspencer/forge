# forge-signal-wasm

Framework-agnostic browser bindings for Forge Signal, with a callback-first
app surface and an optional React adapter.

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

const count = signals.input("count", 1);
const doubled = signals.computed("doubled", () => count() * 2);

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(doubled());
```

## Core Concepts

### `input`

Use `input` for mutable source state.

Simple:

```ts
const count = signals.input("count", 1);
signals.transaction((tx) => tx.set(count, 2));
```

Complex:

```ts
const part = signals.input("part", {
  id: "gear-7",
  teeth: 24,
  enabled: true,
}, {
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

Simple:

```ts
const doubled = signals.computed("doubled", () => count() * 2);
```

Complex:

```ts
const enabled = signals.input("enabled", true);
const name = signals.input("name", "Ada");

const label = signals.computed("label", () => {
  return enabled() ? `${name()} is enabled` : "disabled";
});
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

Simple:

```ts
const panel = signals.output("panel", {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
});
```

Complex:

```ts
const partSummary = signals.output("partSummary", {
  reads: ["part", "label"],
  expr: {
    kind: "object",
    fields: [
      ["part", { kind: "read", id: "part" }],
      ["label", { kind: "read", id: "label" }],
      ["status", {
        kind: "if",
        condition: { kind: "get", target: { kind: "read", id: "part" }, field: "enabled" },
        thenExpr: { kind: "value", value: "active" },
        elseExpr: { kind: "value", value: "inactive" },
      }],
    ],
  },
});
```

`output(() => ...)` is intentionally deferred. Use explicit `outputSpec(...)`
or `output(...)` with a recipe today.

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

Diagnostics are first-class. Start here:

```ts
const diagnostics = signals.diagnostics();
```

Simple:

```ts
const why = diagnostics.why("doubled");
console.log(why.recipeFamily, why.callback?.currentReads);
```

Complex:

```ts
const latestObservation = diagnostics.latestObservation();
const latestFlow = diagnostics.latestFlow();
const perf = diagnostics.performanceSummary();

console.log({
  delivered: latestObservation?.deliveredEventCount,
  callbackReads: perf.computeCallbackCapturedReadCount,
  dependencyPatches: perf.computeCallbackDependencyPatchCount,
});
```

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

## Documentation

- [docs/README.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/README.md)
- [docs/consuming_the_package.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/consuming_the_package.md)
- [docs/app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [docs/diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
- [docs/react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)

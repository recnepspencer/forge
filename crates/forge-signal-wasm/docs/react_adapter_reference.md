# React Adapter Reference

The package exposes a React adapter through:

```ts
import {
  createReactSignalsStore,
  useSignalValue,
  useOutputValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";
```

The adapter is intentionally thin. React consumes runtime truth; it does not
become a second state engine.

## Typical Setup

```ts
import { createSignals } from "forge-signal-wasm";
import { createReactSignalsStore } from "forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);
```

## `createReactSignalsStore(signals)`

Creates the React-domain wrapper around a shared `Signals` instance.

Simple:

```ts
const store = createReactSignalsStore(signals);
```

Complex:

```ts
const store = createReactSignalsStore(signals);

const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
```

The returned `ReactSignalsStore` exposes:

- `signals`
- `subscribeSignal(signal, listener)`
- `getSignalSnapshot(signal)`
- `subscribeDiagnostics(listener)`
- `getDiagnosticsSnapshot()`
- `transaction(callback)`
- `batch(callback)`
- `refreshDiagnostics()`
- `performanceSummary()`
- `dispose()`

## `useSignalValue(signal, store)`

Reads an `InputSignal` or `ComputedSignal`.

Simple:

```tsx
const countValue = useSignalValue<number>(count, store);
```

Complex:

```tsx
function Counter() {
  const countValue = useSignalValue<number>(count, store);
  const doubledValue = useSignalValue<number>(doubled, store);

  return (
    <button onClick={() => store.transaction((tx) => tx.set(count, countValue + 1))}>
      {countValue} / {doubledValue}
    </button>
  );
}
```

## `useOutputValue(output, store)`

Reads an `OutputSignal`.

Simple:

```tsx
const panelValue = useOutputValue(panel, store);
```

Complex:

```tsx
function Panel() {
  const panelValue = useOutputValue<{ count: number; doubled: number }>(panel, store);
  return <pre>{JSON.stringify(panelValue, null, 2)}</pre>;
}
```

## `useSignalsDiagnostics(store)`

Returns:

- `latestObservation`
- `latestFlow`
- `performanceSummary`

Simple:

```tsx
const diagnostics = useSignalsDiagnostics(store);
```

Complex:

```tsx
function DiagnosticsBar() {
  const diagnostics = useSignalsDiagnostics(store);

  return (
    <small>
      deliveries: {diagnostics.latestObservation?.observation.delivered_event_count ?? 0}
      {" | "}
      callback patches: {diagnostics.performanceSummary.computeCallbackDependencyPatchCount}
    </small>
  );
}
```

## Store Methods

### `subscribeSignal(signal, listener)`

Simple:

```ts
const unsubscribe = store.subscribeSignal(count, () => {
  console.log("count changed");
});
```

Complex:

```ts
const unsubscribe = store.subscribeSignal(panel, () => {
  renderPanel(store.getSignalSnapshot(panel));
});
```

### `getSignalSnapshot(signal)`

Simple:

```ts
const value = store.getSignalSnapshot(count);
```

Complex:

```ts
const snapshot = store.getSignalSnapshot(panel) as {
  count: number;
  doubled: number;
};
```

### `subscribeDiagnostics(listener)` and `getDiagnosticsSnapshot()`

Simple:

```ts
const unsubscribe = store.subscribeDiagnostics(() => {
  console.log(store.getDiagnosticsSnapshot());
});
```

Complex:

```ts
const unsubscribe = store.subscribeDiagnostics(() => {
  const diagnostics = store.getDiagnosticsSnapshot();
  queueMicrotask(() => updateDevPanel(diagnostics));
});
```

### `transaction(callback)` and `batch(callback)`

Simple:

```ts
store.transaction((tx) => {
  tx.set(count, 2);
});
```

Complex:

```ts
store.batch((tx) => {
  tx.set(count, 3);
  tx.setWithAspects(part, { ...part(), teeth: 30 }, [1]);
});
```

### `refreshDiagnostics()`

Simple:

```ts
const diagnostics = store.refreshDiagnostics();
```

Complex:

```ts
const diagnostics = store.refreshDiagnostics();
recordDiagnosticsSnapshot(diagnostics);
```

### `performanceSummary()`

Simple:

```ts
console.log(store.performanceSummary());
```

Complex:

```ts
const perf = store.performanceSummary();
console.log({
  subscriptions: perf.activeSignalSubscriptionCount,
  sharedFanoutRatio: perf.sharedFanoutRatio,
});
```

### `dispose()`

Simple:

```ts
store.dispose();
```

Complex:

```ts
window.addEventListener("beforeunload", () => {
  store.dispose();
});
```

## Semantics Notes

- React subscriptions are built on the same committed observation substrate as
  `watch(...)`.
- Rollback still suppresses normal delivery.
- Callback-first `computed(() => ...)` remains runtime-owned derived truth.
- Callback-first `output(() => ...)` is the normal product projection lane.
- The store is an adapter, not a second state container.

## Related Docs

- [consuming_the_package.md](consuming_the_package.md)
- [app_surface_reference.md](app_surface_reference.md)
- [diagnostics_and_history_reference.md](diagnostics_and_history_reference.md)

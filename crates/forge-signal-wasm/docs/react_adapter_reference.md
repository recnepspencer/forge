# React Adapter Reference

The package exposes a React adapter through:

```ts
import {
  createReactSignalsStore,
  useSignalValue,
  useOutputValue,
  useSignalsDiagnostics,
} from "@aust-group/forge-signal-wasm/react";
```

This adapter is intentionally thin. React consumes runtime truth; it does not
become a second state engine.

## Primary Functions

### `createReactSignalsStore(signals)`

Creates a React-domain store wrapper around a `Signals` instance.

The store is responsible for:

- subscription glue
- `useSyncExternalStore` integration
- per-target listener fanout
- diagnostics snapshot refresh
- transaction and batch forwarding

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

### `useSignalValue(signal, store)`

Reads an `InputSignal` or `ComputedSignal` through the React store.

Example:

```ts
const countValue = useSignalValue(count, store);
const doubledValue = useSignalValue(doubled, store);
```

### `useOutputValue(output, store)`

Reads an `OutputSignal` through the React store.

Example:

```ts
const panelValue = useOutputValue(panel, store);
```

### `useSignalsDiagnostics(store)`

Returns current diagnostics snapshots:

- `latestObservation`
- `latestFlow`
- `performanceSummary`

Example:

```ts
const diagnostics = useSignalsDiagnostics(store);
```

## `SignalsDiagnosticsSnapshot`

`useSignalsDiagnostics(...)` and `store.getDiagnosticsSnapshot()` expose:

- `latestObservation`
- `latestFlow`
- `performanceSummary`

## `ReactPerformanceSummary`

`store.performanceSummary()` returns React-adapter counters:

- `activeSignalSubscriptionCount`
- `activeReactSubscriberCount`
- `activeRuntimeWatchHandleCount`
- `diagnosticsSubscriberCount`
- `sharedFanoutRatio`

This surface is useful for checking whether the React adapter is actually
sharing runtime subscriptions instead of fanning out wastefully.

## Store Methods

### `subscribeSignal(signal, listener)`

Subscribes a React-side listener to a signal or output target and returns an
unsubscribe function.

### `getSignalSnapshot(signal)`

Returns the current snapshot value for a signal/output target.

### `subscribeDiagnostics(listener)`

Subscribes a listener to diagnostics snapshot changes and returns an
unsubscribe function.

### `getDiagnosticsSnapshot()`

Returns the current diagnostics snapshot without going through a hook.

### `transaction(callback)` and `batch(callback)`

Forward to the shared `Signals` instance and refresh diagnostics snapshots
after committed writes.

### `refreshDiagnostics()`

Forces a diagnostics snapshot refresh and returns the resulting snapshot.

### `performanceSummary()`

Returns the adapter-level `ReactPerformanceSummary`.

### `dispose()`

Tears down React-side store resources.

## Store Behavior

The React store:

- dedupes subscriptions by signal id
- refreshes diagnostics snapshots after committed writes
- instruments the shared `Signals` instance so both:
  - `store.transaction(...)`
  - `signals.transaction(...)`
  update React diagnostics consumers honestly

## Typical Usage

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "@aust-group/forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);

const count = signals.input("count", 1);

const doubled = signals.computed("doubled", {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
});

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

function Counter() {
  const countValue = useSignalValue(count, store);
  const doubledValue = useSignalValue(doubled, store);
  const panelValue = useOutputValue(panel, store);
  const diagnostics = useSignalsDiagnostics(store);

  return { countValue, doubledValue, panelValue, diagnostics };
}
```

## Semantics Notes

- React subscriptions are built on the same committed observation substrate as
  `watch(...)`
- rollback still suppresses normal delivery
- the store is a framework adapter, not a second source of truth
- current web execution remains serial by default

## Related Docs

- [app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
- [consuming_the_package.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/consuming_the_package.md)

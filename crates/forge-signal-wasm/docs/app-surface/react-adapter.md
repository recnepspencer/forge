# React Adapter Reference

## What This Feature Is

The React adapter is the thin React-facing consumer for a `forge-signal-wasm`
signals instance.

It does not create a second state engine. It subscribes to signal truth that
already exists in the shared runtime.

## Why You Use It

- subscribe React components to signal handles
- reuse one shared signals instance across a React tree
- observe diagnostics from React without inventing a second observer layer
- keep React integration lightweight while the runtime stays authoritative

## Stable Entry Points

- `createReactSignalsStore(signals)`
- `useSignalValue(signal, store)`
- `useOutputValue(output, store)`
- `useSignalsDiagnostics(store)`

## Core Mental Model

The React adapter wraps an existing `signals` instance in a small subscription
store:

- the runtime still owns state and mutation semantics
- the React store owns subscription fanout and cached snapshots
- hooks read from the store, not from a separate React-only model

If your app already has a `signals` instance, React should consume it. Do not
mirror signal state into another store just to render components.

## How It Executes

1. create a shared `signals` instance
2. create a React store from that instance
3. pass handles and the store into hooks
4. React subscribes through runtime `watch(...)`
5. React receives snapshot updates when the underlying signal truth changes

Diagnostics follow the same pattern through the runtime diagnostics surface.

## Small Example

```tsx
import { createSignals } from "forge-signal-wasm";
import { createReactSignalsStore, useSignalValue } from "forge-signal-wasm/react";

const signals = await createSignals();
const count = signals.input(1);
const store = createReactSignalsStore(signals);

function CounterValue() {
  const value = useSignalValue<number>(count, store);
  return <span>{value}</span>;
}
```

This is the smallest honest example because:

- the runtime owns `count`
- React only subscribes and renders
- nothing is copied into a second app store

Add `debugName` only if you want friendlier diagnostics while inspecting the
shared runtime.

## Real Example

```tsx
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

const signals = await createSignals();

const itemWorkspace = signals.graph("itemWorkspace", (graph) => {
  const editor = graph.controller("editor", ({ input, computed }) => {
    const serverItem = input({
      id: "task-7",
      title: "Ship docs",
    });
    const draft = input({});
    const effectiveItem = computed(() => ({
      ...serverItem(),
      ...draft(),
    }));
    const dirtyState = computed(() => Object.keys(draft()).length > 0);

    return {
      inputs: { serverItem, draft },
      outputs: { effectiveItem, dirtyState },
    };
  });

  return graph.expose({
    inputs: {
      draft: graph.input.optional(editor.inputs.draft),
    },
    outputs: {
      effectiveItem: editor.outputs.effectiveItem,
      dirtyState: editor.outputs.dirtyState,
    },
  });
});

const store = createReactSignalsStore(signals);

function ItemEditor() {
  const effectiveItem = useOutputValue<{ title?: string }>(
    itemWorkspace.output("effectiveItem"),
    store,
  );
  const dirtyState = useOutputValue<boolean>(
    itemWorkspace.output("dirtyState"),
    store,
  );
  const diagnostics = useSignalsDiagnostics(store);

  return (
    <section>
      <h2>{effectiveItem.title ?? "Untitled"}</h2>
      <button
        onClick={() => itemWorkspace.patchInput("draft", { title: "Ready to ship" })}
      >
        Patch Draft
      </button>
      <small>
        dirty: {String(dirtyState)}
        {" | "}
        latest flow: {diagnostics.latestFlow?.graph?.id ?? "none"}
      </small>
    </section>
  );
}
```

What is authoritative here:

- the graph and its inputs/outputs still live in the signal runtime
- the React store only tracks subscriptions and snapshots
- component actions still mutate through graph helpers or runtime transactions

## How It Relates To Other Features

- Pair this with the main app surface when React is your view layer.
- Pair it with published graphs when you want stable output handles for
  components.
- Pair it with diagnostics when you want a lightweight dev panel.
- Use host capabilities in the shared signals instance, not by reading browser
  globals directly in React components.

## API Notes

### `createReactSignalsStore(signals)`

Creates the shared React subscription store.

The returned store exposes:

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

Use this for input or computed handles.

### `useOutputValue(output, store)`

Use this for output handles, including graph-published outputs.

### `useSignalsDiagnostics(store)`

Returns:

- `latestObservation`
- `latestFlow`
- `performanceSummary`

## Inspection And Debugging

Useful store surfaces:

- `store.getSignalSnapshot(...)`
- `store.getDiagnosticsSnapshot()`
- `store.refreshDiagnostics()`
- `store.performanceSummary()`

Useful runtime surfaces behind the store:

- `signals.diagnostics()`
- graph `inspectDiagnostics()`

The React store is for consumption and fanout. If you need graph-contract or
history truth, inspect the graph or runtime directly.

## Anti-Patterns

- copying signal state into a second React store
- building component-local mirrors for values you already have as signal handles
- using React state as the authoritative source when the runtime should own it
- reading ambient browser state in components instead of host capability
- disposing the shared store while components still depend on it

## Current Limits

- the React adapter is intentionally thin
- it does not define a React-only mutation language
- it does not replace graph diagnostics/history surfaces
- it assumes you already have a shared `signals` instance to consume

## Related Docs

- [App Surface Overview](./overview.md)
- [Host Capabilities](./host-capabilities.md)
- [Diagnostics And History](./diagnostics-and-history.md)

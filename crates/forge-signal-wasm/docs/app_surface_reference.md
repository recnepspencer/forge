# App Surface Reference

## What This Feature Is

The app surface is the main public API for `forge-signal-wasm`.

It gives you:

- local writable inputs
- derived computed values
- published outputs
- linked writable derived state
- controller composition
- graph publication
- graph-boundary mutation helpers
- runtime observation and mutation entrypoints

This is the surface most application code should live on.

## Why You Use It

- author local app state without explicit ids
- build larger features from controller artifacts instead of loose helpers
- publish stable graph contracts only when you want a boundary
- mutate local and graph-owned state through the same runtime truth
- inspect graphs without dropping to lower runtime plumbing

## Stable Entry Points

- `createSignals(...)`
- `signals.input(...)`
- `signals.computed(...)`
- `signals.output(...)`
- `signals.linked(...)`
- `signals.spec.*`
- `signals.controller(...)`
- `signals.graph(...)`
- `signals.importGraph(...)`
- `signals.transaction(...)`
- `signals.batch(...)`
- `signals.watch(...)`
- `signals.effect(...)`
- `signals.diagnostics()`
- `signals.history()`

## Core Mental Model

The app surface is handle-based.

- local signal identity is runtime-owned
- `debugName` is optional metadata for humans
- local composition happens by handle
- public naming happens at graph publication
- explicit structural naming lives in `signals.spec`

That split matters:

- ordinary app code should not care about ids
- graph contracts should care about public names
- spec and portability work should care about explicit structural names

## How It Executes

The normal flow is:

1. create a `signals` instance
2. author local inputs, computeds, outputs, and linked state
3. optionally organize larger units as controllers
4. optionally publish a graph when you need explicit public inputs and outputs
5. mutate through input helpers, graph helpers, or transactions
6. inspect diagnostics/history/export surfaces only when you need explanation or transport

The app surface does not replace runtime truth. It is the product-facing layer
over that runtime truth.

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const count = signals.input(1);
const doubled = signals.computed(() => count() * 2);
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}));

count.set(2);

console.log(panel());
```

This is the smallest honest example because it shows the current main lane:

- no explicit authored id
- handle-based reads and writes
- derived output as the published view for local code

Keep the first pass this simple. Add `debugName` later when you want friendlier
diagnostics; it is optional metadata, not part of local identity.

## Real Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

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

    const dirtyState = computed(() => Object.keys(draft()).length > 0);

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

itemWorkspace.writeInput("selectedWorkflowTarget", {
  id: "ready",
  label: "Ready",
});

console.log(itemWorkspace.read());
```

What is authoritative here:

- `serverItem` is source state
- `draft` is writable local state
- `selectedWorkflowTarget` is linked writable derived state
- graph publication is where requiredness, authority, and public names become
  explicit

## Local State And Derived Values

### `signals.input(...)`

Use `input(...)` for writable local state:

```ts
const draft = signals.input({ title: "Ship docs" });
```

Input handles support:

- `set(value)`
- `patch(value)`
- `assign(fields)`
- `reset()`

Use `patch(...)` and `assign(...)` for object-ish local state. Keep `set(...)`
for full replacement.

### `signals.computed(...)`

Use `computed(...)` for derived internal state:

```ts
const dirty = signals.computed(() => Object.keys(draft()).length > 0);
```

Only signal and admitted host-capability reads are reactive dependencies.
Ordinary closure variables are not reactive truth.

### `signals.output(...)`

Use `output(...)` for the shaped value you want to expose or observe:

```ts
const panel = signals.output(() => ({
  draft: draft(),
  dirty: dirty(),
}));
```

Outputs are still derived signal handles. They do not become an event channel or
separate store.

## Linked Writable Derived State

`signals.linked(...)` is for writable state that normally follows a reactive
source but may be locally overridden:

```ts
const selectedOption = signals.linked({
  source: () => shippingOptions(),
  computation: (options, previous) => (
    options.find((option) => option.id === previous?.value?.id) ?? options[0]
  ),
});
```

Linked handles support:

- `set(value)`
- `reset()`
- `relink()`

Use `reset()` when you want to go back to the current linked baseline. Use
`relink()` when you want to explicitly re-anchor to the latest source-derived
value.

## Explicit Named Lane

Use `signals.spec` when names are the contract:

```ts
const count = signals.spec.input("count", 1);
const doubled = signals.spec.computedCallback("doubled", () => count() * 2);
const panel = signals.spec.outputCallback("panel", () => ({
  count: count(),
  doubled: doubled(),
}));
```

Use this lane for:

- portable/spec authoring
- compatibility-facing work
- explicit structural naming

Do not use it to recreate the old main-lane id-heavy style for ordinary app
code.

## Controllers

Controllers are real package-understood artifacts, not plain object folklore.

Use the builder form when you want feature-local composition:

```ts
const editor = signals.controller(({ input, computed }) => {
  const serverItem = input(null, { debugName: "serverItem" });
  const draft = input({}, { debugName: "draft" });
  const effectiveItem = computed(() => ({
    ...(serverItem() ?? {}),
    ...draft(),
  }));

  return {
    inputs: { serverItem, draft },
    outputs: { effectiveItem },
  };
});
```

The builder is lighter than the older manual shape, but it still produces a
validated controller contract.

## Graphs

Use graphs when you want an explicit published boundary.

### Public input requiredness

Graphs can declare required and optional public inputs:

```ts
graph.expose({
  inputs: {
    serverItem: graph.input.required(editor.inputs.serverItem, {
      authority: "readOnly",
    }),
    draft: graph.input.optional(editor.inputs.draft),
  },
  outputs: {
    effectiveItem: editor.outputs.effectiveItem,
  },
});
```

### Public input authority

Public input authorities today are:

- `writable`
- `readOnly`
- `imported`

These show up in:

- `contract()`
- `operationalContract()`
- `inspectDiagnostics()`
- `inspectHistory()`

And they are enforced by graph mutation helpers.

### Graph mutation helpers

Published graphs expose:

- `writeInputs(...)`
- `writeInput(...)`
- `patchInputs(...)`
- `patchInput(...)`
- `resetInputs(...)`
- `resetInput(...)`
- `apply(...)`
- `transaction(...)`

Example:

```ts
graph.patchInput("draft", { title: "Queued" });
graph.resetInput("draft");
```

These helpers still lower through one canonical mutation envelope. They are not
a second write engine.

## Observation And Mutation Entry Points

### `signals.transaction(...)` and `signals.batch(...)`

Use transactions when several writes belong together:

```ts
signals.transaction((tx) => {
  tx.set(count, 2);
  tx.patch(draft, { done: true });
});
```

### `signals.watch(...)` and `signals.effect(...)`

Use these when you need runtime observation:

```ts
const handle = signals.watch(panel, (notice) => {
  console.log("panel changed", notice);
});

signals.nuke(handle);
```

Use `watch(...)` or `effect(...)` for observation. Do not turn them into your
main state model.

## Inspection And Debugging

For local runtime inspection:

- `signals.diagnostics()`
- `signals.history()`

For graph-boundary inspection:

- `graph.contract()`
- `graph.operationalContract()`
- `graph.contractHistory()`
- `graph.contractDelta(...)`
- `graph.inspectDiagnostics()`
- `graph.inspectHistory()`
- `graph.exportDefinition()`
- `graph.exportSnapshot()`
- `graph.importPosture()`

These surfaces explain the published boundary honestly. They do not turn local
`debugName` into stable contract identity.

## Anti-Patterns

- using `debugName` as if it were a stable query or mutation key
- reaching for `signals.spec.*` for ordinary local app code
- treating graph publication as optional when you need a real public contract
- using ambient browser reads inside `computed(...)` instead of host capability
- assuming graph mutation helpers own different semantics than transactions

## Current Limits

- portable graph import is still denied on the graph-native import surface
- exact graph restore is the admitted import posture today
- `debugName` may help diagnostics, but it is never addressability
- the forms and resource products are still later layers, not part of this app
  surface

## Related Docs

- [consuming_the_package.md](./consuming_the_package.md)
- [host_capabilities.md](./host_capabilities.md)
- [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
- [react_adapter_reference.md](./react_adapter_reference.md)
- [aspects_reference.md](./aspects_reference.md)

# Graphs And Controllers

A graph turns local signal work into an explicit feature boundary. Controllers
organize that work inside the graph. Use them when a feature needs named inputs,
named outputs, input authority, inspection, export, or restore.

Do not publish every local value. A boundary is useful because it leaves most
things private.

## Stable Entry Points

- `signals.graph(id, builder)`
- `graph.scope(id)`
- `graph.controller(id, builder)`
- `graph.expose(definition)`
- `graph.input.required(handle, options?)`
- `graph.input.optional(handle, options?)`
- `publishedGraph.read()`
- `publishedGraph.writeInput(name, value)`
- `publishedGraph.patchInput(name, patch)`
- `publishedGraph.inspectDiagnostics()`
- `publishedGraph.inspectHistory()`

## Small Example

```ts
const pricing = signals.graph("pricing", (graph) => {
  const state = graph.scope("state");
  const quantity = state.input(2);
  const unitPrice = state.input(18);
  const total = state.computed(() => quantity() * unitPrice());

  return graph.expose({
    inputs: { quantity, unitPrice },
    outputs: { total },
  });
});

await pricing.writeInput("quantity", 4);
console.log(pricing.read()); // { total: 72 }
```

Inside the builder, handles remain local. `graph.expose` is the moment names
and input posture become public contract.

## Controllers Organize Meaning

```ts
const checkout = signals.graph("checkout", (graph) => {
  const cart = graph.controller("cart", ({ input, computed }) => {
    const items = input<Array<{ price: number }>>([]);
    const subtotal = computed(() =>
      items().reduce((sum, item) => sum + item.price, 0),
    );

    return {
      inputs: { items },
      outputs: { subtotal },
    };
  });

  const approval = graph.controller("approval", ({ computed }) => {
    const required = computed(() => cart.outputs.subtotal() >= 10_000);
    return { outputs: { required } };
  });

  return graph.expose({
    controllers: [cart, approval],
  });
});
```

A controller is composition, not a second runtime. Its handles execute in the
same graph-owned runtime and preserve the same observable behavior as an
equivalent flat graph.

## Input Authority

Published inputs can be required or optional and can carry explicit authority
posture. That contract tells callers what they may write; it does not make an
internal handle globally addressable.

Use `operationalContract()` when tooling needs the exact public write and reset
surface.

## Export And Import

Published graphs can export a portable definition and a same-runtime snapshot:

```ts
const definition = pricing.exportDefinition();
const snapshot = pricing.exportSnapshot();

const restored = signals.importGraph(definition, snapshot);
await restored.ready();
```

Read the graph's `importPosture()` before treating an artifact as portable
across runtime versions or environments. Exact restore and portable definition
are related, but they are not interchangeable promises.

## Anti-Patterns

- Do not publish internal values merely to inspect them.
- Do not use controller names as a global service locator.
- Do not put unrelated features in one graph because they share a screen.
- Do not treat an exported snapshot as durable database truth.

## Related Docs

- [Inputs And Computed State](./inputs-and-computed.md)
- [Diagnostics And Explanation](./diagnostics.md)
- [History, Replay, And Runtime Branches](./history.md)

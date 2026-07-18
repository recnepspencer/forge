# Inputs And Computed State

Inputs hold writable runtime state. Computed and output handles derive values
from the inputs and computed handles they read.

Use this pair whenever a value can be rebuilt from other local state. The
runtime keeps the dependency graph; your application keeps the meaning.

## Stable Entry Points

- `signals.input(initial, options?)`
- `signals.computed(callback, options?)`
- `signals.output(callback, options?)`
- `input.set(value)`
- `input.patch(value)`
- `input.assign(fields)`
- `input.reset()`

The `inputAsync`, `computedAsync`, and `outputAsync` variants provide an
explicit promise-returning lane when construction must cross an asynchronous
runtime boundary.

## Small Example

```ts
const hours = signals.input(6, { debugName: "hours" });
const hourlyRate = signals.input(45, { debugName: "hourlyRate" });
const estimate = signals.computed(
  () => hours() * hourlyRate(),
  { debugName: "estimate" },
);

hours.set(8);
console.log(estimate()); // 360
```

Calling a handle reads it. The computed callback captures those reads and
updates its dependency relationship with the runtime.

## Object Updates

```ts
const draft = signals.input({
  title: "Draft",
  approved: false,
});

draft.patch({ approved: true });
draft.assign({ title: "Ready" });
draft.reset();
```

`patch` and `assign` are conveniences over the same runtime mutation boundary.
They do not create a second object store.

## Computed Versus Output

Both are readable derived handles. Use `computed` for intermediate decisions
and `output` when the value represents the outward result of a feature or
graph. The distinction communicates intent; it does not grant the output a
second source of truth.

## Recomputed Does Not Mean Changed

A dependency change can cause a callback to run while the resulting value
remains equivalent:

```ts
const amount = signals.input(9_800);
const reviewBand = signals.computed(
  () => amount() >= 10_000 ? "manual" : "automatic",
);

amount.set(9_900);
// reviewBand was reconsidered, but its value is still "automatic".
```

That distinction matters when reading flow evidence. "The callback ran" and
"the output changed" answer different questions.

## Identity And Names

The runtime issues opaque IDs. `debugName` exists for humans:

```ts
const amount = signals.input(0, { debugName: "transferAmount" });
```

Never store or query a signal by `debugName`. When names must survive export or
become public contract, use a published graph or the explicit `signals.spec`
lane.

## Anti-Patterns

- Do not mirror a signal in component state.
- Do not store a computed value in another input merely to make it readable.
- Do not perform network requests inside a computed callback.
- Do not use `debugName` as identity.
- Do not keep stale handles after their owning runtime is freed.

## Related Docs

- [Transactions And Coordinated Writes](./transactions.md)
- [Graphs And Controllers](./graphs-and-controllers.md)
- [Diagnostics And Explanation](./diagnostics.md)

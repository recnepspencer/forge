# Linked Writable State

Linked state normally follows a derived source but can temporarily hold local
user intent. Use it for selections, defaults, and editable choices that should
survive compatible source updates and re-anchor when the source changes
meaningfully.

## Stable Entry Points

- `signals.linked(source, options?)`
- `signals.linked({ source, computation, debugName? })`
- `linked.set(value)`
- `linked.reset()`
- `linked.relink()`

## Small Example

```ts
const options = signals.input([
  { id: "ground", label: "Ground" },
  { id: "air", label: "Air" },
]);

const selected = signals.linked({
  source: () => options(),
  computation: (nextOptions, previous) =>
    nextOptions.find((option) => option.id === previous?.value?.id)
      ?? nextOptions[0]
      ?? null,
});

selected.set({ id: "air", label: "Air" });
```

The linked handle owns the current writable value. Its source and computation
define the baseline used when it needs to re-anchor.

## Reset And Relink

- `reset()` returns to the current source-derived baseline.
- `relink()` asks the computation to reconcile the current value with the
  current source.

```ts
options.set([
  { id: "ground", label: "Ground" },
  { id: "sea", label: "Sea" },
]);

selected.relink();
console.log(selected()); // ground
```

The computation receives the previous value and previous source. That lets a
selection survive an ordinary refresh while still resetting when a source
revision represents a genuinely new record.

## Real Example: Revision-Aware Selection

```ts
const catalog = signals.input({
  revision: 1,
  options: [
    { id: "draft", label: "Draft" },
    { id: "review", label: "Review" },
  ],
});

const workflowTarget = signals.linked({
  source: () => catalog(),
  computation: (source, previous) => {
    const preserved = previous?.source.revision === source.revision
      ? source.options.find((option) => option.id === previous.value?.id)
      : null;

    return preserved ?? source.options[0] ?? null;
  },
});
```

The revision comparison is application policy. Worth supplies the previous
source/value pair and owns the linked lifecycle; it does not invent semantic
equivalence for your domain.

## Anti-Patterns

- Do not use linked state as a server synchronization protocol.
- Do not hide source identity changes inside a comparator that always preserves
  the previous value.
- Do not store the same selection in React state as well.
- Do not use linked state when a plain computed value should never be writable.

## Related Docs

- [Inputs And Computed State](./inputs-and-computed.md)
- [Graphs And Controllers](./graphs-and-controllers.md)
- [Forms](../forms/index.md)

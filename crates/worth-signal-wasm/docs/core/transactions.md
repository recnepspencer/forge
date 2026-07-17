# Transactions And Coordinated Writes

A transaction groups related writes behind one runtime commit boundary. Use it
when downstream work must not observe a half-applied application decision.

## Stable Entry Points

- `signals.transaction(callback)`
- `signals.batch(callback)`
- `signals.transactionAsync(callback)`
- `signals.batchAsync(callback)`
- `graph.transaction(callback)`
- `graph.apply(request)`

Await the result. Compatibility deployment may complete synchronously, while
worker-first execution crosses an asynchronous boundary.

## Small Example

```ts
const width = signals.input(10);
const height = signals.input(20);
const area = signals.computed(() => width() * height());

await signals.transaction((tx) => {
  tx.set(width, 12);
  tx.set(height, 24);
});

console.log(area()); // 288
```

The runtime validates the writes before committing them. Derived work follows
the committed boundary rather than an accidental sequence of UI assignments.

## Patches And Resets

```ts
const draft = signals.input({ title: "Draft", priority: 1 });

await signals.transaction((tx) => {
  tx.patch(draft, { priority: 2 });
});
```

Use direct `set`, `patch`, `assign`, or `reset` for one obvious local change.
Use a transaction when several mutations represent one decision or when you
need aspect- or region-aware writes.

## Aspect-Aware Writes

```ts
const gear = signals.input(
  { teeth: 18, thickness: 8 },
  { producesAspects: [0, 1] },
);

await signals.transaction((tx) => {
  tx.setWithAspects(
    gear,
    { ...gear(), teeth: 20 },
    [0],
  );
});
```

The aspect list says which semantic lanes changed. It does not excuse an
incorrect next value; application code still submits the complete value owned
by the input.

## Runtime Ownership

A handle belongs to the runtime that created it. Worker-first transactions
reject foreign handles instead of guessing which runtime should commit them.

That denial is intentional. Cross-runtime writes need an explicit bridge or a
published import boundary.

## Transactions Are Not Database Transactions

A Signals transaction coordinates one browser runtime commit. It does not
commit a server database, make a network request atomic, or authenticate an
actor. Pair it with the resource or platform layer that owns those promises.

## Anti-Patterns

- Do not open a transaction around unrelated work just to reduce renders.
- Do not perform long network work inside the transaction callback.
- Do not pass handles between runtimes.
- Do not call a local transaction a durable business transaction.

## Related Docs

- [Inputs And Computed State](./inputs-and-computed.md)
- [Aspects: Semantic Invalidation](./aspects.md)
- [Resources](../resources/index.md)

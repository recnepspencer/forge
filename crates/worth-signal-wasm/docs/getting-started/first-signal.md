# Your First Signal

This tutorial builds a small pricing decision. It shows the normal authoring
lane without introducing graphs, resources, or compatibility APIs before they
have a job.

## Create The Runtime

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();
```

## Add Writable Inputs

```ts
const quantity = signals.input(2, { debugName: "quantity" });
const unitPrice = signals.input(18, { debugName: "unitPrice" });
const customerTier = signals.input<"standard" | "partner">("standard", {
  debugName: "customerTier",
});
```

An input is writable runtime state. Call it to read the current value. Use
`set`, `patch`, `assign`, or `reset` to change it.

## Derive A Decision

```ts
const subtotal = signals.computed(
  () => quantity() * unitPrice(),
  { debugName: "subtotal" },
);

const discount = signals.computed(
  () => customerTier() === "partner" ? subtotal() * 0.1 : 0,
  { debugName: "discount" },
);

const total = signals.output(
  () => subtotal() - discount(),
  { debugName: "total" },
);
```

The runtime observes which handles each callback reads. `discount` depends on
`customerTier` and `subtotal`; `total` depends on `subtotal` and `discount`.
You do not maintain a second dependency array.

## Commit A Coordinated Change

```ts
await signals.transaction((tx) => {
  tx.set(quantity, 4);
  tx.set(customerTier, "partner");
});

console.log(total()); // 64.8
```

The transaction validates both writes and commits them through one runtime
boundary. Downstream work sees the committed state, not an accidental halfway
combination.

## Ask Why

```ts
const explanation = await signals.diagnostics().why(total.id);

console.log(explanation);
```

Diagnostics are runtime evidence. Keep them when they help a person understand
what happened; do not promote them into the source value.

## What You Should Notice

- Inputs own writable values.
- Computed and output handles are disposable derivation.
- Dependencies come from actual reads.
- Transactions coordinate writes.
- Diagnostics explain runtime behavior.
- `debugName` is for people, not addressability.

That model scales surprisingly far. Reach for a graph only when the feature
needs an explicit public boundary.

## Next

- [Inputs And Computed State](../core/inputs-and-computed.md)
- [Transactions And Coordinated Writes](../core/transactions.md)
- [Graphs And Controllers](../core/graphs-and-controllers.md)

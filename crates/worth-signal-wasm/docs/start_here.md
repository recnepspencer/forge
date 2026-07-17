# Start Here

Worth Signals can be as small as two values or as broad as an application
runtime. Start small. The package does not award points for using every
surface.

## Install

```bash
npm install worth-signals-wasm
```

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();
```

The default deployment is worker-first. Construction is asynchronous because
the runtime may need to start and initialize a dedicated worker. Read
[Installation And Deployment](./getting-started/installation.md) before adding
fallback behavior.

## The First Useful Program

```ts
const quantity = signals.input(2, { debugName: "quantity" });
const unitPrice = signals.input(18, { debugName: "unitPrice" });

const total = signals.computed(
  () => quantity() * unitPrice(),
  { debugName: "total" },
);

quantity.set(3);

console.log(total()); // 54
```

`quantity` and `unitPrice` own writable runtime state. `total` is derived. The
runtime records which handles the callback reads, so you do not maintain a
dependency list beside the code.

`debugName` helps people recognize a signal in diagnostics. It is not identity,
and you should never use it as a lookup key.

## Pick The Surface That Matches The Problem

### Local state and derived decisions

Use [Core Signals](./core/README.md) when values live in the browser and derive
from one another.

### API-backed state

Use [Resources](./resources/index.md) when state has request identity,
freshness, loading, retries, optimistic effects, or server reconciliation.
Do not rebuild a resource cache out of ordinary inputs unless you genuinely
want to own all of that machinery.

### Forms

Use [Forms](./forms/index.md) when you need source values, drafts, validation,
readiness, actions, or submission evidence to agree. A form is more than a bag
of input signals.

### Navigation

Use [Router](./router/index.md) when route projection, admission, history,
recovery, or speculative navigation are part of application truth.

### Independent edits and manual merge

Use [Local Truth](./local-truth/README.md) when browser-local branches edit the
same application value and you need aspect-aware conflict review. It is
process-local. Shared durable truth still belongs on the server or in the wider
Worth platform.

## Three Rules That Prevent Most Trouble

1. **One owner for each kind of truth.** Do not mirror runtime state in React
   because subscribing feels unfamiliar.
2. **Derive instead of synchronizing.** If a value can be rebuilt from inputs,
   make it computed.
3. **Treat diagnostics as evidence, not authority.** A `why()` result can
   explain a commit. It does not become the value being committed.

## When The Application Grows

Use a graph when a feature needs an explicit public boundary:

```ts
const cart = signals.graph("cart", (graph) => {
  const state = graph.scope("state");
  const quantity = state.input(2);
  const unitPrice = state.input(18);
  const total = state.computed(() => quantity() * unitPrice());

  return graph.expose({
    inputs: { quantity, unitPrice },
    outputs: { total },
  });
});

await cart.writeInput("quantity", 4);
console.log(cart.read().total); // 72
```

The local handles stay runtime-owned. The graph gives the feature stable public
names, input authority, inspection, and export boundaries.

## Learn In This Order

1. [Your First Signal](./getting-started/first-signal.md)
2. [How Worth Signals Thinks About State](./getting-started/mental-model.md)
3. [Inputs And Computed State](./core/inputs-and-computed.md)
4. [Transactions And Coordinated Writes](./core/transactions.md)
5. [Choose The Right Surface](./getting-started/choosing-a-surface.md)
6. [Explainable Derived State](./core/diagnostics.md)

Then follow the product area that matches your application.

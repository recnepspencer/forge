# Core Signals

Core Signals is the small, local foundation beneath the larger package. Use it
for writable state, derived decisions, coordinated commits, feature boundaries,
and the evidence that explains what the runtime did.

If you only need a counter, use a counter. If you need API freshness, form
readiness, route admission, or branch-aware application truth, move to the
surface that owns that responsibility.

## The Normal Lane

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();

const temperature = signals.input(72);
const warning = signals.computed(() => temperature() >= 90);

temperature.set(95);
console.log(warning()); // true
```

The handles use opaque runtime identity. Add `debugName` when people need a
recognizable label in diagnostics. Publish a graph when names become a real
feature contract.

## The Core Pieces

- [Inputs And Computed State](./inputs-and-computed.md) covers writable and
  derived values.
- [Transactions And Coordinated Writes](./transactions.md) covers atomic
  multi-input changes.
- [Linked Writable State](./linked-state.md) covers values that normally follow
  a source but permit local intent.
- [Graphs And Controllers](./graphs-and-controllers.md) covers explicit feature
  boundaries.
- [Aspects: Semantic Invalidation](./aspects.md) covers semantic change lanes.
- [Diagnostics And Explanation](./diagnostics.md) covers `why()`, flow evidence,
  and ownership of retained UI snapshots.
- [History, Replay, And Runtime Branches](./history.md) covers runtime history
  and its limits.

## The Boundary Worth Defends

Core Signals owns runtime state and derivation. It does not own durable server
truth, form policy, browser URL authority, or application-value merge policy.
Those jobs have dedicated surfaces because silently mixing them into a generic
signal store would make their lifecycle impossible to inspect honestly.

## The Explicit Spec Lane

Use `signals.spec` when structural names and declarative expressions are the
contract:

```ts
const count = signals.spec.input("count", 1);
const doubled = signals.spec.computed("doubled", {
  reads: [count.id],
  expr: {
    kind: "sum",
    args: [
      { kind: "read", id: count.id },
      { kind: "read", id: count.id },
    ],
  },
  identity: { kind: "exact" },
});
```

Do not start there merely because explicit IDs look reassuring. Ordinary app
code is easier to refactor when local identity remains private.

## Current Limits

- Runtime branches are execution history, not durable application truth.
- Worker-first is the default deployment; compatibility-only specialist lanes
  require an explicit main-thread runtime.
- Numeric aspects are bounded by the active native runtime profile.

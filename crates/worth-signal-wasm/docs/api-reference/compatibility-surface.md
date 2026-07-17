# Lower-Level Compatibility Surface

The compatibility surface exposes explicit structural authoring and lower-level
runtime operations. It is supported for migration and specialist work. It is
not the recommended starting point for ordinary application state.

## Published Entrypoints

| Entrypoint | What it does |
| --- | --- |
| `createSignals({ deployment: "mainThreadCompatibility" })` | Creates the callable facade over a main-thread runtime. |
| `createCallableSignals(options?)` | Compatibility alias that always selects `mainThreadCompatibility`. |
| `wrapSignals(rawSignals, options?)` | Wraps an existing raw `Signals` instance. |
| `signals.compatibilityApp()` | Returns the lower-level application-oriented `SignalApp`. |
| `signals.compatibilityRuntime()` | Returns the lower-level `SignalRuntime`. |
| package default export | Initializes the lower-level Wasm module only. |
| `worth-signals-wasm/raw` | Publishes raw runtime types and constructors. |
| `worth-signals-wasm/raw_surface.js` | Published alias of `./raw`. |

## Construct It Explicitly

```ts
// runtime/signals.compatibility.ts
import { createSignals } from "worth-signals-wasm";

export const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});

export const app = signals.compatibilityApp();
export const runtime = signals.compatibilityRuntime();
```

The default worker-first runtime does not quietly move synchronous specialist
work onto the UI thread. If a feature depends on these lower-level doors, make
that main-thread compatibility deployment choice visible where the runtime is
constructed.

`createCallableSignals()` exists for older callers:

```ts
import { createCallableSignals } from "worth-signals-wasm";

const signals = await createCallableSignals();
console.log(signals.contract().deployment); // "mainThreadCompatibility"
```

The function forces compatibility deployment. Use `createSignals()` in new
code so the deployment decision is visible in the call.

## The Default Wasm Initializer

```ts
import initializeWasm from "worth-signals-wasm";

await initializeWasm();
```

The default export initializes the lower-level Wasm module. It does not create
the worker-first callable facade and it is not another spelling of
`createSignals()`.

## Prefer The Callable Surface When

- local handles and callbacks describe the feature clearly;
- names do not need to be portable structural contracts;
- a published graph can express the application boundary;
- resources, forms, or router already own the larger lifecycle.

```ts
const count = signals.input(1);
const doubled = signals.computed(() => count() * 2);
```

## Use The Explicit Spec Lane When Names Are Contract

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

The spec lane is useful for portable definitions, aspect-filtered structural
reads, and migration from explicit ID-based authoring. Do not use it merely to
make private local state look official.

## SignalApp And SignalRuntime

`signals.compatibilityApp()` exposes explicit definition registration, keyed
families, direct reads and writes, diagnostics, history, and adapters.

`signals.compatibilityRuntime()` exposes runtime-oriented definition,
mutation, policy, packed-grid, diagnostics, history, and specialist operations.
Runtime policy presets are specialist configuration, not an ordinary component
concern.

Inspect the active facade before depending on a capability:

```ts
const contract = signals.assertCompatibility({
  requires: ["callableSurface", "specNamespace"],
});

console.log(contract.surfaceFamily, contract.deployment);
```

`assertCompatibility(...)` throws when a required capability is absent. Do not
infer deployment from method presence.

## Authority And Limits

- Compatibility definitions and callable definitions use the same underlying
  runtime truth model where their behavior overlaps.
- Explicit IDs are structural identity on the spec and raw surfaces.
- The compatibility runtime is main-thread by construction.
- `debugName` remains diagnostic metadata, not structural identity.
- Raw and specialist methods can expose more runtime detail than a published
  graph; do not leak that detail into application contracts casually.
- Compatibility is process-local and does not add durable server authority.

## Anti-Patterns

- Starting new ordinary app code on `SignalApp` or `SignalRuntime`.
- Treating compatibility as the "real" API and the callable facade as a toy.
- Mixing opaque local handles and explicit IDs without a named boundary.
- Depending on packed or keyed operations without understanding their cost and
  lifecycle contract.
- importing `./raw` only to avoid awaiting worker-first operations.

## Related Docs

- [Construction API](./construction.md)
- [Callable Signals API](./callable-signals.md)
- [Package Entrypoints And Runtime Contracts](../reference/package-entrypoints-and-contracts.md)
- [Support Status](../reference/support-status.md)

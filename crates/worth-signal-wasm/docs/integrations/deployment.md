# Worker-First And Compatibility Deployment

Worth Signals has two explicit runtime placements. Choose based on architecture,
not convenience.

## Worker-First

```ts
const signals = await createSignals();
```

This is the default. Runtime execution lives in a dedicated worker and the
callable facade crosses that boundary for supported operations.

Use worker-first for ordinary applications. Await mutations and history work so
code remains honest about the transport boundary.

## Main-Thread Compatibility

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
```

Use this when a host cannot construct a worker, when migrating lower-level
code, or when a specialist synchronous surface explicitly requires it.

Compatibility is supported. It is not an invisible fallback and it is not a
performance promise.

## What Stays The Same

The supported callable authoring model remains the same:

- inputs, computed values, and outputs;
- transactions and batches;
- graphs and controllers;
- resources, forms, and router facades;
- diagnostics and runtime history where documented.

Worker-first parity tests compare those surfaces against compatibility runtime
behavior. A feature that is not admitted across the worker boundary should
report that fact, not quietly run somewhere else.

## What Differs

- Worker-first calls may return promises where compatibility completes
  synchronously.
- Lower-level `compatibilityApp()` and `compatibilityRuntime()` are main-thread
  specialist doors.
- Worker-first host-capability event replay currently returns explicit
  unavailable artifacts.
- Handles remain owned by their creating runtime and cannot be mixed in a
  transaction.

## Explicit Recovery

Recover from `workerUnavailableConstruction` only. Do not catch every runtime
error and retry on the main thread; that would turn real configuration or
programming failures into unexplained architecture changes.

See [Installation And Deployment](../getting-started/installation.md) for the
complete recovery example.

## Bundler Asset Placement

Worker-first construction loads a dedicated worker script and a `.wasm` binary.
Bundlers that relocate the package JS (for example Vite optimizeDeps) can break
default relative asset URLs. Prefer:

```ts
import wasmUrl from "worth-signals-wasm/wasm?url";
import workerUrl from "worth-signals-wasm/worker?worker&url";

await createSignals({ assets: { wasmUrl, workerUrl } });
```

with `worker: { format: "es" }` on Vite. Hosts must serve missing asset routes as
404, not SPA HTML. Full matrix: [Support Status](../reference/support-status.md).

## Related Docs

- [Support Status](../reference/support-status.md)
- [Installation And Deployment](../getting-started/installation.md)
- [Lower-Level Compatibility Surface](../api-reference/compatibility-surface.md)
- [Browser Host Capabilities](./host-capabilities.md)

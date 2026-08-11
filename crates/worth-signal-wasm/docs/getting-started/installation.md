# Installation And Deployment

Install the package, create one runtime, and let that runtime own the handles
created from it.

```bash
npm install worth-signals-wasm
```

```ts
import { createSignals } from "worth-signals-wasm";

const signals = await createSignals();
```

## Vite And Bundler Assets

Zero-config is valid on modern Vite (8+) when the package files remain
fetchable and the consumer sets `worker: { format: "es" }`.

When the bundler relocates modules (Vite 7 optimizeDeps, webpack, CDN hashes),
inject emitted asset URLs:

```ts
import { createSignals } from "worth-signals-wasm";
import wasmUrl from "worth-signals-wasm/wasm?url";
import workerUrl from "worth-signals-wasm/worker?worker&url";

const signals = await createSignals({
  assets: { wasmUrl, workerUrl },
});
```

Host rule: missing `.wasm` / worker routes must return 404, not SPA
`index.html`. Do not use `optimizeDeps.exclude` as the primary fix; keep it only
as a legacy Vite workaround. See [Support Status](../reference/support-status.md)
for the host/bundler matrix.

## The Default Is Worker-First

`createSignals()` selects the dedicated-worker deployment. The promise resolves
after the worker-backed callable surface is ready.

Worker-first keeps runtime work away from the UI thread and is the normal
application lane. It is not a hint that may silently degrade into another
architecture.

## Compatibility Is Explicit

Some environments cannot construct a dedicated worker. Recover only after the
runtime returns the specific construction artifact:

```ts
import { createSignals } from "worth-signals-wasm";

let signals;

try {
  signals = await createSignals();
} catch (error) {
  if (
    !error ||
    typeof error !== "object" ||
    !("artifactFamily" in error) ||
    error.artifactFamily !== "workerUnavailableConstruction"
  ) {
    throw error;
  }

  signals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
}
```

Do not assume `createSignals()` silently falls back. A silent fallback would
change execution placement without telling the application, which is exactly
the sort of "helpful" behavior that becomes impossible to debug later.

Use `mainThreadCompatibility` deliberately for unsupported worker environments,
migration, or synchronous specialist lanes. It is supported, but it is not the
default architecture.

## Inspect Construction Before Running It

```ts
import {
  explainCreateSignalsConstruction,
  planCreateSignalsDeployment,
} from "worth-signals-wasm";

console.log(explainCreateSignalsConstruction());
console.log(planCreateSignalsDeployment());
```

The plan reports the requested deployment, selected family, reason, and any
explicit compatibility recovery path. This is useful when an application must
explain why a runtime could not start.

## React

Install React separately and import the adapter from the package subpath:

```bash
npm install worth-signals-wasm react react-dom
```

```ts
import { createReactSignalsStore } from "worth-signals-wasm/react";

const store = createReactSignalsStore(signals);
```

Read [React](../integrations/react.md) before wrapping signals in component
state.

## Lifecycle

Call `signals.free()` when the owning application runtime is permanently done.
Free subscriptions and feature-level handles when their owning scope ends.
Do not free a runtime while application code still holds live handles from it.

## Current Limits

- A worker-first transaction rejects handles owned by another runtime.
- Compatibility-only lower-level surfaces require explicit
  `mainThreadCompatibility` construction.
- Worker-first host-capability event replay has named unavailable results; the
  ordinary host values themselves remain supported.

## Related Docs

- [Your First Signal](./first-signal.md)
- [Worker-First And Compatibility Deployment](../integrations/deployment.md)
- [Support Status](../reference/support-status.md)

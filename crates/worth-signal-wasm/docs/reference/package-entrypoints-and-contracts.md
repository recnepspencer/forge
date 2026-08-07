# Package Entrypoints And Runtime Contracts

The package publishes one normal application entrypoint, one React entrypoint,
and two aliases for the raw compatibility surface. Choose at the package
boundary; do not let individual components decide their own deployment.

## Published Export Map

| Import | Status | Primary exports |
| --- | --- | --- |
| `worth-signals-wasm` | Stable named facade; compatibility default export | `createSignals`, callable/domain facades, public types, and the default Wasm initializer. |
| `worth-signals-wasm/react` | Stable | React store, subscriptions, and form bindings. |
| `worth-signals-wasm/wasm` | Stable asset | `worth_signal_wasm_bg.wasm` for bundler `?url` imports. |
| `worth-signals-wasm/worker` | Stable asset | Worker-first runtime worker for bundler `?worker&url` imports. |
| `worth-signals-wasm/raw` | Compatibility-only | Raw `SignalApp`, `SignalRuntime`, structural definitions, and branch commands. |
| `worth-signals-wasm/raw_surface.js` | Compatibility-only | Alias of `./raw`. |

The package does not publish deep imports into `product/` or `types/` as
supported npm subpaths. Import public values and types from the root, `./react`,
the asset subpaths, or the explicit raw entrypoint.

## Recommended Runtime Module

Keep construction in one module:

```ts
// platform/signals.runtime.ts
import {
  createSignals,
  hostCapabilityPlan,
  onlineCapability,
} from "worth-signals-wasm";
import { browserOnlineSource } from "./browser-online-source.js";

export const signals = await createSignals({
  hostCapabilities: hostCapabilityPlan({
    online: onlineCapability({ source: browserOnlineSource }),
  }),
});

export const runtimeContract = signals.assertCompatibility({
  requires: ["callableSurface", "scopedAuthoring", "workerRuntime"],
});
```

Feature modules import `signals` from this platform module. They do not call
`createSignals()` again, because a handle belongs to the runtime that created
it.

## Construction Functions

| Function | Return | Deployment behavior |
| --- | --- | --- |
| `createSignals(options?)` | `Promise<CallableSignals>` | Defaults to `workerFirst`; uses compatibility only when explicitly requested. |
| `createCallableSignals(options?)` | `Promise<CallableSignals>` | Forces `mainThreadCompatibility`; compatibility-only alias. |
| `wrapSignals(rawSignals, options?)` | `CallableSignals` | Wraps an existing raw runtime synchronously. |
| `planCreateSignalsDeployment(options?)` | deployment plan | Normalizes and explains selection without constructing. |
| `explainCreateSignalsConstruction(options?)` | `SignalsConstructionExplanation` | Returns only the public explanation artifact. |

`CreateSignalsOptions` accepts:

- `deployment?: "workerFirst" | "mainThreadCompatibility"`;
- `hostCapabilities?: HostCapabilityPlan` created by
  `hostCapabilityPlan(...)`;
- `assets?: { wasmUrl?: string | URL; workerUrl?: string | URL }` for
  bundler-emitted asset URLs. Worker-first requires both URLs when `assets` is
  provided; main-thread compatibility accepts `wasmUrl` only.

Host and bundler asset loading (Vite zero-config vs portable `assets`, SPA 404
rules, and the `optimizeDeps.exclude` workaround) is summarized in
[Support Status](./support-status.md).

Unknown keys, unknown deployment strings, and plain objects passed as host
capability plans are rejected as invalid construction input.

## Runtime Contract

Every callable root and scoped namespace exposes:

```ts
const contract = signals.contract();
```

`SignalsRuntimeContract` contains:

- `surfaceFamily` — the selected callable/scoped and worker/compatibility
  family;
- `surfaceVersion` — currently `"1"`;
- `deployment` — `"workerFirst"` or `"mainThreadCompatibility"`;
- `scopeId` — `null` on the root or the owning scope ID;
- `capabilities` — the declared capability booleans.

The capability names are:

- `callableSurface`;
- `scopedAuthoring`;
- `specNamespace`;
- `workerRuntime`.

Use `assertCompatibility(...)` when an integration requires a capability:

```ts
export function requireWorkerRuntime() {
  return signals.assertCompatibility({
    requires: ["callableSurface", "workerRuntime"],
  });
}
```

The assertion returns the contract when all requirements are present. Otherwise
it throws `SignalsCompatibilityAssertionError` with:

- `code: "signalsCompatibilityAssertionFailed"`;
- `requiredCapabilities`;
- `missingCapabilities`;
- `contract`.

Do not parse the message or infer capability from a method that happens to be
present.

## Handle Ownership

`InputSignalHandle`, `ComputedSignalHandle`, and `OutputSignalHandle` are owned
by the runtime that constructed them. `debugName` helps diagnostics; it is not
portable identity and cannot admit a handle into another runtime.

A published graph is the named application boundary. A scope is an authoring
and identity boundary within the same runtime. Neither creates a second truth
store.

## Cleanup

The callable root exposes `terminate()`, `free()`, and `[Symbol.dispose]()`. The
normal application owner should release the root once. Handles, diagnostics,
history, adapters, and specialist views also expose cleanup where they retain a
runtime resource or subscription.

Do not scatter root cleanup through feature components. The module that owns
construction owns termination.

## Related Reference

- [Construction API](../api-reference/construction.md)
- [Callable Signals API](../api-reference/callable-signals.md)
- [Support Status](./support-status.md)
- [Lower-Level Compatibility Surface](../api-reference/compatibility-surface.md)

# Construction API

`createSignals()` is the stable application constructor. It selects
worker-first by default and returns a `Promise<CallableSignals>`. Compatibility
deployment is always an explicit choice.

## Stable Entry Points

```ts
createSignals(options?): Promise<CallableSignals>
planCreateSignalsDeployment(options?): { explanation: SignalsConstructionExplanation }
explainCreateSignalsConstruction(options?): SignalsConstructionExplanation
hostCapabilityPlan(input?): HostCapabilityPlan
```

Host capability registration helpers:

- `viewportCapability(...)`;
- `visibilityCapability(...)`;
- `onlineCapability(...)`;
- `clockCapability(...)`;
- `persistenceCapability(...)`.

Compatibility-only constructors:

```ts
createCallableSignals(options?): Promise<CallableSignals>
wrapSignals(rawSignals, options?): CallableSignals
```

## Options

```ts
interface CreateSignalsOptions<TPersistence = SignalValue> {
  deployment?: "workerFirst" | "mainThreadCompatibility";
  hostCapabilities?: HostCapabilityPlan<TPersistence>;
}
```

`deployment` defaults to `"workerFirst"`. `hostCapabilities` must be created
with `hostCapabilityPlan(...)`; an equivalent-looking plain object is rejected.

Unknown option names and deployment strings are invalid input and throw a
`TypeError` before construction begins.

## Small Example

```ts
// platform/signals.runtime.ts
import { createSignals } from "worth-signals-wasm";

export const signals = await createSignals();
```

This is the smallest honest worker-first setup. Do not call the package default
export first; `createSignals()` initializes the runtime it needs.

## Host Capability Example

```ts
// platform/browser-online-source.ts
export const browserOnlineSource = {
  current: () => (navigator.onLine ? "online" as const : "offline" as const),
  subscribe(listener: () => void) {
    window.addEventListener("online", listener);
    window.addEventListener("offline", listener);
    return () => {
      window.removeEventListener("online", listener);
      window.removeEventListener("offline", listener);
    };
  },
};
```

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
```

The browser owns the online fact. The capability plan explicitly admits it into
the runtime; worker callbacks do not read ambient browser state directly.

## Inspect Before Constructing

```ts
const explanation = explainCreateSignalsConstruction();
```

`SignalsConstructionExplanation` contains:

- `requestedDeployment`;
- `selectedFamily`: `"workerFirst"`, `"mainThreadCompatibility"`,
  `"workerUnavailable"`, or `"denied"`;
- `selectedDeployment`: the selected deployment or `null` when construction
  cannot proceed;
- `reason`;
- `compatibilityRecovery`.

Treat the explanation as an inspection artifact, not a factory token.

## Construction Failure

Default construction rejects instead of silently falling back when no Worker
constructor is available. The error carries:

```ts
interface SignalsConstructionArtifact {
  artifactFamily: "workerUnavailableConstruction" | "signalsConstructionDenied";
  requestedDeployment: SignalsDeployment;
  reason: string;
  message: string;
  compatibilityRecovery: SignalsCompatibilityRecovery;
}
```

The current ordinary unavailable case is:

- `artifactFamily: "workerUnavailableConstruction"`;
- `reason: "workerConstructorUnavailable"`.

The `compatibilityRecovery` object tells you how to retry explicitly. It does
not authorize the library to change deployment automatically.

```ts
try {
  await createSignals();
} catch (error) {
  if (
    error instanceof Error &&
    "artifactFamily" in error &&
    error.artifactFamily === "workerUnavailableConstruction"
  ) {
    showWorkerRequirement(error);
  } else {
    throw error;
  }
}
```

## Compatibility Construction

```ts
const signals = await createSignals({
  deployment: "mainThreadCompatibility",
});
```

This produces the callable facade on the main thread. Use it when the host
cannot provide Worker or when a named specialist integration requires a
synchronous lower-level runtime door.

`createCallableSignals()` is the legacy compatibility alias. It overwrites the
deployment option with `"mainThreadCompatibility"`; it is not the worker-first
constructor despite returning the same `CallableSignals` type.

## Runtime Contract After Construction

```ts
const contract = signals.contract();
signals.assertCompatibility({ requires: ["workerRuntime"] });
```

Use the runtime contract to inspect the selected family. Do not infer
deployment from timing: callable mutations may be synchronous on compatibility
and asynchronous worker-first, so portable code awaits their result.

## Cleanup

The construction owner must eventually call `signals.free()`, await
`signals.terminate()` when it uses the asynchronous lifecycle, or use
`Symbol.dispose` in an environment that manages explicit resources.

## Current Limits

- worker-first needs a usable dedicated `Worker` constructor;
- there is no hidden fallback;
- host capabilities must be declared, not ambiently read by worker callbacks;
- construction artifacts are process-local evidence, not durable deployment
  records.

## Related Docs

- [Package Entrypoints And Runtime Contracts](../reference/package-entrypoints-and-contracts.md)
- [Support Status](../reference/support-status.md)
- [Callable Signals API](./callable-signals.md)
- [Lower-Level Compatibility Surface](./compatibility-surface.md)

# forge-signal-wasm

Framework-agnostic browser bindings for Forge Signal, with a callback-first
app surface, a typed host-capability lane, and an optional React adapter.

## Install

Public npm package:

```bash
npm install forge-signal-wasm
```

Before publishing a new version from this repo, always run the package proof:

```powershell
node scripts/wasm/verify-forge-signal-wasm-package.mjs crates/forge-signal-wasm/pkg
```

Or use the release-gate helper:

```powershell
scripts/wasm/publish-forge-signal-wasm.ps1 -SkipPublish
```

React adapter:

```bash
npm install forge-signal-wasm react
```

## Quick Start

```ts
import { createSignals } from "forge-signal-wasm";

const signals = createSignals();

const count = signals.input(1, { id: "count" });
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });

signals.transaction((tx) => {
  tx.set(count, 2);
});

console.log(panel());
```

## Host Capabilities

Use host capabilities when callback-authored derived state needs approved
browser/runtime-local facts.

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
  viewportCapability,
} from "forge-signal-wasm";

const signals = createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current() {
          return document.visibilityState;
        },
        subscribe(listener) {
          document.addEventListener("visibilitychange", listener);
          return () => document.removeEventListener("visibilitychange", listener);
        },
      },
      compatibility: "LiveOnly",
    }),
    viewport: viewportCapability({
      source: {
        current() {
          return { width: window.innerWidth, height: window.innerHeight };
        },
        subscribe(listener) {
          window.addEventListener("resize", listener);
          return () => window.removeEventListener("resize", listener);
        },
      },
    }),
  }),
});

const layout = signals.computed(() => (
  signals.host.visibility.isVisible() && signals.host.viewport.width() > 900
    ? "wide"
    : "narrow"
), { id: "layout" });
```

Good to know:

- host capability reads are typed `signals.host.*` reads, not ambient closure
  reads
- unsupported host reads stay non-reactive by contract
- diagnostics and transport surfaces preserve denied vs unavailable family
  posture

For the full guide, see
[docs/host_capabilities.md](./docs/host_capabilities.md).

## Core Concepts

### `input`

Use `input` for mutable source state.

Simple:

```ts
const count = signals.input(1, { id: "count" });
signals.transaction((tx) => tx.set(count, 2));
```

Complex:

```ts
const part = signals.input({
  id: "gear-7",
  teeth: 24,
  enabled: true,
}, {
  id: "part",
  producesAspects: [1, 2],
});

signals.transaction((tx) => {
  tx.setWithAspects(part, {
    id: "gear-7",
    teeth: 26,
    enabled: true,
  }, [1]);
});
```

### `computed`

Use `computed` for runtime-owned derived state. Callback form is the normal
authoring lane.

Only callable signal reads are tracked. Ordinary closure variables are not
reactive dependencies, and a callback that captures no signal reads can be
lowered into a constantized node.

Simple:

```ts
const doubled = signals.computed(() => count() * 2, { id: "doubled" });
```

Complex:

```ts
const enabled = signals.input(true, { id: "enabled" });
const name = signals.input("Ada", { id: "name" });

const label = signals.computed(() => {
  return enabled() ? `${name()} is enabled` : "disabled";
}, { id: "label" });
```

Advanced recipe form still exists:

```ts
const doubled = signals.computedSpec("doubled", {
  reads: ["count"],
  expr: {
    kind: "multiply",
    args: [
      { kind: "read", id: "count" },
      { kind: "value", value: 2 },
    ],
  },
});
```

### `output`

Use `output` for public projections you hand to UI layers, tables, panels, or
other consumers.

Callback outputs follow the same capture rule as callback computed nodes:
signal reads are tracked, ordinary closure variables are not, and richer
aspect-targeted projection contracts belong on the explicit spec lane.

Simple:

```ts
const panel = signals.output(() => ({
  count: count(),
  doubled: doubled(),
}), { id: "panel" });
```

Complex:

```ts
const partSummary = signals.output(() => ({
  part: part(),
  label: label(),
  status: part().enabled ? "active" : "inactive",
}), { id: "partSummary" });
```

Advanced recipe form still exists when you need explicit portable specs:

```ts
const partSummary = signals.outputSpec("partSummary", {
  reads: ["part", "label"],
  expr: {
    kind: "object",
    fields: [
      ["part", { kind: "read", id: "part" }],
      ["label", { kind: "read", id: "label" }],
    ],
  },
});
```

### `watch` and `effect`

Use `watch` when you want the notice payload. Use `effect` when you only need a
committed side-effect trigger.

Simple:

```ts
const handle = signals.watch(panel, (notice) => {
  console.log(notice.signalId, notice.meaningfulChange);
});
```

Complex:

```ts
const saveHandle = signals.effect(partSummary, () => {
  const payload = partSummary();
  queueMicrotask(() => saveDraft(payload));
});

signals.nuke(saveHandle);
```

### `transaction`

Use `transaction` or `batch` for all writes.

Simple:

```ts
signals.transaction((tx) => {
  tx.set(count, count() + 1);
});
```

Complex:

```ts
signals.transaction((tx) => {
  tx.setWithRegionsAndAspects(
    part,
    { ...part(), teeth: 30 },
    [{ region: "geometry" }],
    [1],
  );
});
```

## Diagnostics

Diagnostics are first-class. Start here:

```ts
const diagnostics = signals.diagnostics();
```

Simple:

```ts
const why = diagnostics.why("doubled");
console.log(why.recipeFamily, why.callback?.currentReads);
```

Host-capability-specific inspection is also available:

```ts
const hostReport = diagnostics.hostCapabilityReport();
const latestHostEvent = diagnostics.latestHostCapabilityEvent();
```

Complex:

```ts
const latestObservation = diagnostics.latestObservation();
const latestFlow = diagnostics.latestFlow();
const perf = diagnostics.performanceSummary();

console.log({
  delivered: latestObservation?.observation.delivered_event_count,
  callbackReads: perf.computeCallbackCapturedReadCount,
  dependencyPatches: perf.computeCallbackDependencyPatchCount,
  callbackNodes: latestFlow?.callbackNodes.map((node) => node.id) ?? [],
});
```

## React Adapter

```ts
import { createSignals } from "forge-signal-wasm";
import {
  createReactSignalsStore,
  useSignalValue,
  useOutputValue,
  useSignalsDiagnostics,
} from "forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);
```

Simple:

```tsx
function Counter() {
  const countValue = useSignalValue(count, store);
  return <button onClick={() => store.transaction((tx) => tx.set(count, countValue + 1))}>
    {countValue}
  </button>;
}
```

Complex:

```tsx
function PartPanel() {
  const summary = useOutputValue(partSummary, store);
  const diagnostics = useSignalsDiagnostics(store);

  return (
    <>
      <pre>{JSON.stringify(summary, null, 2)}</pre>
      <small>{diagnostics.performanceSummary.computeCallbackDependencyPatchCount}</small>
    </>
  );
}
```

## Advanced Lanes

- Prefer callback-first `computed(() => ...)` and `output(() => ...)` for
  ordinary app code.
- Prefer `signals.input(value, { id })` when you want the family to read with
  one coherent grammar.
- Keep `computedSpec(...)` and `outputSpec(...)` for explicit portable recipe
  authoring.
- Keep `compatibilityApp()` and `compatibilityRuntime()` for expert or migration
  scenarios, not for the default product lane.

## Documentation

- [docs/README.md](docs/README.md)
- [docs/consuming_the_package.md](docs/consuming_the_package.md)
- [docs/app_surface_reference.md](docs/app_surface_reference.md)
- [docs/diagnostics_and_history_reference.md](docs/diagnostics_and_history_reference.md)
- [docs/react_adapter_reference.md](docs/react_adapter_reference.md)

# forge-signal-wasm

Framework-agnostic web runtime bindings for Forge Signal, packaged for browser
bundlers and private npm distribution.

## Documentation

- [docs/README.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/README.md)
  Documentation index.
- [docs/consuming_the_package.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/consuming_the_package.md)
  Build, prepare, install, and import guide.
- [docs/app_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/app_surface_reference.md)
  Encyclopedia-style reference for the app-first API.
- [docs/diagnostics_and_history_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/diagnostics_and_history_reference.md)
  Diagnostics, history, branch, and adapter surfaces.
- [docs/compatibility_surface_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/compatibility_surface_reference.md)
  Lower-level compatibility/runtime surface reference.
- [docs/aspects_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/aspects_reference.md)
  Aspect-aware node, read, invalidation, and versioning reference.
- [docs/react_adapter_reference.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/react_adapter_reference.md)
  React adapter reference.
- [docs/host_callback_computed_spec.md](/C:/Users/shepworth/Documents/programming/forge/crates/forge-signal-wasm/docs/host_callback_computed_spec.md)
  Proposed callback-first computed-node milestone for normal TypeScript
  authoring with dynamic dependencies and diagnostics parity.

## App-First Surface

The primary entrypoint is `createSignals()`:

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";

const signals = createSignals();

const count = signals.input("count", 1);

const doubled = signals.computed("doubled", () => count() * 2);

const panel = signals.output("panel", {
  reads: ["count", "doubled"],
  expr: {
    kind: "object",
    fields: [
      ["count", { kind: "read", id: "count" }],
      ["doubled", { kind: "read", id: "doubled" }],
    ],
  },
});

const watchHandle = signals.watch(panel, (notice) => {
  console.log("panel changed", notice);
});

const effectHandle = signals.effect(panel, () => {
  console.log("panel effect fired");
});

signals.transaction((tx) => {
  tx.set(count, 2);
});

const latestObservation = signals.diagnostics().latestObservation();
const latestFlow = signals.diagnostics().latestFlow();
const perf = signals.diagnostics().performanceSummary();

signals.nuke(watchHandle);
signals.nuke(effectHandle);
```

## React Adapter

The package can also expose a React-domain adapter through the `./react`
subpath:

```ts
import { createSignals } from "@aust-group/forge-signal-wasm";
import {
  createReactSignalsStore,
  useOutputValue,
  useSignalValue,
  useSignalsDiagnostics,
} from "@aust-group/forge-signal-wasm/react";

const signals = createSignals();
const store = createReactSignalsStore(signals);
const count = signals.input("count", 1);
const panel = signals.output("panel", {
  reads: ["count"],
  expr: {
    kind: "object",
    fields: [["count", { kind: "read", id: "count" }]],
  },
});

function Counter() {
  const countValue = useSignalValue(count, store);
  const panelValue = useOutputValue(panel, store);
  const diagnostics = useSignalsDiagnostics(store);

  return { countValue, panelValue, diagnostics };
}

store.transaction((tx) => {
  tx.set(count, 2);
});
```

The React adapter is intentionally thin:

- `createReactSignalsStore(signals)` owns React subscription glue
- `useSignalValue(...)` reads `input` and `computed` handles
- `useOutputValue(...)` reads `output` handles
- `useSignalsDiagnostics(...)` exposes latest observation, latest flow, and
  performance summary snapshots
- `store.transaction(...)` and `store.batch(...)` are the React-friendly write
  lanes and refresh diagnostics snapshots after committed writes
- the store also instruments the shared `Signals` instance so app-first
  `signals.transaction(...)` and `signals.batch(...)` refresh diagnostics
  snapshots for React consumers

The app-first concepts are:

- `input`
- `computed`
- `output`
- `watch`
- `effect`
- `transaction`
- `nuke`

Those concepts are also aspect-aware now. Web consumers can declare produced
aspects, read only selected aspects, and write or invalidate only the aspects
that changed, while keeping watcher/effect semantics node-scoped.

## Aspect Profile Selection

The package now passes the `forge-signal` storage-profile policy through to
WASM builds:

- default / `profile-standard`: 8 aspect slots
- `profile-extended`: 16 aspect slots

Use `forgeSignalMaxAspects()` and `forgeSignalCoreProfile()` at runtime to
inspect the compiled profile. Build extended WASM with:

```bash
cargo build -p forge-signal-wasm --no-default-features --features profile-extended
```

Aspect identifiers remain `u8` at the JS/WASM boundary, but the compiled
signal profile determines which ids are admitted.

## Semantics

- `input` is mutable source state.
- `computed` is derived internal state and supports callback-first authoring
  through `computed(() => ...)`.
- `output` is a public projection intended for host/framework consumption.
- `output` callback authoring is intentionally deferred for now; use
  `outputSpec(...)` / `output(...)` with an explicit recipe.
- `watch` and `effect` inherit committed observation semantics from
  `forge-signal`.
- node definitions, reads, invalidation, and version reporting support real
  Forge Signal aspects instead of collapsing the web layer to a single default
  aspect.
- rollback suppresses normal watch/effect delivery.
- `nuke(handle)` tears down future deliveries for that handle.
- diagnostics expose both `latestObservation()` and `latestFlow()` so host code
  can inspect the same committed boundary the watcher/effect layer saw.
- `performanceSummary()` exposes the web-layer cert counters for active
  handles, matched watcher breadth, rollback-suppressed deliveries, callback
  invocations, output serialization, and compatibility read breadth.

## Compatibility Surface

The package still exports the lower-level compatibility/runtime surfaces:

- `SignalApp`
- `SignalRuntime`
- `SignalDiagnostics`
- `SignalHistory`
- `SignalSpecialist`
- `SignalAdapters`

Those remain available for advanced or legacy usage, but they are no longer the
primary product story. New web code should start from `createSignals()`.

## Status

Current app-first coverage includes:

- `createSignals()`
- `input`, `computed`, `output`
- callback-first `computed(() => ...)`
- explicit `outputCallbackDeferred` denial for callback-shaped output authoring
- `watch`, `effect`, `nuke`
- `transaction` / `batch`
- diagnostics latest observation / latest flow access
- compatibility runtime/history/adapter surfaces

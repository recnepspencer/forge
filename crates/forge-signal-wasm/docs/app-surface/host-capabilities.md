# Host Capabilities

## What This Feature Is

Host capabilities are the typed browser- and runtime-fact lane in
`forge-signal-wasm`.

Use them when derived signal code needs approved host inputs such as:

- document visibility
- viewport size
- online or offline state
- clock time
- persistence-backed local values

Host capability reads are not ambient closure reads. They are explicit
runtime-owned dependencies registered when the signals instance is created.

## Why You Use It

- make browser facts participate in derived signal truth honestly
- avoid ad hoc event glue inside `computed(...)` or `output(...)`
- keep invalidation runtime-owned and inspectable
- preserve restore, import, and export posture per host family
- understand host-caused reevaluation through diagnostics

## What Distinguishes This

Host capability is not just “helpers for browser globals.” It admits host facts
into the reactive runtime as typed, inspectable dependencies.

That is different from the usual frontend pattern where code reads
`window.innerWidth`, `document.visibilityState`, or `navigator.onLine` inside a
callback and then stitches reactivity back together with local event glue.

What this enables here is:

- runtime-owned invalidation for host facts
- explicit compatibility posture per host family
- diagnostics that can attribute reevaluation or denial to one host source

Without this lane, values like `window.innerWidth`,
`document.visibilityState`, or `Date.now()` can influence a callback’s return
value but do not become tracked reactive dependencies.

## Stable Entry Points

Import the stable surface from the package root:

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
  viewportCapability,
  onlineCapability,
  clockCapability,
  persistenceCapability,
} from "forge-signal-wasm";
```

Stable entry points today:

- `hostCapabilityPlan(...)`
- `visibilityCapability(...)`
- `viewportCapability(...)`
- `onlineCapability(...)`
- `clockCapability(...)`
- `persistenceCapability(...)`
- `signals.host.visibility`
- `signals.host.viewport`
- `signals.host.online`
- `signals.host.clock`
- `signals.host.persistence`
- `signals.diagnostics().hostCapabilityReport()`
- `signals.adapters().hostCapabilityTransportReport(envelope?)`

## Core Mental Model

Think about host capability in three layers:

1. registration
2. typed reads
3. compatibility posture

Registration is explicit:

```ts
const signals = await createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({ source: visibilitySource() }),
    viewport: viewportCapability({ source: viewportSource() }),
  }),
});
```

Typed reads happen through `signals.host.*`, not through ambient globals:

```ts
const layout = signals.computed(() => {
  const visible = signals.host.visibility.isVisible();
  const width = signals.host.viewport.width();
  return visible && width > 900 ? "wide" : "narrow";
}, { debugName: "layout" });
```

Each family also carries explicit compatibility posture:

- `LiveOnly`
- `Reattachable`
- `SnapshotPortable`
- `ImportDenied`

That posture tells restore and export surfaces whether a capability can remain
live, be reattached, survive only as committed snapshot truth, or must deny
portable import.

## How It Executes

At runtime, each admitted host family is lowered to a framework-owned internal
source plus a typed public handle.

Important consequences:

- callback dependency capture records host reads explicitly
- ambient closure reads do not become tracked dependencies
- push-driven families batch invalidation through the runtime
- polled families expose polling work through public counters
- manually committed families require explicit `commit()`
- exported runtime artifacts preserve denied vs unavailable family posture

The runtime still owns derivation semantics. Host capabilities feed facts into
that system. They do not become a second truth engine.

## Small Example

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
} from "forge-signal-wasm";

const signals = await createSignals({
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
  }),
});

const visibilityLabel = signals.computed(() => (
  signals.host.visibility.isVisible() ? "visible" : "hidden"
), { debugName: "visibilityLabel" });
```

Why this is the smallest honest example:

- registration is explicit
- the callback reads through `signals.host.visibility`
- visibility invalidation is runtime-owned

## Real Example

```ts
import {
  clockCapability,
  createSignals,
  hostCapabilityPlan,
  onlineCapability,
  persistenceCapability,
  viewportCapability,
  visibilityCapability,
} from "forge-signal-wasm";

let persistedDraft = { mode: "draft", revision: 1 };

const signals = await createSignals({
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
    online: onlineCapability({
      source: {
        current() {
          return navigator.onLine ? "online" : "offline";
        },
        subscribe(listener) {
          window.addEventListener("online", listener);
          window.addEventListener("offline", listener);
          return () => {
            window.removeEventListener("online", listener);
            window.removeEventListener("offline", listener);
          };
        },
      },
    }),
    clock: clockCapability({
      source: {
        current() {
          return Date.now();
        },
      },
      pollMs: 1000,
    }),
    persistence: persistenceCapability({
      source: {
        current() {
          return persistedDraft;
        },
      },
    }),
  }),
});

const banner = signals.output(() => ({
  visible: signals.host.visibility.isVisible(),
  viewport: signals.host.viewport.size(),
  online: signals.host.online.isOnline(),
  second: Math.floor(signals.host.clock.now() / 1000),
  revision: signals.host.persistence.value().revision,
}), { debugName: "banner" });

persistedDraft = { mode: "published", revision: 2 };
signals.host.persistence.commit();
```

This example mixes three invalidation modes:

- push-driven: `visibility`, `viewport`, `online`
- polled: `clock`
- manually committed: `persistence`

That mix is supported intentionally. Each family stays attributable in
diagnostics and transport artifacts.

## Host Families

### `visibility`

Use when you need document visibility as reactive truth.

- typical compatibility posture: `LiveOnly`
- typed reads:
  - `isVisible()`
  - `state()`

### `viewport`

Use when you need viewport size as reactive truth.

- typical compatibility posture: `Reattachable`
- typed reads:
  - `size()`
  - `width()`
  - `height()`

### `online`

Use when you need online or offline state.

- typical compatibility posture: `Reattachable`
- typed reads:
  - `isOnline()`
  - `state()`

### `clock`

Use when you need runtime-owned time progression.

- typical compatibility posture: `SnapshotPortable`
- typed reads:
  - `now()`

### `persistence`

Use when you need manually committed local durable state.

- typical compatibility posture: `ImportDenied`
- typed reads:
  - `value()`
- explicit commit:
  - `commit()`

## How It Relates To Other Features

- Pair this with `computed(...)` and `output(...)` when browser facts belong in
  derived truth.
- Pair this with diagnostics when you need to explain invalidation caused by
  host events.
- Pair this with adapters when you need host-capability transport posture in
  exported runtime artifacts.
- Pair this with the React adapter by sharing one `signals` instance; React
  should consume host-backed derived truth, not own host lifecycle.

## Inspection And Debugging

Start with:

```ts
const diagnostics = signals.diagnostics();
```

Useful entry points:

- `latestHostCapabilityEvent()`
- `recentHostCapabilityEvents()`
- `hostCapabilityReport()`
- `performanceSummary()`
- `latestFlow()`
- `latestObservation()`

For exported runtime artifacts:

```ts
const envelope = signals.adapters().exportRuntimeEnvelope();
const transportReport = signals.adapters().hostCapabilityTransportReport(envelope);
```

Use these when you need to answer:

- which host family invalidated work
- how much queued invalidation or reevaluation happened
- which callback ids were denied on portable import
- whether an artifact was denied or merely unavailable

Host-read denial is also explicit. A known host family that was not admitted in
`hostCapabilityPlan(...)` denies with
`computeCallbackMissingHostCapabilityReadDenied`; a host handle retained after
runtime termination denies with
`computeCallbackDetachedHostCapabilityReadDenied`. Both paths are counted in
`hostCapabilityReport().totals.readDenialCount`, and the latest denial is
reflected in
`hostCapabilityReport().callbackHostReadCertification.ambientHostReadDenialArtifact`.
This is separate from the zero per-read RPC claim: callback host reads either
lower into admitted worker-owned dependencies or deny with a typed artifact.
Host ingress refresh failures are reported separately as
`hostCapabilityReport().totals.dependencyRefreshFailureCount`; they are
availability/refresh failures, not host-read denials.

## Anti-Patterns

- reading browser globals directly in callbacks and expecting reactivity

```ts
signals.computed(() => window.innerWidth, { debugName: "bad" });
```

- treating `signals.host.*` handles as user-owned lifecycle objects
- using React mount and unmount as your host registration model
- assuming portable import means live reevaluation succeeded
- mutating a persistence source without calling `commit()`

## Current Limits

- only the admitted families are supported today:
  - `visibility`
  - `viewport`
  - `online`
  - `clock`
  - `persistence`
- unsupported host reads stay non-reactive by contract
- host capability is a wasm product lane, not a forms or resources layer
- family compatibility differs intentionally and is part of the public posture

## Related Docs

- [App Surface Overview](./overview.md)
- [Diagnostics And History](./diagnostics-and-history.md)
- [React Adapter](./react-adapter.md)

# Host Capabilities

## What This Feature Is

Host capabilities are the typed product lane for browser- and runtime-local
facts in `forge-signal-wasm`.

Use them when callback-authored derived state needs approved host inputs such
as:

- document visibility
- viewport size
- online/offline state
- clock/timer-facing host time
- persistence-backed local facts

Host capability reads are not ambient closure reads. They are explicit
framework-owned dependencies registered when the runtime is created.

## Why You Use It

Use host capabilities when you want host-local facts to participate in
`signals.computed(...)` or `signals.output(...)` without lying about
reactivity.

They give you:

- typed `signals.host.*` accessors
- runtime-owned invalidation instead of ad hoc event glue
- honest restore/import/export posture per family
- diagnostics that name which host family dirtied or denied work
- public counters for host-capability lifecycle and reevaluation cost

Without this lane, `window.innerWidth`, `document.visibilityState`, or
`Date.now()` are just ambient closure reads. They may affect a callback’s
return value, but they are not reactive dependencies.

## Stable Entry Points

Import the stable host-capability surface from the package root:

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

Think in three layers:

1. registration
2. typed reads
3. compatibility posture

Registration is explicit:

```ts
const signals = createSignals({
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
}, { id: "layout" });
```

Each family also carries an explicit compatibility posture:

- `LiveOnly`
- `Reattachable`
- `SnapshotPortable`
- `ImportDenied`

That posture tells restore/import/export surfaces whether a capability can stay
live, be reattached, survive only as committed snapshot truth, or must deny
portable import.

## How It Executes

At runtime, each admitted host family is lowered to a framework-owned hidden
source plus one typed host handle.

Important consequences:

- callback dependency capture records host capability reads explicitly
- ambient closure reads do not become tracked dependencies
- push-driven families batch invalidation through the runtime
- polled families expose polling work through public counters
- manually committed families require explicit `commit()`
- exported runtime envelopes preserve denied vs unavailable family posture

The runtime still owns derivation semantics. Host capabilities feed typed facts
into that system; they do not become a second truth engine.

## Small Example

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
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
  }),
});

const label = signals.computed(() => (
  signals.host.visibility.isVisible() ? "visible" : "hidden"
), { id: "label" });
```

Good to know:

- `signals.host.visibility` is framework-owned; there is no public `free()`
- changing unrelated closure variables does not re-run `label`
- visibility events invalidate only the capability-dependent frontier

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

let draft = { mode: "draft", revision: 1 };

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
          return draft;
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
}), { id: "banner" });

draft = { mode: "published", revision: 2 };
signals.host.persistence.commit();
```

This example mixes three invalidation modes:

- push-driven: `visibility`, `viewport`, `online`
- polled: `clock`
- manually committed: `persistence`

That mix is supported intentionally. Each family stays attributable in
diagnostics and transport artifacts.

## How It Relates To Other Features

- `computed(...)` and `output(...)`
  Host capabilities are an input lane for callback-first derivation. They do
  not replace signal reads.
- `transaction(...)`
  Host families drive invalidation through runtime-owned paths. Manual writes
  still go through `transaction(...)`.
- diagnostics
  Use host-capability diagnostics when you need causality, counters, or
  transport posture.
- adapters/history
  Runtime envelope export/import surfaces preserve whether a family was live,
  unavailable, reattachable, snapshot-portable, or denied.
- React
  The React adapter consumes host-capability-backed derived truth; it does not
  own host-capability lifecycle.

## Inspection And Debugging

Start with the ordinary diagnostics surface:

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

For exported runtime envelopes:

```ts
const envelope = signals.adapters().exportRuntimeEnvelope();
const transportReport = signals.adapters().hostCapabilityTransportReport(envelope);
```

Use these when you need to answer:

- which host family invalidated work
- how much queued invalidation or reevaluation happened
- which callback ids were denied on portable import
- whether an artifact was denied or merely unavailable

## Anti-Patterns

- Reading ambient browser state directly in callbacks and expecting reactivity

```ts
signals.computed(() => window.innerWidth, { id: "bad" });
```

- Treating `signals.host.*` handles as user-owned lifecycle objects
- Using React mount/unmount as your host-capability registration model
- Assuming portable import means live reevaluation succeeded
- Using `persistence` without `commit()` and expecting updates to publish

## Current Limits

- only the admitted families are supported today:
  - `visibility`
  - `viewport`
  - `online`
  - `clock`
  - `persistence`
- unsupported host reads stay non-reactive by contract
- host capability is a wasm/product lane; it does not yet teach forms or API
  resources how to consume host-local facts directly
- family compatibility differs intentionally:
  - `visibility`: `LiveOnly`
  - `viewport`: `Reattachable`
  - `online`: `Reattachable`
  - `clock`: `SnapshotPortable`
  - `persistence`: `ImportDenied`

## Related Docs

- [README.md](../README.md)
- [app_surface_reference.md](./app_surface_reference.md)
- [diagnostics_and_history_reference.md](./diagnostics_and_history_reference.md)
- [react_adapter_reference.md](./react_adapter_reference.md)
- [host_capability_spec.md](./host_capability_spec.md)

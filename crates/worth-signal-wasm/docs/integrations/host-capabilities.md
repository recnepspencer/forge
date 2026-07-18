# Host Capabilities

Browser facts such as visibility, viewport size, connectivity, time, and
persistence belong to the host. Registering a host capability gives the runtime
an explicit, inspectable way to read those facts without hiding ambient browser
access inside computed callbacks.

## Stable Entry Points

- `hostCapabilityPlan(...)`
- `visibilityCapability(...)`
- `viewportCapability(...)`
- `onlineCapability(...)`
- `clockCapability(...)`
- `persistenceCapability(...)`
- `signals.host`

## Register Visibility

```ts
import {
  createSignals,
  hostCapabilityPlan,
  visibilityCapability,
} from "worth-signals-wasm";

const signals = await createSignals({
  hostCapabilities: hostCapabilityPlan({
    visibility: visibilityCapability({
      source: {
        current: () => document.visibilityState,
        subscribe(listener) {
          document.addEventListener("visibilitychange", listener);
          return () => document.removeEventListener("visibilitychange", listener);
        },
      },
    }),
  }),
});

const shouldPoll = signals.computed(
  () => signals.host.visibility?.isVisible() ?? false,
);
```

The browser owns the raw fact. The registration owns its subscription. The
runtime owns the admitted signal value and dependency relationship.

## Why Ambient Reads Are Denied

A callback that reads `document.visibilityState` directly hides a dependency
from the runtime. It cannot be replayed or refreshed honestly because the host
read never crossed a declared boundary.

Host diagnostics expose `ambientHostReadDenialArtifact` when a host read is
missing or detached. Useful counters include:

- `readDenialCount`
- `dependencyRefreshFailureCount`

Named denial reasons include
`computeCallbackMissingHostCapabilityReadDenied` and
`computeCallbackDetachedHostCapabilityReadDenied`.

Those are structured diagnostic facts. Do not branch product behavior on their
human-readable messages.

## Persistence Is A Host Capability, Not A Database

The persistence capability admits a host-owned current value and commit
lifecycle. It does not turn browser storage into shared durable platform truth.
Authentication, cross-device convergence, and durable relational authority
remain outside this runtime.

## Worker-First Limits

Worker-first supports admitted host-capability plans and keeps `signals.host`
live across runtime replacement. Host-capability event replay currently returns
an explicit unavailable result in worker-first diagnostics. Current host values
and their declared refresh behavior remain supported.

## Anti-Patterns

- Do not read ambient browser state inside a computed callback.
- Do not register a capability without disposing its subscription.
- Do not describe local persistence as durable server truth.
- Do not silently switch deployment because a capability plan is unsupported.

## Related Docs

- [Worker-First And Compatibility Deployment](./deployment.md)
- [Diagnostics And Explanation](../core/diagnostics.md)
- [Support Status](../reference/support-status.md)

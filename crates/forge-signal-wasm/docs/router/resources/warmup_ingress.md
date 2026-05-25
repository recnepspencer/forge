# Warmup Ingress

## What This Feature Is

Warmup ingress is the host-event boundary for route warmup. It turns real host
signals such as hover, focus, viewport, or intent into typed router envelopes.

## Why You Use It

- keep host-driven warmup explicit
- separate route warmup events from direct app calls
- inspect whether a host event actually warmed any matching route resources

## Stable Entry Points

- `signals.router.warmup.hover(...)`
- `signals.router.warmup.focus(...)`
- `signals.router.warmup.viewport(...)`
- `signals.router.warmup.intent(...)`
- `routes.applyWarmupIngress(...)`

## Core Mental Model

Warmup ingress is not just shorthand for `routes.warmup(...)`. It is the
boundary artifact for a real host event.

## How It Executes

1. create a typed ingress envelope
2. hand it to `routes.applyWarmupIngress(...)`
3. receive a warmup report
4. optionally take the returned artifact and keep warming or admit later

## Small Example

```ts
const ingress = signals.router.warmup.hover("/warm/user-8");
const report = routes.applyWarmupIngress(ingress);
```

## Real Example

```ts
const ingress = signals.router.warmup.hover("/warm/user-8", {
  sourceId: "sidebar-link",
  sourceValue: { lane: "hover" },
  routeIdentity: "warmRoute",
});

const report = routes.applyWarmupIngress(ingress);

console.log(report.diagnostics());
console.log(report.artifact()?.resourceNames());
```

## How It Relates To Other Features

- the warmed artifact is the same shape discussed in [Resource Warmup](./resource_warmup.md)
- broader browser boundary work lives under [History](../history/README.md)

## Inspection And Debugging

- `report.diagnostics().boundaryArtifact`
- `report.diagnostics().warmedResourceNames`
- `report.diagnostics().skippedResourceNames`
- `report.artifact()`
- `report.verification()`

## Anti-Patterns

- faking host ingress by passing raw event objects around your app
- throwing away the report and guessing why nothing warmed
- treating `noMatchingWarmupResources` as an error instead of an explicit
  boundary result

## Current Limits

- warmup ingress is route warmup only; it does not replace browser-history
  ingress
- unroutable locations fail closed with `noProjectedCandidate`

## Related Docs

- [Resource Warmup](./resource_warmup.md)
- [Browser History Ingress](../history/browser_history_ingress.md)

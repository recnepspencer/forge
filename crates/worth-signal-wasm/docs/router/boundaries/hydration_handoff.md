# Hydration Handoff

## What This Feature Is

Hydration handoff is the explicit server-to-client route boundary represented
by `signals.router.hydration.server(...)`.

## Why You Use It

- compare client route truth to what the server rendered
- distinguish matched hydration from drifted hydration
- fail closed when client admission does not produce a route outcome

## Stable Entry Points

- `signals.router.hydration.server(...)`
- `routes.admitHydrationHandoff(...)`
- `RouterHydrationAdmissionReport`

## Core Mental Model

Hydration is not ordinary browser ingress. It is one explicit boundary where
client route truth may match or drift from server route truth.

## How It Executes

1. capture server route identity and optional server href
2. build a hydration handoff envelope
3. admit it on the client
4. classify the result as matched, drifted, or not admitted

## Small Example

```ts
const handoff = signals.router.hydration.server("/detail", {
  serverRouteIdentity: "detail",
  serverHref: "/detail",
});
```

## Real Example

```ts
const report = await routes.admitHydrationHandoff(
  signals.router.hydration.server("/detail", {
    serverRouteIdentity: "home",
    serverHref: "/",
  }),
);

console.log(report.outcome().kind);
console.log(report.diagnostics().boundaryArtifact);
```

## How It Relates To Other Features

- the broader explanation layer lives in
  [Navigation Auditability](../history/navigation_auditability.md)
- ordinary browser ingress is different:
  [Browser History Ingress](../history/browser_history_ingress.md)

## Inspection And Debugging

- `report.diagnostics().boundaryArtifact`
- `report.diagnostics().routeId`
- `report.diagnostics().href`
- `report.verification()`

## Anti-Patterns

- treating hydration as just another `load(...)` event
- assuming server route identity is always correct

## Current Limits

- the current hydration handoff surface is the explicit `server(...)` lane

## Related Docs

- [Browser Authority Coherence](./browser_authority_coherence.md)
- [Navigation Auditability](../history/navigation_auditability.md)

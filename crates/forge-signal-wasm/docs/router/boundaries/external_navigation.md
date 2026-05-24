# External Navigation

## What This Feature Is

External navigation is the browser-authority lane for route events that came
from outside the current router-controlled tab flow.

## Why You Use It

- distinguish external navigation from same-tab or cross-tab authority
- classify unmatched external route truth honestly
- preserve external authority in history inspection and auditability

## Stable Entry Points

- `signals.router.browserHistory.external(...)`
- `signals.router.browserHistory.coherence.externalNavigation(...)`

## Core Mental Model

External navigation can still admit route truth, drift from expected truth, or
fail closed with no admitted route. The important part is that the boundary
source remains explicit.

## How It Executes

1. create an external browser ingress
2. optionally attach external-navigation coherence
3. admit the route
4. retain the outcome and external source classification

## Small Example

```ts
const ingress = signals.router.browserHistory.external("/missing");
```

## Real Example

```ts
const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.external("/missing", {
    routeIdentity: "settings",
    coherence: signals.router.browserHistory.coherence.externalNavigation({
      channelId: "workspace-main",
    }),
  }),
);

console.log(report.diagnostics().boundaryArtifact);
console.log(report.diagnostics().coherenceKind);
```

## How It Relates To Other Features

- general coherence model is in [Browser Authority Coherence](./browser_authority_coherence.md)
- worker fallback retains the same categories in
  [Worker History Fallback](../runtime_placement/worker_history_fallback.md)

## Inspection And Debugging

- `report.diagnostics().boundaryArtifact`
- `story.latestBoundaryEvent()?.coherenceKind`
- `auditability.summary().latestBoundaryCoherenceKind`

## Anti-Patterns

- flattening external and same-tab load into one untyped route event
- inventing admitted route truth when the external event did not admit

## Current Limits

- external navigation is a boundary classification, not a general tab sync API

## Related Docs

- [Browser Authority Coherence](./browser_authority_coherence.md)
- [Cross-Tab Navigation](./cross_tab_navigation.md)

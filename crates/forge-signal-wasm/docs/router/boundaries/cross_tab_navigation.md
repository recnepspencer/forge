# Cross-Tab Navigation

## What This Feature Is

Cross-tab navigation is the browser-authority lane where one tab observes route
authority that originated from another tab.

## Why You Use It

- keep route drift explicit when another tab changed browser authority
- preserve the source tab and channel identity at the boundary
- explain current route truth honestly in auditability surfaces

## Stable Entry Points

- `signals.router.browserHistory.coherence.crossTab(...)`
- browser ingress or writeback with `coherence`

## Core Mental Model

Cross-tab coherence does not automatically mean drift. Drift only happens when
the admitted route truth differs from the expected route witness carried by the
coherence artifact.

## How It Executes

1. create a cross-tab coherence artifact
2. attach it to browser ingress
3. admit the route
4. classify the event as converged or drifted from authority

## Small Example

```ts
const crossTab = signals.router.browserHistory.coherence.crossTab(
  "workspace-main",
  { sourceTabId: "tab-b", expectedRouteId: "home" },
);
```

## Real Example

```ts
const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.external("/settings", {
    routeIdentity: "home",
    coherence: crossTab,
  }),
);

console.log(report.diagnostics().boundaryArtifact);
console.log(report.diagnostics().coherenceKind);
```

## How It Relates To Other Features

- the generic coherence model is in [Browser Authority Coherence](./browser_authority_coherence.md)
- retained visibility explanation shows up in
  [Navigation Auditability](../history/navigation_auditability.md)

## Inspection And Debugging

- `report.diagnostics().boundaryArtifact`
- `report.coherence()`
- `story.currentRouteTruthEvent()?.coherenceKind`

## Anti-Patterns

- assuming all cross-tab events are errors
- dropping the expected route witness when drift is meaningful

## Current Limits

- cross-tab classification depends on explicit coherence metadata

## Related Docs

- [Browser Authority Coherence](./browser_authority_coherence.md)
- [External Navigation](./external_navigation.md)

# Browser Authority Coherence

## What This Feature Is

Browser authority coherence is the explicit artifact that says whether a
browser-driven route event came from the same tab, another tab, or an external
navigation source.

## Why You Use It

- keep browser authority explicit at the route boundary
- detect route truth drift against an expected route witness
- preserve same-tab, cross-tab, and external-navigation meaning in history

## Stable Entry Points

- `signals.router.browserHistory.coherence.sameTab(...)`
- `signals.router.browserHistory.coherence.crossTab(...)`
- `signals.router.browserHistory.coherence.externalNavigation(...)`

## Core Mental Model

Coherence is not route outcome truth. It is boundary truth about where the
navigation authority came from and what route it expected to reproduce.

## How It Executes

1. build a coherence artifact
2. attach it to browser ingress or writeback
3. let the router classify converged, drifted, or not-admitted authority

## Small Example

```ts
const coherence = signals.router.browserHistory.coherence.sameTab({
  channelId: "workspace-main",
});
```

## Real Example

```ts
const coherence = signals.router.browserHistory.coherence.crossTab(
  "workspace-main",
  {
    sourceTabId: "tab-b",
    expectedRouteId: "home",
  },
);

const report = await routes.admitBrowserHistoryIngress(
  signals.router.browserHistory.external("/settings", {
    routeIdentity: "home",
    coherence,
  }),
);
```

## How It Relates To Other Features

- history inspection retains coherence later in
  [History Inspection](../history/history_inspection.md)
- worker-side parity is covered in
  [Host And Worker Boundary](../runtime_placement/host_worker_boundary.md)

## Inspection And Debugging

- `report.diagnostics().coherenceKind`
- `inspection.summary().sameTabCoherencePresent`
- `inspection.summary().crossTabCoherencePresent`
- `inspection.summary().externalNavigationCoherencePresent`

## Anti-Patterns

- inferring cross-tab drift from route ids alone
- treating coherence as a replacement for route admission

## Current Limits

- drift only makes sense when an explicit expected route witness exists

## Related Docs

- [Cross-Tab Navigation](./cross_tab_navigation.md)
- [External Navigation](./external_navigation.md)

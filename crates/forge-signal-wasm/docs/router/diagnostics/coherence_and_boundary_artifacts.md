# Coherence And Boundary Artifacts

## What This Feature Is

Coherence and boundary artifacts are the public vocabulary that explains what
happened at the browser or hydration boundary: whether route truth converged,
drifted, was not admitted, or escaped the router entirely.

## Why You Use It

- keep same-tab, cross-tab, and external authority explicit
- explain why a browser-history or hydration boundary produced the route truth
  it did
- debug drift and no-match cases without private bridge or host logic

## Stable Entry Points

- `signals.router.browserHistory.coherence.sameTab(...)`
- `signals.router.browserHistory.coherence.crossTab(...)`
- `signals.router.browserHistory.coherence.externalNavigation(...)`
- `report.diagnostics().coherenceKind`
- `report.diagnostics().boundaryArtifact`
- `story.latestBoundaryEvent()`
- `story.currentRouteTruthEvent()`
- `story.auditability().summary()`

## Core Mental Model

There are two separate questions here:

- **coherence kind**: where browser authority came from
  - `sameTab`
  - `crossTab`
  - `externalNavigation`
- **boundary artifact**: what happened when the router processed that boundary
  - browser-history ingress or local writeback:
    `routeTruthConverged`, `routeTruthDriftedFromAuthority`,
    `routeOutcomeNotAdmitted`
  - external writeback:
    `externalNavigationEscaped`
  - hydration handoff:
    `routeTruthMatchedServer`, `routeTruthDriftedFromServer`,
    `routeOutcomeNotAdmitted`

Coherence tells you who spoke. The boundary artifact tells you what happened.

## How It Executes

1. host code creates a coherence artifact when authority source matters
2. ingress, writeback, or hydration crosses the router boundary
3. the router classifies the boundary result with a public artifact
4. browser-history story retains that event
5. inspection and auditability summarize the retained boundary evidence

## Small Example

```ts
const coherence = signals.router.browserHistory.coherence.crossTab(
  "workspace-main",
  {
    sourceTabId: "tab-b",
    expectedRouteId: "home",
  },
);
```

This is the smallest honest example because the coherence artifact itself is
the point where you declare browser authority expectations.

## Real Example

```ts
const ingress = signals.router.browserHistory.external("/settings", {
  routeIdentity: "home",
  coherence: signals.router.browserHistory.coherence.crossTab(
    "workspace-main",
    {
      sourceTabId: "tab-b",
      expectedRouteId: "home",
    },
  ),
});

const report = await routes.admitBrowserHistoryIngress(ingress);
const story = signals.router.browserHistory.story(report);

console.log(report.diagnostics().coherenceKind);
console.log(report.diagnostics().boundaryArtifact);
console.log(story.auditability().summary());
```

If the actual route resolves to `settings`, the coherence kind is still
`crossTab`, but the boundary artifact becomes
`routeTruthDriftedFromAuthority`. The router keeps those meanings separate so a
drifted route does not look like a host failure.

## How It Relates To Other Features

- Read [Browser Authority Coherence](../boundaries/browser_authority_coherence.md)
  for the feature-level authoring guidance around coherence artifacts.
- Read [Hydration Handoff](../boundaries/hydration_handoff.md) for the server
  boundary lane and its distinct artifact vocabulary.
- Read [Diagnostics Surfaces](./diagnostics_surfaces.md) when you need to place
  these artifacts inside the larger explanation surfaces.

## Inspection And Debugging

- read `report.diagnostics()` first for one boundary event
- read `story.latestBoundaryEvent()` when the newest boundary crossing matters
- read `story.currentRouteTruthEvent()` when the question is "which boundary
  explains the current route truth?"
- read `story.inspection().summary()` or `story.auditability().summary()` for
  aggregate counts and current-route explanation

## Anti-Patterns

- inferring cross-tab drift from `routeIdentity` alone instead of declaring
  `expectedRouteId`
- treating `externalNavigationEscaped` as if it admitted route truth
- flattening hydration drift and browser-authority drift into one generic
  "mismatch" bucket

## Current Limits

- drift is only meaningful when an explicit expected-route witness exists
- worker-side mirrors retain the same coherence categories, but hydration
  boundary truth still lives on the host/runtime side

## Related Docs

- [Browser Authority Coherence](../boundaries/browser_authority_coherence.md)
- [Hydration Handoff](../boundaries/hydration_handoff.md)
- [History Inspection](../history/history_inspection.md)
- [Navigation Auditability](../history/navigation_auditability.md)

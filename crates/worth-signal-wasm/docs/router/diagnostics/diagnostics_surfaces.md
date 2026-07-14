# Diagnostics Surfaces

## What This Feature Is

Router diagnostics surfaces are the public read APIs that explain what the
router decided, why it decided it, and which retained navigation facts still
back that explanation.

## Why You Use It

- inspect why a route was admitted, denied, redirected, or recovered
- answer "what made this route visible?" without rebuilding the story in app
  code
- read retained browser-history, breadcrumb, restore, and coherence evidence
  from stable surfaces

## Stable Entry Points

- `outcome.diagnostics()`
- `outcome.provenance()`
- `story.inspection()`
- `story.auditability()`
- `boundaryReport.diagnostics()`

## Core Mental Model

The router has more than one explanation surface because not every question is
the same question:

- `outcome.diagnostics()` explains one admission result
- `outcome.provenance()` explains how the router got there
- `story.inspection()` inventories retained route-history truth
- `story.auditability()` answers why the current route is visible now
- `boundaryReport.diagnostics()` explains one host boundary crossing

Read the smallest surface that answers your question first. You do not need
`auditability()` every time.

## How It Executes

1. projection or admission decides a route outcome
2. ingress or writeback may turn that outcome into a boundary report
3. a browser-history story retains admitted route truth and boundary events
4. inspection derives a history-centered summary
5. auditability derives the final visible-route explanation surface

## Small Example

```ts
const outcome = await routes.admit("/projects/p7", facts);

console.log(outcome.diagnostics().outcomeKind);
console.log(outcome.provenance().terminalSource);
```

This is the smallest honest example because it stays at the route-outcome
layer. It does not pretend you need browser-history state just to explain one
admission decision.

## Real Example

```ts
const ingress = signals.router.browserHistory.external("/projects/p7", {
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
const inspection = story.inspection();
const auditability = story.auditability();

console.log(report.diagnostics());
console.log(inspection.summary());
console.log(auditability.summary());
```

Here the ingress report explains one browser-authority crossing, inspection
describes the retained story state, and auditability explains the current
visible route. The router keeps those layers separate so you do not have to
invent your own explanation model.

## How It Relates To Other Features

- Pair this with [History Inspection](../history/history_inspection.md) when
  the question is about retained entries, breadcrumbs, or restore availability.
- Pair this with [Navigation Auditability](../history/navigation_auditability.md)
  when the question is about current visible route truth.
- Pair this with [Coherence And Boundary Artifacts](./coherence_and_boundary_artifacts.md)
  when the explanation depends on same-tab, cross-tab, external, or hydration
  boundary posture.

## Inspection And Debugging

- use `outcome.diagnostics()` for one outcome-shaped answer
- use `outcome.provenance()` when prerequisite or recovery trail matters
- use `story.inspection().summary()` for counts and availability posture
- use `story.auditability().summary()` for the final current-route explanation
- use `verification()` only when you need proof packages, not ordinary logging

## Anti-Patterns

- reading `story.events()` and rebuilding your own summary counts
- using `auditability()` as a substitute for ordinary route-outcome diagnostics
- treating `diagnostics()` and `provenance()` as interchangeable

## Current Limits

- diagnostics surfaces explain router truth; they do not replace app-specific
  analytics or logging
- worker-side auditability only sees the retained browser-history lane there,
  not hydration truth

## Related Docs

- [History Inspection](../history/history_inspection.md)
- [Navigation Auditability](../history/navigation_auditability.md)
- [Verification Packages](./verification_packages.md)
- [Coherence And Boundary Artifacts](./coherence_and_boundary_artifacts.md)

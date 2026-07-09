# Browser History Ingress

## What This Feature Is

Browser-history ingress is the explicit host envelope for URL changes that come
from browser or user navigation rather than graph-issued writeback.

## Why You Use It

- admit browser-driven location changes through one typed boundary
- preserve navigation kind, coherence, restore, and breadcrumb carry at ingress
- avoid ambient URL reads in product code

## Stable Entry Points

- `signals.router.browserHistory.load(...)`
- `signals.router.browserHistory.push(...)`
- `signals.router.browserHistory.replace(...)`
- `signals.router.browserHistory.pop(...)`
- `signals.router.browserHistory.manual(...)`
- `signals.router.browserHistory.external(...)`
- `routes.admitBrowserHistoryIngress(...)`

## Core Mental Model

Ingress is a boundary envelope. It is not itself route truth. The route outcome
is derived from the ingress report.

## How It Executes

1. create a browser-history ingress envelope
2. admit it through the resolved tree
3. inspect the resulting boundary report
4. optionally record the report in a browser-history story

## Small Example

```ts
const ingress = signals.router.browserHistory.pop("/projects/p7", {
  routeIdentity: "project",
});

const report = await routes.admitBrowserHistoryIngress(ingress);
```

## Real Example

```ts
const coherence = signals.router.browserHistory.coherence.crossTab("workspace-main", {
  sourceTabId: "tab-b",
  expectedRouteId: "home",
});

const ingress = signals.router.browserHistory.external("/projects/p7", {
  routeIdentity: "home",
  coherence,
});

const report = await routes.admitBrowserHistoryIngress(ingress);

console.log(report.outcome().kind);
console.log(report.diagnostics().boundaryArtifact);
```

## How It Relates To Other Features

- use [Browser History Story](./browser_history_story.md) to retain reports over
  time
- use [Navigation Auditability](./navigation_auditability.md) for the final
  "why is this route visible?" explanation surface

## Inspection And Debugging

- `report.diagnostics()`
- `report.coherence()`
- `report.restoreBoundary()`
- `report.verification()`

## Anti-Patterns

- reading browser location directly in app code and pretending the router saw
  it
- treating ingress envelopes as admitted route truth before admission runs

## Current Limits

- ingress explains boundary truth; retained history is a separate story layer

## Related Docs

- [Browser History Story](./browser_history_story.md)
- [Navigation Auditability](./navigation_auditability.md)

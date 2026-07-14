# Worker History Fallback

## What This Feature Is

Worker history fallback is the explicit route-history and breadcrumb surface
the worker bridge uses when it does not have richer declared route capability.

## Why You Use It

- keep worker-side route history usable on thinner reports
- understand why breadcrumb trails may come back as history-fallback entries
- preserve replay-aware route history even when richer route composition is not
  available

## Stable Entry Points

- `WorkerBrowserHistoryStory`
- `story.breadcrumbTrail()`
- `story.inspection().summary().historyFallbackBreadcrumbPresent`

## Core Mental Model

Fallback does not mean the worker bridge is broken. It means the bridge is
being honest about the information it actually has.

## How It Executes

1. worker boundary reports arrive without richer declared crumb capability
2. worker story derives route-history entries and fallback breadcrumbs
3. inspection and auditability summarize that reduced truth explicitly

## Small Example

```ts
const trail = story.breadcrumbTrail();
console.log(trail.entries[0].sourceKind);
```

## Real Example

```ts
console.log(
  story.breadcrumbTrail().entries.map((entry) => ({
    crumbId: entry.crumbId,
    sourceKind: entry.sourceKind,
    replayAvailability: entry.provenance().replayAvailability,
  })),
);
```

## How It Relates To Other Features

- main-runtime fallback crumb docs are in
  [History Fallback Crumbs](../breadcrumbs/history_fallback_crumbs.md)
- worker auditability is in [Worker Navigation Auditability](./worker_navigation_auditability.md)

## Inspection And Debugging

- `entry.sourceKind`
- `entry.provenance().replayAvailability`
- `story.inspection().summary()`

## Anti-Patterns

- assuming worker fallback crumbs are route declarations
- hiding fallback status from support or tooling surfaces

## Current Limits

- fallback trails are intentionally weaker than declared breadcrumb trails

## Related Docs

- [Host And Worker Boundary](./host_worker_boundary.md)
- [History Fallback Crumbs](../breadcrumbs/history_fallback_crumbs.md)

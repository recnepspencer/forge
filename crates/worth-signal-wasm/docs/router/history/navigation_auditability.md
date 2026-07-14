# Navigation Auditability

## What This Feature Is

Navigation auditability is the route-first explanation surface that answers why
the current route is visible now and which hydration, history, restore, or
coherence facts certify that claim.

## Why You Use It

- get one final explanation surface for visible route truth
- close the gap between hydration truth and retained browser-history truth
- inspect the full navigation explanation story without private reconstruction

## Stable Entry Points

- `story.auditability()`
- `story.auditability(hydrationReport)`
- `auditability.summary()`
- `auditability.hydrationBoundary()`
- `auditability.historyInspection()`
- `auditability.currentRestoreBoundary()`

## Core Mental Model

`inspection()` tells you what retained history exists. `auditability()` tells
you why the current route is visible and which boundary or restore authority
explains it.

## How It Executes

1. start from browser-history inspection
2. optionally add hydration boundary truth
3. derive one route-first explanation summary
4. expose verification for hostile proof and inspection

## Small Example

```ts
const auditability = story.auditability();

console.log(auditability.summary().currentVisibleRouteSource);
```

## Real Example

```ts
const hydrationReport = await routes.admitHydrationHandoff(
  signals.router.hydration.server("/projects/p7", {
    serverRouteIdentity: "home",
    serverHref: "/",
  }),
);

const auditability = story.auditability(hydrationReport);

console.log(auditability.summary());
console.log(auditability.hydrationBoundary()?.diagnostics());
```

In this example the visible route may be explained by hydration alone, or by a
later retained route-history entry if one exists. The summary tells you which
lane won honestly.

## How It Relates To Other Features

- use [History Inspection](./history_inspection.md) when you want the
  history-centered aggregate surface
- use [Browser History Ingress](./browser_history_ingress.md) and
  [Browser History Writeback](./browser_history_writeback.md) for the boundary
  reports that feed the story

## Inspection And Debugging

- `summary()`
- `hydrationBoundary()`
- `historyInspection()`
- `currentRestoreBoundary()`
- `verification()`

## Anti-Patterns

- rebuilding visible-route explanation from raw story fields
- treating hydration and browser-history truth as one undifferentiated source

## Current Limits

- worker-side auditability currently has no hydration boundary lane, because the
  worker mirror only sees retained browser-history truth there

## Related Docs

- [History Inspection](./history_inspection.md)
- [Browser History Story](./browser_history_story.md)

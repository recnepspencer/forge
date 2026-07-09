# Browser History Story

## What This Feature Is

A browser-history story is the retained router-owned history of admitted
browser-boundary reports.

## Why You Use It

- keep one retained route-history story from ingress and writeback reports
- derive back provenance, breadcrumb trails, inspection, and auditability from
  one canonical artifact stream

## Stable Entry Points

- `signals.router.browserHistory.story(...)`
- `story.record(...)`
- `story.current()`
- `story.latestBoundaryEvent()`
- `story.currentRouteTruthEvent()`
- `story.backProvenance()`
- `story.breadcrumbTrail()`
- `story.inspection()`
- `story.auditability(...)`

## Core Mental Model

The story is the retained history authority for browser-boundary route truth.
It consumes explicit reports and derives richer inspection surfaces from them.

## How It Executes

1. create a story
2. record ingress or writeback reports
3. derive retained entries and boundary events
4. read back, breadcrumb, inspection, or auditability surfaces

## Small Example

```ts
const story = signals.router.browserHistory.story();

story.record(await routes.admitBrowserHistoryIngress(ingress));
console.log(story.current()?.routeId);
```

## Real Example

```ts
const story = signals.router.browserHistory.story();

story.record(await routes.admitBrowserHistoryIngress(loadReport));
story.record(await routes.applyBrowserHistoryWriteback(writebackReport));

console.log(story.currentRouteTruthEvent()?.boundaryArtifact);
console.log(story.auditability().summary());
```

## How It Relates To Other Features

- use [Route History Entries](./route_history_entries.md) for each retained
  admitted entry
- use [History Inspection](./history_inspection.md) and
  [Navigation Auditability](./navigation_auditability.md) for explanation
  surfaces

## Inspection And Debugging

- `events()`
- `admittedEntries()`
- `verification()`

## Anti-Patterns

- inventing your own retained route-history list beside the story
- recording only current hrefs and losing boundary provenance

## Current Limits

- the story only knows what you record through explicit boundary reports

## Related Docs

- [Route History Entries](./route_history_entries.md)
- [History Inspection](./history_inspection.md)
- [Navigation Auditability](./navigation_auditability.md)

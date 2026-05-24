# Route History Entries

## What This Feature Is

A route-history entry is one retained admitted route truth entry inside a
browser-history story.

## Why You Use It

- restore or replay earlier admitted route truth
- inspect the retained route, href, outlet composition, and breadcrumb trail

## Stable Entry Points

- `story.current()`
- `story.back()`
- `entry.previous()`
- `entry.restoreBoundary()`
- `entry.restore(...)`
- `entry.replay(...)`
- `entry.breadcrumbTrail()`
- `entry.outletComposition()`

## Core Mental Model

A route-history entry is retained admitted truth, not a raw boundary report. It
exists only when a boundary report advanced admitted route truth.

## How It Executes

1. story records a boundary report
2. if the report advanced route truth, the story creates an entry
3. the entry can then explain or restore that committed route truth later

## Small Example

```ts
const current = story.current();

console.log(current?.routeId);
console.log(current?.href);
```

## Real Example

```ts
const previous = story.back();

if (previous) {
  console.log(previous.breadcrumbTrail());
  console.log(previous.outletComposition());
}
```

## How It Relates To Other Features

- use [Back Provenance](./back_provenance.md) when you want the previous entry
  wrapped as a dedicated back artifact
- use [Browser History Story](./browser_history_story.md) for the containing
  retained surface

## Inspection And Debugging

- `routeId`
- `href`
- `boundaryArtifact`
- `coherenceKind`
- `verification()`

## Anti-Patterns

- treating non-admitted boundary events as route-history entries
- rebuilding restore/replay authority outside the entry artifact

## Current Limits

- only admitted route truth becomes an entry

## Related Docs

- [Back Provenance](./back_provenance.md)
- [History Inspection](./history_inspection.md)

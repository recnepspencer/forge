# Breadcrumb Replay And Restore

## What This Feature Is

This feature is the restore- and replay-capable breadcrumb lane exposed by
browser-history breadcrumb entries.

## Why You Use It

- return through breadcrumb UI with exact restore boundaries
- replay route and continuity truth from breadcrumb entries
- keep breadcrumb return as strong as back navigation

## Stable Entry Points

- `story.breadcrumbTrail()`
- `entry.restoreBoundary()`
- `entry.restore(history)`
- `entry.replay(history)`

## Core Mental Model

Not every breadcrumb entry can restore or replay. History-backed breadcrumb
entries can. Plain route breadcrumb entries may only expose provenance.

The important rule is parity: breadcrumb return should use the same authority
classes as back provenance when that authority exists.

## How It Executes

1. record restore-backed browser-history entries
2. materialize breadcrumb entries from them
3. surface restore and replay availability on provenance
4. allow the breadcrumb entry to call restore or replay directly

## Small Example

```ts
const entry = story.breadcrumbTrail().entries[0];

if (entry.restoreBoundary()) {
  await entry.restore(signals.history());
}
```

## Real Example

```ts
const entry = story.breadcrumbTrail().entries[0];
const restoreResult = await entry.restore(signals.history());
const replayResult = await entry.replay(signals.history());

console.log(restoreResult.routeId);
console.log(replayResult.routeReplay);
console.log(replayResult.continuityReplay);
```

## How It Relates To Other Features

- restore authority comes from [Restore Boundaries](../restore/restore_boundaries.md)
- restored breadcrumbs are declared in [Restored Provenance](./restored_provenance.md)
- story-level inspection is covered in [History Inspection](../history/history_inspection.md)

## Inspection And Debugging

- `entry.restoreBoundary()`
- `entry.provenance().restoreAvailability`
- `entry.provenance().replayAvailability`
- `entry.restore(...)`
- `entry.replay(...)`

## Anti-Patterns

- assuming every crumb can restore
- rebuilding breadcrumb return from `targetHref` when restore authority exists
- ignoring replay availability when debugging continuity

## Current Limits

- restore and replay helpers live on browser-history breadcrumb entries
- plain breadcrumb artifacts outside history story do not automatically expose
  the stronger history-boundary helpers

## Related Docs

- [Restored Provenance](./restored_provenance.md)
- [Restore Boundaries](../restore/restore_boundaries.md)
- [Replay History](../restore/replay_history.md)

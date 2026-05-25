# Replay History

## What This Feature Is

Replay history is the route-facing contract for replaying stored runtime route
and continuity sources from route-history or breadcrumb entries.

## Why You Use It

- inspect the recorded route source behind a history or breadcrumb return
- prove parity between current, back, and breadcrumb replay
- debug continuity-specific navigation behavior

## Stable Entry Points

- `entry.replay(history)`
- `backProvenance.replay(history)`
- `story.current()?.replay(history)`

## Core Mental Model

Replay is not restore. Restore re-enters exact snapshot truth. Replay asks the
history surface for the recorded route and continuity summaries behind that
entry.

## How It Executes

1. preserve runtime route and continuity source ids on boundary reports
2. attach them to history or breadcrumb entries
3. ask the replay history facade for summaries by id
4. return a typed replay result

## Small Example

```ts
const current = story.current();

if (current) {
  const replay = await current.replay(signals.history());
  console.log(replay.routeReplay);
}
```

## Real Example

```ts
const current = story.current();
const currentReplay = current
  ? await current.replay(signals.history())
  : null;
const backReplay = await story.backProvenance().replay(signals.history());
const breadcrumbReplay = await story
  .breadcrumbTrail()
  .entries[0]
  .replay(signals.history());
```

## How It Relates To Other Features

- restore is the stronger return lane: [Restore Boundaries](./restore_boundaries.md)
- breadcrumb replay uses the same replay facade:
  [Breadcrumb Replay And Restore](../breadcrumbs/breadcrumb_replay_restore.md)

## Inspection And Debugging

- `replay.routeReplay`
- `replay.continuityReplay`
- `replay.replayedEntryDigest`

## Anti-Patterns

- calling replay when you actually need exact restore
- assuming replay exists when runtime source ids were never recorded

## Current Limits

- replay depends on a history facade that exposes `replay_for(id)`
- replay is an inspection lane, not a substitute for restore

## Related Docs

- [Replay Results](./replay_results.md)
- [Restore Boundaries](./restore_boundaries.md)

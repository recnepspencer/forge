# Replay Results

## What This Feature Is

Replay results are the typed artifacts returned when route-history or
breadcrumb entries replay their recorded route and continuity sources.

## Why You Use It

- inspect replayed route truth without reverse-engineering ids
- compare route replay and continuity replay side by side
- retain source kind on replayed evidence

## Stable Entry Points

- `RouteHistoryReplayResult`
- `entry.replay(history)`

## Core Mental Model

Replay results are one-step evidence objects. They tell you which entry
replayed, what route it belonged to, and which replay summaries came back.

## How It Executes

1. entry or provenance calls `replay(history)`
2. the router resolves route and continuity ids
3. the history facade returns `ReplaySummary` values
4. the router wraps them in a replay result

## Small Example

```ts
const replay = await story.backProvenance().replay(signals.history());
console.log(replay.replaySourceKind);
```

## Real Example

```ts
const replay = await story
  .breadcrumbTrail()
  .entries[0]
  .replay(signals.history());

console.log({
  routeId: replay.routeId,
  href: replay.href,
  routeReplay: replay.routeReplay,
  continuityReplay: replay.continuityReplay,
});
```

## How It Relates To Other Features

- replay inputs come from [Replay History](./replay_history.md)
- restore results are separate and stronger than replay results

## Inspection And Debugging

- `replay.replaySourceKind`
- `replay.routeReplay`
- `replay.continuityReplay`
- `replay.verification()`

## Anti-Patterns

- collapsing route and continuity replay into one untyped log line
- using replay results to claim restore happened

## Current Limits

- replay results only expose summaries returned by the history facade

## Related Docs

- [Replay History](./replay_history.md)
- [Restore Boundaries](./restore_boundaries.md)

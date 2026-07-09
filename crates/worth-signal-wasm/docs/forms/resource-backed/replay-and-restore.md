# Replay And Restore

## What This Feature Is

This page covers exact replay and exact restore for resource-backed forms.

## Why You Use It

- replay or restore the bound resource source exactly when history allows it
- inspect whether exact history is unavailable
- keep replay/restore history on the controller surface

## Stable Entry Points

- `form.replayExactResourceSource(...)`
- `form.restoreExactResourceSource(...)`
- `form.replayRestoreHistory()`

## Core Mental Model

Replay and restore are exact-history moves. They are different from local reset
and different from ordinary refresh/revalidate behavior.

## How It Executes

1. replay or restore is requested
2. the runtime checks exact history availability
3. a replay/restore artifact records the result and before/after digests

## Small Example

```ts
const replay = await form.replayExactResourceSource({ reason: "debug exact state" });
console.log(replay.resultKind);
```

## Real Example

```ts
const restore = await form.restoreExactResourceSource({ reason: "return to snapshot" });

console.log(restore.resultKind);
console.log(restore.resourceReplayRestore);
console.log(form.replayRestoreHistory());
```

## How It Relates To Other Features

- Read [Resource Reset](./resource-reset.md) when the goal is rollback/reset
  rather than exact history movement.
- Read [Resource Shape And Visible Selection](./resource-shape-and-visible-selection.md)
  when replay or restore changes the visible source selection.

## Inspection And Debugging

- `resultKind` is the main read
- `resourceReplayRestore` explains exact-history availability and mode
- history shows replay vs restore over time

## Anti-Patterns

- treating replay/restore as if they were just refresh
- assuming exact history is always available

## Current Limits

- route-level replay/restore behavior belongs in router docs

## Related Docs

- [Resource Reset](./resource-reset.md)
- [Resource Shape And Visible Selection](./resource-shape-and-visible-selection.md)

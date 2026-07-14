# Check Status, Freshness, And History

## What This Feature Is

This page is about the read surfaces that explain whether a line is pending,
fulfilled, rejected, timed out, stale, or exactly recoverable.

## Why You Use It

Use it when you need to answer:

- is the current visible value still usable?
- is the line waiting, fulfilled, rejected, or timed out?
- should I retry, refresh, or revalidate?
- is exact replay or exact restore still available?

## Stable Entry Points

- `line.status()`
- `line.freshness()`
- `line.history()`
- `line.history().availability`
- `line.history().lifecycle`
- `line.history().replayExact()`
- `line.history().restoreExact()`

## Core Mental Model

`status` tells you the line's current operation posture.
`freshness` tells you whether the visible value is current, stale, or pending.
`history()` tells you what exact retained operations are still available.

These are adjacent reads, but they are not the same thing.

This page intentionally does not claim a separate resource-line `settlement`
artifact. That richer settlement summary exists on the resource-backed forms
bridge, not on the raw line surface.

## How It Executes

The history surface keeps:

- lifecycle entries such as `pending`, `patched`, `delivered`, `fulfilled`,
  `rejected`, `timedOut`, `restored`, and `replayed`
- typed availability for replay, lineage, branch, and exact restore
- exact replay and exact restore results when the runtime still retains them

## Small Example

```ts
const line = taskDetail.line({ taskId: "t1" });

console.log(line.status());
console.log(line.freshness());
console.log(line.history().availability);
```

## Real Example

```ts
const line = taskDetail.line({ taskId: "t1" });
const history = line.history();

if (history.availability.restoreExact.kind === "available") {
  const restored = history.restoreExact();
  console.log(restored.kind);
}

console.log(history.lifecycle.at(-1)?.event);
console.log(history.availability.replayExact.kind);
```

## How It Relates To Other Features

- Use [Inspect A Resource Line](./inspect-a-resource-line.md) for the grouped
  first read.
- Use [Restore, Replay, And Recover](./restore-replay-and-recover.md) when
  recovery is the main job.
- Use [Caching And Refresh](../caching/README.md) when the question is stale
  versus fresh behavior over time.

## Inspection And Debugging

If the line is not doing what you expect:

1. check `line.status()`
2. check `line.freshness()`
3. inspect `line.history().availability`
4. inspect the latest lifecycle entry

## Anti-Patterns

- Do not collapse `status` and `freshness` into one boolean.
- Do not promise exact restore unless `history().availability.restoreExact`
  actually says `available`.
- Do not assume replay and restore fail for the same reason. They are typed
  separately.

## Current Limits

Exact replay and exact restore depend on runtime retention. The denied result is
still useful because it tells you why the exact path is unavailable.

## Related Docs

- [Restore, Replay, And Recover](./restore-replay-and-recover.md)
- [History And Restore](../../resource-contracts/history-and-restore.md)

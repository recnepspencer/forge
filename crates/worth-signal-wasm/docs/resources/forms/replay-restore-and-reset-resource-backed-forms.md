# Replay, Restore, And Reset Resource-Backed Forms

## What This Feature Is

This page covers the form-side helpers that use retained resource history while
preserving local draft truth.

## Why You Use It

Use it when you need to:

- replay exact resource source truth into the form
- restore exact branch-backed resource truth after a speculative write
- keep local draft truth explicit while resource source truth changes under it

## Stable Entry Points

- `form.replayExactResourceSource()`
- `form.restoreExactResourceSource()`
- `form.replayRestoreHistory()`

## Core Mental Model

These helpers do not erase the form draft by default. They update the resource
source lane while preserving draft truth unless the specific operation says
otherwise.

## How It Executes

Replay and restore return typed results and append to replay/restore history.
The form verification package also keeps digests for that retained history.

## Small Example

```ts
const replay = form.replayExactResourceSource();
console.log(replay.resultKind);
```

## Real Example

```ts
const restore = form.restoreExactResourceSource();

console.log(restore.resultKind);
console.log(form.resourceSource()?.visibleSelection.kind);
console.log(form.replayRestoreHistory().length);
```

## How It Relates To Other Features

- Use [Restore, Replay, And Recover](../debugging/restore-replay-and-recover.md)
  for the raw resource-line history lane.
- Use [Replay And Restore](../../forms/resource-backed/replay-and-restore.md)
  for the deeper form-side guide.

## Inspection And Debugging

Inspect:

- replay/restore result kind
- visible selection after restore
- replay/restore history length
- verification replay/restore digests

## Anti-Patterns

- Do not assume replay or restore means "reset the draft."
- Do not hide unavailable replay/restore results. They explain retained-history
  limits honestly.

## Current Limits

Exact replay and exact restore still depend on retained resource history. The
form helper does not invent history the resource line no longer has.

## Related Docs

- [Replay And Restore](../../forms/resource-backed/replay-and-restore.md)
- [History And Restore](../../resource-contracts/history-and-restore.md)

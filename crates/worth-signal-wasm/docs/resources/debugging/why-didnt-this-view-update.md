# Why Didn't This View Update?

## What This Feature Is

This page explains how to debug the opposite problem: the line stayed stale,
kept the old visible value, or could not admit the exact recovery path you
expected.

## Why You Use It

Use it when you need to answer:

- why is the old value still visible?
- why did refresh not replace the visible line yet?
- why can I not replay or restore exact truth?
- why did speculative branch proof stay unavailable?

## Stable Entry Points

- `line.status()`
- `line.freshness()`
- `line.summary().current`
- `line.history().availability`
- `line.history().verificationPackage().continuity`
- `line.history().verificationPackage().typedDenials`

## Core Mental Model

No visible change can still be honest.

Sometimes the runtime is preserving visible truth on purpose because the new
operation is pending, rejected, or denied. The correct question is not "why did
the runtime fail to update?" but "what continuity or denial did it admit?"

## How It Executes

The line keeps continuity and typed-denial reads that explain why an exact path
was unavailable or why visible truth stayed preserved.

## Small Example

```ts
console.log(line.status());
console.log(line.freshness());
console.log(line.history().availability.restoreExact);
```

## Real Example

```ts
const verification = line.history().verificationPackage();

console.log(line.summary().current.status);
console.log(verification.continuity.visibleSelection.kind);
console.log(verification.typedDenials.restoreExact);
console.log(verification.typedDenials.replayExact);
```

## How It Relates To Other Features

- Use [Caching And Refresh](../caching/README.md) for stale and revalidate
  behavior.
- Use [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
  when you need the full history availability lane.
- Use [Effects](../effects/README.md) when the missing update is really about
  speculative branch denial or unavailable rollback.

## Inspection And Debugging

Look at:

1. `line.status()`
2. `line.freshness()`
3. `line.history().availability`
4. `line.history().verificationPackage().typedDenials`

## Anti-Patterns

- Do not assume "no change" means "no event."
- Do not flatten typed denials into one generic error string.
- Do not promise exact replay or restore in UI copy unless those reads actually
  say `available`.

## Current Limits

Some exact operations become unavailable when retained runtime history has been
evicted or when identity migration makes an exact path impossible.

## Related Docs

- [Caching And Refresh](../caching/README.md)
- [Restore, Replay, And Recover](./restore-replay-and-recover.md)

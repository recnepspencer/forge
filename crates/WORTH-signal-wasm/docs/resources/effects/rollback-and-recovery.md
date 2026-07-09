# Rollback And Recovery

## What This Feature Is

This page explains what kind of recovery a resource effect can honestly claim.

## Why You Use It

Use it when you need to answer:

- can I roll back the latest effect exactly?
- is compact inverse the real fallback?
- is this write committed-only and therefore not rollback-capable?

## Stable Entry Points

- `line.diagnostics().lastEffect.optimistic.rollback`
- `line.history().rollbackLastEffect()`
- `line.history().restoreExact()`

## Core Mental Model

Rollback is effect-owned.
Exact restore is history-owned.

Sometimes they line up. Sometimes the effect only has compact inverse. Sometimes
there is no honest rollback at all.

## How It Executes

The runtime can report:

- `exactBranchRestoreAvailable`
- `compactInverseAvailable`
- `unavailable`
- `notApplicable`

## Small Example

```ts
const rollback = line.diagnostics().lastEffect?.optimistic.rollback;
console.log(rollback?.kind);
```

## Real Example

```ts
const effect = line.diagnostics().lastEffect;
const rollbackResult = line.history().rollbackLastEffect();

console.log(effect.optimistic.rollback.kind);
console.log(rollbackResult.kind);
console.log(line.history().availability.restoreExact.kind);
```

## How It Relates To Other Features

- Use [Restore, Replay, And Recover](../debugging/restore-replay-and-recover.md)
  for the broader retained-history lane.
- Use [Branch-Native Effects](./branch-native-effects.md) for profile-level
  optimistic behavior.

## Inspection And Debugging

Inspect both:

- the claimed rollback posture on the effect
- the actual rollback result from history

## Anti-Patterns

- Do not call compact inverse "exact restore."
- Do not promise rollback for `pessimistic()` or other committed-only cases.

## Current Limits

Broad replacements and missing retained branch proof can make exact rollback
unavailable even when the line still has meaningful diagnostics.

## Related Docs

- [History And Restore](../../resource-contracts/history-and-restore.md)
- [Restore, Replay, And Recover](../debugging/restore-replay-and-recover.md)

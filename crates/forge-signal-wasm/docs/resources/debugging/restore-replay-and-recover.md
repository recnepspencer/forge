# Restore, Replay, And Recover

## What This Feature Is

This page is the recovery-focused view of retained history.

## Why You Use It

Use it when you need:

- exact replay of a line value
- exact restore of branch-backed visible truth
- rollback of the latest resource effect
- typed reasons when those paths are unavailable

## Stable Entry Points

- `line.history().availability`
- `line.history().replayExact()`
- `line.history().restoreExact()`
- `line.history().rollbackLastEffect()`
- `line.history().verificationPackage()`

## Core Mental Model

Replay, restore, and rollback are related, but they are not interchangeable.

- replay replays exact line truth
- restore restores exact branch-backed line truth
- rollback rolls back the latest effect when the effect profile admitted it

## How It Executes

The history surface exposes availability before execution, then returns a typed
result when you call replay, restore, or rollback.

## Small Example

```ts
const history = line.history();

if (history.availability.replayExact.kind === "available") {
  console.log(history.replayExact());
}
```

## Real Example

```ts
const history = line.history();

console.log(history.availability.restoreExact);
console.log(history.availability.replayExact);

const rollback = history.rollbackLastEffect();
console.log(rollback.kind);

const verification = history.verificationPackage();
console.log(verification.historyReplayRestore.lastLifecycleEvent);
```

## How It Relates To Other Features

- Use [Effects](../effects/README.md) when rollback depends on effect-profile
  posture.
- Use [Check Status, Freshness, And History](./check-status-settlement-and-history.md)
  for the broader line-history read.
- Use [History And Restore](../../resource-contracts/history-and-restore.md)
  for the deeper retained-history contract.

## Inspection And Debugging

Always inspect `history().availability` first. It tells you whether the exact
path is actually admitted before you try to run it.

## Anti-Patterns

- Do not assume `rollbackLastEffect()` is the same as exact restore.
- Do not hide unavailable recovery results. They carry the reason you need for
  honest UI and logs.

## Current Limits

Exact replay and restore depend on retained runtime history. Rollback depends on
the effect profile and the proof the runtime captured for that effect.

## Related Docs

- [History And Restore](../../resource-contracts/history-and-restore.md)
- [Branch-Native Effects](../effects/branch-native-effects.md)

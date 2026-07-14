# Resource Reset

## What This Feature Is

This page covers controller reset and resource rollback for resource-backed
forms.

## Why You Use It

- reset local form state or roll back the last resource effect
- inspect whether a rollback actually happened or was unavailable
- keep reset and rollback history explicit

## Stable Entry Points

- `form.reset(...)`
- `form.rollbackLastResourceEffect(...)`
- `form.resetHistory()`

## Core Mental Model

Reset and rollback are not the same thing. A reset can accept canonical state
locally, while a resource rollback can move the backing line itself back to an
earlier effect state.

## How It Executes

1. a reset or rollback is requested
2. the runtime records the previous and next digests
3. resource rollback details are attached when relevant

## Small Example

```ts
const reset = form.reset({ reason: "discard draft" });
console.log(reset.resultKind);
```

## Real Example

```ts
const rollback = await form.rollbackLastResourceEffect({ reason: "undo effect" });

console.log(rollback.resultKind);
console.log(rollback.resourceRollback);
console.log(form.resetHistory());
```

## How It Relates To Other Features

- Read [Replay And Restore](./replay-and-restore.md) when the goal is exact
  replay or restore rather than rollback.
- Read [Resource Settlement](./resource-settlement.md) when retry and failure
  posture drive the reset decision.

## Inspection And Debugging

- `resultKind` is the first read
- `resourceRollback` explains exact rollback details or unavailability
- `resetHistory()` shows the retained reset/rollback trail

## Anti-Patterns

- treating every reset as if it also rolled back the resource line
- hiding rollback unavailability from the user

## Current Limits

- route-coupled discard semantics live in later docs

## Related Docs

- [Replay And Restore](./replay-and-restore.md)
- [Resource Settlement](./resource-settlement.md)

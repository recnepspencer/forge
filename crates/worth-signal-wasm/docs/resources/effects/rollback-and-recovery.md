# Rollback And Recovery

## What This Feature Is

Effect rollback rejects one open optimistic request by identity. It retires that
effect branch and rebuilds the visible projection. It does not restore a shared
snapshot and cannot erase successful siblings.

Historical restore is a different feature: it moves a line to an explicit
retained history target.

## Why You Use It

- report one failed server request without disturbing concurrent work;
- cancel a parent and its dependent descendants;
- expose a deterministic "reject latest open effect" convenience action;
- keep explicit time-travel restore separate from request failure.

## Stable Entry Points

- `line.effects().reject(effectId, options)`
- `line.history().rollbackEffect(effectId)`
- `line.history().rollbackLastEffect()`
- `line.history().restoreExact()`

## Core Mental Model

An open effect owns a branch. Rejecting it retires that branch. Canonical server
truth does not move; only the projection changes.

`rollbackLastEffect()` finds the last open effect in stable admission order and
lowers to the same targeted rejection path. It is convenience, not shared
snapshot rollback.

`restoreExact()` restores an explicit retained historical target. Use it for
time travel or recovery, never as the failure handler for one concurrent request.

## How It Executes

Targeted rejection returns `rejectedAndRetired` with canonical value,
projection, closeout, retirement, and any dependency-cancelled descendants.
Unavailable requests return a typed result with `noOpenEffect`,
`unknownEffect`, or `effectAlreadySettled`.

## Small Example

```ts
const result = await line.history().rollbackEffect(effectId);

if (result.kind === "rejectedAndRetired") {
  console.log(result.projection.projectedValue);
}
```

## Real Example

```ts
const failed = await line.effects().reject(parentEffectId, {
  responseId: response.id,
});

for (const retired of failed.retired ?? []) {
  cancelTransportFor(retired.effectId);
}

console.log(failed.canonicalValue); // unchanged by rejection
console.log(failed.projection);     // rebuilt from remaining open effects
```

## How It Relates To Other Features

- [Concurrent Optimistic Effects](./concurrency-and-dependencies.md) explains
  sibling and parent/child settlement.
- [History And Restore](../../resource-contracts/history-and-restore.md) covers
  explicit restore and replay.
- Resource-backed form rollback actions lower through the same targeted effect
  rejection path.

## Inspection And Debugging

Inspect the target with `line.effects().get(effectId)` before closeout. Inspect
the returned projection, retirement, and `retired` descendants afterward.
If closeout is interrupted, retry `confirm(...)` or `reject(...)` with the same
`responseId`; the runtime resumes any recorded native-closeout checkpoint
without duplicating branch retirement or canonical commit.

## Anti-Patterns

- Do not call `restoreExact()` because one request failed.
- Do not apply a delete or inverse patch after `reject(...)`.
- Do not infer the target from the current visible value.
- Do not call compact inverse rollback "exact restore."

## Current Limits

Only open effects can be rejected. Explicit replay and restore availability
depends on the retained history posture of the runtime.

## Related Docs

- [Branch-Native Effects](./branch-native-effects.md)
- [Concurrent Optimistic Effects](./concurrency-and-dependencies.md)
- [History And Restore](../../resource-contracts/history-and-restore.md)

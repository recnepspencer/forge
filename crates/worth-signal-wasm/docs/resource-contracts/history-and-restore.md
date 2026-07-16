# History And Restore

## What This Feature Is

Resource history records what a line retained across loads, deliveries,
effects, closeout, branches, and explicit restore. It exposes two different
recovery jobs: reject one open request effect, or restore an explicit historical
target.

## Why You Use It

- inspect lifecycle, basis, branch, replay, and verification evidence;
- reject a known open effect without touching siblings;
- restore a retained historical target for deliberate recovery or time travel;
- receive typed unavailability instead of guessing what the runtime retained.

## Stable Entry Points

- `line.history().availability`
- `line.history().lifecycle`
- `line.history().basis`
- `line.history().verificationPackage()`
- `line.history().rollbackEffect(effectId)`
- `line.history().rollbackLastEffect()`
- `line.history().restoreExact()`
- `line.history().replayExact()`

## Core Mental Model

Effect rejection and historical restore have different targets and different
result vocabularies.

- `rollbackEffect(effectId)` rejects one open effect branch. Canonical server
  truth stays in place and the optimistic projection rebuilds.
- `rollbackLastEffect()` deterministically finds the latest open effect and
  lowers to `rollbackEffect(...)`.
- `restoreExact()` moves the line to an explicit retained history target. It is
  not a request failure mechanism.

No effect rollback path restores a shared line snapshot.

## How It Executes

Targeted rollback returns the same settlement results as
`line.effects().reject(...)`, including `rejectedAndRetired`, projection,
retirement, and dependent closeout. It can instead return `unavailable` with
`noOpenEffect`, `unknownEffect`, or `effectAlreadySettled`.

Exact restore returns `restored` with its branch and snapshot identity, or a
typed `unavailable` reason. Exact replay remains typed by runtime support.

## Small Example

```ts
const open = line.effects().open();
const result = open.length === 0
  ? await line.history().rollbackLastEffect()
  : await line.history().rollbackEffect(open[0].effectId);

console.log(result.kind);
```

## Real Example

```ts
const history = line.history();
const before = history.verificationPackage();

const rejected = await history.rollbackEffect(failedEffectId);
if (rejected.kind === "rejectedAndRetired") {
  audit({
    effectId: rejected.effectId,
    retirement: rejected.retirement,
    projection: rejected.projection,
  });
}

// A separate operator action may deliberately restore retained history.
if (history.availability.restoreExact.kind === "available") {
  const restored = history.restoreExact();
  audit({ before, restored });
}
```

## How It Relates To Other Features

- [Concurrent Optimistic Effects](../resources/effects/concurrency-and-dependencies.md)
  explains effect identities and dependency closeout.
- [Rollback And Recovery](../resources/effects/rollback-and-recovery.md) is the
  task-first effect rejection guide.
- [Inspection And History Contract](./inspection-and-history.md) lists the
  lower-level history fields.

## Inspection And Debugging

Use `line.effects()` to identify concurrent open work. Use lifecycle and the
verification package for retained evidence. Compare the returned settlement or
restore receipt to the action you actually requested.

## Anti-Patterns

- Do not use `lastEffect` as the target when several effects are open.
- Do not call `restoreExact()` to undo one failed request.
- Do not label compact inverse or effect rejection as exact historical restore.
- Do not infer support from a method existing; read `availability` and the typed
  result.

## Current Limits

Only open effects can be rejected. Historical restore and replay depend on
same-runtime retained proof and can be unavailable. Retired effect summaries
remain inspectable, while live dependency and locus indexes are released.

## Related Docs

- [Concurrent Optimistic Effects](../resources/effects/concurrency-and-dependencies.md)
- [Branch-Native Effects](../resources/effects/branch-native-effects.md)
- [Effect Envelope Contract](./effect-envelope.md)

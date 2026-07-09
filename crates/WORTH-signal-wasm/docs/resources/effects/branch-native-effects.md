# Branch-Native Effects

## What This Feature Is

This is the normal optimistic lane for resource writes that should participate
in branch history, merge planning, rollback, and visible-selection proof.

## Why You Use It

Use `branchNative()` when you want:

- local optimistic patches on the visible line
- merge-aware or rebase-aware resource effects
- exact restore or compact inverse rollback when available
- one effect envelope you can inspect later

## Stable Entry Points

- `signals.resource.effects.branchNative()`
- `signals.resource.effects.serverCanonical()`
- `signals.resource.effects.pessimistic()`
- `signals.resource.effects.deliveryAuthoritative()`
- `signals.resource.effects.nonReversible()`
- `signals.resource.effects.sensitive()`
- `line.patch(...)`
- `line.deliver(...)`
- `line.diagnostics().lastEffect`

## Core Mental Model

An effect profile does not just choose optimism. It also chooses:

- confirmation posture
- rollback posture
- rebase posture
- preimage retention posture

`branchNative()` is the profile for local optimistic branch speculation when the
runtime can honestly admit it.

## How It Executes

When you patch or deliver through a family using an effect profile, the runtime
issues an effect envelope and attaches it to diagnostics and history. That
envelope records whether the write was speculative, committed-only, or denied
for optimism.

## Small Example

```ts
const tasks = signals.api({
  effects: signals.resource.effects.branchNative(),
}).url("/tasks")
  .items((task: { id: string; title: string }) => task.id)
  .aspect("title", (task) => task.title, (task, title: string) => ({
    ...task,
    title,
  }))
  .list({
    load: () => [{ id: "t1", title: "Loaded" }],
  });

const line = tasks.line({});
line.patch(tasks.patch.itemAspect({ itemId: "t1", aspect: "title", value: "Draft" }));

console.log(line.diagnostics().lastEffect?.profile?.name);
```

## Real Example

```ts
const line = tasks.line({});

line.patch(tasks.patch.itemAspect({
  itemId: "t1",
  aspect: "title",
  value: "Committed After Snapshot Rejection",
}));

const effect = line.diagnostics().lastEffect;

console.log(effect.plan.branch.kind);
console.log(effect.optimistic.rollback.kind);
console.log(effect.branchLifecycle.kind);
```

That read tells you whether the line really became speculative or whether the
runtime kept the write committed-only or optimism-unavailable instead.

## How It Relates To Other Features

- Use [Choose An Effect Profile](../updating/choose-an-effect-profile.md) when
  you are still deciding between profiles.
- Use [Rollback And Recovery](./rollback-and-recovery.md) when the main question
  is how far recovery goes.
- Use [Merge And Rebase](./merge-and-rebase.md) when you need to bind a concrete
  effect to a native merge preview.

## Inspection And Debugging

Inspect:

- `line.diagnostics().lastEffect`
- `line.history().verificationPackage().lifecycle.lastEffect`
- `line.history().rollbackLastEffect()`

## Anti-Patterns

- Do not assume `branchNative()` always produces speculative branch truth.
- Do not promise rollback if the envelope says rollback is unavailable.
- Do not synthesize your own effect record beside the runtime.

## Current Limits

Broad replacement or missing runtime branch proof can make optimism or exact
rollback unavailable. The runtime reports that explicitly instead of pretending.

## Related Docs

- [Branch-Native Resource Effects](../branch-native-effects.md)
- [Rollback And Recovery](./rollback-and-recovery.md)

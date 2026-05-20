# Resource-Line Forms

## What This Feature Is

This feature binds a form to a resource line so source truth, branch proof,
resource effect lowering, merge/drift posture, replay/restore, and rollback all
stay resource-owned instead of becoming a second local write engine.

## Why You Use It

- edit resource-backed truth through the form surface without losing resource
  identity
- lower submit and custom actions into resource effects
- project upload, processing, and download posture onto attachment or evidence
  fields without inventing field ownership the resource line did not prove
- inspect merge, drift, settlement, replay, restore, and rollback posture

## Stable Entry Points

- `signals.form.source.resourceLine(...)`
- `resourceSource()`
- `resourceMerge()`
- `resourceDrift()`
- `attachmentTransfers()`
- `previewResourceMerge(...)`
- `clearResourceMerge(...)`
- `reset()`
- `resetHistory()`
- `rollbackLastResourceEffect()`
- `replayExactResourceSource()`
- `restoreExactResourceSource()`
- `replayRestoreHistory()`

## Core Mental Model

The resource line owns source truth. The form owns draft truth. Submit and
resource-backed actions lower into resource-owned effect artifacts. Replay,
restore, rollback, merge, and drift remain resource proof that the form
consumes and projects.

## How It Executes

The runtime binds the form source to one resource line, reads request/lifecycle
and visible-selection posture from that line, lowers patch plans into resource
effects, then projects settlement, merge, drift, replay, and restore posture
back through form-readable artifacts.

## Small Example

```ts
const form = signals.form({
  source: signals.form.source.resourceLine(taskLine, { id: "task-resource" }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

console.log(form.resourceSource());
```

This is the smallest honest example because it shows the one real authority
move: source truth now comes from the resource line.

## Real Example

```ts
const form = signals.form({
  source: signals.form.source.resourceLine(taskLine, {
    id: "task-resource-action",
  }),
  fields: ({ field }) => ({
    title: field("title"),
  }),
  actions: ({ submit, action }) => ({
    submit: submit({
      resourceEffectProfile: signals.resource.effects.branchNative(),
    }),
    replayResourceSource: action("replayResourceSource", {
      resourceAction: { kind: "replayExact" },
    }),
    rollbackResourceEffect: action("rollbackResourceEffect", {
      resourceAction: { kind: "rollbackLastEffect" },
    }),
  }),
});

const execution = form.executeAction("submit");
console.log(execution.resultKind);
console.log(form.resourceSource());
console.log(form.verification().digests.resourceSourceDigest);
```

The form plan lowers into resource effect posture. The resource line still owns
visible branch selection, settlement, and replay/restore truth.

## How It Relates To Other Features

- Pair it with [Actions And Submit](./actions-and-submit.md) because submit and
  resource-backed actions use the same plan protocol.
- Pair it with [Collaboration](./collaboration.md) when lock, lease, or
  branch-per-actor posture comes from the resource-backed lane.
- Pair it with [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)
  when you need retained resource-linked proof digests.

## Inspection And Debugging

- `resourceSource()` shows request, lifecycle, effect profile, visible
  selection, settlement, and verification package linkage.
- `attachmentTransfers()` shows which attachment or evidence fields are inside
  the resource transfer surface, which ones are mapping-unavailable, and which
  ones are outside the transfer surface entirely.
- `resourceMerge()` and `resourceDrift()` show conflict, mapping-unavailable,
  rebased, and blocked posture.
- `reset()`, `rollbackLastResourceEffect()`, `replayExactResourceSource()`, and
  `restoreExactResourceSource()` emit typed history artifacts.
- `resetHistory()` and `replayRestoreHistory()` let you inspect those artifacts
  later without rereading the whole diagnostics package.

## Anti-Patterns

- treating a resource-line form as a second source authority
- inventing per-field transfer ownership when the line only proved ambiguous
  line-scoped transfer posture
- inventing a local optimistic cache beside resource effect posture
- rewriting merge or drift outcomes in UI code instead of consuming the typed
  artifacts

## Current Limits

- exact replay and exact restore stay explicit when runtime support or retained
  history is unavailable
- transfer posture can stay mapping-unavailable when the runtime cannot prove
  which attachment field owns a line-scoped transfer
- resource authority, verification packages, and closeout matrices remain
  resource-owned
- plain local forms should not use this lane just to get history-shaped APIs

## Related Docs

- [Actions And Submit](./actions-and-submit.md)
- [Collaboration](./collaboration.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)

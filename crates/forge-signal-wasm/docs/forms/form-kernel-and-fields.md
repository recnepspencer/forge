# Form Kernel And Fields

## What This Feature Is

The form kernel is the runtime-owned boundary behind `signals.form(...)`. It
owns source authority, draft authority, effective projection, field identity,
repeated-item identity, attachment evidence fields, and input adapter posture.

## Why You Use It

- build ordinary local forms without hand-rolling draft state
- bind a form to graph public input or a resource line without losing source
  authority
- declare repeated items and evidence fields with stable identities

## Stable Entry Points

- `signals.form(...)`
- `signals.form.source.signal(...)`
- `signals.form.source.graphPublicInput(...)`
- `signals.form.source.resourceLine(...)`
- field builders: `field(...)`, `repeated(...)`, `evidence(...)`
- `source()`
- `draft()`
- `effective()`
- `sourceAdmission()`
- `draftRestore()`
- `fieldWritePosture(fieldId)`
- `field(fieldId)`

## Core Mental Model

Source is the saved or authoritative value. Draft is the user's in-progress
edit state. Effective is what the form currently reads after layering draft on
top of source. Raw input can exist before it is accepted into draft. The
controller is the public object that lets you inspect and update those layers
without building your own store.

## How It Executes

The runtime declares the form contract, binds source authority, allocates draft
storage, admits field loci and adapter posture, then derives effective values
and field handles from those declarations.

## Small Example

```ts
import { createSignals } from "forge-signal-wasm";

const signals = await createSignals();
const source = signals.input({ title: "Ship docs", done: false });

const form = signals.form({
  source,
  fields: ({ field }) => ({
    title: field<string>("title"),
    done: field<boolean>("done"),
  }),
});

form.fields.title.set("Ship docs today");

console.log(form.source());
console.log(form.draft());
console.log(form.effective());
```

This is the smallest honest example because it shows the three truths the form
runtime keeps separate: source, draft, and effective.

## Real Example

```ts
const publicTask = signals.publicInput(source, { authority: "readOnly" });

const form = signals.form({
  source: signals.form.source.graphPublicInput(publicTask, {
    id: "task-public-input",
  }),
  fields: ({ field, repeated, evidence }) => ({
    title: field<string>("title"),
    auditItems: repeated<Array<{ id: string; label: string }>>("auditItems", {
      itemIdentity: "id",
      resourceLocus: { kind: "collectionItems", placement: "append" },
    }),
    evidence: evidence<{ digest: string; name: string }>("evidence", {
      attachmentIdentity: "digest",
      metadata: { required: true },
      resourceLocus: { kind: "region", region: "evidenceRegion" },
    }),
  }),
});
```

Here the graph public input still owns source truth. The form runtime owns the
draft and repeated-item/evidence identities. You inspect the admitted contract
through `declaration()`, `fieldContract()`, `inputAdapters()`, and
`sourceAuthority()`.

## How It Relates To Other Features

- Pair it with [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
  once writes should become semantic patch truth.
- Pair it with [Resource-Line Forms](./resource-line-forms.md) when the form
  source must stay resource-owned.
- Use [Presentation And External Lanes](./presentation-and-external-lanes.md)
  for attachment/media/handoff/exit visibility. Those do not change kernel
  truth.

## Inspection And Debugging

- `declaration()` shows form id, source kind, and field family counts.
- `source()`, `draft()`, and `effective()` let you compare the three value
  layers directly.
- `sourceAdmission()` and `draftRestore()` show whether the form is still
  waiting on bootstrap or restore work.
- `sourceAuthority()` shows which boundary owns source truth.
- `fieldContract()` shows stable paths, repeated identity posture, and resource
  loci.
- `fieldWritePosture(fieldId)` explains why one field can or cannot currently
  be edited or patched.
- `inputAdapters()` shows admitted adapter capability and unavailable posture.

## Anti-Patterns

- treating `effective()` as if it were source authority
- storing your own second draft object beside the form controller
- using repeated fields without stable item identity when item-level patch truth
  matters

## Current Limits

- route authority is still external; controller-local steps are the shipped
  form-owned step lane
- non-native adapters participate only through declared proof, not ambient DOM
  behavior
- resource-owned writes, replay, and restore belong to resource-line forms, not
  plain local sources

## Related Docs

- [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
- [Resource-Line Forms](./resource-line-forms.md)
- [Diagnostics, History, And Verification](./diagnostics-history-and-verification.md)

# Dirty, Patch, And Readiness

## What This Feature Is

This feature turns "changed" into semantic source-equivalence truth instead of
touch state or object identity. It also lowers source plus draft into an exact
patch plan and derives submit readiness from that truth.

## Why You Use It

- deny unchanged submit by default
- inspect whether a draft changed meaningfully or only cosmetically
- lower nested values, repeated items, and optional fields into explicit patch
  posture

## Stable Entry Points

- field writes: `set(...)`, `input(...)`, `commitInput()`
- `dirty()`
- `patchPlan()`
- `readiness()`

## Core Mental Model

Dirty truth is semantic by default. If the current effective value is source
equivalent, the form is not dirty even if the user touched and reverted fields.
The patch plan is a derived artifact, not a second source of truth.

## How It Executes

Field writes update draft or raw input posture. The runtime compares source and
effective values, derives semantic dirty state, lowers patch operations, then
computes submit blockers from that patch plus adjacent validation and admission
facts.

## Small Example

```ts
form.fields.title.set("Ship docs now");

console.log(form.dirty());
console.log(form.patchPlan());
console.log(form.readiness());
```

This is the smallest honest example because the runtime immediately shows the
three coupled artifacts: dirty state, patch plan, and readiness.

## Real Example

```ts
form.fields.title.input("Ship docs now", { source: "typing" });
form.fields.title.commitInput();

const dirty = form.dirty();
const patch = form.patchPlan();
const readiness = form.readiness();

console.log(dirty.isDirty);
console.log(patch.operations);
console.log(readiness.blockers);
```

Raw input exists before commit. Once committed, the runtime moves that input
into draft truth, recomputes semantic equality, and updates the patch and
readiness surfaces together.

## How It Relates To Other Features

- Pair it with [Validation And Messages](./validation-and-messages.md) when
  readiness should include validation blockers.
- Pair it with [Actions And Submit](./actions-and-submit.md) when multiple
  actions need patch-aware planning.
- Pair it with [Resource-Line Forms](./resource-line-forms.md) when the patch
  plan must lower into resource effects.

## Inspection And Debugging

- `dirty()` explains whether the form is semantically dirty.
- `patchPlan()` shows exact operations, broad replacement posture, and
  equivalence digest.
- `readiness()` shows blockers and the patch plan the submit lane would consume.
- `fieldWritePosture(fieldId, capability?)` explains field-level edit or patch
  denial without forcing you to inspect the whole form.

## Anti-Patterns

- using touched state as a substitute for semantic dirty truth
- treating `patchPlan()` as mutable authoring input
- assuming object reference inequality means the form changed semantically

## Current Limits

- route transitions and browser-history semantics are not part of readiness;
  those stay outside the controller-local form lane
- visibility or measurement changes do not change dirty truth
- resource-line write admission still belongs to the resource-aware lane

## Related Docs

- [Validation And Messages](./validation-and-messages.md)
- [Actions And Submit](./actions-and-submit.md)
- [Resource-Line Forms](./resource-line-forms.md)

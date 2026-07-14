# Unchanged Forms And Submit Readiness

## What This Feature Is

This page explains the default denial posture for unchanged forms and how that
shows up in readiness.

## Why You Use It

- disable submit when nothing changed
- explain why a form looks editable but still cannot submit
- keep unchanged denial tied to semantic truth instead of UI heuristics

## Stable Entry Points

- `form.readiness()`
- `form.dirty()`
- `form.patchPlan()`

## Core Mental Model

The form does not submit just because the user interacted with it. If the
current effective value is still source-equivalent, readiness can deny submit
with an `unchanged` blocker.

## How It Executes

1. field edits update draft truth
2. semantic dirty truth is recomputed
3. the patch plan is derived
4. readiness uses that dirty and patch posture to allow or deny submit

## Small Example

```ts
console.log(form.readiness().canSubmit);
console.log(form.readiness().blockers);
```

## Real Example

```ts
form.fields.title.set("Ready to ship");
console.log(form.readiness().canSubmit);

form.fields.title.set("Ship docs");
console.log(form.dirty().isDirty);
console.log(form.patchPlan().empty);
console.log(form.readiness().blockers);
```

Once the second write returns the form to source-equivalent truth, readiness
can deny submit again even though the user did edit the form.

## How It Relates To Other Features

- Read [Dirty State](./dirty-state.md) for the semantic change lane underneath
  unchanged denial.
- Read [Patch Plans](./patch-plans.md) for the empty-patch lane that readiness
  consumes.

## Inspection And Debugging

- `readiness().blockers` is the first read
- `dirty()` tells you whether the form changed semantically
- `patchPlan().empty` tells you whether the derived write lane is empty

## Anti-Patterns

- disabling submit from touched or visited state alone
- treating unchanged denial like a validation error

## Current Limits

- this page covers default readiness semantics only
- action-specific overrides and resource-backed submit posture belong in later
  sections

## Related Docs

- [Dirty State](./dirty-state.md)
- [Patch Plans](./patch-plans.md)

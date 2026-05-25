# Dirty State

## What This Feature Is

Dirty state is the form's semantic change read, not just touch state or object
inequality.

## Why You Use It

- know whether the form changed in meaning
- deny unchanged submit by default
- debug why a field write did or did not count as a real change

## Stable Entry Points

- `form.dirty()`
- `fieldHandle.dirty()`
- `form.readiness()`

## Core Mental Model

If source and effective values are semantically equivalent, the form is not
dirty even if the user typed, blurred, and reverted a field.

## How It Executes

1. field writes update draft or raw-input posture
2. the runtime compares source and effective truth
3. semantic dirty state is derived for the form and each field
4. readiness can consume that dirty truth later

## Small Example

```ts
form.fields.title.set("Ready to ship");

console.log(form.dirty());
```

## Real Example

```ts
form.fields.title.set("Ready to ship");
console.log(form.dirty().isDirty);

form.fields.title.set("Ship docs");
console.log(form.dirty().isDirty);
```

The second write returns the form to semantic equality, so dirty truth goes
false again.

## How It Relates To Other Features

- Read [Patch Plans](./patch-plans.md) for the write operations derived from
  dirty truth.
- Read [Unchanged Forms And Submit Readiness](./unchanged-forms-and-submit-readiness.md)
  for how unchanged denial consumes this lane.

## Inspection And Debugging

- `dirty().fields` shows field-level changed posture
- `dirty().breadth` and `dirty().equality` show the comparison footprint
- field `dirty()` explains one field without scanning the whole form mentally

## Anti-Patterns

- using touched state as a substitute for dirty truth
- assuming any object change automatically means a semantic change

## Current Limits

- dirty truth is one part of readiness, not the whole readiness model

## Related Docs

- [Patch Plans](./patch-plans.md)
- [Unchanged Forms And Submit Readiness](./unchanged-forms-and-submit-readiness.md)

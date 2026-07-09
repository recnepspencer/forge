# Controller-Local Steps

## What This Feature Is

This page covers step declarations inside one form controller.

## Why You Use It

- break one form into named steps without creating separate forms
- keep step posture tied to field truth, validation, and patches
- inspect step-level progress directly from the controller

## Stable Entry Points

- `steps: ({ step }) => ...`
- `form.steps()`

## Core Mental Model

Steps are derived views over one form, not separate draft stores. Each step
declares a field set and optional posture rules, and the runtime derives
progress from the underlying form truth.

## How It Executes

1. declare steps and their field membership
2. optional step rules resolve posture such as `active` or `blocked`
3. the runtime derives step readiness, dirty state, patch scope, and progress
4. navigation and step actions consume those artifacts later

## Small Example

```ts
const form = signals.form({
  source: { title: "", summary: "" },
  fields: ({ field }) => ({
    title: field("title"),
    summary: field("summary"),
  }),
  steps: ({ step }) => ({
    details: step("details", ["title", "summary"]),
  }),
});

console.log(form.steps().artifacts);
```

## Real Example

```ts
const steps = form.steps();

console.log(steps.summary);
console.log(steps.artifacts.map((step) => ({
  id: step.id,
  posture: step.posture,
  progress: step.progress,
})));
```

## How It Relates To Other Features

- Read [Step Readiness](./step-readiness.md) for the blocker and progress lane.
- Read [Step Actions](./step-actions.md) when next/back/jump actions are part
  of the form.

## Inspection And Debugging

- `steps().artifacts` is the main read
- `summary` shows how many steps are active, blocked, skipped, or complete
- each step artifact exposes dirty, patch, validation, and message slices

## Anti-Patterns

- treating each step like a separate unrelated form
- rebuilding step progress in app code from raw field values

## Current Limits

- route-coupled steps get a separate later doc section

## Related Docs

- [Step Readiness](./step-readiness.md)
- [Step Actions](./step-actions.md)

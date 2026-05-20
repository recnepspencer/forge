# Availability, Admission, And Steps

## What This Feature Is

This feature turns dynamic control topology, authority gates, and
controller-local step state into derived form artifacts.

## Why You Use It

- enable, disable, omit, or freeze fields from declared dependencies
- block edit, patch, submit, or action lanes through typed admission facts
- build multi-step forms without inventing route semantics inside the form

## Stable Entry Points

- availability builders inside `signals.form({... availability: ... })`
- admission builders inside `signals.form({... admission: ... })`
- steps builders inside `signals.form({... steps: ... })`
- `availability()`
- `admission()`
- `steps()`

## Core Mental Model

Availability is not DOM state. Admission is not a button-local boolean. Steps
are derived from form artifacts and controller-local declarations, not from URL
ownership. These surfaces gate capabilities without mutating source truth.

## How It Executes

The runtime evaluates declared availability and admission dependencies, derives
field/control/group/section/action posture, then projects that truth into step
summaries, step blockers, and step progress.

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs", done: false },
  fields: ({ field }) => ({
    title: field("title"),
    done: field("done"),
  }),
  availability: ({ field }) => ({
    titleAvailability: field("title", ["done"], (values) => (
      values.done ? { state: "readonly", draftPolicy: "freeze" } : "enabled"
    )),
  }),
});
```

This is the smallest honest example because it shows the intended lane:
availability is derived from declared dependencies, not set imperatively.

## Real Example

```ts
const report = form.steps();
const admission = form.admission();
const availability = form.availability();

console.log(report.summary);
console.log(admission.summary);
console.log(availability.summary);
```

Here the runtime has already combined field posture, action gates, and step
topology. The step surface is a projection over the same canonical form truth.

## How It Relates To Other Features

- Pair it with [Actions And Submit](./actions-and-submit.md) when action plans
  must consume the same readiness and admission truth.
- Pair it with [Resource-Line Forms](./resource-line-forms.md) when lock,
  drift, or freshness posture should gate submit.
- Pair it with [Collaboration](./collaboration.md) when multi-actor locks or
  leases should block local edits.

## Inspection And Debugging

- `availability()` shows dynamic control posture and summaries.
- `admission()` shows capability gates such as approval, review, or lock
  posture.
- `steps()` shows per-step readiness, validation, dirty, and progress summaries.

## Anti-Patterns

- toggling local UI booleans instead of declaring availability dependencies
- treating route-coupled step behavior as controller-local truth
- collapsing approval, permission, and lock posture into one generic blocked
  bit

## Current Limits

- route authority remains external; route-coupled behavior stays on typed
  deferred posture
- admission artifacts block capabilities; they do not mutate source or draft
  values directly
- browser-history ownership is not part of controller-local steps

## Related Docs

- [Actions And Submit](./actions-and-submit.md)
- [Collaboration](./collaboration.md)
- [Resource-Line Forms](./resource-line-forms.md)

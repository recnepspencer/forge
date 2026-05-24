# Resource-Backed Collaboration

## What This Feature Is

This page covers collaboration modes that depend on resource-backed truth,
especially `branchPerActor`.

## Why You Use It

- align collaboration posture with speculative or branch-backed resource truth
- preserve native branch identity instead of inventing a second branch model in
  app code
- keep collaboration proof tied to the resource-backed form surface

## Stable Entry Points

- `signals.form.source.resourceLine(...)`
- `collaboration: { mode: "branchPerActor" }`
- `form.collaboration().resourceProof`
- `form.collaboration().branchId`

## Core Mental Model

`branchPerActor` only works honestly when the resource source can prove the
visible branch truth the collaboration report is talking about.

The collaboration report does not invent a branch. It consumes the branch
identity already exposed by the resource-backed form source.

## How It Executes

1. build the form from a resource line
2. declare `branchPerActor`
3. the runtime derives resource proof from the admitted visible selection
4. branch-backed collaboration updates retain the same branch identity

## Small Example

```ts
console.log(form.collaboration().resourceProof);
console.log(form.collaboration().branchId);
```

## Real Example

```ts
const form = signals.form({
  source: signals.form.source.resourceLine(taskLine, { id: "task-resource" }),
  collaboration: {
    mode: "branchPerActor",
    actorId: "me",
  },
  fields: ({ field }) => ({
    title: field("title"),
  }),
});

form.reportCollaboration({
  posture: "settling",
  branchId: 7,
  reason: "branch-backed collaboration update is settling",
});

console.log(form.collaboration().resourceProof);
console.log(form.collaboration().events);
```

## How It Relates To Other Features

- Read [Resource Line Source](../resource-backed/resource-line-source.md) for
  the resource-backed form source itself.
- Read [Resource Shape And Visible Selection](../resource-backed/resource-shape-and-visible-selection.md)
  for how the form sees speculative or visible branch truth.

## Inspection And Debugging

- `form.collaboration().resourceProof` shows whether resource proof was
  required and admitted
- `form.collaboration().branchId` preserves native string or numeric branch id
- `form.collaboration().events` shows branch changes directly

## Anti-Patterns

- stringifying numeric branch ids before reporting them
- using branch-per-actor collaboration on a non-resource-backed form and
  expecting native branch proof

## Current Limits

- resource-backed collaboration only knows what the form source can expose

## Related Docs

- [Collaboration Overview](./collaboration-overview.md)
- [Resource Line Source](../resource-backed/resource-line-source.md)

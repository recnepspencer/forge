# Step Groups

## What This Feature Is

This page covers grouped step organization and step-level layout hints.

## Why You Use It

- cluster related steps under one named group
- keep multi-step forms readable without inventing your own grouping model
- inspect ordering and layout posture at step scope

## Stable Entry Points

- step option `group`
- step options `order`, `density`, `alignment`, `responsive`
- `form.steps()`

## Core Mental Model

A step can carry both semantic membership and presentation hints. Grouping
does not change the underlying form truth; it just gives the step layer better
structure.

## How It Executes

1. declare steps with optional group and order hints
2. the runtime normalizes those into step artifacts
3. step reports expose the grouped structure to navigation and layout readers

## Small Example

```ts
steps: ({ step }) => ({
  details: step("details", ["title"], { group: "core", order: 1 }),
  review: step("review", ["approved"], { group: "core", order: 2 }),
})
```

## Real Example

```ts
console.log(form.steps().artifacts.map((step) => ({
  id: step.id,
  group: step.group,
  order: step.order,
  layout: step.layout,
})));
```

## How It Relates To Other Features

- Read [Controller-Local Steps](./controller-local-steps.md) for basic step
  declaration.
- Read [Layout Overview](../layout/layout-overview.md) when you need the
  field-level layout lane instead of step-level grouping.

## Inspection And Debugging

- `steps().artifacts[*].group` and `order` are the main reads
- `steps().dependencyBreadth` shows step membership and layout hints together

## Anti-Patterns

- using app-only metadata for step grouping when the controller already knows
  the step structure
- expecting group names to change semantic step progress on their own

## Current Limits

- external presentation timing for steps belongs in the later lifecycle docs

## Related Docs

- [Controller-Local Steps](./controller-local-steps.md)
- [Layout Overview](../layout/layout-overview.md)

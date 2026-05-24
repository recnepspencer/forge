# Step Readiness

## What This Feature Is

This page explains when a step can be entered or completed, and how step
progress is derived.

## Why You Use It

- block users from moving forward before a step is ready
- inspect step-level validation and patch posture
- understand why one step is `changed`, `blocked`, or `complete`

## Stable Entry Points

- `form.steps()`
- `stepArtifact.readiness`
- `stepArtifact.progress`

## Core Mental Model

Each step has its own readiness view derived from the same form truth. That
view is narrower than whole-form readiness because it only considers the
fields and rules inside the step.

## How It Executes

1. step membership is declared
2. dirty, patch, validation, and messages are projected into each step
3. the runtime derives `canEnter`, `canComplete`, blockers, and progress

## Small Example

```ts
const details = form.steps().artifacts.find((step) => step.id === "details");
console.log(details?.readiness);
console.log(details?.progress);
```

## Real Example

```ts
for (const step of form.steps().artifacts) {
  console.log(step.id, step.readiness.blockers, step.progress);
}
```

## How It Relates To Other Features

- Read [Validation Overview](../validation/validation-overview.md) for the
  validation results behind step blockers.
- Read [Readiness Blockers](../availability/readiness-blockers.md) for the
  whole-form and action-level version of the same idea.

## Inspection And Debugging

- `step.readiness.blockers` tells you why the step cannot proceed
- `step.validation.artifacts` tells you which validation results are in scope
- `step.patch` shows whether the step's fields currently carry changes

## Anti-Patterns

- deciding step completion from route position alone
- duplicating step progress logic outside the controller

## Current Limits

- route-coupled step authority belongs in the later route-coupling docs

## Related Docs

- [Controller-Local Steps](./controller-local-steps.md)
- [Readiness Blockers](../availability/readiness-blockers.md)

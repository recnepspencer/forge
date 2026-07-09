# Controller-Local Step Navigation

## What This Feature Is

This page covers the controller-local navigation report for multi-step forms.

## Why You Use It

- inspect the current step plus visited and skipped step state
- keep step navigation tied to form state instead of route state
- debug missing or unavailable step targets

## Stable Entry Points

- `form.navigation()`
- `form.steps()`

## Core Mental Model

Controller-local navigation is the in-form navigation layer. It is separate
from route navigation and only knows about the steps declared on this form.

## How It Executes

1. steps are declared
2. step posture and action state are derived
3. navigation summarizes the current local target structure
4. step actions can consume that structure later

## Small Example

```ts
console.log(form.navigation());
```

## Real Example

```ts
const navigation = form.navigation();

console.log(navigation.summary);
console.log(navigation.history);
```

## How It Relates To Other Features

- Read [Controller-Local Steps](./controller-local-steps.md) for the declared
  step structure underneath navigation.
- Read [Step Actions](./step-actions.md) for next/back/jump commands that use
  this local navigation model.

## Inspection And Debugging

- `navigation().summary` is the fastest read
- `history` shows how local step movement changed over time
- `steps().artifacts` helps explain why local movement was blocked or why a
  step was skipped

## Anti-Patterns

- mixing route navigation and controller-local step navigation together
- inferring step movement only from button clicks

## Current Limits

- route-coupled navigation lives in the later route-coupling docs

## Related Docs

- [Controller-Local Steps](./controller-local-steps.md)
- [Step Actions](./step-actions.md)

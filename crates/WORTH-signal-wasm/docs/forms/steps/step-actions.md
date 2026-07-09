# Step Actions

## What This Feature Is

This page covers step-shaped actions such as next, back, jump, skip, revisit,
and custom step commands.

## Why You Use It

- keep step navigation inside the form action model
- inspect whether a step action is accepted or denied before execution
- share action planning rules across submit and step commands

## Stable Entry Points

- `actions: ({ step }) => ...`
- `form.actionPlan(actionId)`
- `form.actions()`

## Core Mental Model

Step actions are still actions. They just carry step command intent such as
`next` or `back`.

That means they inherit action planning, blockers, and history instead of
living in a separate navigation-only subsystem.

## How It Executes

1. declare a step action bound to a step id and command
2. action planning derives blockers and status
3. execution or attempt history records the result like any other action

## Small Example

```ts
actions: ({ step }) => ({
  nextDetails: step("nextDetails", "details", "next"),
})
```

## Real Example

```ts
console.log(form.actionPlan("nextDetails"));
console.log(form.actions().plans.filter((plan) => plan.kind === "step"));
```

## How It Relates To Other Features

- Read [Controller-Local Step Navigation](./controller-local-step-navigation.md)
  for the navigation view that consumes these commands.
- Read [Action Plans](../actions/action-plans.md) for the broader planning
  model.

## Inspection And Debugging

- `actionPlan(...)` shows whether the step action is accepted
- `actions().summary.step` shows how many step plans are present
- `actionHistory()` and `actionExecutionHistory()` show attempts and
  executions over time

## Anti-Patterns

- creating a separate step-button state machine beside form actions
- bypassing the action planner for next/back logic

## Current Limits

- route-coupled step commands live in the later route-coupling docs

## Related Docs

- [Action Plans](../actions/action-plans.md)
- [Controller-Local Step Navigation](./controller-local-step-navigation.md)

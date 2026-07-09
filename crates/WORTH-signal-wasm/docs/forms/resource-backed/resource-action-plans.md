# Resource Action Plans

## What This Feature Is

This page covers action plans whose execution is resource-backed.

## Why You Use It

- inspect how a form action maps onto resource-backed work
- see resource action declaration, effect profile, and patch scope before
  execution
- understand why a resource-backed action is accepted, denied, or unavailable

## Stable Entry Points

- resource-backed action declarations
- `form.actionPlan(actionId)`

## Core Mental Model

A resource-backed action plan is still an ordinary form action plan, but it
also tells you how the action maps onto a resource line and effect profile.

## How It Executes

1. an action is declared with resource-backed behavior
2. the planner derives the normal action plan
3. the resource action and effect profile reads are attached to that plan

## Small Example

```ts
const plan = form.actionPlan("submit");
console.log(plan.resourceAction);
console.log(plan.resourceEffectProfile);
```

## Real Example

```ts
const plan = form.actionPlan("submit");

console.log(plan.patch);
console.log(plan.resourceAction);
console.log(plan.resourceEffectProfile);
console.log(plan.readiness.blockers);
```

## How It Relates To Other Features

- Read [Action Plans](../actions/action-plans.md) for the general action-plan
  model.
- Read [Resource Action Execution](./resource-action-execution.md) for what
  happens after the plan runs.

## Inspection And Debugging

- `resourceAction` shows whether the action is patch, refresh, revalidate,
  replay, restore, or rollback-based
- `resourceEffectProfile` shows profile inheritance or mismatch
- `patch` shows the form-side change set the resource-backed action will use

## Anti-Patterns

- treating resource-backed actions like ordinary local custom actions
- ignoring resource profile mismatch on the action plan

## Current Limits

- resource-family transport detail belongs in the resource docs

## Related Docs

- [Action Plans](../actions/action-plans.md)
- [Resource Action Execution](./resource-action-execution.md)

# Action Plans

## What This Feature Is

This page covers the derived plan behind one action.

## Why You Use It

- know whether an action can run before executing it
- inspect blockers, patch posture, and validation summary together
- understand why one action is accepted while another is denied

## Stable Entry Points

- `form.actionPlan(actionId)`
- `form.actions()`

## Core Mental Model

An action plan is the action runtime's answer to "what would happen if I tried
this right now?"

It bundles readiness, patch behavior, validation summary, availability
summary, admission summary, and host requirements into one read.

## How It Executes

1. the action declaration is read
2. current form state is projected into action scope
3. the runtime derives status, blockers, patch behavior, and the plan summary

## Small Example

```ts
console.log(form.actionPlan("submit"));
```

## Real Example

```ts
const plan = form.actionPlan("submit");

console.log(plan.status);
console.log(plan.readiness.blockers);
console.log(plan.patch);
console.log(plan.validation.summary);
```

## How It Relates To Other Features

- Read [Readiness Blockers](../availability/readiness-blockers.md) for the
  blocker model beneath plans.
- Read [Action Execution](./action-execution.md) for what happens after a plan
  is executed.

## Inspection And Debugging

- `status` and `resultKind` are the first reads
- `patch` shows whether the action consumes real changes
- `recoveryActions` shows what the runtime thinks the next useful move is

## Anti-Patterns

- deciding action state from `canSubmit` alone
- treating the action plan as mutable input

## Current Limits

- resource-backed action plan details get their own later docs

## Related Docs

- [Submit Actions](./submit-actions.md)
- [Action Execution](./action-execution.md)

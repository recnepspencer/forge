# Recovery Actions

## What This Feature Is

This page covers the runtime-suggested next actions attached to denied plans
and failed executions.

## Why You Use It

- guide the user toward the next useful step
- avoid inventing one-off recovery logic for each form
- inspect whether the runtime thinks the best move is retry, edit, refresh, or
  focus

## Stable Entry Points

- `form.actionPlan(actionId).recoveryActions`
- `form.actionHistory()`
- `form.actionExecutionHistory()`

## Core Mental Model

Recovery actions are suggestions derived from the current blocker or failure
shape. They are not a replacement for execution history; they sit on top of
it.

## How It Executes

1. planning or execution produces blockers or failures
2. the runtime derives recovery suggestions
3. those suggestions are retained on the action plan or result record

## Small Example

```ts
console.log(form.actionPlan("submit").recoveryActions);
```

## Real Example

```ts
const attempt = form.attemptAction("submit");

console.log(attempt.blockers);
console.log(attempt.recoveryActions);
```

## How It Relates To Other Features

- Read [Action Plans](./action-plans.md) for plan-side recovery suggestions.
- Read [Action Execution](./action-execution.md) for execution-side failure
  history.

## Inspection And Debugging

- `recoveryActions[*].kind` shows the recovery category
- `blockerKind` and `reason` explain why that recovery was suggested

## Anti-Patterns

- ignoring recovery suggestions and rebuilding the same routing logic in every
  screen
- treating recovery suggestions as guaranteed imperative effects

## Current Limits

- resource-line recovery actions get deeper treatment in the later
  resource-backed docs

## Related Docs

- [Action Plans](./action-plans.md)
- [Action Execution](./action-execution.md)

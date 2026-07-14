# Action Execution

## What This Feature Is

This page covers the execution lifecycle after an action starts running.

## Why You Use It

- inspect pending, fulfilled, rejected, cancelled, timed-out, or superseded
  executions
- keep async submit state on the controller surface
- retain a real execution history instead of one mutable loading flag

## Stable Entry Points

- `form.executeAction(actionId)`
- `form.fulfillAction(operationId, payload?)`
- `form.rejectAction(operationId, payload?)`
- `form.cancelAction(operationId, payload?)`
- `form.timeoutAction(operationId, payload?)`
- `form.actionExecutionHistory()`

## Core Mental Model

Execution is separate from planning. First the runtime decides whether the
action is allowed. Then it records what happened when the action actually ran.

## How It Executes

1. an action plan exists
2. `executeAction(...)` creates an execution record
3. the action stays pending until a result is reported
4. history retains the execution outcome

## Small Example

```ts
const pending = form.executeAction("submit");
console.log(pending.resultKind);
```

## Real Example

```ts
const pending = form.executeAction("submit");

form.rejectAction(pending.operationId, {
  reason: "server rejected the request",
});

console.log(form.actionExecutionHistory().at(-1)?.resultKind);
```

## How It Relates To Other Features

- Read [Action Plans](./action-plans.md) for the pre-execution decision.
- Read [Recovery Actions](./recovery-actions.md) for the "what should the user
  do next?" lane after denial or rejection.

## Inspection And Debugging

- `actionExecutionHistory()` shows the lifecycle history
- `actionHistory()` shows the higher-level attempt history beside it
- `pending.resultKind`, `reason`, and `serverMessages` explain one execution

## Anti-Patterns

- modeling async action state with a single boolean
- losing rejected or cancelled execution history immediately after the UI
  updates

## Current Limits

- resource-backed execution details live in the later resource-backed section

## Related Docs

- [Action Plans](./action-plans.md)
- [Recovery Actions](./recovery-actions.md)

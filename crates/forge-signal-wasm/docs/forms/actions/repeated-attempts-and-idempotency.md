# Repeated Attempts And Idempotency

## What This Feature Is

This page covers repeated-action posture such as `collapse`, `supersede`,
`queue`, and `deny`.

## Why You Use It

- control what happens when the user triggers the same action repeatedly
- inspect whether attempts collapse, queue, or supersede each other
- keep repeated-submit behavior explicit

## Stable Entry Points

- action option `idempotency`
- `form.actionPlan(actionId)`
- `form.actionHistory()`
- `form.actionExecutionHistory()`

## Core Mental Model

Repeated attempts are a declared policy, not a side effect of timing. The
runtime records which policy was used and what happened because of it.

## How It Executes

1. declare an action with an idempotency policy
2. repeated attempts are handled according to that policy
3. history records the result and any superseded or collapsed relationship

## Small Example

```ts
actions: ({ submit }) => ({
  submit: submit({ idempotency: "collapse" }),
})
```

## Real Example

```ts
const first = form.executeAction("submit");
const second = form.executeAction("submit");

console.log(first.resultKind);
console.log(second.resultKind);
console.log(form.actionExecutionHistory());
```

## How It Relates To Other Features

- Read [Action Execution](./action-execution.md) for the execution lifecycle
  that records these relationships.
- Read [Submit Actions](./submit-actions.md) for the ordinary submit lane where
  repeated attempts matter most.

## Inspection And Debugging

- `actionPlan(...).diagnostics.repeatedAttemptPolicy` shows the active policy
- action history and execution history show superseded, collapsed, or queued
  outcomes

## Anti-Patterns

- relying on UI debounce alone for repeated-submit safety
- assuming repeated attempts are always harmless no-ops

## Current Limits

- host-specific network retry posture belongs in later docs

## Related Docs

- [Submit Actions](./submit-actions.md)
- [Action Execution](./action-execution.md)

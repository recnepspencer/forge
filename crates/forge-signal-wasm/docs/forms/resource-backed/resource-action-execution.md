# Resource Action Execution

## What This Feature Is

This page covers action execution when the action is backed by a resource line.

## Why You Use It

- inspect resource submission, settlement, replay/restore, and rollback from
  one execution record
- understand the difference between local action success and resource-backed
  execution outcome
- keep resource execution history on the controller surface

## Stable Entry Points

- `form.executeAction(actionId)`
- `form.actionExecutionHistory()`

## Core Mental Model

Resource-backed execution is still action execution, but it carries extra
resource detail such as submission patches, effect profile, settlement,
resource lifecycle, and resource recovery.

## How It Executes

1. a resource-backed action plan is executed
2. the runtime records an execution artifact
3. resource submission, settlement, lifecycle, and recovery details are
   attached as they become available

## Small Example

```ts
const pending = form.executeAction("submit");
console.log(pending.resourceSubmission);
```

## Real Example

```ts
const execution = form.actionExecutionHistory().at(-1);

console.log(execution?.resourceSubmission);
console.log(execution?.resourceSettlement);
console.log(execution?.resourceLifecycle);
console.log(execution?.resourceRecovery);
```

## How It Relates To Other Features

- Read [Resource Action Plans](./resource-action-plans.md) for the pre-execution
  planning side.
- Read [Mutation-Response Readback](./mutation-response-readback.md) for the
  confirmation detail surfaced through the resource source.

## Inspection And Debugging

- `resourceSubmission` shows patch count, loci, and effect profile
- `resourceSettlement` shows confirmation/failure detail
- `resourceRecovery` shows reset, replay, or restore results tied to execution

## Anti-Patterns

- flattening resource-backed execution into one generic "submit succeeded"
  message
- reconstructing resource execution detail from several separate stores

## Current Limits

- deeper effect and topology semantics live in the resource docs

## Related Docs

- [Resource Action Plans](./resource-action-plans.md)
- [Mutation-Response Readback](./mutation-response-readback.md)

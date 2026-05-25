# Submit Actions

## What This Feature Is

This page covers the ordinary submit lane.

## Why You Use It

- declare one standard submit action
- inspect submit-specific patch and readiness behavior
- accept canonical server values through the same action lifecycle

## Stable Entry Points

- `submit()`
- `form.actionPlan("submit")`
- `form.executeAction("submit")`
- `form.fulfillAction(operationId, { canonicalValue })`

## Core Mental Model

Submit is just the most common action kind. It still uses the same planning
and execution surfaces as every other action.

## How It Executes

1. declare `submit()`
2. inspect the submit plan
3. execute the action
4. fulfill, reject, cancel, or time out the execution
5. canonicalization can update source truth on fulfillment

## Small Example

```ts
const pending = form.executeAction("submit");
form.fulfillAction(pending.operationId, {
  canonicalValue: { title: "Ship docs", status: "published" },
});
```

## Real Example

```ts
const plan = form.actionPlan("submit");
const pending = form.executeAction("submit");

console.log(plan.patch);
console.log(form.actionExecutionHistory());
```

## How It Relates To Other Features

- Read [Action Plans](./action-plans.md) for the planner surface.
- Read [Server Canonicalization](../validation/server-canonicalization.md) for
  the canonical-value consequence of fulfillment.

## Inspection And Debugging

- `actionPlan("submit")` is the first read
- `actionExecutionHistory()` shows the execution lifecycle
- `canonicalizationHistory()` shows source updates after fulfillment

## Anti-Patterns

- treating submit as a special UI-only event outside the action runtime
- skipping the submit plan and only looking at execute-time failures

## Current Limits

- resource-backed submit execution gets its own later forms section

## Related Docs

- [Action Plans](./action-plans.md)
- [Action Execution](./action-execution.md)

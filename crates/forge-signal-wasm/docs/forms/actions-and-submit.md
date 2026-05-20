# Actions And Submit

## What This Feature Is

This feature turns submit and other commands into one declared action protocol
with runtime-owned planning, readiness, admission, idempotency, and execution
posture.

## Why You Use It

- keep custom actions from bypassing form truth
- inspect submit and non-submit commands before effects run
- model retry, recovery, destructive posture, and repeated-attempt policy

## Stable Entry Points

- action builders inside `signals.form({... actions: ... })`
- `actions()`
- `actionReadiness(actionId)`
- `actionPlan(actionId)`
- `attemptAction(actionId)`
- `executeAction(actionId)`
- `fulfillAction(operationId, payload?)`
- `rejectAction(operationId, payload?)`
- `cancelAction(operationId, payload?)`
- `timeoutAction(operationId, payload?)`
- `retryAction(operationId)`
- `actionHistory()`
- `actionExecutionHistory()`

## Core Mental Model

An action is not a button callback. It is a runtime-owned plan built from the
current form state. Submit is just one first-class action in that same system.
The runtime decides whether an action is allowed before effects run, and it
retains the attempt and execution history afterward.

## How It Executes

The runtime derives readiness, admission, patch, schema, host, and effect
binding posture for an action, lowers that into a plan, records an attempt
artifact, then runs effect execution only if the plan is admitted.

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({ title: field("title") }),
  actions: ({ action }) => ({
    approve: action("approve", {
      patchPolicy: "allowEmpty",
      hostEffect: "workflow.approve",
    }),
  }),
});

console.log(form.actionPlan("approve"));
```

This is the smallest honest example because it shows the stable object you use
first: the action plan, not the execution side effect.

## Real Example

```ts
form.fields.title.set("Ship docs now");

const plan = form.actionPlan("submit");
const attempt = form.attemptAction("submit");
const execution = form.executeAction("submit");

console.log(plan.planDigest);
console.log(attempt.resultKind);
console.log(execution.resultKind);
```

The runtime keeps planning truth, attempt truth, and execution truth distinct.
That is what makes stale completions, retries, and diagnostics honest.

## How It Relates To Other Features

- Pair it with [Availability, Admission, And Steps](./availability-admission-and-steps.md)
  because actions consume those gates directly.
- Pair it with [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
  when action execution is effect-backed or server-returned.
- Pair it with [Resource-Line Forms](./resource-line-forms.md) when submit or
  custom actions must lower into resource effects.

## Inspection And Debugging

- `actions()` shows catalog, plans, summaries, and plan digests.
- `actionReadiness(actionId)` is the quick way to ask "can this action run
  right now, and what is blocking it?"
- `actionPlan(actionId)` shows the exact proof and blockers one action would
  consume.
- `actionHistory()` and `actionExecutionHistory()` show retained attempts and
  execution lifecycle artifacts.
- `fulfillAction(...)`, `rejectAction(...)`, `cancelAction(...)`,
  `timeoutAction(...)`, and `retryAction(...)` settle or continue long-running
  executions without bypassing the action protocol.

## Anti-Patterns

- running side effects from local button handlers instead of through
  `executeAction(...)`
- assuming submit owns different semantics than other effectful actions
- recomputing your own blocker logic outside the action plan

## Current Limits

- route authority still belongs outside the controller-local lane
- actions can consume host, resource, and collaboration posture, but those
  authorities remain owned by their own subsystems
- empty-patch submit stays denied unless explicitly declared otherwise

## Related Docs

- [Availability, Admission, And Steps](./availability-admission-and-steps.md)
- [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
- [Resource-Line Forms](./resource-line-forms.md)

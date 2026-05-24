# Offline And Host Blockers

## What This Feature Is

This page covers the readiness blockers that come from host facts and host
requirements.

## Why You Use It

- explain why a form or action is blocked by offline or missing host support
- keep host-dependent denial separate from validation or admission
- inspect action-side host requirements directly

## Stable Entry Points

- `form.host()`
- `form.readiness()`
- `form.actionPlan(actionId)`

## Core Mental Model

Some readiness failures are not about the form's data at all. They are about
whether the host can currently support the action being attempted.

## How It Executes

1. host facts and action requirements are read
2. the runtime derives blockers like `host:offline` or `host:unavailable`
3. readiness and action plans expose those blockers

## Small Example

```ts
console.log(form.readiness().blockers);
```

## Real Example

```ts
const plan = form.actionPlan("submit");

console.log(form.host().facts.online);
console.log(plan.host.requirements);
console.log(plan.host.blockers);
```

## How It Relates To Other Features

- Read [Host Facts](./host-facts.md) for the environment reads beneath these
  blockers.
- Read [Readiness Blockers](../availability/readiness-blockers.md) for the
  full blocker model.

## Inspection And Debugging

- `readiness().blockers` shows form-level host blockers
- `actionPlan(...).host` shows action-specific host requirements
- `host().facts` shows the underlying fact state

## Anti-Patterns

- treating offline state like a validation error
- burying host requirements inside button handlers

## Current Limits

- external handoff and exit host behavior live in later lifecycle docs

## Related Docs

- [Host Facts](./host-facts.md)
- [Readiness Blockers](../availability/readiness-blockers.md)

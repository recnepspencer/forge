# Readiness Blockers

## What This Feature Is

This page explains the blocker list behind `form.readiness()` and
`form.actionReadiness(...)`.

## Why You Use It

- disable submit from real runtime reasons instead of UI heuristics
- explain exactly why a form or action cannot proceed
- distinguish unchanged, validation, availability, admission, step, and host
  blockers

## Stable Entry Points

- `form.readiness()`
- `form.actionReadiness(actionId)`
- `form.actions()`

## Core Mental Model

Readiness is the final "can this run?" read. It consumes several lower layers:

- dirty and patch truth
- validation
- availability
- admission
- step posture
- action-specific requirements

## How It Executes

1. lower-level reports are derived
2. the runtime collects their blocking consequences
3. `readiness()` exposes form-level submit posture
4. `actionReadiness(...)` exposes action-level posture

## Small Example

```ts
console.log(form.readiness().canSubmit);
console.log(form.readiness().blockers);
```

## Real Example

```ts
console.log(form.readiness().blockers.map((blocker) => blocker.kind));
console.log(form.actionReadiness("submit").blockers);
```

## How It Relates To Other Features

- Read [Dirty State](../changes/dirty-state.md) when the blocker is
  `unchanged`.
- Read [Validation Overview](../validation/validation-overview.md) when the
  blocker is validation-shaped.
- Read [Admission Rules](./admission-rules.md) when the blocker is
  permission-shaped.

## Inspection And Debugging

- `readiness().blockers` is the first read
- `actionReadiness(...)` tells you whether an action adds tighter blockers than
  the form as a whole
- `actions().plans` helps explain blocker combinations at action scope

## Anti-Patterns

- deciding submit state from one signal such as touched or invalid alone
- stripping blocker detail before it reaches the UI or logs

## Current Limits

- recovery actions for denied or rejected executions live in the actions
  section

## Related Docs

- [Admission Rules](./admission-rules.md)
- [Action Overview](../actions/action-overview.md)

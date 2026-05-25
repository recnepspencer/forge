# Approval, Signature, And Review Requirements

## What This Feature Is

This page covers the regulated admission postures that keep a field or action
from proceeding until approval, signature, review, or a required reason is
present.

## Why You Use It

- make regulated submit posture explicit
- keep "needs approval" separate from plain denial
- inspect capability-specific blockers for audit-heavy forms

## Stable Entry Points

- admission capabilities: `approval`, `signature`, `review`, `reason`
- `form.admission()`
- `form.actionReadiness(actionId)`

## Core Mental Model

These are not generic errors. They are named capability requirements with
their own posture:

- `requiresApproval`
- `requiresSignature`
- `requiresReview`
- `requiresReason`

## How It Executes

1. declare one regulated capability on a field or action
2. evaluate the current form values and binding context
3. the runtime emits the matching regulated posture
4. readiness and actions report the requirement without flattening it into
   plain denial

## Small Example

```ts
const report = form.admission();
console.log(report.summary.requiresApproval);
```

## Real Example

```ts
const plan = form.actionPlan("submit");

console.log(plan.admission.summary);
console.log(plan.readiness.blockers);
console.log(plan.regulatedActionBindings);
```

## How It Relates To Other Features

- Read [Admission Rules](./admission-rules.md) for the general capability
  model.
- Read [Submit Actions](../actions/submit-actions.md) when the regulated
  requirement is shaping the submit plan itself.

## Inspection And Debugging

- `admission().artifacts` shows which regulated posture was emitted
- `actionPlan("submit").regulatedActionBindings` gives you the action-side
  binding detail
- `actionReadiness("submit")` shows the blocker the UI actually sees

## Anti-Patterns

- mapping every regulated requirement to one generic "not allowed" message
- hiding approval or signature state entirely inside app-specific submit code

## Current Limits

- route-authority approval semantics belong in the later route-coupling docs

## Related Docs

- [Admission Rules](./admission-rules.md)
- [Submit Actions](../actions/submit-actions.md)

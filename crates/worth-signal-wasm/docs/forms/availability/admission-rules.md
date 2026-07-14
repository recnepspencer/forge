# Admission Rules

## What This Feature Is

Admission rules are the form lane for capability-based permission decisions
such as `edit`, `patch`, `submit`, `action`, `approval`, `signature`,
`review`, and `reason`.

## Why You Use It

- separate permission posture from plain availability
- declare field or action capabilities explicitly
- inspect stale or mismatched regulated bindings instead of guessing

## Stable Entry Points

- `admission: ({ field, action }) => ...`
- `form.admission()`
- `form.actionReadiness(actionId)`

## Core Mental Model

Availability answers "is this available right now?" Admission answers "is this
capability allowed right now?"

That is why admission posture includes states like `requiresApproval`,
`requiresSignature`, and `requiresReview`, not just `denied`.

## How It Executes

1. declare field or action capabilities
2. admission rules read current values and binding context
3. the runtime emits admission results such as `admitted`, `denied`, or
   `requiresApproval`
4. readiness and action planning consume that capability posture later

## Small Example

```ts
const form = signals.form({
  source: { approved: false },
  fields: ({ field }) => ({
    approved: field("approved"),
  }),
  admission: ({ action }) => ({
    submitRequiresApproval: action("submit", "approval", ["approved"], ({ approved }) => (
      approved ? "admitted" : "requiresApproval"
    )),
  }),
});

console.log(form.admission().artifacts);
```

## Real Example

```ts
const report = form.admission();

console.log(report.summary);
console.log(form.actionReadiness("submit"));
```

## How It Relates To Other Features

- Read [Field And Control Availability](./field-and-control-availability.md)
  when the issue is visibility or readonly state instead of permission.
- Read [Approval, Signature, And Review Requirements](./approval-signature-and-review-requirements.md)
  for the regulated-posture cases that often sit on top of admission rules.

## Inspection And Debugging

- `admission().artifacts` shows each capability decision
- `summary` shows whether the form is mostly admitted, denied, or waiting on a
  regulated requirement
- `actionReadiness(...)` shows the submit-facing blocker list

## Anti-Patterns

- treating `disabled` UI as if it already explained capability posture
- collapsing approval, signature, and review requirements into one generic
  denial

## Current Limits

- route-coupled authority requirements live in the later route-coupling docs

## Related Docs

- [Field And Control Availability](./field-and-control-availability.md)
- [Approval, Signature, And Review Requirements](./approval-signature-and-review-requirements.md)

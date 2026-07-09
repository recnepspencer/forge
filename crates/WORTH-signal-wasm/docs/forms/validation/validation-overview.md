# Validation Overview

## What This Feature Is

Validation turns correctness checks into typed results instead of raw error
strings.

## Why You Use It

- keep validation logic declarative and read-only
- derive readiness blockers from typed validation results
- separate valid, warning, invalid, pending, blocked, unavailable, and parse
  failure posture

## Stable Entry Points

- `validation: ({ field, form, asyncField }) => ...`
- `form.validation()`
- `form.visibleMessages()`
- `form.messages()`

## Core Mental Model

Validators read form truth. They do not mutate it.

The output is a typed validation result set. Message visibility and readiness
are derived from that set.

## How It Executes

1. field or form validators read current form truth
2. the runtime records typed validation results
3. visible messages are derived from those results and visibility posture
4. readiness can consume the results later

## Small Example

```ts
const form = signals.form({
  source: { title: "" },
  fields: ({ field }) => ({
    title: field("title"),
  }),
  validation: ({ field }) => ({
    titleRequired: field("title", (value) => (
      value.length > 0 || {
        kind: "invalid",
        message: {
          code: "title.required",
          severity: "error",
          audience: "user",
          visibility: "visible",
        },
      }
    )),
  }),
});
```

## Real Example

```ts
const report = form.validation();

console.log(report.summary);
console.log(form.visibleMessages());
console.log(form.readiness().blockers);
```

Validation owns correctness results. Visible messages and readiness consume
them without rewriting validation logic.

## How It Relates To Other Features

- Read [Parse Failures](./parse-failures.md) for raw-input parse boundary
  behavior.
- Read [Async Validation](./async-validation.md) for remote or deferred
  validator lifecycle.
- Read [Visible Messages](./visible-messages.md) for user-facing message reads.

## Inspection And Debugging

- `validation().artifacts` shows the raw validation results
- `validation().summary` shows counts by artifact class
- `validation().dependencyBreadth` shows validator breadth and dependencies

## Anti-Patterns

- mutating form state inside a validator
- collapsing all validation outcomes into one generic error string

## Current Limits

- validation declares async posture, but async execution is a separate lane
- route authority and resource reconciliation are outside this validation layer

## Related Docs

- [Parse Failures](./parse-failures.md)
- [Visible Messages](./visible-messages.md)
- [Async Validation](./async-validation.md)

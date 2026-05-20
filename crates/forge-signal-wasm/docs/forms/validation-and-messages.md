# Validation And Messages

## What This Feature Is

This feature turns parse failures, validation artifacts, visible messages, and
submit blockers into structured runtime-owned facts.

## Why You Use It

- keep parse failures separate from admitted draft truth
- declare field-local and cross-field validation without hand-managed error
  state
- derive visible messages and summary posture from one canonical artifact set

## Stable Entry Points

- validation builders inside `signals.form({... validation: ... })`
- `validation()`
- `visibleMessages()`
- `messages()`

## Core Mental Model

Validators read form truth; they do not mutate it. Parse failures, warnings,
invalid artifacts, pending artifacts, blocked artifacts, and unavailable
artifacts stay distinct. Message visibility is derived from those artifacts and
interaction/policy posture.

## How It Executes

The runtime evaluates field-local and cross-field validators over read-only
views, records typed artifacts, derives visible messages, then feeds those
artifacts into readiness and action planning.

## Small Example

```ts
const form = signals.form({
  source: { title: "" },
  fields: ({ field }) => ({ title: field("title") }),
  validation: ({ field }) => ({
    titleRequired: field("title", (value) => (
      value.length > 0
        ? { kind: "valid", field: "title", digest: value }
        : {
            kind: "invalid",
            field: "title",
            message: {
              code: "task.title.required",
              severity: "error",
              target: "title",
              audience: "user",
              visibility: "visible",
            },
          }
    )),
  }),
});
```

This is the smallest honest example because it shows the real unit of
validation truth: a typed artifact, not a string.

## Real Example

```ts
const report = form.validation();
const messages = form.visibleMessages();
const readiness = form.readiness();

console.log(report.summary);
console.log(messages.map((message) => message.code));
console.log(readiness.blockers);
```

Validation owns correctness artifacts. Message visibility owns presentation of
those artifacts. Readiness consumes both without rewriting either surface.

## How It Relates To Other Features

- Pair it with [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
  when unchanged forms should still deny submit.
- Pair it with [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
  for async validators and stale-safe completions.
- Pair it with [Presentation And External Lanes](./presentation-and-external-lanes.md)
  when visible message timing or settlement matters.

## Inspection And Debugging

- `validation()` shows typed artifacts, summaries, and dependency breadth.
- `visibleMessages()` shows what the user-facing lane can actually see now.
- `messages()` shows first-class message-lane posture and counts.
- `reportMessages(...)` and `clearMessages(...)` belong to the presentation
  lane, not the validation lane. Use them only when you need to report visible
  message settlement without forging new validation results.

## Anti-Patterns

- mutating form state inside validators
- collapsing parse failures and domain validation into one generic error string
- treating visible-message timing as validation truth

## Current Limits

- route authority and external handoff are not validation concerns
- validators can declare async posture, but async execution belongs to the async
  lifecycle lane
- host capability absence stays explicit rather than being forged into a local
  validation result

## Related Docs

- [Dirty, Patch, And Readiness](./dirty-patch-and-readiness.md)
- [Async Lifecycle And Canonicalization](./async-lifecycle-and-canonicalization.md)
- [Presentation And External Lanes](./presentation-and-external-lanes.md)

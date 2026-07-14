# Visible Messages

## What This Feature Is

Visible messages are the user-facing messages currently surfaced from
validation and the broader message system.

## Why You Use It

- render user-visible errors and warnings without rebuilding message logic
- keep message visibility separate from validation truth
- inspect summary, blocked, and visible message posture explicitly

## Stable Entry Points

- `form.visibleMessages()`
- `form.messages()`
- `form.validation()`

## Core Mental Model

Validation produces message records. Visibility decides which ones are
currently readable by users.

## How It Executes

1. validation emits results with message payloads
2. visibility posture admits some of them to the visible message list
3. `visibleMessages()` returns the current user-facing messages
4. `messages()` summarizes the broader message presentation lane

## Small Example

```ts
console.log(form.visibleMessages());
```

## Real Example

```ts
const report = form.validation();
const visible = form.visibleMessages();
const messages = form.messages();

console.log(report.summary);
console.log(visible.map((message) => message.code));
console.log(messages.summary);
```

## How It Relates To Other Features

- Read [Validation Overview](./validation-overview.md) for where validation
  messages come from.
- Read [Async Validation](./async-validation.md) when visible messages depend
  on pending or fulfilled remote checks.

## Inspection And Debugging

- `visibleMessages()` is the simplest user-facing read
- `messages().summary` shows active channel and semantic visible counts
- `validation().artifacts` shows the underlying validation results

## Anti-Patterns

- treating visible messages as the whole validation model
- rendering blocked or hidden messages as if they were visible user messages

## Current Limits

- richer message presentation and external message-lane lifecycle are part of a
  later forms docs section

## Related Docs

- [Validation Overview](./validation-overview.md)
- [Async Validation](./async-validation.md)

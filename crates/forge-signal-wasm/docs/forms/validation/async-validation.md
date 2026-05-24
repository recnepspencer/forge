# Async Validation

## What This Feature Is

Async validation is the forms lane for validators that resolve later than the
current synchronous read.

## Why You Use It

- run remote or deferred uniqueness checks
- keep pending, fulfilled, cancelled, timed out, and stale completion posture
  explicit
- inspect async validation lifecycle without inventing your own promise state

## Stable Entry Points

- `asyncField(...)`
- `form.startAsyncValidation(validationId)`
- `form.fulfillAsyncValidation(operationId, payload?)`
- `form.asyncValidationHistory()`
- `form.validation()`

## Core Mental Model

Async validation is a lifecycle layer on top of ordinary validation results.
The runtime owns the pending and completion history for you.

## How It Executes

1. declare an async validator with `asyncField(...)`
2. start one validation operation
3. validation report can expose pending posture
4. fulfill, reject, cancel, or time out the operation
5. history and visible validation state update from that result

## Small Example

```ts
const pending = form.startAsyncValidation("slugUnique");
form.fulfillAsyncValidation(pending.operationId, {
  reason: "slug is unique",
});
```

## Real Example

```ts
const pending = form.startAsyncValidation("slugUnique");

console.log(form.validation().summary.pending);

form.fulfillAsyncValidation(pending.operationId, {
  reason: "slug is unique",
});

console.log(form.asyncValidationHistory().at(-1)?.resultKind);
```

## How It Relates To Other Features

- Read [Validation Overview](./validation-overview.md) for the synchronous
  validation model around this lifecycle.
- Read [Visible Messages](./visible-messages.md) when async results surface user
  messages.

## Inspection And Debugging

- `validation().summary.pending` shows current pending count
- `asyncValidationHistory()` shows fulfilled, cancelled, superseded, or stale
  completion results
- `validation().dependencyBreadth` shows declared trigger posture

## Anti-Patterns

- treating async validation like a plain promise that can update any time
- ignoring stale completion posture and replaying older results into the form

## Current Limits

- this page covers validator lifecycle only
- action execution and server canonicalization are separate lanes

## Related Docs

- [Validation Overview](./validation-overview.md)
- [Visible Messages](./visible-messages.md)
- [Server Canonicalization](./server-canonicalization.md)

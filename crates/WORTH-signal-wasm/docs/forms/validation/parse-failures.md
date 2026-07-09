# Parse Failures

## What This Feature Is

Parse failures are the validation results emitted when raw input cannot cross
the parse boundary into committed draft truth.

## Why You Use It

- keep malformed raw input separate from committed draft values
- explain why a field still shows its previous effective value
- surface parse errors through readiness and visible messages honestly

## Stable Entry Points

- field option `parse`
- `fieldHandle.input(...)`
- `fieldHandle.commitInput(...)`
- `fieldHandle.diagnostics().pendingRawInput`
- `form.validation()`

## Core Mental Model

Raw input exists before draft truth. A parse failure means the raw value did
not become committed draft state.

## How It Executes

1. raw input is reported to a field
2. commit crosses the parse boundary
3. parse succeeds and draft updates, or parse fails and a typed validation
   result is
   recorded
4. effective value and readiness reflect that result

## Small Example

```ts
form.fields.age.input("not-a-number").commitInput();

console.log(form.validation().summary.parseFailure);
```

## Real Example

```ts
form.fields.age.input("not-a-number").commitInput();

console.log(form.fields.age.effectiveValue());
console.log(form.fields.age.diagnostics().pendingRawInput);
console.log(form.readiness().blockers);
```

The effective value can stay on the previous admitted number while readiness
denies submit with a `validation:parseFailure` blocker.

## How It Relates To Other Features

- Read [Validation Overview](./validation-overview.md) for the broader
  validation model around parse failures.
- Read [Patch Plans](../changes/patch-plans.md) for how uncommitted raw input
  and parse failures affect write posture differently.

## Inspection And Debugging

- field diagnostics show whether raw input is still pending
- `validation().artifacts` shows the parse failure result
- `readiness().blockers` shows the submit-facing consequence

## Anti-Patterns

- treating parse failures like committed draft values
- clearing parse problems by mutating effective state directly

## Current Limits

- parse failure is local to the field/input boundary
- async validation and server canonicalization are separate lanes

## Related Docs

- [Validation Overview](./validation-overview.md)
- [Patch Plans](../changes/patch-plans.md)
- [Async Validation](./async-validation.md)

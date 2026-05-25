# Raw Input, Compose, And Commit

## What This Feature Is

This page covers the input boundary between raw input, composition, and
committed draft truth.

## Why You Use It

- wire inputs that emit raw text before a committed value
- handle IME or composition-aware controls honestly
- separate "user is typing" from "form draft changed"

## Stable Entry Points

- field adapter capabilities:
  `reportsRawInput`, `reportsCommitBoundary`, `reportsComposition`
- `fieldHandle.input(rawValue, options?)`
- `fieldHandle.compose(rawValue)`
- `fieldHandle.commitInput(parser?)`
- `fieldHandle.diagnostics()`

## Core Mental Model

Raw input is not the same as admitted draft truth.

The form can retain:

- raw input activity
- composition state
- a later commit into draft truth

## How It Executes

1. declare what the adapter can report
2. call `input(...)`, `compose(...)`, or `commitInput(...)` as the control
   emits events
3. the runtime updates diagnostics, interaction state, parse failures, and
   eventually draft truth

## Small Example

```ts
form.fields.title.input("Shi", { source: "typing" });
form.fields.title.input("Ship", { source: "typing", commit: true });
```

## Real Example

```ts
form.fields.query.compose("shi");
form.fields.query.compose("ship");
form.fields.query.commitInput((raw) => raw.trim());

console.log(form.fields.query.diagnostics().pendingRawInput);
console.log(form.fields.query.diagnostics().parseFailure);
```

## How It Relates To Other Features

- Read [Focus, Blur, Touch, And Visit Reporting](./focus-blur-touch-and-visit-reporting.md)
  for the neighboring interaction lane.
- Read [Parse Failures](../validation/parse-failures.md) when the commit path
  can fail validation or parsing.

## Inspection And Debugging

- `pendingRawInput` shows whether raw input has not fully settled
- `parseFailure` shows whether the commit path failed
- `interaction().history` keeps the raw-input and composition timeline

## Anti-Patterns

- calling `set(...)` for every keystroke when the control really has a raw
  input boundary
- pretending composition events do not matter for search or text-entry controls

## Current Limits

- the form does not invent commit boundaries for an adapter that cannot report
  them

## Related Docs

- [Parse Failures](../validation/parse-failures.md)
- [Focus, Blur, Touch, And Visit Reporting](./focus-blur-touch-and-visit-reporting.md)

# Focus, Touch, And Visited State

## What This Feature Is

This page covers the interaction history the form controller keeps for each
field.

## Why You Use It

- inspect touched, visited, focused, and blurred field state
- keep composition and raw-input history on the controller surface
- understand submit intent without inventing a second UI state machine

## Stable Entry Points

- `form.interaction()`
- `form.reportFieldInteraction(fieldId, event)`
- `form.reportSubmitIntent(...)`
- `form.clearSubmitIntent(...)`

## Core Mental Model

Interaction state is separate from dirty state and validation. A field can be
touched without being dirty, and a form can have submit intent even when submit
is denied.

## How It Executes

1. field interaction events are reported
2. the runtime updates per-field interaction state
3. summary and history are derived from those events
4. validation, accessibility, and later presentation reads can consume that
   state

## Small Example

```ts
form.reportFieldInteraction("title", {
  kind: "focus",
  source: "pointer",
});

console.log(form.interaction().summary);
```

## Real Example

```ts
form.reportFieldInteraction("title", {
  kind: "input",
  source: "typing",
  rawValue: "Ship docs",
});

form.reportSubmitIntent({ source: "keyboard" });

console.log(form.interaction().fields);
console.log(form.interaction().history);
```

## How It Relates To Other Features

- Read [Input Capabilities](./input-capabilities.md) when interaction support
  differs by adapter.
- Read [Visible Messages](../validation/visible-messages.md) when focus and
  submit intent need to line up with user-facing messages.

## Inspection And Debugging

- `interaction().fields` shows current per-field state
- `summary.focusedField` and `summary.submitIntent` are the fastest reads
- `history` shows the actual interaction event stream

## Anti-Patterns

- using touched state as a substitute for dirty or validation state
- keeping separate submit-intent booleans outside the controller

## Current Limits

- route-coupled interaction flows belong in later docs

## Related Docs

- [Input Capabilities](./input-capabilities.md)
- [Visible Messages](../validation/visible-messages.md)

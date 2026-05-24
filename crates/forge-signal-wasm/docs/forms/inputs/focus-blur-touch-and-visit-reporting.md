# Focus, Blur, Touch, And Visit Reporting

## What This Feature Is

This page covers the interaction-reporting lane most inputs use beyond raw
value entry.

## Why You Use It

- keep focus and blur state on the form controller
- distinguish touched, visited, focused, and blurred state
- support better validation timing and richer control UX

## Stable Entry Points

- field adapter capability `reportsFocus`
- `fieldHandle.touch()`
- `fieldHandle.visit()`
- `fieldHandle.focus()`
- `fieldHandle.blur()`
- `form.reportFieldInteraction(...)`
- `form.interaction()`

## Core Mental Model

Focus, blur, touch, and visit are not decoration. They are first-class form
interaction truth.

## How It Executes

1. declare whether the input can report focus
2. send interaction events through field handles or the controller
3. the runtime retains interaction state and history

## Small Example

```ts
form.fields.title.focus();
form.fields.title.blur();

console.log(form.fields.title.diagnostics().interaction);
```

## Real Example

```ts
form.reportFieldInteraction("query", {
  kind: "focus",
  source: "pointer",
});

form.reportFieldInteraction("query", {
  kind: "input",
  source: "keyboard",
  rawValue: "ship docs",
});

console.log(form.interaction().history);
```

## How It Relates To Other Features

- Read [Input Capability Matrix](./input-capability-matrix.md) for capability
  readiness and unavailability.
- Read [Focus, Touch, And Visited State](../interaction/focus-touch-and-visited-state.md)
  for the broader interaction report once the input is wired up.

## Inspection And Debugging

- field diagnostics show current interaction state per field
- `form.interaction().history` shows the retained timeline
- focus posture stays explicit when the adapter cannot report it

## Anti-Patterns

- inventing separate touched or focused state stores beside the form
- assuming every external control can report focus honestly

## Current Limits

- host-level environment facts still belong in the interaction and host docs,
  not the input adapter declaration

## Related Docs

- [Input Capability Matrix](./input-capability-matrix.md)
- [Focus, Touch, And Visited State](../interaction/focus-touch-and-visited-state.md)

# Input Adapter Overview

## What This Feature Is

This page covers the adapter declaration lane behind real form inputs and
controls.

## Why You Use It

- tell the form what an input can honestly report
- integrate dropdowns, searches, editors, and other non-native controls without
  pretending they behave like signal-native inputs
- keep control differences explicit on the form surface

## Stable Entry Points

- field option `input: { adapter: ... }`
- `form.inputAdapters()`
- `form.inputCapabilities()`
- `fieldHandle.diagnostics().inputAdapter`

## Core Mental Model

The input adapter is the contract between a UI control and the form runtime.

It answers questions like:

- can this control report raw input?
- can it tell the form where a commit boundary happened?
- can it report composition state?
- can it report focus?
- can it participate in label/help/message tracks and layout sync?

## How It Executes

1. declare field adapter capability hints
2. the runtime normalizes them per field
3. the controller exposes both per-field and aggregate capability reports

## Small Example

```ts
const form = signals.form({
  source: { title: "Ship docs" },
  fields: ({ field }) => ({
    title: field("title", {
      input: {
        adapter: {
          tier: "externalImperative",
          reportsRawInput: true,
          reportsCommitBoundary: true,
          reportsComposition: false,
          reportsFocus: true,
        },
      },
    }),
  }),
});

console.log(form.inputAdapters());
```

## Real Example

```ts
const title = form.inputCapability("title");

console.log(title?.tier);
console.log(title?.capabilities);
console.log(title?.unavailableCapabilities);
console.log(form.fields.title.diagnostics().inputAdapter);
```

## How It Relates To Other Features

- Read [Raw Input, Compose, And Commit](./raw-input-compose-and-commit.md) for
  the value-entry side of the same lane.
- Read [Input Capability Matrix](./input-capability-matrix.md) for the main
  inspection surface.
- Read [Layout Configuration Reference](../layout/layout-configuration-reference.md)
  for the layout side of the same control.

## Inspection And Debugging

- `form.inputAdapters()` shows the declared adapter posture by field
- `form.inputCapabilities()` shows admitted capabilities and unavailable
  capability counts
- `fieldHandle.diagnostics().inputAdapter` is the most local read

## Anti-Patterns

- assuming every external control behaves like a native text input
- using adapter declarations as a substitute for layout configuration
- hiding unavailable capability truth inside renderer-specific code

## Current Limits

- the adapter declaration does not create the UI control for you

## Related Docs

- [Raw Input, Compose, And Commit](./raw-input-compose-and-commit.md)
- [Input Capability Matrix](./input-capability-matrix.md)

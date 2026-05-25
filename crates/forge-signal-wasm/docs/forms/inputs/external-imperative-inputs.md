# External Imperative Inputs

## What This Feature Is

This page covers inputs that are not signal-native and need an explicit bridge
to the form runtime.

## Why You Use It

- integrate third-party inputs, editors, pickers, or widgets
- declare exactly which parts of input behavior the widget can report
- keep the form honest when a control is partially capable

## Stable Entry Points

- adapter tier `externalImperative`
- field option `input: { adapter: ... }`
- `form.bindInput(fieldId, options?)`
- field handle methods such as `input(...)`, `commitInput()`, `focus()`,
  `blur()`, and `set(...)`
- `form.inputCapabilities()`

## Core Mental Model

An imperative widget can still be a first-class form input. It just should not
pretend to have capabilities it does not actually expose.

## How It Executes

1. declare the field with an `externalImperative` adapter
2. wire widget events into the field handle or controller
3. inspect capability posture and interaction state through the form

## Small Example

```ts
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
})
```

## Real Example

```ts
const statusInput = form.bindInput("status");

thirdPartyPicker.onChange(statusInput.set);
thirdPartyPicker.onOpen(statusInput.focus);
thirdPartyPicker.onClose(statusInput.blur);

console.log(form.fields.status.diagnostics());
```

## How It Relates To Other Features

- Read [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
  for the most common imperative control families.
- Read [Control-Level Availability](./control-level-availability.md) when the
  widget needs separate enable/disable posture.

## Inspection And Debugging

- the adapter tier tells you you are on the imperative lane
- `bindInput(...)` gives you the common callback bundle when the widget mostly
  needs event wiring
- field diagnostics show what the widget actually reported
- input capabilities show what the widget could not promise

## Anti-Patterns

- treating an imperative widget like a fully signal-native text input
- pushing widget-local booleans for focus, pending raw input, or commit state
  into a separate store

## Current Limits

- the form gives you the integration boundary; it does not wrap every third
  party control for you

## Related Docs

- [Dropdowns, Comboboxes, And Search](./dropdowns-comboboxes-and-search.md)
- [Control-Level Availability](./control-level-availability.md)

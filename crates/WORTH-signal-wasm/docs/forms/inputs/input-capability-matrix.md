# Input Capability Matrix

## What This Feature Is

This page covers the aggregate capability report behind
`form.inputCapabilities()`.

## Why You Use It

- inspect what each field input can honestly do
- debug unavailable layout-track or commit-boundary support
- compare signal-native, signal-bridge, and external-imperative inputs

## Stable Entry Points

- `form.inputCapabilities()`
- `form.inputAdapters()`

## Core Mental Model

The capability matrix is the inspection read for input-tool integration.

It tells you which inputs can report:

- raw input
- commit boundaries
- composition
- focus
- label/help/message track support
- min-height sync
- responsive token support

## How It Executes

1. each field declares input adapter capabilities
2. the runtime normalizes them into a field-by-field capability report
3. the aggregate summary exposes unavailable counts by capability family

## Small Example

```ts
console.log(form.inputCapabilities().summary);
```

## Real Example

```ts
const title = form.inputCapability("title");

console.log(title?.tier);
console.log(title?.posture);
console.log(title?.capabilities);
console.log(title?.unavailableCapabilities);
```

## How It Relates To Other Features

- Read [Input Adapter Overview](./input-adapter-overview.md) for the adapter
  declaration side.
- Read [Layout Configuration Reference](../layout/layout-configuration-reference.md)
  when unavailable capability counts affect layout behavior.

## Inspection And Debugging

- `summary` gives counts by unavailable capability kind
- `fields[*]` gives the per-field explanation
- `inputAdapters()` shows the declaration-shaped companion read

## Anti-Patterns

- assuming capability support from the control's marketing name instead of the
  declared adapter posture
- using aggregate counts when the bug is clearly one field

## Current Limits

- this report explains capability support; it does not render or wrap the input
  control

## Related Docs

- [Input Adapter Overview](./input-adapter-overview.md)
- [Layout Configuration Reference](../layout/layout-configuration-reference.md)

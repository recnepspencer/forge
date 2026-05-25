# Input Capabilities

## What This Feature Is

This page covers the declared capabilities of each field's input adapter.

## Why You Use It

- know whether a field can report raw input, focus, composition, or layout
  tracks
- debug why one adapter supports a feature and another one does not
- keep adapter differences explicit instead of implied

## Stable Entry Points

- field option `adapter` or `inputAdapter`
- `form.inputCapabilities()`
- `form.inputAdapters()`

## Core Mental Model

Input capability is a declaration question, not a guess. The controller
exposes which adapter features are ready or unavailable for each field.

## How It Executes

1. field adapters declare their capabilities
2. the runtime normalizes them per field
3. `inputCapabilities()` exposes the declared support and unavailability

## Small Example

```ts
console.log(form.inputCapabilities().summary);
```

## Real Example

```ts
const title = form.inputCapability("title");

console.log(title?.tier);
console.log(title?.capabilities);
console.log(title?.unavailableCapabilities);
```

## How It Relates To Other Features

- Read [Input Adapter Overview](../inputs/input-adapter-overview.md) for the
  declaration side and integration guidance.
- Read [Dropdowns, Comboboxes, And Search](../inputs/dropdowns-comboboxes-and-search.md)
  for common real control families.
- Read [Focus, Touch, And Visited State](./focus-touch-and-visited-state.md)
  when capability differences affect interaction history.
- Read [Layout Hints](../layout/layout-hints.md) when label/help/message track
  support affects layout output.

## Inspection And Debugging

- `summary` shows counts by unavailable capability kind
- `fields[*].posture` tells you whether a field is ready or unavailable
- `inputAdapters()` gives the lower-level adapter declaration summary

## Anti-Patterns

- assuming every input can report focus or raw-input boundaries
- hiding adapter limitations in renderer-specific code

## Current Limits

- host-level capabilities like online or persistence live in the host facts
  docs

## Related Docs

- [Input Adapter Overview](../inputs/input-adapter-overview.md)
- [Dropdowns, Comboboxes, And Search](../inputs/dropdowns-comboboxes-and-search.md)
- [Focus, Touch, And Visited State](./focus-touch-and-visited-state.md)
- [Host Facts](./host-facts.md)

# Layout Configuration Reference

## What This Feature Is

This page is the direct answer to "what can I actually configure on form
layout?"

## Why You Use It

- see the full declared layout surface in one place
- separate field layout options from measured geometry
- know which knobs belong to layout and which belong to input adapter posture

## Stable Entry Points

- field option `layout`
- `form.layout()`

## Core Mental Model

Layout configuration is declarative.

It answers:

- where does the field go?
- how dense is it?
- how should it align?
- what minimum height or growth hint does it carry?
- what responsive tokens does it declare?

It does not answer actual measured geometry. That belongs to layout
measurement.

## How It Executes

1. declare layout on fields or steps
2. the runtime normalizes those hints into sections, rows, and field entries
3. capability posture stays explicit when an input cannot honor part of the
   layout contract

## Small Example

```ts
title: field("title", {
  layout: {
    row: "hero",
    column: "left",
    density: "comfortable",
    alignment: "stretch",
    minHeight: 44,
    grow: true,
    wrap: false,
    responsive: ["md", "lg"],
  },
})
```

## Real Example

```ts
const field = form.layoutField("title");

console.log({
  row: field?.row,
  column: field?.column,
  density: field?.density,
  alignment: field?.alignment,
  minHeight: field?.minHeight,
  grow: field?.grow,
  wrap: field?.wrap,
  responsive: field?.responsive,
});
```

## How It Relates To Other Features

- Read [Rows, Sections, And Placement](./rows-sections-and-placement.md) for
  how these declarations become layout structure.
- Read [Control Sizing](./control-sizing.md) for the sizing side.
- Read [Input Capability Matrix](../inputs/input-capability-matrix.md) when
  adapter capability limits affect the layout result.

## Inspection And Debugging

- `layout` is the clearest declaration lane for new code
- `form.layout().fields` is the first read for declared layout
- `capabilityPosture` shows whether a field's input can honor the declared
  layout capabilities

## Anti-Patterns

- treating measured geometry as if it were declarative layout config
- putting layout config inside adapter declarations

## Current Limits

- layout config shapes intent, not final DOM geometry

## Related Docs

- [Rows, Sections, And Placement](./rows-sections-and-placement.md)
- [Control Sizing](./control-sizing.md)

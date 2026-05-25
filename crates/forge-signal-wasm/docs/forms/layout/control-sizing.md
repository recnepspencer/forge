# Control Sizing

## What This Feature Is

This page covers the declared sizing surface for labels and controls and the
measured follow-up layer that can refine it.

## Why You Use It

- configure minimum control height
- understand growth and wrapping behavior
- separate declared size intent from measured size facts

## Stable Entry Points

- field layout options: `minHeight`, `grow`, `wrap`
- `form.layout()`
- `form.layoutMeasurement()`

## Core Mental Model

Sizing has two layers:

- declared size intent in layout
- measured size facts in layout measurement

If you want to configure height, start with layout.
If you want to inspect the actual rendered height, read layout measurement.

## How It Executes

1. fields declare `minHeight`, `grow`, and `wrap`
2. the layout report normalizes those hints by field and row
3. layout measurement can later record the actual row and track heights

## Small Example

```ts
const title = form.layoutField("title");
console.log(title?.minHeight);
```

## Real Example

```ts
const row = form.layout().rows.find((entry) => entry.id === "hero");
const snapshot = form.layoutMeasurement().latestSnapshot;

console.log(row?.maxMinHeight);
console.log(snapshot?.rows.find((entry) => entry.row === "hero"));
```

## How It Relates To Other Features

- Read [Layout Configuration Reference](./layout-configuration-reference.md)
  for the full declaration surface.
- Read [Layout Measurement](./layout-measurement.md) for the measured follow-up
  layer.

## Inspection And Debugging

- `layout().fields[*].minHeight` shows declared field sizing
- `layout().rows[*].maxMinHeight` shows normalized row sizing intent
- `layoutMeasurement().latestSnapshot` shows actual retained row heights

## Anti-Patterns

- expecting measured geometry from layout declaration alone
- burying control-height policy entirely in renderer code

## Current Limits

- media- or widget-specific geometry still depends on the control and renderer

## Related Docs

- [Layout Measurement](./layout-measurement.md)
- [Layout Configuration Reference](./layout-configuration-reference.md)

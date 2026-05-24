# Layout Overview

## What This Feature Is

This page covers the derived layout report for one form.

## Why You Use It

- inspect rows, columns, sections, and field hints from the controller
- keep layout structure declared beside the form instead of rebuilding it in
  the renderer
- understand which layout capabilities are unavailable in the current adapter
- separate declared layout config from measured geometry

## Stable Entry Points

- field options such as `row`, `column`, `density`, `alignment`, `minHeight`
- `form.layout()`

## Core Mental Model

Layout is a declared hint layer, not semantic form truth. The runtime reads
field and step hints, normalizes them, and exposes one layout report.

## How It Executes

1. field and step layout hints are declared
2. the runtime normalizes them into sections, rows, and field hints
3. unsupported layout capabilities stay explicit in the report

## Small Example

```ts
console.log(form.layout());
```

## Real Example

```ts
const layout = form.layout();

console.log(layout.sections);
console.log(layout.rows);
console.log(layout.fields);
```

## How It Relates To Other Features

- Read [Layout Configuration Reference](./layout-configuration-reference.md)
  for the full declaration side.
- Read [Layout Hints](./layout-hints.md) for the concise authoring entry point.
- Read [Layout Measurement](./layout-measurement.md) for the measured
  follow-up layer.

## Inspection And Debugging

- `layout().fields` shows the normalized per-field hints
- `layout().rows` shows row-level structure
- `summary.unavailableFields` tells you whether the adapter can honor all
  declared hints

## Anti-Patterns

- treating layout hints like semantic field state
- rebuilding row and section structure in app code when the controller already
  exposes it

## Current Limits

- presentation timing for layout updates belongs in the lifecycle section, not
  the layout section

## Related Docs

- [Layout Configuration Reference](./layout-configuration-reference.md)
- [Layout Hints](./layout-hints.md)
- [Layout Measurement](./layout-measurement.md)

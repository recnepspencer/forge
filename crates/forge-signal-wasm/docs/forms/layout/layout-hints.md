# Layout Hints

## What This Feature Is

This page covers the declaration-side layout hints on fields and steps.

## Why You Use It

- place fields into rows and columns
- choose density, alignment, wrapping, and growth hints
- inspect which declarations actually reached the layout report

## Stable Entry Points

- field options: `row`, `column`, `density`, `alignment`, `minHeight`,
  `grow`, `wrap`, `responsive`
- field option `layout`
- step options `density`, `alignment`, `responsive`
- `form.layout()`

## Core Mental Model

Layout hints are normal form declarations. They travel with the field or step
instead of living in a second parallel layout map.

## How It Executes

1. declare field or step layout hints
2. the runtime normalizes those hints
3. `form.layout()` exposes the normalized shape

## Small Example

```ts
title: field("title", {
  row: "hero",
  column: "left",
  minHeight: 44,
  grow: true,
})
```

## Real Example

```ts
const hint = form.layoutField("title");

console.log(hint?.row);
console.log(hint?.column);
console.log(hint?.tracks);
```

## How It Relates To Other Features

- Read [Layout Configuration Reference](./layout-configuration-reference.md)
  when you want the full "what can I configure?" answer.
- Read [Control Sizing](./control-sizing.md) for the sizing questions people
  usually ask next.
- Read [Accessibility Artifacts](./accessibility-artifacts.md) for label and
  described-by output on the accessibility side.

## Inspection And Debugging

- `layout().fields[*].tracks` shows label, control, help, and message tracks
- `capabilityPosture` shows whether the current adapter can honor the hints

## Anti-Patterns

- keeping a second renderer-only layout schema beside the form declaration
- assuming every adapter can honor every layout hint

## Current Limits

- measured geometry is a separate layer, not part of the declaration hints

## Related Docs

- [Layout Overview](./layout-overview.md)
- [Layout Configuration Reference](./layout-configuration-reference.md)
- [Control Sizing](./control-sizing.md)

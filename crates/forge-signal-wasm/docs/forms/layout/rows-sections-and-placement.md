# Rows, Sections, And Placement

## What This Feature Is

This page covers how declared field layout becomes sections, rows, and
placement structure.

## Why You Use It

- understand where a field lands
- inspect row grouping and section grouping
- debug placement without reconstructing layout structure in renderer code

## Stable Entry Points

- field options `row` and `column`
- `form.layout().sections`
- `form.layout().rows`
- `form.layout().fields`

## Core Mental Model

Fields declare placement locally. The form runtime turns those local hints into
one normalized layout structure.

## How It Executes

1. fields declare row and column hints
2. the runtime groups those fields into rows and sections
3. the layout report exposes the resulting normalized placement

## Small Example

```ts
title: field("title", { row: "hero", column: "left" })
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
  for the declaration knobs.
- Read [Label, Help, Message, And Control Tracks](./label-help-and-message-tracks.md)
  for the within-row track model.

## Inspection And Debugging

- `sections[*].fields` and `sections[*].rows` show section grouping
- `rows[*].fields` and `rows[*].columns` show row grouping
- `fields[*].row` and `fields[*].column` show where one field landed

## Anti-Patterns

- rebuilding row maps manually in UI code when the form already exposes them
- assuming placement is purely renderer-owned

## Current Limits

- actual geometry still belongs to layout measurement

## Related Docs

- [Layout Configuration Reference](./layout-configuration-reference.md)
- [Layout Measurement](./layout-measurement.md)

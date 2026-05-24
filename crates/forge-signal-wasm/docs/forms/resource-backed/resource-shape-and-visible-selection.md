# Resource Shape And Visible Selection

## What This Feature Is

This page covers the shape and visible-selection reads attached to a
resource-backed form source.

## Why You Use It

- understand what kind of resource line the form is bound to
- inspect how the visible source selection was chosen
- debug speculative, restored, or merged visible resource state

## Stable Entry Points

- `form.resourceSource()?.shape`
- `form.resourceSource()?.visibleSelection`

## Core Mental Model

Shape tells you what kind of resource line the form is bound to. Visible
selection tells you which source version is currently feeding the form and why.

## How It Executes

1. the runtime derives resource shape from the bound line
2. the visible selection tracks the current committed, speculative, restored,
   or merged resource source
3. the form uses that visible selection as source truth

## Small Example

```ts
console.log(form.resourceSource()?.shape);
console.log(form.resourceSource()?.visibleSelection);
```

## Real Example

```ts
const resource = form.resourceSource();

console.log(resource?.shape.familyKind);
console.log(resource?.shape.patchLowering);
console.log(resource?.visibleSelection.kind);
console.log(resource?.visibleSelection.source);
```

## How It Relates To Other Features

- Read [Resource Line Source](./resource-line-source.md) for the larger
  resource-backed source report.
- Read [Resource Settlement](./resource-settlement.md) when visible selection
  changes because of delivery or confirmation.

## Inspection And Debugging

- `shape.familyKind` and `patchLowering` explain the resource-side shape
- `visibleSelection.kind` and `source` explain where the current source came
  from
- branch and rebase checks explain why a visible selection was accepted

## Anti-Patterns

- assuming a visible resource source is always the last committed server value
- ignoring visible-selection kind during replay, restore, or merge flows

## Current Limits

- resource-family topology details live in the resource docs

## Related Docs

- [Resource Line Source](./resource-line-source.md)
- [Resource Settlement](./resource-settlement.md)

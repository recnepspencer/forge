# External Lanes

## What This Feature Is

This page covers presentation lanes that target route, modal, or external
surfaces rather than ordinary inline form content.

## Why You Use It

- inspect which presentation lanes are external-facing
- keep modal, route, and external surface state on typed controller reads
- understand unavailable external presentation without inventing app-local
  flags

## Stable Entry Points

- `form.presentation()`
- `form.handoff()`
- `form.exit()`
- `form.media()`

## Core Mental Model

External lanes are still form presentation state. They just target surfaces
outside the ordinary inline field tree.

## How It Executes

1. one or more presentation lanes are declared or updated
2. the runtime records scope kind and target details
3. summary and history reads expose the external-facing state

## Small Example

```ts
console.log(form.handoff().summary);
console.log(form.exit().summary);
```

## Real Example

```ts
console.log(form.handoff().summary.scopeKind);
console.log(form.exit().summary.scopeKind);
console.log(form.media().summary.scopeKind);
```

## How It Relates To Other Features

- Read [Handoffs](./handoffs.md) for the handoff-specific lane.
- Read [Media Visibility](../media/media-visibility.md) when the external
  surface is a media preview or capture flow.

## Inspection And Debugging

- `presentation().lanes` shows the broader lifecycle state
- handoff, exit, and media summaries give the clearest external-surface reads

## Anti-Patterns

- treating every modal or external surface like a separate unrelated state
  machine
- hiding external-surface availability entirely in renderer code

## Current Limits

- route authority is a separate concern from external presentation

## Related Docs

- [Handoffs](./handoffs.md)
- [Media Visibility](../media/media-visibility.md)

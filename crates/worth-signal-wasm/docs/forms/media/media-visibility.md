# Media Visibility

## What This Feature Is

This page covers the media presentation lane for preview, capture, crop, and
annotate surfaces.

## Why You Use It

- inspect current media preview or capture state from the controller
- keep media UI state separate from attachment identity and transfer state
- understand when media presentation is unavailable or settling

## Stable Entry Points

- `form.media()`
- `form.presentation()`

## Core Mental Model

Media visibility is about the current media surface, not the attachment field's
stored value. It tells you what preview/capture surface is active and what
state it is in.

## How It Executes

1. media presentation updates are reported
2. the runtime derives current media state and history
3. summary exposes scope, mode, target, and status

## Small Example

```ts
console.log(form.media().summary);
```

## Real Example

```ts
console.log(form.media().current);
console.log(form.media().history);
console.log(form.presentation().lanes.find((lane) => lane.lane === "media"));
```

## How It Relates To Other Features

- Read [Attachments](./attachments.md) for the attachment presentation lane.
- Read [External Lanes](../lifecycle/external-lanes.md) for the broader
  external-surface presentation model.

## Inspection And Debugging

- `summary.mode` shows preview/capture/crop/annotate mode
- `current.operation` shows the latest media transition
- `history` shows failed or unavailable media updates over time

## Anti-Patterns

- treating media preview state as if it were the persisted attachment value
- burying media visibility entirely in renderer-local state

## Current Limits

- media-specific processing flows live in later docs

## Related Docs

- [Attachments](./attachments.md)
- [External Lanes](../lifecycle/external-lanes.md)

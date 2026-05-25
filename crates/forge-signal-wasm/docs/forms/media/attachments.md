# Attachments

## What This Feature Is

This page covers the attachment presentation lane for selection, staging,
preview, removal, and clear operations.

## Why You Use It

- inspect current attachment UI state from the controller
- keep attachment selection and staging history on one surface
- distinguish attachment presentation from transfer state

## Stable Entry Points

- `form.attachments()`

## Core Mental Model

Attachment presentation is about what the user is doing with attachments right
now. It is separate from attachment identity and separate from resource-backed
transfer state.

## How It Executes

1. attachment presentation updates are reported
2. the runtime derives current attachment state and history
3. summary exposes selected, staged, and failed counts

## Small Example

```ts
console.log(form.attachments().summary);
```

## Real Example

```ts
console.log(form.attachments().current);
console.log(form.attachments().history);
```

## How It Relates To Other Features

- Read [Attachment Transfers](./attachment-transfers.md) when the attachment is
  also connected to a resource transfer surface.
- Read [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)
  for the field declaration side.

## Inspection And Debugging

- `summary.selectedCount` and `summary.stagedCount` are the fastest reads
- `current.operation` tells you the active attachment presentation action
- `history` shows failed or unavailable updates over time

## Anti-Patterns

- mixing attachment presentation with attachment identity or transfer state
- keeping a second selected-file state store beside the controller

## Current Limits

- resource-backed upload/download lifecycle belongs in the resource-backed docs

## Related Docs

- [Attachment Transfers](./attachment-transfers.md)
- [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)

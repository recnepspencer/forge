# Attachment Transfers

## What This Feature Is

This page covers the resource transfer readback associated with attachment and
evidence fields.

## Why You Use It

- inspect upload, processing, and download state for attachment fields
- keep transfer-surface mapping explicit
- understand why a field is ready, busy, or unavailable for transfer work

## Stable Entry Points

- `form.attachmentTransfers()`

## Core Mental Model

Attachment transfers are derived from attachment fields plus a resource-backed
transfer surface. A field can have an attachment without having a transfer
mapping.

## How It Executes

1. the runtime reads attachment/evidence fields
2. it tries to match them to the resource transfer surface
3. each field gets a transfer report with status, mapping, and download
   compatibility

## Small Example

```ts
console.log(form.attachmentTransfers().summary);
```

## Real Example

```ts
const field = form.attachmentTransfers().fields.find((entry) => entry.field === "evidence");

console.log(field?.status);
console.log(field?.upload);
console.log(field?.processing);
console.log(field?.matchingDownloadDescriptors);
```

## How It Relates To Other Features

- Read [Attachments](./attachments.md) for the UI presentation lane.
- Read [Resource Line Source](../resource-backed/resource-line-source.md) when
  the transfer readback comes from a resource-backed form.

## Inspection And Debugging

- `bindingKind` tells you whether the field mapped to the transfer surface
- `status` tells you whether the transfer read is ready, busy, or unavailable
- descriptor counts help explain download compatibility

## Anti-Patterns

- assuming every attachment field automatically maps to upload/download state
- collapsing upload, processing, and download into one generic boolean

## Current Limits

- richer binary/media workflows belong in later docs

## Related Docs

- [Attachments](./attachments.md)
- [Resource Line Source](../resource-backed/resource-line-source.md)

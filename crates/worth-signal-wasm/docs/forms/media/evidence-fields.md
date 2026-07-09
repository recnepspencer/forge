# Evidence Fields

## What This Feature Is

This page explains when to choose an evidence field and how it connects to the
attachment and media surfaces.

## Why You Use It

- understand the media-facing consequences of using `evidence(...)`
- inspect transfer and presentation reads for evidence-shaped fields
- keep evidence field behavior discoverable without losing the earlier field
  declaration docs

## Stable Entry Points

- `evidence(...)`
- `form.attachments()`
- `form.attachmentTransfers()`

## Core Mental Model

An evidence field is still a form field with attachment identity, but it often
participates in richer attachment, transfer, and media flows than a plain
scalar field.

## How It Executes

1. declare an evidence field with attachment identity
2. field writes preserve evidence identity
3. attachment and transfer reports can reflect that field later

## Small Example

```ts
fields: ({ evidence }) => ({
  receipt: evidence("receipt", { attachmentIdentity: "digest" }),
})
```

## Real Example

```ts
console.log(form.fields.receipt.attachmentIdentity());
console.log(form.attachmentTransfers().fields.find((entry) => entry.field === "receipt"));
```

## How It Relates To Other Features

- Read [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)
  for the declaration-side field model.
- Read [Attachment Transfers](./attachment-transfers.md) for the transfer
  readback side.

## Inspection And Debugging

- `attachmentIdentity()` shows the evidence identity
- attachment transfer reads show whether the field mapped to upload/download
  state

## Anti-Patterns

- treating evidence fields like ordinary text fields
- duplicating evidence-specific transfer state outside the controller

## Current Limits

- resource-backed execution of evidence uploads belongs in the resource-backed
  docs

## Related Docs

- [Attachments And Evidence Fields](../state/attachments-and-evidence-fields.md)
- [Attachment Transfers](./attachment-transfers.md)

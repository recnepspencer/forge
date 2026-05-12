# Transfers

Use this page when the endpoint prepares upload work or returns deferred
processing state.

## What This Covers

- `.signedUpload(...)`
- `.multipartUpload(...)`
- `.processing("poll" | "callback" | "webhook", ...)`
- `line.upload()`
- `line.processing()`

## Signed Upload Happy Path

```ts
const prepareReceiptUpload = api.url("/receipts/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .create({
    load: ({ body }) => ({ receiptId: body.receiptId }),
  });

const line = prepareReceiptUpload.line({
  body: { receiptId: "r1" },
});

console.log(line.upload());
console.log(line.processing());
```

## Multipart Upload Happy Path

```ts
const taskUpload = api.url("/tasks/upload")
  .multipartUpload({
    finalizeRequired: false,
  })
  .detail({
    load: () => ({ accepted: true }),
  });
```

Multipart upload stays in the same grammar. It is not a separate resource
surface.

## Deferred Processing Happy Path

Use `.processing(...)` whenever the endpoint can return accepted or in-progress
work instead of final business data.

```ts
const reportStatus = api.url("/reports/:reportId/status")
  .processing("callback", {
    callbackId: "report-status",
  })
  .detail({
    load: ({ reportId }) => ({ id: reportId, status: "accepted" }),
  });
```

## Where To Go Next

- downloads and multipart downloads:
  [Downloads](./downloads.md)
- line reads and diagnostics:
  [Line Inspection](./line-inspection.md)
- transfer reference details:
  [Resource Transfers Reference](../api-reference/resource-transfers.md)

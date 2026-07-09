# Upload Files

Use this page when the endpoint prepares an upload instead of immediately
returning final structured value.

## Stable Entry Points

- `.signedUpload(...)`
- `.multipartUpload(...)`
- `line.upload()`
- `line.request().uploadTransport`

## The Two Main Lanes

- `signedUpload(...)`
  Server prepares a signed PUT or POST handoff.
- `multipartUpload(...)`
  Server prepares a direct multipart POST handoff.

## Example

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

console.log(line.request().uploadTransport);
console.log(line.upload());
```

## Related Docs

- [Understand Transfer Results](./understand-transfer-results.md)
- [Track Processing Jobs](./track-processing-jobs.md)

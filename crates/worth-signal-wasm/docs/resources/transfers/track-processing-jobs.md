# Track Processing Jobs

Use this page when the endpoint can accept work now and complete it later.

## Stable Entry Points

- `.processing("poll")`
- `.processing("callback", ...)`
- `.processing("webhook", ...)`
- `line.processing()`
- `line.request().processingJob`

## The Main Lanes

- `poll`
  The host or app can check later.
- `callback`
  The endpoint identifies a callback lane.
- `webhook`
  The endpoint identifies an external webhook lane.

## Example

```ts
const reportStatus = api.url("/reports/:reportId/status")
  .processing("callback", {
    callbackId: "report-ready",
  })
  .detail({
    load: ({ reportId }) => ({ id: reportId, status: "accepted" }),
  });

const line = reportStatus.line({ reportId: "r1" });

console.log(line.request().processingJob);
console.log(line.processing());
```

## What The Result Means

Processing reads can be:

- `ready`
- `accepted`
- `processing`

The job is not hidden inside generic status text. It has a first-class read.

## Related Docs

- [Understand Transfer Results](./understand-transfer-results.md)
- [Compatibility And Readiness](./compatibility-and-readiness.md)

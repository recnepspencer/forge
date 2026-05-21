# Resource Transfers Reference

If your question is "how do I do signed upload, multipart upload, or deferred
processing?", start with [Transfers](../resources/transfers.md) before
using this lower-level reference page.

## What This Feature Is

This is the resource surface for uploads and deferred processing.

Use it when the first important state of a resource is not "the final value is
loaded", but something like:

- "the upload is ready"
- "the file was uploaded, now the server is processing it"
- "the server accepted a processing job"

## Why You Use It

- keep upload state on the same line as the rest of the resource state
- model signed-upload and multipart flows without a second client state machine
- show processing state without inventing another store
- inspect upload and processing from the same line you already render

## Stable Entry Points

Recommended declaration lane:

- `signals.api(...)`
- `api.url(...)`
- `.signedUpload(...)`
- `.multipartUpload(...)`
- `.processing(...)`

Transfer posture helpers:

- `resourceProcessingJob.none()`
- `resourceProcessingJob.poll()`
- `resourceProcessingJob.callback(...)`
- `resourceProcessingJob.webhook(...)`
- `resourceUploadTransport.none()`
- `resourceUploadTransport.directMultipart(...)`
- `resourceUploadTransport.signed(...)`

Transfer result helpers:

- `resourceProcessingResult.accepted(...)`
- `resourceProcessingResult.processing(...)`
- `resourceUploadResult.prepared(...)`
- `resourceUploadResult.uploaded(...)`

Line inspection:

- `line.processing()`
- `line.upload()`

## Core Mental Model

Upload and processing state live on the line.

That means you do not need:

- one object for the eventual value
- one store for upload progress
- another store for processing status

One line can move through states like:

- ready for upload
- uploaded and waiting for processing
- processing
- final value ready

## How It Executes

At declaration time you can set:

- `processingJob`
- `uploadTransport`

At load time you can return:

- `resourceUploadResult.*(...)`
- `resourceProcessingResult.*(...)`
- a normal structured value

The line then exposes:

- request posture through `line.request()`
- upload state through `line.upload()`
- processing state through `line.processing()`

## Small Example

```ts
import {
  createSignals,
  resourceUploadResult,
} from "forge-signal-wasm";

const signals = await createSignals();

const receiptUpload = signals.api({
  baseUrl: "/api",
}).url("/receipts/:receiptId/upload")
  .signedUpload({
    method: "PUT",
    finalizeRequired: true,
  })
  .detail({
    load: ({ receiptId }) =>
      resourceUploadResult.prepared({
        uploadId: `upload:${receiptId}`,
        descriptor: {
          kind: "signed",
          url: `https://uploads.example/${receiptId}`,
          method: "PUT",
          headers: { "x-upload-token": "demo" },
          fields: {},
          objectKey: `receipts/${receiptId}.png`,
          expiresAt: null,
        },
        finalizeRequired: true,
      }),
  });

const line = receiptUpload.line({ receiptId: "r1" });
console.log(line.upload());
```

Use this when the first thing the UI needs is upload instructions, not the
final value.

## Real Example

```ts
import {
  createSignals,
  resourceProcessingResult,
  resourceUploadResult,
} from "forge-signal-wasm";

const signals = await createSignals();
let callCount = 0;

const receiptPipeline = signals.api({
  baseUrl: "/api",
}).url("/receipts/:receiptId/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .detail({
    load: ({ receiptId }) => {
      callCount += 1;
      if (callCount === 1) {
        return resourceUploadResult.uploaded({
          uploadId: `upload:${receiptId}`,
          finalizeRequired: true,
          awaitingProcessing: true,
          message: "processing upload",
        });
      }
      if (callCount === 2) {
        return resourceProcessingResult.processing({
          jobId: `job:${receiptId}`,
          message: "extracting receipt",
        });
      }
      return { id: receiptId, status: "ready" };
    },
  });

const line = receiptPipeline.line({ receiptId: "r1" });

console.log(line.upload());
console.log(line.processing());
line.refresh();
console.log(line.processing());
line.refresh();
console.log(line.value());
```

This is the normal long-running workflow:

1. upload finished
2. server processing started
3. final value is ready

## How It Relates To Other Features

- Request/policy docs explain how upload and processing posture are declared.
- Binary/download docs explain the separate case where the final value also has
  downloadable files.
- Line docs explain where transfer state sits beside status and diagnostics.

## Inspection And Debugging

Check these first:

- `line.request().processingJob`
- `line.request().uploadTransport`
- `line.upload()`
- `line.processing()`
- `line.diagnostics().upload`
- `line.diagnostics().processing`

## Anti-Patterns

- inventing a second upload-progress store for states already on the line
- returning upload results from a family with no upload transport declaration
- returning processing results from a family with no processing declaration

## Current Limits

- this surface models upload and deferred processing state
- it is not a full mutation-orchestration system by itself

## Related Docs

- [Route Authoring Reference](./route-authoring.md)
- [Resource Request And Policy Reference](./resource-request-and-policy.md)
- [Resource Binary And Download Reference](./resource-binary-and-download.md)
- [Resource Line Reference](./resource-line.md)

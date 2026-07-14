import { createSignals, resourceProcessingResult, resourceUploadResult } from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const api = signals.api({});

const directTaskUploadPages = api.url("/workspaces/:workspaceId/direct-task-upload-pages")
  .items((item: { id: string; title: string }) => item.id)
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .paged({
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: ({ workspaceId }) =>
      resourceUploadResult.uploaded({
        uploadId: `upload:${workspaceId}`,
        finalizeRequired: true,
        awaitingProcessing: true,
        message: "processing upload",
      }),
  });

const fluentCatalogUpload = api.url("/workspaces/:workspaceId/fluent-catalog-upload")
  .items((item: { id: string; title: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string; title: string }> }) => value.items,
    (
      value: { items: Array<{ id: string; title: string }> },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  )
  .multipartUpload({
    finalizeRequired: false,
  })
  .list({
    load: ({ workspaceId }) =>
      resourceUploadResult.prepared({
        uploadId: `upload:${workspaceId}`,
        descriptor: {
          kind: "directMultipart",
          url: `https://uploads.example/${workspaceId}`,
          method: "POST",
          headers: {},
          fields: { workspaceId: String(workspaceId) },
          objectKey: null,
          expiresAt: null,
        },
        finalizeRequired: false,
        message: "ready",
      }),
  });

const directTaskUploadLine = directTaskUploadPages.line({ workspaceId: "demo" });
const fluentCatalogUploadLine = fluentCatalogUpload.line({ workspaceId: "demo" });
const callbackTaskPages = api.url("/workspaces/:workspaceId/callback-task-pages")
  .items((item: { id: string }) => item.id)
  .processing("callback", {
    callbackId: "task-ready",
  })
  .paged({
    accumulatePage: (existing: Array<{ id: string }>, next: Array<{ id: string }>) =>
      [...existing, ...next],
    load: ({ workspaceId }) =>
      resourceProcessingResult.accepted({
        jobId: `job:${workspaceId}`,
        message: "queued",
      }),
  });
const callbackTaskPagesLine = callbackTaskPages.line({ workspaceId: "demo" });

void directTaskUploadLine.upload();
void directTaskUploadLine.processing();
void fluentCatalogUploadLine.upload();
void callbackTaskPagesLine.processing();

import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";
import { normalizeTransferLineArtifact } from "./route_transfer_line_artifact_proof.mjs";

test("api.url(...).signedUpload(...).processing(...).create(...) lowers into the same upload and processing truth as the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    let callCount = 0;
    const routeReceipt = signals.api({}).url("/receipts/:receiptId/upload")
      .signedUpload({
        method: "POST",
        finalizeRequired: true,
      })
      .processing("poll")
      .create({
        baseUrl: "/api",
        load: ({ receiptId, body }) => {
          callCount += 1;
          if (callCount === 1) {
            return signalsMod.resourceUploadResult.uploaded({
              uploadId: `upload:${receiptId}:${body.fileName}`,
              finalizeRequired: true,
              awaitingProcessing: true,
              message: "processing upload",
            });
          }
          return { id: receiptId, fileName: body.fileName, status: "ready" };
        },
      });
    let rawCallCount = 0;
    const rawReceipt = signals.resource.detail({
      params: signalsMod.resourceParams(),
      baseUrl: "/api",
      method: "POST",
      requestBody: (params) => params.body,
      processingJob: signalsMod.resourceProcessingJob.poll(),
      uploadTransport: signalsMod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId, body }) =>
        signalsMod.resourceParamIdentity(
          { receiptId, body },
          `/receipts/${receiptId}/upload#body=${JSON.stringify(body)}`,
        ),
      load: ({ receiptId, body }) => {
        rawCallCount += 1;
        if (rawCallCount === 1) {
          return signalsMod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}:${body.fileName}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        return { id: receiptId, fileName: body.fileName, status: "ready" };
      },
    });

    const routeLine = routeReceipt.line({
      receiptId: "r1",
      body: { fileName: "receipt.png" },
    });
    const rawLine = rawReceipt.line({
      receiptId: "r1",
      body: { fileName: "receipt.png" },
    });

    assert.deepEqual(
      normalizeTransferLineArtifact(routeLine),
      normalizeTransferLineArtifact(rawLine),
    );

    routeLine.refresh();
    rawLine.refresh();

    assert.deepEqual(
      normalizeTransferLineArtifact(routeLine),
      normalizeTransferLineArtifact(rawLine),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).multipartUpload(...).detail(...) lowers into the same direct-multipart upload truth as the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReceipt = signals.api({}).url("/receipts/:receiptId/multipart")
      .multipartUpload({
        finalizeRequired: false,
      })
      .detail({
        load: ({ receiptId }) =>
          signalsMod.resourceUploadResult.prepared({
            uploadId: `upload:${receiptId}`,
            descriptor: {
              kind: "directMultipart",
              url: `https://uploads.example/${receiptId}`,
              method: "POST",
              headers: {},
              fields: { key: receiptId },
              objectKey: null,
              expiresAt: null,
            },
            finalizeRequired: false,
            message: "ready",
          }),
      });
    const rawReceipt = signals.resource.detail({
      params: signalsMod.resourceParams(),
      uploadTransport: signalsMod.resourceUploadTransport.directMultipart({
        finalizeRequired: false,
      }),
      normalizeParams: ({ receiptId }) =>
        signalsMod.resourceParamIdentity(
          { receiptId },
          `/receipts/${receiptId}/multipart`,
        ),
      load: ({ receiptId }) =>
        signalsMod.resourceUploadResult.prepared({
          uploadId: `upload:${receiptId}`,
          descriptor: {
            kind: "directMultipart",
            url: `https://uploads.example/${receiptId}`,
            method: "POST",
            headers: {},
            fields: { key: receiptId },
            objectKey: null,
            expiresAt: null,
          },
          finalizeRequired: false,
          message: "ready",
        }),
    });

    assert.deepEqual(
      normalizeTransferLineArtifact(routeReceipt.line({ receiptId: "r2" })),
      normalizeTransferLineArtifact(rawReceipt.line({ receiptId: "r2" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).processing(\"callback\", ...).detail(...) lowers into the same deferred-processing truth as the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReport = signals.api({}).url("/reports/:reportId")
      .processing("callback", {
        callbackId: "report-ready",
      })
      .detail({
        load: ({ reportId }) =>
          signalsMod.resourceProcessingResult.accepted({
            jobId: `job:${reportId}`,
            message: "queued",
          }),
      });
    const rawReport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      processingJob: signalsMod.resourceProcessingJob.callback({
        callbackId: "report-ready",
      }),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) =>
        signalsMod.resourceProcessingResult.accepted({
          jobId: `job:${reportId}`,
          message: "queued",
        }),
    });

    assert.deepEqual(
      normalizeTransferLineArtifact(routeReport.line({ reportId: "report-1" })),
      normalizeTransferLineArtifact(rawReport.line({ reportId: "report-1" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...) route-finalizer transfer builders own transport posture fields", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/receipts/:receiptId/upload")
          .signedUpload()
          .create({
            uploadTransport: runtime.signalsMod.resourceUploadTransport.signed(),
            load: ({ receiptId, body }) => ({ receiptId, body }),
          }),
      /own uploadTransport/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/reports/:reportId")
          .processing("webhook", {
            correlationKey: "report:1",
            provider: "stripe",
          })
          .detail({
            processingJob: runtime.signalsMod.resourceProcessingJob.webhook({
              correlationKey: "report:1",
              provider: "stripe",
            }),
            load: ({ reportId }) => ({ id: reportId }),
          }),
      /owns processingJob/,
    );
  } finally {
    await runtime.cleanup();
  }
});

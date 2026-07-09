import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";
import { normalizeTransferLineArtifact } from "./route_transfer_line_artifact_proof.mjs";

test("api.url(...).items(...).signedUpload(...).paged(...) lowers direct-array transfer posture into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeTasks = signals.api({}).url("/workspaces/:workspaceId/task-uploads")
      .items((item) => item.id)
      .signedUpload({
        method: "POST",
        finalizeRequired: true,
      })
      .processing("poll")
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ workspaceId }) =>
          signalsMod.resourceUploadResult.uploaded({
            uploadId: `upload:${workspaceId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          }),
      });
    const rawTasks = signals.resource.paged({
      params: signalsMod.resourceParams(),
      method: "GET",
      processingJob: signalsMod.resourceProcessingJob.poll(),
      uploadTransport: signalsMod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/task-uploads`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
      }),
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId }) =>
        signalsMod.resourceUploadResult.uploaded({
          uploadId: `upload:${workspaceId}`,
          finalizeRequired: true,
          awaitingProcessing: true,
          message: "processing upload",
        }),
    });

    assert.deepEqual(
      normalizeTransferLineArtifact(routeTasks.line({ workspaceId: "demo" })),
      normalizeTransferLineArtifact(rawTasks.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).multipartUpload(...).list(...) lowers envelope-shaped transfer posture into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeCatalog = signals.api({}).url("/workspaces/:workspaceId/catalog-upload")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .multipartUpload({
        finalizeRequired: false,
      })
      .list({
        load: ({ workspaceId }) =>
          signalsMod.resourceUploadResult.prepared({
            uploadId: `upload:${workspaceId}`,
            descriptor: {
              kind: "directMultipart",
              url: `https://uploads.example/${workspaceId}`,
              method: "POST",
              headers: {},
              fields: { workspaceId },
              objectKey: null,
              expiresAt: null,
            },
            finalizeRequired: false,
            message: "ready",
          }),
      });
    const rawCatalog = signals.resource.collection({
      params: signalsMod.resourceParams(),
      uploadTransport: signalsMod.resourceUploadTransport.directMultipart({
        finalizeRequired: false,
      }),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/catalog-upload`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      load: ({ workspaceId }) =>
        signalsMod.resourceUploadResult.prepared({
          uploadId: `upload:${workspaceId}`,
          descriptor: {
            kind: "directMultipart",
            url: `https://uploads.example/${workspaceId}`,
            method: "POST",
            headers: {},
            fields: { workspaceId },
            objectKey: null,
            expiresAt: null,
          },
          finalizeRequired: false,
          message: "ready",
        }),
    });

    assert.deepEqual(
      normalizeTransferLineArtifact(routeCatalog.line({ workspaceId: "demo" })),
      normalizeTransferLineArtifact(rawCatalog.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("collection-owned transfer builders keep final declaration ownership honest", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    assert.throws(
      () =>
        runtime.signals.api({}).url("/tasks")
          .items((item) => item.id)
          .signedUpload()
          .list({
            uploadTransport: runtime.signalsMod.resourceUploadTransport.signed(),
            load: () => [{ id: "t1" }],
          }),
      /own uploadTransport/,
    );
    assert.throws(
      () =>
        runtime.signals.api({}).url("/catalog")
          .items((item) => item.id)
          .reconcile(
            (value) => value.items,
            (value, nextItems) => ({ ...value, items: [...nextItems] }),
          )
          .processing("poll")
          .list({
            processingJob: runtime.signalsMod.resourceProcessingJob.poll(),
            load: () => ({ items: [{ id: "t1" }] }),
          }),
      /owns processingJob/,
    );
  } finally {
    await runtime.cleanup();
  }
});

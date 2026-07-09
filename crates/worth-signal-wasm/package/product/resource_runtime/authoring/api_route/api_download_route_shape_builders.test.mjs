import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";
import { normalizeDownloadLineArtifact } from "./route_download_line_artifact_proof.mjs";

test("api.url(...).downloads(...).create(...) lowers write-shaped binary declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeExport = signals.api({}).url("/reports/export")
      .downloads((_params, value, download) => [
        download.export("report-export", {
          fileName: `${value.jobId}.zip`,
          mediaType: "application/zip",
          byteLength: value.jobId.length * 64,
          download: download.ready({
            url: `https://downloads.example/${value.jobId}.zip`,
          }),
        }),
      ])
      .create({
        load: ({ body }) => ({ jobId: body.jobId }),
      });
    const rawExport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      method: "POST",
      requestBody: ({ body }) => body,
      normalizeParams: ({ body }) =>
        signalsMod.resourceParamIdentity(
          { body },
          `/reports/export#body=${JSON.stringify(body)}`,
        ),
      load: ({ body }) =>
        signalsMod.resourceBinaryValue({
          value: { jobId: body.jobId },
          descriptors: [
            signalsMod.resourceBinaryDescriptor.export({
              id: "report-export",
              fileName: `${body.jobId}.zip`,
              mediaType: "application/zip",
              byteLength: body.jobId.length * 64,
              download: signalsMod.resourceDownload.ready({
                url: `https://downloads.example/${body.jobId}.zip`,
                method: "GET",
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeExport.line({ body: { jobId: "job-7" } })),
      normalizeDownloadLineArtifact(rawExport.line({ body: { jobId: "job-7" } })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).items(...).reconcile(...).downloads(...).list(...) lowers envelope-shaped multipart download declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeCatalog = signals.api({}).url("/workspaces/:workspaceId/catalog")
      .items((item) => item.id)
      .reconcile(
        (value) => value.items,
        (value, nextItems) => ({ ...value, items: [...nextItems] }),
      )
      .downloads(({ workspaceId }, value, download) => [
        download.export("catalog-export", {
          fileName: `${workspaceId}-catalog.zip`,
          mediaType: "application/zip",
          byteLength: value.total * 256,
          download: download.multipart({
            url: `https://downloads.example/${workspaceId}/catalog`,
            fields: { token: workspaceId },
            objectKey: `catalog/${workspaceId}.zip`,
          }),
        }),
      ])
      .list({
        load: ({ workspaceId }) => ({
          items: [{ id: `${workspaceId}:1`, title: "First" }],
          total: 1,
        }),
      });
    const rawCatalog = signals.resource.collection({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/catalog`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      }),
      load: ({ workspaceId }) =>
        signalsMod.resourceBinaryValue({
          value: {
            items: [{ id: `${workspaceId}:1`, title: "First" }],
            total: 1,
          },
          descriptors: [
            signalsMod.resourceBinaryDescriptor.export({
              id: "catalog-export",
              fileName: `${workspaceId}-catalog.zip`,
              mediaType: "application/zip",
              byteLength: 256,
              download: signalsMod.resourceDownload.multipart({
                url: `https://downloads.example/${workspaceId}/catalog`,
                fields: { token: workspaceId },
                objectKey: `catalog/${workspaceId}.zip`,
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeCatalog.line({ workspaceId: "demo" })),
      normalizeDownloadLineArtifact(rawCatalog.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

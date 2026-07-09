import assert from "node:assert/strict";
import test from "node:test";

import {
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDownloadDetail } from "../runtime_fixture/real_download_resources.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("multipart-ready descriptors stay actionable while exposing direct-multipart handoff truth", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: { id: assetId, title: "Bundle" },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.export({
              id: "bundle-export",
              fileName: `${assetId}.zip`,
              mediaType: "application/zip",
              byteLength: 4096,
              download: runtime.mod.resourceDownload.multipart({
                url: `https://downloads.example/${assetId}`,
                headers: { authorization: "secret-token" },
                fields: { token: assetId },
                objectKey: `exports/${assetId}.zip`,
                expiresAt: "2026-05-04T12:00:00Z",
              }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "bundle-q1" });

    assert.deepEqual(normalizeForProof(line.download()), {
      count: 1,
      readyCount: 1,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptors: [
        {
          kind: "export",
          id: "bundle-export",
          label: null,
          fileName: "bundle-q1.zip",
          mediaType: "application/zip",
          byteLength: 4096,
          download: {
            kind: "ready",
            transportKind: "directMultipart",
            url: "https://downloads.example/bundle-q1",
            method: "POST",
            headers: { authorization: "secret-token" },
            fields: { token: "bundle-q1" },
            objectKey: "exports/bundle-q1.zip",
            expiresAt: "2026-05-04T12:00:00Z",
          },
        },
      ],
    });
    assert.deepEqual(normalizeForProof(line.diagnostics().download), {
      count: 1,
      readyCount: 1,
      unavailableCount: 0,
      incompatibleCount: 0,
      descriptors: [
        {
          kind: "export",
          id: "bundle-export",
          label: null,
          fileName: "bundle-q1.zip",
          mediaType: "application/zip",
          byteLength: 4096,
          download: {
            kind: "ready",
            transportKind: "directMultipart",
            url: "https://downloads.example/bundle-q1",
            method: "POST",
            headerNames: ["authorization"],
            fieldNames: ["token"],
            objectKey: "exports/bundle-q1.zip",
            expiresAt: "2026-05-04T12:00:00Z",
          },
        },
      ],
    });
    assert.equal(line.history().lifecycle.at(-1)?.readyDownloadCount, 1);
  } finally {
    await runtime.cleanup();
  }
});

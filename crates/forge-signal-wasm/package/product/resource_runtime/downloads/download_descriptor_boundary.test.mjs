import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDownloadDetail } from "../runtime_fixture/real_download_resources.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("stale descriptor refresh surfaces explicit incompatible download truth without changing structured value", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "descriptor-refresh");
    const snapshotId = Number(runtime.signals.history().branch_snapshot_id(branch.id));
    let staleDescriptor = false;
    const structuredValue = { id: "asset-2", title: "Spec" };
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: structuredValue,
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "spec-download",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              byteLength: 640,
              download: staleDescriptor
                ? runtime.mod.resourceDownload.incompatible({
                    reason: "staleDescriptor",
                    detail:
                      "descriptor was issued for an older branch snapshot; refresh before downloading",
                  })
                : runtime.mod.resourceDownload.ready({
                    url: `https://downloads.example/${assetId}.pdf`,
                    method: "GET",
                    expiresAt: "2026-05-04T12:00:00Z",
                  }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "asset-2" });
    const firstVersion = line.diagnostics().visibleValueVersion;
    staleDescriptor = true;
    line.refresh();
    const history = line.history();

    assert.deepEqual(line.value(), structuredValue);
    assert.deepEqual(normalizeForProof(line.download()), {
      count: 1,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 1,
      descriptors: [
        {
          kind: "file",
          id: "spec-download",
          label: null,
          fileName: "asset-2.pdf",
          mediaType: "application/pdf",
          byteLength: 640,
          download: {
            kind: "incompatible",
            reason: "staleDescriptor",
            detail:
              "descriptor was issued for an older branch snapshot; refresh before downloading",
          },
        },
      ],
    });
    assert.equal(line.diagnostics().visibleValueVersion, firstVersion);
    assert.equal(line.diagnostics().download.incompatibleCount, 1);
    assert.equal(history.lifecycle.at(-1)?.incompatibleDownloadCount, 1);
    assert.ok(Array.isArray(history.replay?.frames));
    assert.ok(history.replay.frames.length > 0);
    assert.deepEqual(history.branch, {
      id: branch.id,
      name: "descriptor-refresh",
      parentBranchId: 0,
      headSnapshotId: snapshotId,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("transport-boundary download incompatibility stays self-describing through diagnostics and history", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: { id: assetId, status: "ready" },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.export({
              id: "bundle",
              fileName: `${assetId}.zip`,
              mediaType: "application/zip",
              byteLength: 4096,
              download: runtime.mod.resourceDownload.incompatible({
                reason: "transportBoundary",
                detail:
                  "download requires a host-owned session handoff; resource lines expose only the descriptor boundary",
              }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "bundle-7" });

    assert.equal(line.download().incompatibleCount, 1);
    assert.equal(line.download().readyCount, 0);
    assert.equal(line.download().unavailableCount, 0);
    assert.deepEqual(normalizeForProof(line.diagnostics().download), {
      count: 1,
      readyCount: 0,
      unavailableCount: 0,
      incompatibleCount: 1,
      descriptors: [
        {
          kind: "export",
          id: "bundle",
          label: null,
          fileName: "bundle-7.zip",
          mediaType: "application/zip",
          byteLength: 4096,
          download: {
            kind: "incompatible",
            reason: "transportBoundary",
            detail:
              "download requires a host-owned session handoff; resource lines expose only the descriptor boundary",
          },
        },
      ],
    });
    assert.equal(line.history().lifecycle.at(-1)?.incompatibleDownloadCount, 1);
  } finally {
    await runtime.cleanup();
  }
});

test("multipart-ready downloads reject non-string field values at the descriptor boundary", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: { id: assetId, status: "ready" },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.export({
              id: "bundle",
              fileName: `${assetId}.zip`,
              mediaType: "application/zip",
              byteLength: 4096,
              download: runtime.mod.resourceDownload.multipart({
                url: `https://downloads.example/${assetId}`,
                fields: { token: 7 },
              }),
            }),
          ],
        }),
    });

    assert.throws(
      () => detail.line({ assetId: "bundle-8" }),
      /resourceDownload fields\.token must be a string/,
    );
  } finally {
    await runtime.cleanup();
  }
});

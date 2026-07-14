import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceRuntime,
} from "../runtime_fixture/real_resource_signals.mjs";
import {
  createRealDownloadCollection,
  createRealDownloadDetail,
} from "../runtime_fixture/real_download_resources.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("descriptor-bearing detail lines keep structured value and download truth distinct", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: { id: assetId, title: "Quarterly Report" },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "report-pdf",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              byteLength: 2048,
              download: runtime.mod.resourceDownload.ready({
                url: `https://downloads.example/${assetId}.pdf`,
                method: "GET",
                headers: { authorization: "secret-token" },
                expiresAt: "2026-05-04T12:00:00Z",
              }),
            }),
            runtime.mod.resourceBinaryDescriptor.media({
              id: "report-preview",
              fileName: `${assetId}.png`,
              mediaType: "image/png",
              byteLength: 512,
              download: runtime.mod.resourceDownload.unavailable({
                reason: "notReady",
                detail: "preview is still rendering",
              }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "report-q1" });

    assert.deepEqual(line.value(), {
      id: "report-q1",
      title: "Quarterly Report",
    });
    assert.deepEqual(normalizeForProof(line.download()), {
      count: 2,
      readyCount: 1,
      unavailableCount: 1,
      incompatibleCount: 0,
      descriptors: [
        {
          kind: "file",
          id: "report-pdf",
          label: null,
          fileName: "report-q1.pdf",
          mediaType: "application/pdf",
          byteLength: 2048,
          download: {
            kind: "ready",
            transportKind: "simple",
            url: "https://downloads.example/report-q1.pdf",
            method: "GET",
            headers: { authorization: "secret-token" },
            fields: {},
            objectKey: null,
            expiresAt: "2026-05-04T12:00:00Z",
          },
        },
        {
          kind: "media",
          id: "report-preview",
          label: null,
          fileName: "report-q1.png",
          mediaType: "image/png",
          byteLength: 512,
          download: {
            kind: "unavailable",
            reason: "notReady",
            detail: "preview is still rendering",
          },
        },
      ],
    });
    assert.deepEqual(normalizeForProof(line.diagnostics().download), {
      count: 2,
      readyCount: 1,
      unavailableCount: 1,
      incompatibleCount: 0,
      descriptors: [
        {
          kind: "file",
          id: "report-pdf",
          label: null,
          fileName: "report-q1.pdf",
          mediaType: "application/pdf",
          byteLength: 2048,
          download: {
            kind: "ready",
            transportKind: "simple",
            url: "https://downloads.example/report-q1.pdf",
            method: "GET",
            headerNames: ["authorization"],
            fieldNames: [],
            objectKey: null,
            expiresAt: "2026-05-04T12:00:00Z",
          },
        },
        {
          kind: "media",
          id: "report-preview",
          label: null,
          fileName: "report-q1.png",
          mediaType: "image/png",
          byteLength: 512,
          download: {
            kind: "unavailable",
            reason: "notReady",
            detail: "preview is still rendering",
          },
        },
      ],
    });
    assert.deepEqual(
      normalizeForProof(line.diagnosticsSummary().download),
      normalizeForProof(line.diagnostics().download),
    );
    assert.equal(line.history().lifecycle.at(-1)?.downloadCount, 2);
    assert.equal(line.history().lifecycle.at(-1)?.readyDownloadCount, 1);
    assert.equal(line.history().lifecycle.at(-1)?.unavailableDownloadCount, 1);
    assert.equal(line.history().lifecycle.at(-1)?.incompatibleDownloadCount, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("collection patch preserves download descriptors while structured items reconcile narrowly", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const collection = createRealDownloadCollection(runtime.mod, runtime.signals, {
      load: ({ workspaceId }) =>
        runtime.mod.resourceBinaryValue({
          value: {
            items: [{ id: `${workspaceId}:1`, title: "First" }],
          },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "export-csv",
              fileName: `${workspaceId}.csv`,
              mediaType: "text/csv",
              byteLength: 128,
              download: runtime.mod.resourceDownload.ready({
                url: `https://downloads.example/${workspaceId}.csv`,
                method: "GET",
              }),
            }),
          ],
        }),
    });

    const line = collection.line({ workspaceId: "demo" });
    const beforeDownload = line.download();

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Updated",
      }),
    );

    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Updated" }],
    });
    assert.deepEqual(line.download(), beforeDownload);
    assert.equal(line.diagnostics().lastPatchScope, "aspect");
    assert.equal(line.history().lifecycle.at(-1)?.downloadCount, 1);
    assert.equal(line.history().lifecycle.at(-1)?.readyDownloadCount, 1);
    assert.equal(line.history().lifecycle.at(-1)?.incompatibleDownloadCount, 0);
    assert.equal(line.history().lifecycle.at(-1)?.visibleValueVersion, 2);
  } finally {
    await runtime.cleanup();
  }
});

test("broad replace patch with semantically equal structured value does not advance visible value version", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const collection = createRealDownloadCollection(runtime.mod, runtime.signals, {
      load: ({ workspaceId }) =>
        runtime.mod.resourceBinaryValue({
          value: {
            items: [{ id: `${workspaceId}:1`, title: "First" }],
          },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "export-csv",
              fileName: `${workspaceId}.csv`,
              mediaType: "text/csv",
              byteLength: 128,
              download: runtime.mod.resourceDownload.ready({
                url: `https://downloads.example/${workspaceId}.csv`,
                method: "GET",
              }),
            }),
          ],
        }),
    });

    const line = collection.line({ workspaceId: "demo" });
    const firstVersion = line.diagnostics().visibleValueVersion;

    line.patch(
      runtime.mod.resourcePatch.replace({
        items: [{ id: "demo:1", title: "First" }],
      }),
    );

    assert.equal(line.diagnostics().visibleValueVersion, firstVersion);
    assert.equal(line.history().lifecycle.at(-1)?.visibleValueVersion, firstVersion);
    assert.equal(line.history().lifecycle.at(-1)?.event, "patched");
    assert.equal(line.diagnostics().lastPatchScope, "line");
  } finally {
    await runtime.cleanup();
  }
});

test("refresh can change download readiness without changing visible structured value", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const structuredValue = { id: "asset-1", title: "Manual" };
    let downloadReady = false;
    const branch = createBranchHead(runtime.signals, "downloads");
    const snapshotId = Number(runtime.signals.history().branch_snapshot_id(branch.id));
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: structuredValue,
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "manual-pdf",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              download: downloadReady
                ? runtime.mod.resourceDownload.ready({
                    url: `https://downloads.example/${assetId}.pdf`,
                    method: "GET",
                  })
                : runtime.mod.resourceDownload.unavailable({
                    reason: "notReady",
                    detail: "manual is still generating",
                  }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "asset-1" });
    const firstVersion = line.diagnostics().visibleValueVersion;
    downloadReady = true;
    line.refresh();
    const history = line.history();

    assert.deepEqual(line.value(), structuredValue);
    assert.equal(line.download().readyCount, 1);
    assert.equal(line.download().unavailableCount, 0);
    assert.equal(line.download().incompatibleCount, 0);
    assert.equal(line.diagnostics().visibleValueVersion, firstVersion);
    assert.deepEqual(history.branch, {
      id: branch.id,
      name: "downloads",
      parentBranchId: 0,
      headSnapshotId: snapshotId,
    });
    assert.deepEqual(history.availability, {
      replay: { kind: "available" },
      replayExact: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      },
      lineage: { kind: "available" },
      branch: { kind: "available" },
      restoreExact: {
        kind: "available",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
      },
    });
    assert.ok(Array.isArray(history.replay?.frames));
    assert.ok(history.replay.frames.length > 0);
    assert.equal(history.lifecycle.at(-1)?.downloadCount, 1);
    assert.equal(history.lifecycle.at(-1)?.readyDownloadCount, 1);
    assert.equal(history.lifecycle.at(-1)?.incompatibleDownloadCount, 0);
    assert.equal(history.lifecycle.at(-1)?.visibleValueVersion, firstVersion);
  } finally {
    await runtime.cleanup();
  }
});

test("refresh with a real nested structured value change still advances visible value version", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    let revision = 1;
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: {
            id: assetId,
            sections: [{ id: "summary", title: `Revision ${revision}` }],
          },
          descriptors: [
            runtime.mod.resourceBinaryDescriptor.file({
              id: "manual-pdf",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              download: runtime.mod.resourceDownload.ready({
                url: `https://downloads.example/${assetId}.pdf`,
                method: "GET",
              }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "asset-2" });
    const firstVersion = line.diagnostics().visibleValueVersion;
    revision = 2;

    line.refresh();

    assert.deepEqual(line.value(), {
      id: "asset-2",
      sections: [{ id: "summary", title: "Revision 2" }],
    });
    assert.equal(line.diagnostics().visibleValueVersion, firstVersion + 1);
    assert.equal(
      line.history().lifecycle.at(-1)?.visibleValueVersion,
      firstVersion + 1,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resourceBinaryValue rejects upload or processing result truth wrapped as structured value", async () => {
  const runtime = await createRealResourceRuntime();
  try {
    const detail = createRealDownloadDetail(runtime.mod, runtime.signals, {
      uploadTransport: runtime.mod.resourceUploadTransport.signed({
        method: "PUT",
        finalizeRequired: true,
      }),
      load: ({ assetId }) =>
        runtime.mod.resourceBinaryValue({
          value: runtime.mod.resourceUploadResult.uploaded({
            uploadId: `upload:${assetId}`,
            finalizeRequired: true,
            awaitingProcessing: false,
          }),
        }),
    });

    assert.throws(
      () => detail.line({ assetId: "bad-wrap" }),
      /resourceBinaryValue\(\.\.\.\) cannot wrap resourceProcessingResult\.\*\(\) or resourceUploadResult\.\*\(\)/,
    );
  } finally {
    await runtime.cleanup();
  }
});

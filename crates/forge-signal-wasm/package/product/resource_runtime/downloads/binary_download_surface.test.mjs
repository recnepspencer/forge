import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("descriptor-bearing detail lines keep structured value and download truth distinct", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ assetId }) =>
        mod.resourceParamIdentity({ assetId }, assetId),
      load: ({ assetId }) =>
        mod.resourceBinaryValue({
          value: { id: assetId, title: "Quarterly Report" },
          descriptors: [
            mod.resourceBinaryDescriptor.file({
              id: "report-pdf",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              byteLength: 2048,
              download: mod.resourceDownload.ready({
                url: `https://downloads.example/${assetId}.pdf`,
                method: "GET",
                headers: { authorization: "secret-token" },
                expiresAt: "2026-05-04T12:00:00Z",
              }),
            }),
            mod.resourceBinaryDescriptor.media({
              id: "report-preview",
              fileName: `${assetId}.png`,
              mediaType: "image/png",
              byteLength: 512,
              download: mod.resourceDownload.unavailable({
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
            url: "https://downloads.example/report-q1.pdf",
            method: "GET",
            headers: { authorization: "secret-token" },
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
            url: "https://downloads.example/report-q1.pdf",
            method: "GET",
            headerNames: ["authorization"],
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
    await mod.cleanup();
  }
});

test("collection patch preserves download descriptors while structured items reconcile narrowly", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});
    const collection = resource.collection({
      params: mod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        aspects: mod.resourceItemAspects({
          title: {
            read: (item) => item.title,
            write: (item, title) => ({ ...item, title: String(title) }),
          },
        }),
      }),
      load: ({ workspaceId }) =>
        mod.resourceBinaryValue({
          value: {
            items: [{ id: `${workspaceId}:1`, title: "First" }],
          },
          descriptors: [
            mod.resourceBinaryDescriptor.file({
              id: "export-csv",
              fileName: `${workspaceId}.csv`,
              mediaType: "text/csv",
              byteLength: 128,
              download: mod.resourceDownload.ready({
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
      mod.resourcePatch.itemAspect({
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
    await mod.cleanup();
  }
});

test("refresh can change download readiness without changing visible structured value", async () => {
  const mod = await loadResourceModule();
  try {
    const structuredValue = { id: "asset-1", title: "Manual" };
    let downloadReady = false;
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 14n,
          name: "downloads",
          parent_branch_id: 3n,
          head_snapshot_id: 28n,
        };
      },
      branch_snapshot() {
        return Object.freeze({ snapshotRestoreToken: "branch-14-snapshot" });
      },
      restore_exact_branch_snapshot() {},
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ assetId }) =>
        mod.resourceParamIdentity({ assetId }, assetId),
      load: ({ assetId }) =>
        mod.resourceBinaryValue({
          value: structuredValue,
          descriptors: [
            mod.resourceBinaryDescriptor.file({
              id: "manual-pdf",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              download: downloadReady
                ? mod.resourceDownload.ready({
                    url: `https://downloads.example/${assetId}.pdf`,
                    method: "GET",
                  })
                : mod.resourceDownload.unavailable({
                    reason: "notReady",
                    detail: "manual is still generating",
                  }),
            }),
          ],
        }),
    });

    const line = detail.line({ assetId: "asset-1" });
    const signalId = line.signal().id;
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
      id: 14,
      name: "downloads",
      parentBranchId: 3,
      headSnapshotId: 28,
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
        branchId: 14,
        snapshotId: 28,
      },
    });
    assert.equal(history.replay.id, signalId);
    assert.equal(history.lifecycle.at(-1)?.downloadCount, 1);
    assert.equal(history.lifecycle.at(-1)?.readyDownloadCount, 1);
    assert.equal(history.lifecycle.at(-1)?.incompatibleDownloadCount, 0);
    assert.equal(history.lifecycle.at(-1)?.visibleValueVersion, firstVersion);
  } finally {
    await mod.cleanup();
  }
});

test("resourceBinaryValue rejects upload or processing result truth wrapped as structured value", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "PUT",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) =>
        mod.resourceBinaryValue({
          value: mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: false,
          }),
        }),
    });

    assert.throws(
      () => detail.line({ receiptId: "bad-wrap" }),
      /resourceBinaryValue\(\.\.\.\) cannot wrap resourceProcessingResult\.\*\(\) or resourceUploadResult\.\*\(\)/,
    );
  } finally {
    await mod.cleanup();
  }
});

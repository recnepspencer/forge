import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("stale descriptor refresh surfaces explicit incompatible download truth without changing structured value", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 18n,
          name: "descriptor-refresh",
          parent_branch_id: 4n,
          head_snapshot_id: 29n,
        };
      },
      restore_exact_branch_snapshot() {},
    });
    let staleDescriptor = false;
    const structuredValue = { id: "asset-2", title: "Spec" };
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
              id: "spec-download",
              fileName: `${assetId}.pdf`,
              mediaType: "application/pdf",
              byteLength: 640,
              download: staleDescriptor
                ? mod.resourceDownload.incompatible({
                    reason: "staleDescriptor",
                    detail:
                      "descriptor was issued for an older branch snapshot; refresh before downloading",
                  })
                : mod.resourceDownload.ready({
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
    assert.equal(history.replay.id, line.signal().id);
    assert.deepEqual(history.branch, {
      id: 18,
      name: "descriptor-refresh",
      parentBranchId: 4,
      headSnapshotId: 29,
    });
  } finally {
    await mod.cleanup();
  }
});

test("transport-boundary download incompatibility stays self-describing through diagnostics and history", async () => {
  const mod = await loadResourceModule();
  try {
    const resource = mod.createResourceNamespace(createFakeSignalNamespace(), {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ exportId }) =>
        mod.resourceParamIdentity({ exportId }, exportId),
      load: ({ exportId }) =>
        mod.resourceBinaryValue({
          value: { id: exportId, status: "ready" },
          descriptors: [
            mod.resourceBinaryDescriptor.export({
              id: "bundle",
              fileName: `${exportId}.zip`,
              mediaType: "application/zip",
              byteLength: 4096,
              download: mod.resourceDownload.incompatible({
                reason: "transportBoundary",
                detail:
                  "download requires a host-owned session handoff; resource lines expose only the descriptor boundary",
              }),
            }),
          ],
        }),
    });

    const line = detail.line({ exportId: "bundle-7" });

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
    await mod.cleanup();
  }
});

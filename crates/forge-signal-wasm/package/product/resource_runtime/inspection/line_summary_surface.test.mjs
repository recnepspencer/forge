import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceRuntime } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealDownloadDetail } from "../runtime_fixture/real_download_resources.mjs";

function normalizeForProof(value) {
  return JSON.parse(JSON.stringify(value));
}

test("line.summary() groups the common line-consumption reads without changing their truth", async () => {
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
          ],
        }),
    });

    const line = detail.line({ assetId: "report-q1" });
    const summary = line.summary();

    assert.deepEqual(normalizeForProof(summary), normalizeForProof({
      current: line.diagnosticsSummary().current,
      request: line.request(),
      processing: line.processing(),
      upload: line.upload(),
      download: line.download(),
      diagnostics: line.diagnosticsSummary(),
      explainability: line.history().availability,
    }));
    assert.equal("history" in summary, false);
    assert.equal("lifecycle" in summary, false);
    assert.equal("replay" in summary, false);
  } finally {
    await runtime.cleanup();
  }
});

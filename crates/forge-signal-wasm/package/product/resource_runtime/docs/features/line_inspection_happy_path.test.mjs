import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";

test("line inspection doc happy path keeps summary, request, diagnostics, and history aligned", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const reportDetail = runtime.signals.api({
      baseUrl: "/api",
    }).url("/reports/:reportId").downloads(({ reportId }, _value, download) => [
      download.file("report-pdf", {
        fileName: `${reportId}.pdf`,
        mediaType: "application/pdf",
        download: download.ready({
          url: `https://downloads.example/${reportId}.pdf`,
        }),
      }),
    ]).detail({
      load: ({ reportId }) => ({ id: reportId }),
    });

    const line = reportDetail.line({ reportId: "r1" });

    assert.equal(line.summary().download.readyCount, 1);
    assert.equal(line.request().target.url, "/api/reports/r1");
    assert.equal(line.diagnostics().lastOutcome, "fulfilled");
    assert.equal(typeof line.history().availability.replay.kind, "string");
  } finally {
    await runtime.cleanup();
  }
});

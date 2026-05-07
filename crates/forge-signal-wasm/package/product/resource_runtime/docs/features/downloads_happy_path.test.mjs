import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";

test("downloads doc happy path covers builder-owned ready downloads and multipart handoff", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const api = runtime.signals.api({});
    const manualDetail = api.url("/assets/:assetId")
      .downloads(({ assetId }, _value, download) => [
        download.file("manual-pdf", {
          fileName: `${assetId}.pdf`,
          mediaType: "application/pdf",
          download: download.ready({
            url: `https://downloads.example/${assetId}.pdf`,
          }),
        }),
      ])
      .detail({
        load: ({ assetId }) => ({ id: assetId }),
      });
    const exportDetail = api.url("/exports/:exportId")
      .downloads(({ exportId }, _value, download) => [
        download.export("export-bundle", {
          fileName: `${exportId}.zip`,
          mediaType: "application/zip",
          download: download.multipart({
            url: `https://downloads.example/${exportId}`,
            fields: { token: exportId },
            objectKey: `exports/${exportId}.zip`,
          }),
        }),
      ])
      .detail({
        load: ({ exportId }) => ({ id: exportId }),
      });

    const manualLine = manualDetail.line({ assetId: "asset-1" });
    const exportLine = exportDetail.line({ exportId: "exp-1" });

    assert.equal(manualLine.download().readyCount, 1);
    assert.equal(
      exportLine.download().descriptors[0].download.transportKind,
      "directMultipart",
    );
  } finally {
    await runtime.cleanup();
  }
});

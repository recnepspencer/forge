import assert from "node:assert/strict";
import test from "node:test";

import { createRealTransferRuntime } from "../../runtime_fixture/real_transfer_runtime.mjs";
import { normalizeDownloadLineArtifact } from "./route_download_line_artifact_proof.mjs";

test("api.url(...).downloads(...).detail(...) lowers pleasant binary declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReport = signals.api({}).url("/reports/:reportId").downloads(({ reportId }, value, download) => [
        download.file("report-pdf", {
          fileName: `${reportId}.pdf`,
          mediaType: "application/pdf",
          byteLength: value.title.length * 100,
          download: download.ready({
            url: `https://downloads.example/${reportId}.pdf`,
            headers: { authorization: "secret-token" },
            expiresAt: "2026-05-05T12:00:00Z",
          }),
        }),
      ])
      .detail({
        load: ({ reportId }) => ({
        id: reportId,
        title: "Quarterly Report",
        }),
      });
    const rawReport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) =>
        signalsMod.resourceBinaryValue({
          value: {
            id: reportId,
            title: "Quarterly Report",
          },
          descriptors: [
            signalsMod.resourceBinaryDescriptor.file({
              id: "report-pdf",
              fileName: `${reportId}.pdf`,
              mediaType: "application/pdf",
              byteLength: "Quarterly Report".length * 100,
              download: signalsMod.resourceDownload.ready({
                url: `https://downloads.example/${reportId}.pdf`,
                method: "GET",
                headers: { authorization: "secret-token" },
                expiresAt: "2026-05-05T12:00:00Z",
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeReport.line({ reportId: "report-q1" })),
      normalizeDownloadLineArtifact(rawReport.line({ reportId: "report-q1" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...).items(...).paged(...) lowers collection download declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReports = signals.api({}).url("/workspaces/:workspaceId/reports")
      .items((item) => item.id)
      .downloads(({ workspaceId }, value, download) => [
        download.export("report-export", {
          fileName: `${workspaceId}-reports.csv`,
          mediaType: "text/csv",
          byteLength: value.length * 128,
          download: download.unavailable({
            reason: "notReady",
            detail: "export is still rendering",
          }),
        }),
      ])
      .paged({
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ workspaceId }) => [{ id: `${workspaceId}:r1`, title: "Report" }],
      });
    const rawReports = signals.resource.paged({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        signalsMod.resourceParamIdentity(
          { workspaceId },
          `/workspaces/${encodeURIComponent(String(workspaceId))}/reports`,
        ),
      itemIdentity: (item) => item.id,
      reconcile: signalsMod.resourceCollectionShape({
        items: (value) => value,
        replaceItems: (_value, nextItems) => [...nextItems],
      }),
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ workspaceId }) =>
        signalsMod.resourceBinaryValue({
          value: [{ id: `${workspaceId}:r1`, title: "Report" }],
          descriptors: [
            signalsMod.resourceBinaryDescriptor.export({
              id: "report-export",
              fileName: `${workspaceId}-reports.csv`,
              mediaType: "text/csv",
              byteLength: 128,
              download: signalsMod.resourceDownload.unavailable({
                reason: "notReady",
                detail: "export is still rendering",
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeReports.line({ workspaceId: "demo" })),
      normalizeDownloadLineArtifact(rawReports.line({ workspaceId: "demo" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...).detail(...) lowers async binary declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReport = signals.api({}).url("/reports/:reportId").downloads(({ reportId }, value, download) => [
        download.media("report-preview", {
          fileName: `${reportId}.png`,
          mediaType: "image/png",
          byteLength: value.title.length * 64,
          download: download.ready({
            url: `https://downloads.example/${reportId}.png`,
          }),
        }),
      ])
      .detail({
        load: async ({ reportId }) => ({
        id: reportId,
        title: "Preview",
        }),
      });
    const rawReport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: async ({ reportId }) =>
        signalsMod.resourceBinaryValue({
          value: {
            id: reportId,
            title: "Preview",
          },
          descriptors: [
            signalsMod.resourceBinaryDescriptor.media({
              id: "report-preview",
              fileName: `${reportId}.png`,
              mediaType: "image/png",
              byteLength: "Preview".length * 64,
              download: signalsMod.resourceDownload.ready({
                url: `https://downloads.example/${reportId}.png`,
                method: "GET",
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeReport.line({ reportId: "report-q2" })),
      normalizeDownloadLineArtifact(rawReport.line({ reportId: "report-q2" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...).detail(...) lowers multipart-ready download declarations into the raw lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const { signals, signalsMod } = runtime;
    const routeReport = signals.api({}).url("/reports/:reportId").downloads(({ reportId }, value, download) => [
      download.export("report-export", {
        fileName: `${reportId}.zip`,
        mediaType: "application/zip",
        byteLength: value.title.length * 256,
        download: download.multipart({
          url: `https://downloads.example/${reportId}`,
          headers: { authorization: "secret-token" },
          fields: { token: reportId },
          objectKey: `exports/${reportId}.zip`,
          expiresAt: "2026-05-05T12:00:00Z",
        }),
      }),
    ]).detail({
      load: ({ reportId }) => ({
        id: reportId,
        title: "Bundle",
      }),
    });
    const rawReport = signals.resource.detail({
      params: signalsMod.resourceParams(),
      normalizeParams: ({ reportId }) =>
        signalsMod.resourceParamIdentity({ reportId }, `/reports/${reportId}`),
      load: ({ reportId }) =>
        signalsMod.resourceBinaryValue({
          value: {
            id: reportId,
            title: "Bundle",
          },
          descriptors: [
            signalsMod.resourceBinaryDescriptor.export({
              id: "report-export",
              fileName: `${reportId}.zip`,
              mediaType: "application/zip",
              byteLength: "Bundle".length * 256,
              download: signalsMod.resourceDownload.multipart({
                url: `https://downloads.example/${reportId}`,
                headers: { authorization: "secret-token" },
                fields: { token: reportId },
                objectKey: `exports/${reportId}.zip`,
                expiresAt: "2026-05-05T12:00:00Z",
              }),
            }),
          ],
        }),
    });

    assert.deepEqual(
      normalizeDownloadLineArtifact(routeReport.line({ reportId: "report-q3" })),
      normalizeDownloadLineArtifact(rawReport.line({ reportId: "report-q3" })),
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...) owns resourceBinaryValue(...) in the pleasant lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const report = runtime.signals.api({}).url("/reports/:reportId").downloads((_params, _value, download) => [
        download.file("report-pdf", {
          fileName: "report.pdf",
          mediaType: "application/pdf",
          download: download.ready({
            url: "https://downloads.example/report.pdf",
          }),
        }),
      ])
      .detail({
        load: ({ reportId }) =>
        runtime.signalsMod.resourceBinaryValue({
          value: { id: reportId, title: "Quarterly Report" },
          descriptors: [],
        }),
      });
    assert.throws(
      () => report.line({ reportId: "report-q1" }),
      /owns resourceBinaryValue/,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...) rejects promised resourceBinaryValue(...) in the pleasant lane", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    const report = runtime.signals.api({}).url("/reports/:reportId").downloads((_params, _value, download) => [
        download.file("report-pdf", {
          fileName: "report.pdf",
          mediaType: "application/pdf",
          download: download.ready({
            url: "https://downloads.example/report.pdf",
          }),
        }),
      ])
      .detail({
        load: async ({ reportId }) =>
        runtime.signalsMod.resourceBinaryValue({
          value: { id: reportId, title: "Quarterly Report" },
          descriptors: [],
        }),
      });
    const line = report.line({ reportId: "report-q1" });
    await new Promise((resolve) => setTimeout(resolve, 0));

    assert.deepEqual(line.status(), {
      kind: "rejected",
      operation: "initialLoad",
      message:
        'api.url("/reports/:reportId") downloads(...) owns resourceBinaryValue(...) in the pleasant lane',
      continuity: "noVisibleValueYet",
    });
    assert.equal(
      line.diagnostics().lastErrorMessage,
      'api.url("/reports/:reportId") downloads(...) owns resourceBinaryValue(...) in the pleasant lane',
    );
  } finally {
    await runtime.cleanup();
  }
});

test("api.url(...).downloads(...).detail(...) denies mixed builder and declaration download ownership", async () => {
  const runtime = await createRealTransferRuntime();
  try {
    assert.throws(
      () => runtime.signals.api({}).url("/reports/:reportId")
        .downloads((_params, _value, _download) => [])
        .detail({
          downloads: (_params, _value, download) => [
            download.file("report-pdf", {
              fileName: "report.pdf",
              mediaType: "application/pdf",
              download: download.ready({
                url: "https://downloads.example/report.pdf",
              }),
            }),
          ],
          load: ({ reportId }) => ({ id: reportId, title: "Quarterly Report" }),
        }),
      /owns downloads/,
    );
  } finally {
    await runtime.cleanup();
  }
});

import { createSignals } from "../../../index.js";

const signals = createSignals();

signals.api({}).url("/reports/:reportId").downloads(
  // @ts-expect-error downloads(...) must return binary descriptors, not arbitrary values
  (_params, value: { id: string }, _download) => [
    value,
  ],
).detail({
  load: ({ reportId }) => ({ id: String(reportId) }),
});

signals.api({}).url("/reports/:reportId").downloads((_params, _value, download) => [
  download.file("report-pdf", {
    fileName: "report.pdf",
    mediaType: "application/pdf",
    // @ts-expect-error pleasant download ready(...) keeps the raw GET/POST boundary
    download: download.ready({ url: "https://downloads.example/report.pdf", method: "PUT" }),
  }),
]).detail({
  load: ({ reportId }) => ({ id: reportId }),
});

signals.api({}).url("/reports/:reportId").downloads((_params, _value, _download) => []).detail({
  // @ts-expect-error builder-owned downloads(...) forbids restating downloads in the final declaration
  downloads: (_params, _value, download) => [
    download.file("report-pdf", {
      fileName: "report.pdf",
      mediaType: "application/pdf",
      download: download.ready({ url: "https://downloads.example/report.pdf" }),
    }),
  ],
  load: ({ reportId }) => ({ id: reportId }),
});

signals.api({}).url("/reports/export").downloads((_params, _value, _download) => []).create({
  // @ts-expect-error builder-owned downloads(...) forbids restating downloads in write finalizers too
  downloads: (_params, _value, download) => [
    download.export("report-export", {
      fileName: "report.zip",
      mediaType: "application/zip",
      download: download.ready({ url: "https://downloads.example/report.zip" }),
    }),
  ],
  load: ({ body }: { body: { jobId: string } }) => ({ jobId: body.jobId }),
});

signals.api({}).url("/reports/:reportId").downloads((_params, _value, download) => [
  download.export("report-export", {
    fileName: "report.zip",
    mediaType: "application/zip",
    download: download.multipart({
      url: "https://downloads.example/report",
      // @ts-expect-error multipart download fields must stay string-valued
      fields: { token: 7 },
    }),
  }),
]).detail({
  load: ({ reportId }) => ({ id: reportId }),
});

signals.api({}).url("/workspaces/:workspaceId/catalog")
  .items((item: { id: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string }>; total: number }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .downloads((_params, _value, _download) => [])
  .list({
    // @ts-expect-error reconcile collection lanes cannot restate downloads after builder ownership
    downloads: (_params, _value, download) => [
      download.export("catalog-export", {
        fileName: "catalog.zip",
        mediaType: "application/zip",
        download: download.multipart({
          url: "https://downloads.example/catalog",
          fields: { token: "demo" },
        }),
      }),
    ],
    load: ({ workspaceId }) => ({
      items: [{ id: `${workspaceId}:1` }],
      total: 1,
    }),
  });

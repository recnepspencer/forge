import { createSignals } from "../../../index.js";

const signals = createSignals();
const api = signals.api({});

const reportDetail = api.url("/reports/:reportId")
  .downloads(({ reportId }, value: { id: string; title: string }, download) => [
    download.file("report-pdf", {
      fileName: `${reportId}.pdf`,
      mediaType: "application/pdf",
      byteLength: value.title.length * 100,
      download: download.ready({
        url: `https://downloads.example/${reportId}.pdf`,
        headers: { authorization: "secret-token" },
      }),
    }),
  ])
  .detail({
    load: ({ reportId }) => ({
      id: String(reportId),
      title: "Quarterly Report",
    }),
  });

const reportPages = api.url("/workspaces/:workspaceId/reports")
  .items((item: { id: string; title: string }) => item.id)
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
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: `${workspaceId}:r1`, title: "Report" }],
  });

const reportDetailLine = reportDetail.line({ reportId: "report-q1" });
const reportPagesLine = reportPages.line({ workspaceId: "demo" });

void reportDetailLine.value();
void reportDetailLine.download();
void reportPagesLine.value();
void reportPagesLine.download();

const multipartReport = api.url("/reports/:reportId")
  .downloads(({ reportId }, _value: { id: string }, download) => [
    download.export("report-export", {
      fileName: `${reportId}.zip`,
      mediaType: "application/zip",
      download: download.multipart({
        url: `https://downloads.example/${reportId}`,
        headers: { authorization: "secret-token" },
        fields: { token: String(reportId) },
        objectKey: `exports/${reportId}.zip`,
      }),
    }),
  ])
  .detail({
    load: ({ reportId }) => ({ id: String(reportId) }),
  });

void multipartReport.line({ reportId: "report-q2" }).download();

const exportJob = api.url("/reports/export")
  .downloads((_params, value: { jobId: string }, download) => [
    download.export("report-export", {
      fileName: `${value.jobId}.zip`,
      mediaType: "application/zip",
      download: download.ready({
        url: `https://downloads.example/${value.jobId}.zip`,
      }),
    }),
  ])
  .create({
    load: ({ body }: { body: { jobId: string } }) => ({ jobId: body.jobId }),
  });

const catalogList = api.url("/workspaces/:workspaceId/catalog")
  .items((item: { id: string; title: string }) => item.id)
  .reconcile(
    (value: { items: Array<{ id: string; title: string }>; total: number }) => value.items,
    (value, nextItems) => ({ ...value, items: [...nextItems] }),
  )
  .downloads(({ workspaceId }, value, download) => [
    download.export("catalog-export", {
      fileName: `${workspaceId}-catalog.zip`,
      mediaType: "application/zip",
      byteLength: value.total * 256,
      download: download.multipart({
        url: `https://downloads.example/${workspaceId}/catalog`,
        fields: { token: String(workspaceId) },
        objectKey: `catalog/${workspaceId}.zip`,
      }),
    }),
  ])
  .list({
    load: ({ workspaceId }) => ({
      items: [{ id: `${workspaceId}:1`, title: "First" }],
      total: 1,
    }),
  });

void exportJob.line({ body: { jobId: "job-7" } }).download();
void catalogList.line({ workspaceId: "demo" }).download();

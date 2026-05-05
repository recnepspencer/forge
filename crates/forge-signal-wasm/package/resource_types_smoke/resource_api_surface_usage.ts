import { createSignals } from "../index.js";

const signals = createSignals();

const api = signals.api({
  headers: {
    authorization: "Bearer shared",
  },
});

const userDetail = api.url("/users/:userId").detail({
  headers: ({ userId }) => ({
    "x-user-id": String(userId),
  }),
  load: ({ userId }) => ({ id: userId }),
});

const taskList = api.url("/workspaces/:workspaceId/tasks").list({
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ workspaceId }) => [{ id: workspaceId }],
});

const taskPages = api.url("/workspaces/:workspaceId/tasks").paged({
  itemIdentity: (item: { id: string }) => item.id,
  accumulatePage: (
    existing: Array<{ id: string }>,
    next: Array<{ id: string }>,
  ) => [...existing, ...next],
  load: ({ workspaceId }) => [{ id: String(workspaceId) }],
});

const scopedFeatureApi = signals.scope("catalog").api({
  headers: {
    "x-feature": "catalog",
  },
});

const scopedDetail = scopedFeatureApi.url("/products/:productId").detail({
  load: ({ productId }) => ({ id: productId }),
});
const homeDetail = api.url("/").detail({
  load: () => ({ ok: true }),
});

const userLine = userDetail.line({ userId: "u1" });
const taskListLine = taskList.line({ workspaceId: "demo" });
const taskPagesLine = taskPages.line({ workspaceId: "demo" });
const scopedLine = scopedDetail.line({ productId: "p1" });
const homeLine = homeDetail.line({});
const exportReport = api.url("/reports/export:csv").detail({
  load: () => ({ ok: true }),
});
const exportReportLine = exportReport.line({});

void userLine.value();
void taskListLine.value();
void taskPagesLine.value();
void scopedLine.value();
void homeLine.value();
void exportReportLine.value();

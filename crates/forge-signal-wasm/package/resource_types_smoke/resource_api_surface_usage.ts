import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceValueSummaries,
} from "../index.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspect,
  ResourceValueSummary,
} from "../index.js";

const signals = createSignals();

type TaskCatalogValue = {
  items: Array<{ id: string; title: string }>;
  total: number;
};

type TaskCatalogItem = { id: string; title: string };

type TaskCatalogReconcile = ResourceCollectionShape<
  TaskCatalogValue,
  TaskCatalogItem,
  {
    title: ResourceItemAspect<TaskCatalogItem, string>;
  },
  {
    total: ResourceValueSummary<TaskCatalogValue, number>;
  }
>;

const api = signals.api({
  baseUrl: "/api",
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

const userSearch = api.url("/users").params<{
  search?: string;
  page?: number;
}>().list({
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ params }) => [{ id: `${params.search ?? "all"}:${params.page ?? 1}` }],
});

const workspaceAudit = api.url("/workspaces/:workspaceId/audit").params<{
  includeArchived?: boolean;
  sort?: "asc" | "desc";
}>().paged({
  itemIdentity: (item: { id: string }) => item.id,
  accumulatePage: (
    existing: Array<{ id: string }>,
    next: Array<{ id: string }>,
  ) => [...existing, ...next],
  load: ({ workspaceId, params }) => [
    { id: `${workspaceId}:${params.sort ?? "asc"}:${params.includeArchived ?? false}` },
  ],
});

const taskCatalog = api.url("/workspaces/:workspaceId/task-catalog").list<
  TaskCatalogValue,
  TaskCatalogItem,
  TaskCatalogReconcile
>({
  itemIdentity: (item: TaskCatalogItem) => item.id,
  reconcile: resourceCollectionShape({
    items: (value: TaskCatalogValue) => value.items,
    replaceItems: (
      value: TaskCatalogValue,
      nextItems: readonly TaskCatalogItem[],
    ) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (item: TaskCatalogItem) => item.title,
        write: (item: TaskCatalogItem, title: string) => ({ ...item, title }),
      },
    }),
    summaries: resourceValueSummaries({
      total: {
        read: (value: TaskCatalogValue) => value.total,
        write: (value: TaskCatalogValue, total: number) => ({ ...value, total }),
      },
    }),
  }),
  load: ({ workspaceId }) => ({
    items: [{ id: String(workspaceId), title: "Task" }],
    total: 1,
  }),
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
  baseUrl: "/catalog",
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
const userSearchLine = userSearch.line({
  params: {
    search: "ada",
    page: 2,
  },
});
const workspaceAuditLine = workspaceAudit.line({
  workspaceId: "demo",
  params: {
    includeArchived: true,
    sort: "desc",
  },
});
const taskListLine = taskList.line({ workspaceId: "demo" });
const taskPagesLine = taskPages.line({ workspaceId: "demo" });
const scopedLine = scopedDetail.line({ productId: "p1" });
const homeLine = homeDetail.line({});
const exportReport = api.url("/reports/export:csv").detail({
  load: () => ({ ok: true }),
});
const exportReportLine = exportReport.line({});
const taskCatalogLine = taskCatalog.line({ workspaceId: "demo" });
const userRequestBaseUrl: string | null = userLine.request().baseUrl;
const userRequestPath: string | null = userLine.request().target.requestPath;
const userRequestTargetUrl: string | null = userLine.request().target.url;
const taskCatalogTitlePatch = taskCatalog.patch.itemAspect({
  itemId: "demo",
  aspect: "title",
  value: "Updated",
});

void userLine.value();
void userSearchLine.value();
void workspaceAuditLine.value();
void taskListLine.value();
void taskPagesLine.value();
void scopedLine.value();
void homeLine.value();
void exportReportLine.value();
void taskCatalogLine.patch(taskCatalogTitlePatch);
void userRequestBaseUrl;
void userRequestPath;
void userRequestTargetUrl;

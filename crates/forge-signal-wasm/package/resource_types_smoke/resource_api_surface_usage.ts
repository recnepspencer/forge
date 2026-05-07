import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceProcessingResult,
  resourceUploadResult,
  resourceValueSummaries,
} from "../index.js";
import type {
  ApiCollectionResourceFamily,
  ApiDetailResourceFamily,
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
const createUser = api.url("/users").create({
  load: ({ body }: { body: { userId: string; name: string } }) => ({
    id: body.userId,
    name: body.name,
  }),
});
const updateUser = api.url("/users/:userId").update({
  load: ({ userId, body }: { userId: string; body: { name: string } }) => ({
    id: userId,
    name: body.name,
  }),
});
const removeUser = api.url("/users/:userId").remove({
  load: ({ userId }) => ({ removed: userId }),
});
const prepareReceiptUpload = api.url("/receipts/:receiptId/upload")
  .signedUpload({
    method: "POST",
    finalizeRequired: true,
  })
  .processing("poll")
  .create({
    load: ({ receiptId, body }: { receiptId: string; body: { fileName: string } }) =>
      resourceUploadResult.prepared({
        uploadId: `upload:${receiptId}`,
        descriptor: {
          kind: "signed",
          url: `https://uploads.example/${receiptId}`,
          method: "POST",
          headers: { "x-upload-token": body.fileName },
          fields: {},
          objectKey: `receipts/${receiptId}`,
          expiresAt: null,
        },
        finalizeRequired: true,
        message: "ready",
      }),
  });
const reportStatus = api.url("/reports/:reportId")
  .processing("callback", {
    callbackId: "report-ready",
  })
  .detail({
    load: ({ reportId }) =>
      resourceProcessingResult.accepted({
        jobId: `job:${reportId}`,
        message: "queued",
      }),
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
const fluentTaskCatalog = api.url("/workspaces/:workspaceId/fluent-task-catalog")
  .items((item: TaskCatalogItem) => item.id)
  .reconcile(
    (value: TaskCatalogValue) => value.items,
    (
      value: TaskCatalogValue,
      nextItems: readonly TaskCatalogItem[],
    ) => ({ ...value, items: [...nextItems] }),
  )
  .aspect(
    "title",
    (item: TaskCatalogItem) => item.title,
    (item: TaskCatalogItem, title: string) => ({ ...item, title }),
  )
  .summary(
    "total",
    (value: TaskCatalogValue) => value.total,
    (value: TaskCatalogValue, total: number) => ({ ...value, total }),
  )
  .list({
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
const directTaskList = api.url("/workspaces/:workspaceId/direct-tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
  });
const directTaskPages = api.url("/workspaces/:workspaceId/direct-task-pages")
  .items((item: { id: string; title: string }) => item.id)
  .paged({
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
  });
const directTaskCatalog = api.url("/workspaces/:workspaceId/direct-task-catalog")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  )
  .summary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  .list({
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
  });
const directTaskPageWindow = api.url("/workspaces/:workspaceId/direct-task-window")
  .items((item: { id: string; title: string }) => item.id)
  .aspect(
    "title",
    (item: { id: string; title: string }) => item.title,
    (item: { id: string; title: string }, title: string) => ({ ...item, title }),
  )
  .pageWindowSummary(
    "count",
    (value: readonly { id: string; title: string }[]) => value.length,
    (
      value: readonly { id: string; title: string }[],
      count: number,
    ) => value.slice(0, count),
  )
  .paged({
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
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
const asyncWorkspaceDetail = api.url("/workspaces/:workspaceId").detail({
  load: async ({ workspaceId }) => {
    const typedWorkspaceId: string | number = workspaceId;
    void typedWorkspaceId;
    return { id: String(workspaceId) };
  },
});
const asyncWorkspaceVersions = api.url("/workspaces/:workspaceId/versions/:versionId").list({
  itemIdentity: (item: { id: string }) => item.id,
  load: async ({ workspaceId, versionId }) => {
    const typedWorkspaceId: string | number = workspaceId;
    const typedVersionId: string | number = versionId;
    void typedWorkspaceId;
    void typedVersionId;
    return [{ id: `${workspaceId}:${versionId}` }];
  },
});
const typedAsyncWorkspaceDetail:
  ApiDetailResourceFamily<"/workspaces/:workspaceId", undefined, { id: string }> =
    asyncWorkspaceDetail;
const typedAsyncWorkspaceVersions:
  ApiCollectionResourceFamily<
    "/workspaces/:workspaceId/versions/:versionId",
    undefined,
    readonly { id: string }[],
    { id: string }
  > = asyncWorkspaceVersions;

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
const exportUsers = api.url("/users/export").create({
  load: ({ body }: { body: { jobId: string } }) => ({ jobId: body.jobId }),
});
const createUserLine = createUser.line({
  body: {
    userId: "u2",
    name: "Ada",
  },
});
const updateUserLine = updateUser.line({
  userId: "u1",
  body: {
    name: "Grace",
  },
});
const removeUserLine = removeUser.line({ userId: "u1" });
const prepareReceiptUploadLine = prepareReceiptUpload.line({
  receiptId: "r1",
  body: {
    fileName: "receipt.png",
  },
});
const reportStatusLine = reportStatus.line({ reportId: "report-1" });
const exportReportLine = exportReport.line({});
const exportUsersLine = exportUsers.line({
  body: {
    jobId: "job-1",
  },
});
const taskCatalogLine = taskCatalog.line({ workspaceId: "demo" });
const fluentTaskCatalogLine = fluentTaskCatalog.line({ workspaceId: "demo" });
const directTaskListLine = directTaskList.line({ workspaceId: "demo" });
const directTaskPagesLine = directTaskPages.line({ workspaceId: "demo" });
const directTaskCatalogLine = directTaskCatalog.line({ workspaceId: "demo" });
const directTaskPageWindowLine = directTaskPageWindow.line({ workspaceId: "demo" });
const asyncWorkspaceDetailLine = typedAsyncWorkspaceDetail.line({ workspaceId: "demo" });
const asyncWorkspaceVersionsLine = typedAsyncWorkspaceVersions.line({
  workspaceId: "demo",
  versionId: 7,
});
const userRequestBaseUrl: string | null = userLine.request().baseUrl;
const createUserRequestMethod = createUserLine.request().method;
const createUserRequestBody = createUserLine.request().body;
const prepareReceiptUploadTransportKind = prepareReceiptUploadLine.request().uploadTransport.kind;
const prepareReceiptProcessingKind = prepareReceiptUploadLine.request().processingJob.kind;
const reportStatusProcessingKind = reportStatusLine.request().processingJob.kind;
const userRequestPath: string | null = userLine.request().target.requestPath;
const userRequestTargetUrl: string | null = userLine.request().target.url;
const taskCatalogTitlePatch = taskCatalog.patch.itemAspect({
  itemId: "demo",
  aspect: "title",
  value: "Updated",
});
const taskCatalogSummaryDelivery = taskCatalog.delivery.summary({
  packetId: "pkt-task-total",
  basisId: "basis-1",
  nextBasisId: "basis-2",
  summary: "total",
  value: 2,
});
const fluentTaskCatalogAspectPatch = fluentTaskCatalog.patch.itemAspect({
  itemId: "demo",
  aspect: "title",
  value: "Fluent",
});
const fluentTaskCatalogSummaryDelivery = fluentTaskCatalog.delivery.summary({
  packetId: "pkt-fluent-total",
  basisId: "basis-1",
  nextBasisId: "basis-2",
  summary: "total",
  value: 2,
});
const taskPagesReplaceDelivery = taskPages.delivery.replace({
  packetId: "pkt-task-pages-replace",
  basisId: "basis-1",
  nextBasisId: "basis-2",
  nextValue: [{ id: "demo-replaced" }],
});
const directTaskItemPatch = directTaskList.patch.item({
  itemId: "demo",
  nextItem: { id: "demo", title: "Updated" },
});
const directTaskItemDelivery = directTaskPages.delivery.item({
  packetId: "pkt-direct-task",
  basisId: null,
  nextBasisId: "basis-1",
  itemId: "demo",
  nextItem: { id: "demo", title: "Delivered" },
});
const directTaskAspectPatch = directTaskCatalog.patch.itemAspect({
  itemId: "demo",
  aspect: "title",
  value: "Renamed",
});
const directTaskSummaryDelivery = directTaskCatalog.delivery.summary({
  packetId: "pkt-direct-summary",
  basisId: "basis-1",
  nextBasisId: "basis-2",
  summary: "count",
  value: 1,
});
const directTaskWindowSummaryPatch = directTaskPageWindow.patch.summary({
  summary: "count",
  value: 1,
});
const directTaskWindowSummaryDelivery = directTaskPageWindow.delivery.summary({
  packetId: "pkt-direct-window",
  basisId: "basis-1",
  nextBasisId: "basis-2",
  summary: "count",
  value: 1,
});

void userLine.value();
void createUserLine.value();
void updateUserLine.value();
void removeUserLine.value();
void prepareReceiptUploadLine.upload();
void prepareReceiptUploadLine.processing();
void reportStatusLine.processing();
void userSearchLine.value();
void workspaceAuditLine.value();
void taskListLine.value();
void taskPagesLine.value();
void scopedLine.value();
void homeLine.value();
void exportUsersLine.value();
void exportReportLine.value();
void taskCatalogLine.patch(taskCatalogTitlePatch);
void taskCatalogLine.deliver(taskCatalogSummaryDelivery);
void fluentTaskCatalogLine.patch(fluentTaskCatalogAspectPatch);
void fluentTaskCatalogLine.deliver(fluentTaskCatalogSummaryDelivery);
void taskPagesLine.deliver(taskPagesReplaceDelivery);
void directTaskListLine.patch(directTaskItemPatch);
void directTaskPagesLine.deliver(directTaskItemDelivery);
void directTaskCatalogLine.patch(directTaskAspectPatch);
void directTaskCatalogLine.deliver(directTaskSummaryDelivery);
void directTaskPageWindowLine.patch(directTaskWindowSummaryPatch);
void directTaskPageWindowLine.deliver(directTaskWindowSummaryDelivery);
void userRequestBaseUrl;
void createUserRequestMethod;
void createUserRequestBody;
void prepareReceiptUploadTransportKind;
void prepareReceiptProcessingKind;
void reportStatusProcessingKind;
void userRequestPath;
void userRequestTargetUrl;

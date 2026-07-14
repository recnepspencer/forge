import { createSignals } from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });
const api = signals.api({ baseUrl: "/api" });

const taskList = api.url("/workspaces/:workspaceId/tasks")
  .response(signals.resource.response.collection({
    itemId: (item: { id: string; title: string }) => item.id,
    items: (value: { items: Array<{ id: string; title: string }>; total: number }) =>
      value.items,
    replaceItems: (
      value: { items: Array<{ id: string; title: string }>; total: number },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    summaries: signals.resource.valueSummaries({
      total: {
        read: (value: { items: Array<{ id: string; title: string }>; total: number }) =>
          value.total,
        write: (
          value: { items: Array<{ id: string; title: string }>; total: number },
          total: number,
        ) => ({ ...value, total }),
      },
    }),
  }))
  .list({
    load: ({ workspaceId }) => ({
      items: [{ id: String(workspaceId), title: "Task" }],
      total: 1,
    }),
  });

const removeTask = api.url("/workspaces/:workspaceId/tasks/:taskId")
  .response(signals.resource.response.detail<{
    id: string;
    total: number;
  }>()({
    total: "total",
  }))
  .remove({
    reconciles: [
      {
        family: taskList,
        params: ({ workspaceId }) => ({ workspaceId }),
        fallback: "deletionUnavailable",
        collection: {
          kind: "delete",
          itemId: ({ taskId }) => String(taskId),
        },
      },
      {
        family: taskList,
        params: ({ workspaceId }) => ({ workspaceId }),
        fallback: "refetchRequired",
        summary: { kind: "summary", summary: "total" },
      },
    ],
    load: ({ taskId }) => ({ id: String(taskId), total: 0 }),
  });

const plan = removeTask.line({
  workspaceId: "demo",
  taskId: "demo-task",
}).mutationResponse();
const artifact = plan.executionArtifacts[0];

if (artifact.kind === "exactCollectionDelete") {
  void artifact.itemId;
  void artifact.deliveryScope;
}

const taskDetail = api.url("/workspaces/:workspaceId/tasks/:taskId").detail({
  load: ({ taskId }) => ({ id: String(taskId), title: "Task" }),
});

const invalidateTask = api.url("/workspaces/:workspaceId/tasks/:taskId")
  .response(signals.resource.response.summary<number>()())
  .remove({
    reconciles: [
      {
        family: taskDetail,
        params: ({ workspaceId, taskId }) => ({ workspaceId, taskId }),
        fallback: "refetchRequired",
        detail: { kind: "invalidate" },
      },
    ],
    load: () => 0,
  });

const invalidationArtifact = invalidateTask.line({
  workspaceId: "demo",
  taskId: "demo-task",
}).mutationResponse().executionArtifacts[0];

if (invalidationArtifact.kind === "exactDetailInvalidation") {
  void invalidationArtifact.deliveryKind;
  void invalidationArtifact.deliveryScope;
}

const removeTaskMetadataOnly = api.url("/workspaces/:workspaceId/tasks/:taskId")
  .response(signals.resource.response.summary<number>()())
  .remove({
    reconciles: [
      {
        family: taskList,
        params: ({ workspaceId }) => ({ workspaceId }),
        fallback: "deletionUnavailable",
        collection: {
          kind: "delete",
          itemId: ({ taskId }) => String(taskId),
        },
      },
    ],
    load: () => 0,
  });

const metadataDeleteArtifact = removeTaskMetadataOnly.line({
  workspaceId: "demo",
  taskId: "demo-task",
}).mutationResponse().executionArtifacts[0];

if (metadataDeleteArtifact.kind === "exactCollectionDelete") {
  void metadataDeleteArtifact.itemId;
}

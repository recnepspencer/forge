import { createSignals } from "../../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

type Task = {
  id: string;
  title: string;
  status: "open" | "done";
  assigneeId: string | null;
  metadata: {
    priority: number;
    labels: readonly string[];
    nested: { rank: number };
    note?: string | null;
  };
};

const taskResponse = signals.resource.response.array({
  itemId: (item: Task) => item.id,
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
    status: "status",
    assigneeId: "assigneeId",
  }),
});

const tasks = signals.api({}).url("/workspaces/:workspaceId/tasks")
  .response(taskResponse)
  .list({
    load: ({ workspaceId }) => [{
      id: `${workspaceId}:1`,
      title: "Task",
      status: "open" as const,
      assigneeId: null,
      metadata: {
        priority: 1,
        labels: ["first"],
        nested: { rank: 1 },
      },
    }],
  });

const taskDetailResponse = signals.resource.response.detail<Task>()();
const taskDetail = signals.api({}).url("/tasks/:taskId")
  .response(taskDetailResponse)
  .detail({
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open" as const,
      assigneeId: null,
      metadata: {
        priority: 1,
        labels: ["first"],
        nested: { rank: 1 },
      },
    }),
  });

const createTask = signals.api({}).url("/tasks")
  .response(taskDetailResponse)
  .create({
    identity: {
      submitted: ({ body }: { body: Task }) => body.id,
      response: (value: Task) => value.id,
      canonical: (value: Task) => value.id,
      atomicity: "partialAllowed",
      targets: [{
        family: taskDetail,
        params: ({ body }: { body: Task }) => ({ taskId: body.id }),
        canonicalParams: (_params: { body: Task }, _value: Task, canonicalIdentity: string) => ({
          taskId: canonicalIdentity,
        }),
        fallback: "identityMigrationUnavailable",
      }, {
        family: tasks,
        params: () => ({ workspaceId: "draft-workspace" }),
        canonicalParams: (
          _params: { body: Task },
          _value: Task,
          canonicalIdentity: string,
        ) => ({ workspaceId: canonicalIdentity }),
        fallback: "identityMigrationUnavailable",
        summary: { kind: "summary", summary: "total" },
      }, {
        family: taskDetail,
        params: ({ body }: { body: Task }) => ({ taskId: body.id }),
        canonicalParams: (_params: { body: Task }, _value: Task, canonicalIdentity: string) => ({
          taskId: canonicalIdentity,
        }),
        fallback: "deliveryAwaited",
        selection: { kind: "visibleSelection" },
      }, {
        family: taskDetail,
        params: ({ body }: { body: Task }) => ({ taskId: body.id }),
        fallback: "refetchRequired",
        detailChild: { kind: "detailChild", region: "children" },
      }],
    },
    load: ({ body }: { body: Task }) => body,
  });

void createTask.line({
  body: {
    id: "tmp-1",
    title: "Draft",
    status: "open",
    assigneeId: null,
    metadata: {
      priority: 1,
      labels: [],
      nested: { rank: 1 },
    },
  },
}).mutationResponse()?.identityMigration?.canonicalIdentity;

const taskSummary = signals.api({}).url("/task-summary")
  .response(signals.resource.response.summary<{ total: number }>())
  .detail({
    load: () => ({ total: 1 }),
  });

const taskStatusFields = signals.resource.detailFields({
  status: {
    read: (value: Task) => value.status,
    write: (value: Task, status: Task["status"]) => ({ ...value, status }),
  },
});
const taskPermissionFields = signals.resource.detailFields({
  canEdit: {
    read: (value: { id: string; canEdit: boolean }) => value.canEdit,
    write: (value: { id: string; canEdit: boolean }, canEdit: boolean) => ({ ...value, canEdit }),
  },
});
const taskDetailWithStatus = signals.api({}).url("/tasks/:taskId")
  .response(taskDetailResponse)
  .detail({
    reconcile: taskStatusFields,
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open" as const,
      assigneeId: null,
      metadata: {
        priority: 1,
        labels: ["first"],
        nested: { rank: 1 },
      },
    }),
  });
const taskPermissions = signals.api({}).url("/task-permissions/:taskId")
  .detail({
    reconcile: taskPermissionFields,
    load: ({ taskId }) => ({ id: taskId, canEdit: false }),
  });
const taskAudit = signals.api({}).url("/task-audit/:taskId")
  .response(taskResponse)
  .list({
    load: ({ taskId }) => [{
      id: `${taskId}:entry-1`,
      title: "Audit entry",
      status: "open" as const,
      assigneeId: null,
      metadata: {
        priority: 1,
        labels: ["first"],
        nested: { rank: 1 },
      },
    }],
  });

const saveTask = signals.api({}).url("/tasks/:taskId")
  .response(signals.resource.response.detail<{
    id: string;
    status: Task["status"];
    total: number;
    canEdit: boolean;
    warnings: readonly string[];
  }>()({
    status: "status",
    total: "total",
    canEdit: "canEdit",
    warnings: "warnings",
  }))
  .update({
    atomicity: "partialAllowed",
    reconciles: [{
      family: taskDetailWithStatus,
      params: ({ taskId }: { taskId: string | number }) => ({ taskId: String(taskId) }),
      fallback: "partialReconciliation",
      detail: { kind: "field", field: "status" },
    }, {
      family: tasks,
      params: () => ({ workspaceId: "demo" }),
      fallback: "partialReconciliation",
      collection: { kind: "item" },
    }, {
      family: taskSummary,
      params: () => ({}),
      fallback: "partialReconciliation",
      summary: { kind: "summary", summary: "total" },
    }, {
      family: taskPermissions,
      params: ({ taskId }: { taskId: string | number }) => ({ taskId: String(taskId) }),
      fallback: "deliveryAwaited",
      detail: { kind: "field", field: "canEdit" },
    }, {
      family: taskAudit,
      params: ({ taskId }: { taskId: string | number }) => ({ taskId: String(taskId) }),
      fallback: "unsupportedTarget",
    }],
    diagnostics: [{ kind: "warnings", field: "warnings" }],
    load: ({ taskId }: { taskId: string | number }) => ({
      id: String(taskId),
      status: "done" as const,
      total: 2,
      canEdit: true,
      warnings: ["audit declaration remains fallback-only"],
    }),
  });

void taskSummary.line({}).value();
void saveTask.line({ taskId: "t1", body: {} }).mutationResponse()?.targets[0]?.fallback.kind;

const commandSaveTask = signals.api({}).url("/tasks/:taskId").verb("POST")
  .response(signals.resource.response.detail<{
    id: string;
    status: Task["status"];
  }>()({
    status: "status",
  }))
  .update({
    reconciles: [{
      family: taskDetailWithStatus,
      params: ({ taskId }: { taskId: string | number }) => ({ taskId: String(taskId) }),
      fallback: "partialReconciliation",
      detail: { kind: "field", field: "status" },
    }, {
      family: tasks,
      params: () => ({ workspaceId: "demo" }),
      fallback: "partialReconciliation",
      collection: { kind: "item" },
    }],
    diagnostics: [{ kind: "warnings", field: "status" }],
    load: ({ taskId }: { taskId: string | number }) => ({
      id: String(taskId),
      status: "done" as const,
    }),
  });

const commandDeleteTask = signals.api({}).url("/tasks/:taskId").verb("POST")
  .response(signals.resource.response.summary<{ total: number }>())
  .remove({
    reconciles: [{
      family: tasks,
      params: () => ({ workspaceId: "demo" }),
      fallback: "deletionUnavailable",
      collection: {
        kind: "delete",
        itemId: ({ taskId }: { taskId: string | number }) => String(taskId),
      },
    }, {
      family: taskSummary,
      params: () => ({}),
      fallback: "refetchRequired",
      summary: { kind: "summary", summary: "total" },
    }],
    load: () => ({ total: 1 }),
  });

void commandSaveTask.line({ taskId: "t2", body: {} }).request().method;
void commandDeleteTask.line({ taskId: "t2" }).request().method;

const semanticMutationTask = signals.api({}).url("/tasks/:taskId")
  .response(signals.resource.response.detail<{
    id: string;
    status: Task["status"];
  }>()({
    status: "status",
  }))
  .mutation({
    semantics: "update",
    method: "POST",
    reconciles: [{
      family: taskDetailWithStatus,
      params: ({ taskId }: { taskId: string | number }) => ({ taskId: String(taskId) }),
      fallback: "partialReconciliation",
      detail: { kind: "field", field: "status" },
    }],
    load: ({ taskId }: { taskId: string | number }) => ({
      id: String(taskId),
      status: "done" as const,
    }),
  });

const semanticCommandTask = signals.api({}).url("/tasks/:taskId/archive")
  .response(signals.resource.response.summary<{ total: number }>())
  .command({
    semantics: "relationshipUpdate",
    method: "POST",
    reconciles: [{
      family: tasks,
      params: () => ({ workspaceId: "demo" }),
      fallback: "refetchRequired",
    }],
    load: () => ({ total: 1 }),
  });

void semanticMutationTask.line({ taskId: "t3", body: {} }).request().method;
void semanticCommandTask.line({ taskId: "t3", body: { archived: true } }).request().method;

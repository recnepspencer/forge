import { createSignals } from "../../../index.js";

const signals = createSignals();

type Task = {
  id: string;
  title: string;
  status: "open" | "done";
  assigneeId: string | null;
  metadata: { priority: number };
};

type TaskEnvelope = {
  tasks: readonly Task[];
  nextCursor: string | null;
};

type TaskConnection = {
  edges: readonly { node: Task }[];
  pageInfo: { hasNextPage: boolean };
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
    load: ({ workspaceId }) => [
      {
        id: `${workspaceId}:1`,
        title: "Task",
        status: "open" as const,
        assigneeId: null,
        metadata: { priority: 1 },
      },
    ],
  });

const line = tasks.line({ workspaceId: "demo" });
const titlePatch = tasks.patch.itemAspect({
  itemId: "demo:1",
  aspect: "title",
  value: "Renamed",
});
const statusDelivery = tasks.delivery.itemAspect({
  packetId: "pkt-status",
  basisId: null,
  nextBasisId: "basis-1",
  itemId: "demo:1",
  aspect: "status",
  value: "done",
});

void line.patch(titlePatch);
void line.deliver(statusDelivery);

const taskEnvelopeResponse = signals.resource.response.objectItems<TaskEnvelope>()({
  field: "tasks",
  itemId: (item) => item.id,
  aspects: signals.resource.response.jsonObjectAspects<Task>()({
    metadata: "metadata",
  }),
});

const taskEnvelope = signals.api({}).url("/task-page")
  .response(taskEnvelopeResponse)
  .list({
    load: () => ({
      tasks: [
        {
          id: "t1",
          title: "Task",
          status: "open" as const,
          assigneeId: null,
          metadata: { priority: 1 },
        },
      ],
      nextCursor: null,
    }),
  });

const envelopePatch = taskEnvelope.patch.itemAspect({
  itemId: "t1",
  aspect: "metadata",
  value: { priority: 2 },
});

void taskEnvelope.line({}).patch(envelopePatch);

const taskConnectionResponse = signals.resource.response.collection<TaskConnection>()({
  itemId: (item) => item.id,
  items: (value) => value.edges.map((edge) => edge.node),
  replaceItems: (value, nextItems) => ({
    ...value,
    edges: nextItems.map((node) => ({ node })),
  }),
  aspects: signals.resource.response.objectAspects<Task>()({
    status: "status",
  }),
});

const taskConnection = signals.api({}).url("/task-connection")
  .response(taskConnectionResponse)
  .list({
    load: () => ({
      edges: [
        {
          node: {
            id: "t1",
            title: "Task",
            status: "open" as const,
            assigneeId: null,
            metadata: { priority: 1 },
          },
        },
      ],
      pageInfo: { hasNextPage: false },
    }),
  });

const connectionPatch = taskConnection.patch.itemAspect({
  itemId: "t1",
  aspect: "status",
  value: "done",
});

void taskConnection.line({}).patch(connectionPatch);

const taskDetailResponse = signals.resource.response.detail<Task>();
const taskDetail = signals.api({}).url("/tasks/:taskId")
  .response(taskDetailResponse)
  .detail({
    load: ({ taskId }) => ({
      id: taskId,
      title: "Task",
      status: "open" as const,
      assigneeId: null,
      metadata: { priority: 1 },
    }),
  });

void taskDetail.line({ taskId: "t1" }).value();

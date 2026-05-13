import { createSignals } from "../../../index.js";

const signals = createSignals();

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

type TaskEnvelope = {
  tasks: readonly Task[];
  nextCursor: string | null;
};

type TaskConnectionEdge = { node: Task };
type TaskConnection = {
  edges: readonly TaskConnectionEdge[];
  pageInfo: { hasNextPage: boolean };
};
type TaskTupleEnvelope =
  | { kind: "primary"; primary: readonly Task[]; meta: { total: number } }
  | { kind: "secondary"; secondary: readonly Task[]; meta: { total: number } };

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
        metadata: {
          priority: 1,
          labels: ["first"],
          nested: { rank: 1 },
        },
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
  aspects: signals.resource.response.jsonPathAspects<Task>()<{
    priority: number;
    firstLabel: string;
    optionalNote: string | null;
  }>({
    priority: { field: "metadata", path: ["nested", "rank"] },
    firstLabel: { field: "metadata", path: ["labels", 0] },
    optionalNote: { field: "metadata", path: ["note"], presence: "optional" },
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
          metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
        },
      ],
      nextCursor: null,
    }),
  });

const envelopePatch = taskEnvelope.patch.itemAspect({
  itemId: "t1",
  aspect: "priority",
  value: 2,
});
const envelopeArrayPatch = taskEnvelope.patch.itemAspect({
  itemId: "t1",
  aspect: "firstLabel",
  value: "renamed",
});

void taskEnvelope.line({}).patch(envelopePatch);
void taskEnvelope.line({}).patch(envelopeArrayPatch);

const taskConnectionResponse = signals.resource.response.connection<TaskConnection>()({
  itemId: (item) => item.id,
  edges: (value) => value.edges,
  node: (edge: TaskConnectionEdge) => edge.node,
  edgeIndexForItem: (value, itemId) => {
    const edgeIndex = value.edges.findIndex((edge) => edge.node.id === itemId);
    return edgeIndex === -1 ? null : edgeIndex;
  },
  replaceNodes: (value, nextNodes) => ({
    ...value,
    edges: nextNodes.map((node) => ({ node })),
  }),
  replaceNode: (value, itemId, nextNode) => ({
    ...value,
    edges: value.edges.map((edge) =>
      edge.node.id === itemId ? { node: nextNode } : edge
    ),
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
            metadata: {
              priority: 1,
              labels: ["first"],
              nested: { rank: 1 },
            },
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

const taskTupleResponse = signals.resource.response.discriminated<TaskTupleEnvelope>()({
  itemId: (item: Task) => item.id,
  discriminator: (value) => value.kind,
  variants: {
    primary: {
      items: (value) => value.kind === "primary" ? value.primary : [],
      replaceItems: (value, nextItems) => ({
        ...value,
        kind: "primary" as const,
        primary: nextItems,
      }),
    },
    secondary: {
      items: (value) => value.kind === "secondary" ? value.secondary : [],
      replaceItems: (value, nextItems) => ({
        ...value,
        kind: "secondary" as const,
        secondary: nextItems,
      }),
    },
  },
});

void taskTupleResponse.lensProof.topology;

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

void taskDetail.line({ taskId: "t1" }).value();

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
        canonicalParams: (
          _params: { body: Task },
          _value: Task,
          canonicalIdentity: string,
        ) => ({ taskId: canonicalIdentity }),
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
        summary: {
          kind: "summary",
          summary: "total",
        },
      }, {
        family: taskDetail,
        params: ({ body }: { body: Task }) => ({ taskId: body.id }),
        canonicalParams: (
          _params: { body: Task },
          _value: Task,
          canonicalIdentity: string,
        ) => ({ taskId: canonicalIdentity }),
        fallback: "deliveryAwaited",
        selection: {
          kind: "visibleSelection",
        },
      }, {
        family: taskDetail,
        params: ({ body }: { body: Task }) => ({ taskId: body.id }),
        fallback: "refetchRequired",
        detailChild: {
          kind: "detailChild",
          region: "children",
        },
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

const taskSummaryResponse = signals.resource.response.summary<{ total: number }>();
const taskSummary = signals.api({}).url("/task-summary")
  .response(taskSummaryResponse)
  .detail({
    load: () => ({ total: 1 }),
  });

void taskSummary.line({}).value();

type TaskEntityStore = {
  entities: Record<string, Task>;
};

const taskEntityStoreResponse = signals.resource.response.entityStore<TaskEntityStore>()({
  itemId: (item) => item.id,
  entities: (value) => value.entities,
  replaceEntities: (value, entities) => ({ ...value, entities }),
  replaceEntity: (value, itemId, nextItem) => ({
    ...value,
    entities: { ...value.entities, [itemId]: nextItem },
  }),
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
  }),
});

const taskEntityStore = signals.api({}).url("/task-entity-store")
  .response(taskEntityStoreResponse)
  .list({
    load: () => ({
      entities: {
        t1: {
          id: "t1",
          title: "Task",
          status: "open" as const,
          assigneeId: null,
          metadata: {
            priority: 1,
            labels: ["first"],
            nested: { rank: 1 },
          },
        },
      },
    }),
  });

void taskEntityStore.line({}).patch(
  taskEntityStore.patch.itemAspect({
    itemId: "t1",
    aspect: "title",
    value: "Renamed",
  }),
);

type TaskMapEnvelope = {
  taskMap: Map<string, Task>;
};

const taskMapResponse = signals.resource.response.map<TaskMapEnvelope>()({
  itemId: (item: Task) => item.id,
  entries: (value) => value.taskMap,
  replaceEntries: (value, taskMap) => ({ ...value, taskMap: new Map(taskMap) }),
  replaceEntry: (value, itemId, nextItem) => ({
    ...value,
    taskMap: new Map(value.taskMap).set(itemId, nextItem),
  }),
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
  }),
});

void taskMapResponse.lensProof.topology;

type TaskGroupedEnvelope = {
  groups: Record<string, readonly Task[]>;
};
type TaskSparseEnvelope = {
  pages: Record<string, readonly Task[]>;
};
type TaskNamedEnvelope = {
  collections: Record<string, readonly Task[]>;
};
type TaskTreeNode = Task & { children: readonly TaskTreeNode[] };
type TaskTreeEnvelope = {
  roots: readonly TaskTreeNode[];
};

const taskGroupedResponse = signals.resource.response.grouped<TaskGroupedEnvelope>()({
  itemId: (item: Task) => item.id,
  groupId: (item: Task) => item.status,
  groupForItem: (_itemId: string) => "open" as const,
  groups: (value) => value.groups,
  replaceGroups: (value, groups) => ({ ...value, groups }),
  replaceGroupItem: (value, groupId, itemId, nextItem) => ({
    ...value,
    groups: {
      ...value.groups,
      [groupId]: value.groups[groupId].map((task) =>
        task.id === itemId ? nextItem : task
      ),
    },
  }),
});

void taskGroupedResponse.lensProof.topology;

const taskSparseResponse = signals.resource.response.sparse<TaskSparseEnvelope>()({
  itemId: (item: Task) => item.id,
  pageId: (item: Task) => item.status,
  pageForItem: (_itemId: string) => "open" as const,
  pages: (value) => value.pages,
  replacePages: (value, pages) => ({ ...value, pages }),
  replacePageItem: (value, pageId, itemId, nextItem) => ({
    ...value,
    pages: {
      ...value.pages,
      [pageId]: value.pages[pageId].map((task) =>
        task.id === itemId ? nextItem : task
      ),
    },
  }),
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
  }),
});

void taskSparseResponse.lensProof.topology;

const taskNamedResponse = signals.resource.response.named<TaskNamedEnvelope>()({
  itemId: (item: Task) => item.id,
  collectionId: (item: Task) => item.status,
  collectionForItem: (_itemId: string) => "open" as const,
  collections: (value) => value.collections,
  replaceCollections: (value, collections) => ({ ...value, collections }),
  replaceCollectionItem: (value, collectionId, itemId, nextItem) => ({
    ...value,
    collections: {
      ...value.collections,
      [collectionId]: value.collections[collectionId].map((task) =>
        task.id === itemId ? nextItem : task
      ),
    },
  }),
});

void taskNamedResponse.lensProof.topology;

const taskMultipleResponse = signals.resource.response.multiple<TaskNamedEnvelope>()({
  itemId: (item: Task) => item.id,
  collectionId: (item: Task) => item.status,
  collectionForItem: (_itemId: string) => "open" as const,
  collections: (value) => value.collections,
  replaceCollections: (value, collections) => ({ ...value, collections }),
  replaceCollectionItem: (value, collectionId, itemId, nextItem) => ({
    ...value,
    collections: {
      ...value.collections,
      [collectionId]: value.collections[collectionId].map((task) =>
        task.id === itemId ? nextItem : task
      ),
    },
  }),
});

void taskMultipleResponse.lensProof.topology;

const taskTreeResponse = signals.resource.response.tree<TaskTreeEnvelope>()({
  itemId: (item: TaskTreeNode) => item.id,
  roots: (value) => value.roots,
  children: (item) => item.children,
  replaceChildren: (item, children) => ({ ...item, children }),
  replaceRoots: (value, roots) => ({ ...value, roots }),
  nodeForItem: (itemId) => ["root", itemId],
  replaceNode: (value, _path, _itemId, _nextItem) => value,
});

void taskTreeResponse.lensProof.topology;

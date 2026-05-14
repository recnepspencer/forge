import { createSignals, resourceCollectionShape } from "../../../index.js";

const signals = createSignals();

type Task = {
  id: string;
  title: string;
  status: "open" | "done";
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
};
type TaskTupleEnvelope =
  | { kind: "primary"; primary: readonly Task[] }
  | { kind: "secondary"; secondary: readonly Task[] };
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

signals.resource.response.objectAspects<Task>()({
  title: "title",
  // @ts-expect-error object response aspects must name real item fields
  missing: "missing",
});

signals.resource.response.jsonPathAspects<Task>()({
  missingField: {
    // @ts-expect-error JSON path aspects must name real item fields
    field: "missing",
    path: ["nested", "rank"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  invalidPath: {
    field: "title",
    // @ts-expect-error JSON path arrays accept only string object keys and numeric array indexes
    path: [false],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  scalarRootPath: {
    field: "title",
    // @ts-expect-error JSON path declarations cannot traverse scalar root fields
    path: ["length"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  missingNestedField: {
    field: "metadata",
    // @ts-expect-error JSON path declarations must name real nested object fields
    path: ["missing"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  stringArrayIndex: {
    field: "metadata",
    // @ts-expect-error JSON path array crossings require numeric indexes
    path: ["labels", "0"],
  },
});

signals.resource.response.jsonPathAspects<Task>()({
  invalidPresence: {
    field: "metadata",
    path: ["priority"],
    // @ts-expect-error JSON path presence accepts only required or optional
    presence: "maybe",
  },
});

const taskResponse = signals.resource.response.array({
  itemId: (item: Task) => item.id,
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
    status: "status",
  }),
});

const tasks = signals.api({}).url("/tasks").response(taskResponse).list({
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

const taskEnvelopeResponse = signals.resource.response.objectItems<TaskEnvelope>()({
  field: "tasks",
  itemId: (item) => item.id,
  aspects: signals.resource.response.objectAspects<Task>()({
    title: "title",
  }),
});

const taskEnvelope = signals.api({}).url("/task-page")
  .response(taskEnvelopeResponse)
  .list({
    load: () => ({
      tasks: [{
        id: "t1",
        title: "Task",
        status: "open" as const,
        metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
      }],
      nextCursor: null,
    }),
  });

tasks.patch.itemAspect({
  itemId: "t1",
  // @ts-expect-error response contract patch helpers must reject unknown aspects
  aspect: "missing",
  value: "wrong",
});

tasks.patch.itemAspect({
  itemId: "t1",
  aspect: "status",
  // @ts-expect-error response contract aspects preserve field value types
  value: "blocked",
});

taskEnvelope.patch.itemAspect({
  itemId: "t1",
  // @ts-expect-error typed envelope response contracts reject unknown aspects
  aspect: "status",
  value: "done",
});

signals.resource.response.objectItems<TaskEnvelope>()({
  // @ts-expect-error objectItems response contracts only accept array-valued fields
  field: "nextCursor",
  itemId: (item) => item.id,
});

signals.resource.response.collection<TaskEnvelope, Task>({
  itemId: (item) => item.id,
  items: (value) => value.tasks,
  // @ts-expect-error generic response contracts require replacement to preserve response shape
  replaceItems: (_value, nextItems) => nextItems,
});

signals.resource.response.collection<TaskEnvelope>()({
  itemId: (item: Task) => item.id,
  items: (value) => value.tasks,
  // @ts-expect-error curried generic response contracts require replacement to preserve response shape
  replaceItems: (_value, nextItems) => nextItems,
});

signals.resource.response.map<TaskEnvelope>()({
  itemId: (item: Task) => item.id,
  // @ts-expect-error map response contracts require entries(value) to return a ReadonlyMap
  entries: (value) => value.tasks,
  replaceEntries: (value, _nextEntries) => value,
  replaceEntry: (value, _itemId, _nextItem) => value,
});

signals.resource.response.grouped<TaskEnvelope>()({
  itemId: (item: Task) => item.id,
  groupId: (item: Task) => item.status,
  groupForItem: (_itemId: string) => "open",
  // @ts-expect-error grouped response contracts require groups(value) to return an object record of item arrays
  groups: (value) => value.tasks,
  replaceGroups: (value, _nextGroups) => value,
  replaceGroupItem: (value, _groupId, _itemId, _nextItem) => value,
});

signals.resource.response.sparse<TaskSparseEnvelope>()({
  itemId: (item: Task) => item.id,
  pageId: (item: Task) => item.status,
  pageForItem: (_itemId: string) => "open",
  // @ts-expect-error sparse response contracts require pages(value) to return an object record of item arrays
  pages: (value) => value.pages.open,
  replacePages: (value, _nextPages) => value,
  replacePageItem: (value, _pageId, _itemId, _nextItem) => value,
});

signals.resource.response.named<TaskNamedEnvelope>()({
  itemId: (item: Task) => item.id,
  collectionId: (item: Task) => item.status,
  collectionForItem: (_itemId: string) => "open",
  // @ts-expect-error named response contracts require collections(value) to return an object record of item arrays
  collections: (value) => value.collections.open,
  replaceCollections: (value, _nextCollections) => value,
  replaceCollectionItem: (value, _collectionId, _itemId, _nextItem) => value,
});

signals.resource.response.multiple<TaskNamedEnvelope>()({
  itemId: (item: Task) => item.id,
  collectionId: (item: Task) => item.status,
  collectionForItem: (_itemId: string) => "open",
  // @ts-expect-error multiple collection response contracts require collections(value) to return an object record of item arrays
  collections: (value) => value.collections.open,
  replaceCollections: (value, _nextCollections) => value,
  replaceCollectionItem: (value, _collectionId, _itemId, _nextItem) => value,
});

signals.resource.response.tree<TaskTreeEnvelope>()({
  itemId: (item: TaskTreeNode) => item.id,
  // @ts-expect-error tree response contracts require roots(value) to return an array
  roots: (value) => value.roots[0],
  children: (item) => item.children,
  replaceRoots: (value, _roots) => value,
  nodeForItem: (itemId) => ["root", itemId],
  replaceNode: (value, _path, _itemId, _nextItem) => value,
});

signals.resource.response.tree<TaskTreeEnvelope>()({
  itemId: (item: TaskTreeNode) => item.id,
  roots: (value) => value.roots,
  // @ts-expect-error tree response contracts require children(node) to return an array
  children: (item) => item.children[0],
  replaceRoots: (value, _roots) => value,
  nodeForItem: (itemId) => ["root", itemId],
  replaceNode: (value, _path, _itemId, _nextItem) => value,
});

signals.resource.response.tree<TaskTreeEnvelope>()({
  itemId: (item: TaskTreeNode) => item.id,
  roots: (value) => value.roots,
  children: (item) => item.children,
  // @ts-expect-error tree response contracts require replaceRoots to preserve response shape
  replaceRoots: (_value, roots) => roots,
  nodeForItem: (itemId) => ["root", itemId],
  replaceNode: (value, _path, _itemId, _nextItem) => value,
});

signals.resource.response.tree<TaskTreeEnvelope>()({
  itemId: (item: TaskTreeNode) => item.id,
  roots: (value) => value.roots,
  children: (item) => item.children,
  replaceRoots: (value, _roots) => value,
  nodeForItem: (itemId) => ["root", itemId],
  // @ts-expect-error tree response contracts require replaceNode to preserve response shape
  replaceNode: (_value, _path, _itemId, nextItem) => nextItem,
});

signals.resource.response.named<TaskNamedEnvelope>()({
  itemId: (item: Task) => item.id,
  collectionId: (item: Task) => item.status,
  collectionForItem: (_itemId: string) => "open",
  collections: (value) => value.collections,
  // @ts-expect-error named response contracts require replaceCollections to preserve response shape
  replaceCollections: (_value, nextCollections) => nextCollections,
  replaceCollectionItem: (value, _collectionId, _itemId, _nextItem) => value,
});

signals.resource.response.sparse<TaskSparseEnvelope>()({
  itemId: (item: Task) => item.id,
  pageId: (item: Task) => item.status,
  pageForItem: (_itemId: string) => "open",
  pages: (value) => value.pages,
  // @ts-expect-error sparse response contracts require replacePages to preserve response shape
  replacePages: (_value, nextPages) => nextPages,
  replacePageItem: (value, _pageId, _itemId, _nextItem) => value,
});

signals.resource.response.connection<TaskConnection>()({
  itemId: (item: Task) => item.id,
  edges: (value) => value.edges,
  node: (edge: TaskConnectionEdge) => edge.node,
  edgeIndexForItem: (value, itemId) => {
    const edgeIndex = value.edges.findIndex((edge) => edge.node.id === itemId);
    return edgeIndex === -1 ? null : edgeIndex;
  },
  // @ts-expect-error connection response contracts require replaceNodes to preserve response shape
  replaceNodes: (_value, nextNodes) => nextNodes,
  replaceNode: (value, _itemId, _nextNode) => value,
});

signals.resource.response.discriminated<TaskTupleEnvelope>()({
  itemId: (item: Task) => item.id,
  discriminator: (value) => value.kind,
  variants: {
    primary: {
      items: (value) => value.kind === "primary" ? value.primary : [],
      // @ts-expect-error discriminated response variants must preserve envelope shape
      replaceItems: (_value, nextItems) => nextItems,
    },
  },
});

signals.api({}).url("/tasks").response(taskResponse).list({
  // @ts-expect-error response(...) owns itemIdentity(...) in the route lane
  itemIdentity: (item: Task) => item.id,
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

signals.api({}).url("/tasks").response(taskResponse).list({
  // @ts-expect-error response(...) owns reconcile(...) in the route lane
  reconcile: resourceCollectionShape({
    items: (value: readonly Task[]) => value,
    replaceItems: (_value: readonly Task[], nextItems: readonly Task[]) => [
      ...nextItems,
    ],
  }),
  load: () => [{
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }],
});

// @ts-expect-error response(...) owns aspect definitions through the contract
signals.api({}).url("/tasks").response(taskResponse).aspect(
  "title",
  (item: Task) => item.title,
  (item: Task, title: string) => ({ ...item, title }),
);

// @ts-expect-error response(...) does not declare summary contracts in this lane
signals.api({}).url("/tasks").response(taskResponse).summary(
  "total",
  (value: readonly Task[]) => value.length,
  (value: readonly Task[]) => value,
);

// @ts-expect-error response(...) does not declare page-window summary contracts in this lane
signals.api({}).url("/tasks").response(taskResponse).pageWindowSummary(
  "visible",
  (value: readonly Task[]) => value.length,
  (value: readonly Task[]) => value,
);

// @ts-expect-error response(...) is a collection lane and must not expose detail finalizers
signals.api({}).url("/tasks").response(taskResponse).detail({
  load: () => ({
    id: "t1",
    title: "Task",
    status: "open" as const,
    metadata: { priority: 1, labels: ["first"], nested: { rank: 1 } },
  }),
});

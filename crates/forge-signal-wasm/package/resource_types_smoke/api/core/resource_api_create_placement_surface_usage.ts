import { createSignals } from "../../../index.js";

const signals = createSignals();
const api = signals.api({ baseUrl: "/api" });

const directTaskList = api.url("/workspaces/:workspaceId/direct-tasks")
  .items((item: { id: string; title: string }) => item.id)
  .list({
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
  });

const directTaskCatalog = api.url("/workspaces/:workspaceId/direct-task-catalog")
  .items((item: { id: string; title: string }) => item.id)
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

const directTaskPages = api.url("/workspaces/:workspaceId/direct-task-pages")
  .items((item: { id: string; title: string }) => item.id)
  .paged({
    accumulatePage: (
      existing: Array<{ id: string; title: string }>,
      next: Array<{ id: string; title: string }>,
    ) => [...existing, ...next],
    load: ({ workspaceId }) => [{ id: String(workspaceId), title: "Task" }],
  });

const entityTaskStore = api.url("/workspaces/:workspaceId/entity-task-store")
  .response(signals.resource.response.entityStore()({
    itemId: (item: { id: string; title: string }) => item.id,
    entities: (
      value: { entities: Record<string, { id: string; title: string }> },
    ) => value.entities,
    replaceEntities: (
      value: { entities: Record<string, { id: string; title: string }> },
      nextEntities: Readonly<Record<string, { id: string; title: string }>>,
    ) => ({ ...value, entities: nextEntities }),
    replaceEntity: (
      value: { entities: Record<string, { id: string; title: string }> },
      itemId: string,
      nextItem: { id: string; title: string },
    ) => ({
      ...value,
      entities: {
        ...value.entities,
        [itemId]: nextItem,
      },
    }),
  }))
  .list({
    load: ({ workspaceId }) => ({
      entities: {
        [String(workspaceId)]: { id: String(workspaceId), title: "Task" },
      },
    }),
  });

const connectedTaskList = api.url("/workspaces/:workspaceId/connected-tasks")
  .response(signals.resource.response.connection()({
    itemId: (item: { id: string; title: string }) => item.id,
    edges: (
      value: {
        edges: Array<{ cursor: string; node: { id: string; title: string } }>;
      },
    ) => value.edges,
    node: (edge: { cursor: string; node: { id: string; title: string } }) =>
      edge.node,
    edgeIndexForItem: (
      value: {
        edges: Array<{ cursor: string; node: { id: string; title: string } }>;
      },
      itemId: string,
    ) => {
      const edgeIndex = value.edges.findIndex((edge) => edge.node.id === itemId);
      return edgeIndex === -1 ? null : edgeIndex;
    },
    replaceNodes: (
      value: {
        edges: Array<{ cursor: string; node: { id: string; title: string } }>;
      },
      nextNodes: ReadonlyArray<{ id: string; title: string }>,
    ) => ({
      ...value,
      edges: nextNodes.map((node, index) => ({
        cursor: `cursor:${index}`,
        node,
      })),
    }),
    replaceNode: (
      value: {
        edges: Array<{ cursor: string; node: { id: string; title: string } }>;
      },
      itemId: string,
      nextItem: { id: string; title: string },
    ) => ({
      ...value,
      edges: value.edges.map((edge) => ({
        ...edge,
        node: edge.node.id === itemId ? nextItem : edge.node,
      })),
    }),
  }))
  .list({
    load: ({ workspaceId }) => ({
      edges: [{
        cursor: "cursor:0",
        node: { id: String(workspaceId), title: "Task" },
      }],
    }),
  });
const groupedTaskList = api.url("/workspaces/:workspaceId/grouped-tasks")
  .response(signals.resource.response.grouped()({
    itemId: (item: { id: string; group: string; title: string }) => item.id,
    groupId: (item: { id: string; group: string; title: string }) => item.group,
    groupForItem: () => "todo",
    groups: (
      value: Record<string, Array<{ id: string; group: string; title: string }>>,
    ) => value,
    replaceGroups: (
      _value: Record<string, Array<{ id: string; group: string; title: string }>>,
      nextGroups: Readonly<Record<string, readonly { id: string; group: string; title: string }[]>>,
    ) => nextGroups,
    replaceGroupItem: (
      value: Record<string, Array<{ id: string; group: string; title: string }>>,
      groupId: string,
      itemId: string,
      nextItem: { id: string; group: string; title: string },
    ) => Object.fromEntries(
      Object.entries(value).map(([key, items]) => [
        key,
        key === groupId
          ? items.map((item) => item.id === itemId ? nextItem : item)
          : items,
      ]),
    ),
  }))
  .list({
    load: ({ workspaceId }) => ({
      todo: [{ id: String(workspaceId), group: "todo", title: "Task" }],
      done: [],
    }),
  });
const sparseTaskPages = api.url("/workspaces/:workspaceId/sparse-task-pages")
  .response(signals.resource.response.sparse()({
    itemId: (item: { id: string; page: string; title: string }) => item.id,
    pageId: (item: { id: string; page: string; title: string }) => item.page,
    pageForItem: () => "page-1",
    pages: (
      value: Record<string, Array<{ id: string; page: string; title: string }>>,
    ) => value,
    replacePages: (
      _value: Record<string, Array<{ id: string; page: string; title: string }>>,
      nextPages: Readonly<Record<string, readonly { id: string; page: string; title: string }[]>>,
    ) => nextPages,
    replacePageItem: (
      value: Record<string, Array<{ id: string; page: string; title: string }>>,
      pageId: string,
      itemId: string,
      nextItem: { id: string; page: string; title: string },
    ) => Object.fromEntries(
      Object.entries(value).map(([key, items]) => [
        key,
        key === pageId
          ? items.map((item) => item.id === itemId ? nextItem : item)
          : items,
      ]),
    ),
  }))
  .list({
    load: ({ workspaceId }) => ({
      "page-1": [{ id: String(workspaceId), page: "page-1", title: "Task" }],
      "page-2": [],
    }),
  });
const treeTaskList = api.url("/workspaces/:workspaceId/tree-tasks")
  .response(signals.resource.response.tree()({
    itemId: (item: { id: string; title: string; children: readonly any[] }) => item.id,
    roots: (
      value: { roots: Array<{ id: string; title: string; children: readonly any[] }> },
    ) => value.roots,
    children: (item: { children: readonly { id: string; title: string; children: readonly any[] }[] }) => item.children,
    replaceChildren: (
      item: { id: string; title: string; children: readonly { id: string; title: string; children: readonly any[] }[] },
      nextChildren: readonly { id: string; title: string; children: readonly any[] }[],
    ) => ({ ...item, children: nextChildren }),
    replaceRoots: (
      value: { roots: Array<{ id: string; title: string; children: readonly any[] }> },
      nextRoots: readonly { id: string; title: string; children: readonly any[] }[],
    ) => ({ ...value, roots: [...nextRoots] }),
    nodeForItem: (itemId: string) => itemId === "root" ? ["root"] : ["root", itemId],
    replaceNode: (
      value: { roots: Array<{ id: string; title: string; children: readonly any[] }> },
      _path: readonly string[],
      itemId: string,
      nextItem: { id: string; title: string; children: readonly any[] },
    ) => ({
      ...value,
      roots: value.roots.map((root) => ({
        ...root,
        children: root.children.map((child) => child.id === itemId ? nextItem : child),
      })),
    }),
  }))
  .list({
    load: ({ workspaceId }) => ({
      roots: [{
        id: "root",
        title: "Root",
        children: [{ id: String(workspaceId), title: "Task", children: [] }],
      }],
    }),
  });

const directTaskDetail = api.url("/workspaces/:workspaceId/direct-task-detail/:taskId").detail({
  load: ({ taskId }) => ({ id: String(taskId), title: "Task" }),
});
const draftTaskDetail = api.url("/workspaces/:workspaceId/draft-task-detail/:taskId").detail({
  load: ({ taskId }) => ({ id: String(taskId), title: "Draft Task" }),
});

const createWorkflow = api.url("/workflows")
  .response(signals.resource.response.detail<{ id: string; title: string; total: number }>()({
    total: "total",
  }))
  .create({
    reconciles: [
      {
        family: entityTaskStore,
        params: () => ({ workspaceId: "demo" }),
        fallback: "placementUnavailable",
        collection: { kind: "insert", placement: "append" },
      },
      {
        family: connectedTaskList,
        params: () => ({ workspaceId: "demo" }),
        fallback: "placementUnavailable",
        collection: { kind: "insert", placement: "prepend" },
      },
      {
        family: groupedTaskList,
        params: () => ({ workspaceId: "demo" }),
        fallback: "placementUnavailable",
        collection: { kind: "insert", placement: "append" },
      },
      {
        family: sparseTaskPages,
        params: () => ({ workspaceId: "demo" }),
        fallback: "placementUnavailable",
        collection: { kind: "insert", placement: "append" },
      },
      {
        family: treeTaskList,
        params: () => ({ workspaceId: "demo" }),
        fallback: "placementUnavailable",
        collection: { kind: "insert", placement: "append" },
      },
      {
        family: directTaskCatalog,
        params: () => ({ workspaceId: "demo" }),
        fallback: "refetchRequired",
        summary: { kind: "summary", summary: "count" },
      },
      {
        family: directTaskDetail,
        params: () => ({ workspaceId: "demo", taskId: "demo-3" }),
        fallback: "refetchRequired",
        detail: { kind: "replace" },
      },
    ],
    identity: {
      submitted: ({ body }: { body: { id: string } }) => body.id,
      response: (value: { id: string }) => value.id,
      canonical: (
        value: { id: string },
        responseIdentity: string | null,
      ) => responseIdentity ?? value.id,
      targets: [{
        family: draftTaskDetail,
        params: () => ({ workspaceId: "demo", taskId: "demo-3" }),
        canonicalParams: (
          _params,
          _value,
          canonicalIdentity,
        ) => ({ workspaceId: "demo", taskId: canonicalIdentity }),
        fallback: "identityMigrationUnavailable",
      }],
    },
    load: ({ body }: { body: { id: string; title: string; total: number } }) => body,
  });

const createWorkflowPlan = createWorkflow.line({
  body: { id: "demo-3", title: "Created", total: 1 },
}).mutationResponse();
const directTaskInsertPatch = directTaskList.patch.insert({
  itemId: "demo-2",
  placement: "append",
  nextItem: { id: "demo-2", title: "Inserted" },
});
const directTaskInsertDelivery = directTaskPages.delivery.insert({
  packetId: "pkt-direct-task-insert",
  basisId: null,
  nextBasisId: "basis-1",
  itemId: "demo-2",
  placement: "prepend",
  nextItem: { id: "demo-2", title: "Inserted Delivered" },
});

const directTaskListLine = directTaskList.line({ workspaceId: "demo" });
const directTaskPagesLine = directTaskPages.line({ workspaceId: "demo" });

void createWorkflowPlan?.targets[0]?.reconciliation?.kind;
const createWorkflowArtifact = createWorkflowPlan?.executionArtifacts[0];
if (createWorkflowArtifact?.kind === "exactCollectionInsert") {
  void createWorkflowArtifact.placement;
}
void createWorkflowPlan?.identityMigration?.targets[0]?.execution.kind;
if (createWorkflowPlan?.identityMigration?.targets[0]?.execution.kind === "exactDetailChildRegion") {
  void createWorkflowPlan.identityMigration.targets[0].execution.region;
  void createWorkflowPlan.identityMigration.targets[0].execution.effectProof?.effectId;
}
void directTaskListLine.patch(directTaskInsertPatch);
void directTaskPagesLine.deliver(directTaskInsertDelivery);

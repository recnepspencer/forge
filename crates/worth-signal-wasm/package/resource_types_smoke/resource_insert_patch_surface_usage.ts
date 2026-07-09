import {
  createSignals,
  resourceCollectionShape,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
} from "../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const collection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape({
    items: (value: { items: Array<{ id: string; title: string }> }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string; title: string }> },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  }),
  load: () => ({
    items: [{ id: "workspace:demo", title: "Task" }],
  }),
});

const collectionLine = collection.line({ workspaceId: "demo" });
const entityStore = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.entityStore()({
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
  }),
  load: () => ({
    entities: {
      "workspace:demo": { id: "workspace:demo", title: "Task" },
    },
  }),
});
const entityStoreLine = entityStore.line({ workspaceId: "demo" });
const connectionCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.connection()({
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
  }),
  load: () => ({
    edges: [{
      cursor: "cursor:0",
      node: { id: "workspace:demo", title: "Task" },
    }],
  }),
});
const connectionCollectionLine = connectionCollection.line({ workspaceId: "demo" });
const discriminatedCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.discriminated()({
    itemId: (item: { id: string; title: string }) => item.id,
    discriminator: (
      value: {
        kind: "primary" | "secondary";
        primary: Array<{ id: string; title: string }>;
        secondary: Array<{ id: string; title: string }>;
      },
    ) => value.kind,
    variants: {
      primary: {
        items: (
          value: {
            kind: "primary" | "secondary";
            primary: Array<{ id: string; title: string }>;
            secondary: Array<{ id: string; title: string }>;
          },
        ) => value.primary,
        replaceItems: (
          value: {
            kind: "primary" | "secondary";
            primary: Array<{ id: string; title: string }>;
            secondary: Array<{ id: string; title: string }>;
          },
          nextItems: ReadonlyArray<{ id: string; title: string }>,
        ) => ({ ...value, primary: [...nextItems] }),
      },
      secondary: {
        items: (
          value: {
            kind: "primary" | "secondary";
            primary: Array<{ id: string; title: string }>;
            secondary: Array<{ id: string; title: string }>;
          },
        ) => value.secondary,
        replaceItems: (
          value: {
            kind: "primary" | "secondary";
            primary: Array<{ id: string; title: string }>;
            secondary: Array<{ id: string; title: string }>;
          },
          nextItems: ReadonlyArray<{ id: string; title: string }>,
        ) => ({ ...value, secondary: [...nextItems] }),
      },
    },
  }),
  load: () => ({
    kind: "primary" as const,
    primary: [{ id: "workspace:demo", title: "Task" }],
    secondary: [],
  }),
});
const discriminatedCollectionLine = discriminatedCollection.line({ workspaceId: "demo" });
const groupedCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.grouped()({
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
  }),
  load: () => ({
    todo: [{ id: "workspace:demo", group: "todo", title: "Task" }],
    done: [],
  }),
});
const groupedCollectionLine = groupedCollection.line({ workspaceId: "demo" });
const namedCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.named()({
    itemId: (item: { id: string; collection: string; title: string }) => item.id,
    collectionId: (item: { id: string; collection: string; title: string }) =>
      item.collection,
    collectionForItem: () => "backlog",
    collections: (
      value: Record<string, Array<{ id: string; collection: string; title: string }>>,
    ) => value,
    replaceCollections: (
      _value: Record<string, Array<{ id: string; collection: string; title: string }>>,
      nextCollections: Readonly<Record<string, readonly { id: string; collection: string; title: string }[]>>,
    ) => nextCollections,
    replaceCollectionItem: (
      value: Record<string, Array<{ id: string; collection: string; title: string }>>,
      collectionId: string,
      itemId: string,
      nextItem: { id: string; collection: string; title: string },
    ) => Object.fromEntries(
      Object.entries(value).map(([key, items]) => [
        key,
        key === collectionId
          ? items.map((item) => item.id === itemId ? nextItem : item)
          : items,
      ]),
    ),
  }),
  load: () => ({
    backlog: [{ id: "workspace:demo", collection: "backlog", title: "Task" }],
    active: [],
  }),
});
const namedCollectionLine = namedCollection.line({ workspaceId: "demo" });
const sparseCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.sparse()({
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
  }),
  load: () => ({
    "page-1": [{ id: "workspace:demo", page: "page-1", title: "Task" }],
    "page-2": [],
  }),
});
const sparseCollectionLine = sparseCollection.line({ workspaceId: "demo" });
const treeCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  response: signals.resource.response.tree()({
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
  }),
  load: () => ({
    roots: [{
      id: "root",
      title: "Root",
      children: [{ id: "workspace:demo", title: "Task", children: [] }],
    }],
  }),
});
const treeCollectionLine = treeCollection.line({ workspaceId: "demo" });
const collectionInsertItem = collectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: { id: "workspace:demo:2", title: "Inserted" },
  }),
);
const entityStoreInsertItem = entityStoreLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: { id: "workspace:demo:2", title: "Inserted Store" },
  }),
);
const connectionInsertItem = connectionCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "prepend",
    nextItem: { id: "workspace:demo:2", title: "Inserted Connection" },
  }),
);
const discriminatedInsertItem = discriminatedCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: { id: "workspace:demo:2", title: "Inserted Tuple" },
  }),
);
const groupedInsertItem = groupedCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: {
      id: "workspace:demo:2",
      group: "todo",
      title: "Inserted Grouped",
    },
  }),
);
const namedInsertItem = namedCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:0",
    placement: "prepend",
    nextItem: {
      id: "workspace:demo:0",
      collection: "backlog",
      title: "Inserted Named",
    },
  }),
);
const sparseInsertItem = sparseCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: {
      id: "workspace:demo:2",
      page: "page-1",
      title: "Inserted Sparse",
    },
  }),
);
const treeInsertItem = treeCollectionLine.patch(
  resourcePatch.insert({
    itemId: "workspace:demo:2",
    placement: "append",
    nextItem: {
      id: "workspace:demo:2",
      title: "Inserted Tree",
      children: [],
    },
  }),
);

void collectionInsertItem;
void entityStoreInsertItem;
void connectionInsertItem;
void discriminatedInsertItem;
void groupedInsertItem;
void namedInsertItem;
void sparseInsertItem;
void treeInsertItem;

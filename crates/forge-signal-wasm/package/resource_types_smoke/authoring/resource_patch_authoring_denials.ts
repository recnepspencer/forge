import {
  createSignals,
  resourceBinaryDescriptor,
  resourceCollectionShape,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourceValueSummaries,
} from "../../index.js";

const signals = createSignals();

signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string }) => item.id,
  // @ts-expect-error reconcile must be created with resourceCollectionShape(...)
  reconcile: {
    items: (value: { items: Array<{ id: string }> }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string }> },
      nextItems: readonly { id: string }[],
    ) => ({ ...value, items: [...nextItems] }),
  },
  load: ({ workspaceId }) => ({ items: [{ id: workspaceId }] }),
});

// @ts-expect-error detail lines must not expose patch(...)
detailLine.patch(resourcePatch.replace({ id: "p1" }));

// @ts-expect-error detail lines must not expose reconciliation()
detailLine.reconciliation();

const replaceOnlyCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ workspaceId }) => [{ id: workspaceId }],
});

replaceOnlyCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error collections without reconcile only admit broad replace patch(...)
  resourcePatch.item({
    itemId: "demo",
    nextItem: { id: "demo" },
  }),
);

const typedCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; count: number }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string; count: number }> },
    { id: string; count: number },
    {
      count: {
        read(item: { id: string; count: number }): number;
        write(item: { id: string; count: number }, value: number): {
          id: string;
          count: number;
        };
      };
    }
  >({
    items: (value: { items: Array<{ id: string; count: number }> }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string; count: number }> },
      nextItems: readonly { id: string; count: number }[],
    ) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      count: {
        read: (item: { id: string; count: number }) => item.count,
        write: (item: { id: string; count: number }, value: number) => ({
          ...item,
          count: value,
        }),
      },
    }),
  }),
  load: ({ workspaceId }) => ({ items: [{ id: workspaceId, count: 1 }] }),
});

const maybeReconcile:
  | ReturnType<
      typeof resourceCollectionShape<
        { items: Array<{ id: string; count: number }> },
        { id: string; count: number }
      >
    >
  | undefined = Math.random() > 2 ? undefined : undefined;

const maybeReconciledCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; count: number }) => item.id,
  reconcile: maybeReconcile,
  load: ({ workspaceId }) => ({ items: [{ id: workspaceId, count: 1 }] }),
});

maybeReconciledCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error narrow patching must stay denied when reconcile is not definitely present
  resourcePatch.item({
    itemId: "demo",
    nextItem: { id: "demo", count: 2 },
  }),
);

maybeReconciledCollection.line({ workspaceId: "demo" }).deliver(
  resourceDelivery.patch({
    packetId: "pkt-maybe",
    // @ts-expect-error delivered narrow patching must stay denied when reconcile is not definitely present
    patch: resourcePatch.item({
      itemId: "demo",
      nextItem: { id: "demo", count: 2 },
    }),
  }),
);

const summaryTypedCollection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string }>; total: number },
    { id: string },
    {},
    {
      total: {
        read(value: { items: Array<{ id: string }>; total: number }): number;
        write(
          value: { items: Array<{ id: string }>; total: number },
          total: number,
        ): { items: Array<{ id: string }>; total: number };
      };
    }
  >({
    items: (value: { items: Array<{ id: string }>; total: number }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string }>; total: number },
      nextItems: readonly { id: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    summaries: resourceValueSummaries({
      total: {
        read: (value: { items: Array<{ id: string }>; total: number }) =>
          value.total,
        write: (
          value: { items: Array<{ id: string }>; total: number },
          total: number,
        ) => ({ ...value, total }),
      },
    }),
  }),
  load: ({ workspaceId }) => ({ items: [{ id: workspaceId }], total: 1 }),
});

summaryTypedCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error undeclared summary names must not type-check
  resourcePatch.summary({
    summary: "count",
    value: 1,
  }),
);

summaryTypedCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error declared summary values must match the declared summary type
  resourcePatch.summary({
    summary: "total",
    value: "wrong",
  }),
);

const pagedPlainSummary = signals.resource.paged({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string }>; cursor: string | null; total: number },
    { id: string },
    {},
    {
      total: {
        read(value: { items: Array<{ id: string }>; cursor: string | null; total: number }): number;
        write(
          value: { items: Array<{ id: string }>; cursor: string | null; total: number },
          total: number,
        ): { items: Array<{ id: string }>; cursor: string | null; total: number };
      };
    }
  >({
    items: (value: { items: Array<{ id: string }>; cursor: string | null; total: number }) =>
      value.items,
    replaceItems: (
      value: { items: Array<{ id: string }>; cursor: string | null; total: number },
      nextItems: readonly { id: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    summaries: resourceValueSummaries({
      total: {
        read: (value: {
          items: Array<{ id: string }>;
          cursor: string | null;
          total: number;
        }) => value.total,
        write: (
          value: {
            items: Array<{ id: string }>;
            cursor: string | null;
            total: number;
          },
          total: number,
        ) => ({ ...value, total }),
      },
    }),
  }),
  accumulatePage: (
    existing: { items: Array<{ id: string }>; cursor: string | null; total: number },
    next: { items: Array<{ id: string }>; cursor: string | null; total: number },
  ) => ({
    items: [...existing.items, ...next.items],
    cursor: next.cursor,
    total: next.total,
  }),
  load: ({ workspaceId }) => ({
    items: [{ id: workspaceId }],
    cursor: null,
    total: 1,
  }),
});

pagedPlainSummary.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error paged summary patching requires resourceValueSummaries.pageWindow(...)
  resourcePatch.summary({
    summary: "total",
    value: 2,
  }),
);

resourceBinaryDescriptor.file({
  id: "bad-download",
  download: resourceDownload.incompatible({
    // @ts-expect-error incompatible download reason must stay within the declared vocabulary
    reason: "expired",
    detail: "wrong",
  }),
});

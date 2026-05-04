import {
  createSignals,
  resourceCollectionShape,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceUploadTransport,
  resourceUploadResult,
  resourceValueSummaries,
} from "../index.js";

const signals = createSignals();

const detail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }) => ({ id: productId }),
});

const detailLine = detail.line({ productId: "p1" });

signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  // @ts-expect-error detail families must not admit collection-only identity contracts
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ productId }) => ({ id: productId }),
});

// @ts-expect-error collection families require itemIdentity
signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  load: ({ workspaceId }) => [{ id: workspaceId }],
});

// @ts-expect-error paged families require accumulatePage
signals.resource.paged({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string }) => item.id,
  load: ({ workspaceId }) => [{ id: workspaceId }],
});

signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  // @ts-expect-error resource policy must be created with resourcePolicyProfiles.*()
  policy: { name: "stable" },
  load: ({ productId }) => ({ id: productId }),
});

signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  // @ts-expect-error auth must be created with resourceAuth.*()
  auth: { kind: "authenticated" },
  load: ({ productId }) => ({ id: productId }),
});

signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  // @ts-expect-error requestContext must be created with resourceRequestContext(...)
  requestContext: { headers: { "x-workspace-id": "demo" } },
  load: ({ productId }) => ({ id: productId }),
});

signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  // @ts-expect-error continuation must be created with resourceContinuation.*()
  continuation: { kind: "redirect", returnTo: "/done" },
  load: ({ productId }) => ({ id: productId }),
});

signals.resource.detail({
  params: resourceParams<{ reportId: string }>(),
  normalizeParams: ({ reportId }) =>
    resourceParamIdentity({ reportId }, reportId),
  // @ts-expect-error processingJob must be created with resourceProcessingJob.*()
  processingJob: { kind: "poll" },
  load: ({ reportId }) => ({ id: reportId }),
});

signals.resource.detail({
  params: resourceParams<{ receiptId: string }>(),
  normalizeParams: ({ receiptId }) =>
    resourceParamIdentity({ receiptId }, receiptId),
  // @ts-expect-error uploadTransport must be created with resourceUploadTransport.*()
  uploadTransport: { kind: "signed", method: "PUT", finalizeRequired: true },
  load: ({ receiptId }) => ({ id: receiptId }),
});

const maybeProcessingJob:
  | ReturnType<typeof resourceProcessingJob.poll>
  | undefined = Math.random() > 2 ? undefined : undefined;

signals.resource.detail({
  params: resourceParams<{ reportId: string }>(),
  normalizeParams: ({ reportId }) =>
    resourceParamIdentity({ reportId }, reportId),
  processingJob: maybeProcessingJob,
  load: ({ reportId }) =>
    // @ts-expect-error maybe-present processingJob must not admit processing-result returns
    resourceProcessingResult.accepted({
      jobId: `job:${reportId}`,
    }),
});

const maybeUploadTransport:
  | ReturnType<typeof resourceUploadTransport.signed>
  | undefined = Math.random() > 2 ? undefined : undefined;

signals.resource.detail({
  params: resourceParams<{ receiptId: string }>(),
  normalizeParams: ({ receiptId }) =>
    resourceParamIdentity({ receiptId }, receiptId),
  uploadTransport: maybeUploadTransport,
  load: ({ receiptId }) =>
    // @ts-expect-error maybe-present uploadTransport must not admit upload-result returns
    resourceUploadResult.uploaded({
      uploadId: `upload:${receiptId}`,
      finalizeRequired: true,
      awaitingProcessing: false,
    }),
});

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

typedCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error undeclared aspect names must not type-check
  resourcePatch.itemAspect({
    itemId: "demo",
    aspect: "title",
    value: "wrong",
  }),
);

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

typedCollection.line({ workspaceId: "demo" }).patch(
  // @ts-expect-error declared aspect values must match the declared aspect type
  resourcePatch.itemAspect({
    itemId: "demo",
    aspect: "count",
    value: "wrong",
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

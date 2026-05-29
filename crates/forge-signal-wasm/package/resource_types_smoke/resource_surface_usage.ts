import {
  createSignals,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceAuth,
  resourceCollectionShape,
  resourceContinuation,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourcePolicyProfiles,
  resourceRequestContext,
  resourceValueSummaries,
} from "../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const detail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  policy: resourcePolicyProfiles.immediatelyStale(),
  auth: resourceAuth.authenticated(),
  requestContext: resourceRequestContext({
    headers: { "x-workspace-id": "demo" },
    correlationId: "trace-7",
    branchId: 42,
    basisId: "basis-1",
  }),
  continuation: resourceContinuation.callback({
    callbackId: "invoice-complete",
    returnTo: "/invoices/1",
  }),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }, request) =>
    resourceBinaryValue({
      value: {
        id: productId,
        label: `${request.auth.kind}:${request.continuation.kind}:product:${productId}`,
      },
      descriptors: [
        resourceBinaryDescriptor.file({
          id: "product-sheet",
          fileName: `${productId}.pdf`,
          mediaType: "application/pdf",
          byteLength: 1024,
          download: resourceDownload.ready({
            url: `https://downloads.example/${productId}.pdf`,
            method: "GET",
          }),
        }),
        resourceBinaryDescriptor.export({
          id: "product-export",
          fileName: `${productId}.zip`,
          mediaType: "application/zip",
          byteLength: 4096,
          download: resourceDownload.incompatible({
            reason: "transportBoundary",
            detail: "host session handoff required",
          }),
        }),
      ],
    }),
});

const externalDetail = signals.resource.compatibility.detail({
  version: "forge-resource-external-v1",
  family: "detail",
  definitionId: "external-product-detail",
  requestContract: "native-v1",
  reconciliationContract: "none",
  declaration: {
    params: resourceParams<{ productId: string }>(),
    normalizeParams: ({ productId }) =>
      resourceParamIdentity({ productId }, productId),
    load: ({ productId }) => ({ id: productId }),
  },
});
const externalCompatibilityDelivery = signals.resource.compatibility.delivery;

const detailLine = detail.line({ productId: "p1" });
const externalDetailLine = externalDetail.line({ productId: "p2" });
const detailValue = detailLine.value();
const detailSignal = detailLine.signal();
const detailDescriptor = detailLine.descriptor();
const detailHistory = detailLine.history();
const detailVerificationPackage = detailHistory.verificationPackage();
const detailBranch = detailHistory.branch;
const detailBasisHistory = detailHistory.basis;
const detailHistoryAvailability = detailHistory.availability;
const detailReplayExact = detailHistory.replayExact();
const detailRestoreExact = detailHistory.restoreExact();
const detailRequest = detailLine.request();
const detailDownload = detailLine.download();
const detailDiagnostics = detailLine.diagnostics();
const detailDiagnosticsSummary = detailLine.diagnosticsSummary();
const detailInvalidate = detailLine.invalidate();
const detailRefresh = detailLine.refresh();
const detailRevalidate = detailLine.revalidate();
const detailStatus = detailLine.status();
const detailFreshness = detailLine.freshness();
const detailAwaitedSettlement = await detailLine.awaitSettlement();
const detailLabel = detailLine.view((product) => product?.label ?? null);
const detailGraph = signals.graph("detailResource", {
  outputs: {
    product: detailSignal,
  },
});
detailLine.free();
const detailFamilyInvalidate = detail.invalidate({ productId: "p1" });
const detailFamilyInvalidateAll = detail.invalidateAll();

const collection = signals.resource.collection({
  params: resourceParams<{ workspaceId: string }>(),
  auth: ({ workspaceId }) =>
    workspaceId === "demo"
      ? resourceAuth.workspace()
      : resourceAuth.anonymous(),
  requestContext: ({ workspaceId }) =>
    resourceRequestContext({
      headers: { "x-workspace-id": workspaceId },
    }),
  continuation: ({ workspaceId }) =>
    workspaceId === "demo"
      ? resourceContinuation.redirect({ returnTo: "/workspace/demo" })
      : resourceContinuation.webhook({
          correlationKey: `workspace:${workspaceId}`,
        }),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string; title: string }>; total: number },
    { id: string; title: string },
    {
      title: {
        read(item: { id: string; title: string }): string;
        write(item: { id: string; title: string }, value: string): {
          id: string;
          title: string;
        };
      };
    },
    {
      total: {
        read(value: { items: Array<{ id: string; title: string }>; total: number }): number;
        write(
          value: { items: Array<{ id: string; title: string }>; total: number },
          total: number,
        ): { items: Array<{ id: string; title: string }>; total: number };
      };
    }
  >({
    items: (value: {
      items: Array<{ id: string; title: string }>;
      total: number;
    }) => value.items,
    replaceItems: (
      value: { items: Array<{ id: string; title: string }>; total: number },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    aspects: resourceItemAspects({
      title: {
        read: (item: { id: string; title: string }) => item.title,
        write: (item: { id: string; title: string }, value: string) => ({
          ...item,
          title: String(value),
        }),
      },
    }),
    summaries: resourceValueSummaries({
      total: {
        read: (value: {
          items: Array<{ id: string; title: string }>;
          total: number;
        }) => value.total,
        write: (
          value: { items: Array<{ id: string; title: string }>; total: number },
          total: number,
        ) => ({ ...value, total }),
      },
    }),
  }),
  load: ({ workspaceId }, request) => ({
    items: [
      {
        id: `workspace:${workspaceId}`,
        title: `${request.auth.kind}:${workspaceId}`,
      },
    ],
    total: 1,
  }),
});

const collectionLine = collection.line({ workspaceId: "demo" });
const collectionItems = collectionLine.value();
const collectionReconciliation = collectionLine.reconciliation();
const collectionPatchItem = collectionLine.patch(
  resourcePatch.item({
    itemId: "workspace:demo",
    nextItem: { id: "workspace:demo", title: "Updated" },
  }),
);
const collectionPatchAspect = collectionLine.patch(
  resourcePatch.itemAspect({
    itemId: "workspace:demo",
    aspect: "title",
    value: "Aspect Updated",
  }),
);
const collectionPatchReplace = collectionLine.patch(
  resourcePatch.replace({
    items: [{ id: "workspace:demo", title: "Replaced" }],
    total: 1,
  }),
);
const collectionPatchSummary = collectionLine.patch(
  resourcePatch.summary({
    summary: "total",
    value: 2,
  }),
);
const collectionDeliveredPatch = collectionLine.deliver(
  resourceDelivery.patch({
    packetId: "pkt-1",
    basisId: "basis-1",
    nextBasisId: "basis-2",
    patch: resourcePatch.itemAspect({
      itemId: "workspace:demo",
      aspect: "title",
      value: "Delivered",
    }),
  }),
);
const collectionExternalBasisRefresh = collectionLine.deliver(
  externalCompatibilityDelivery.basisRefresh({
    packetId: "pkt-basis-refresh",
    basisId: "basis-2",
    nextBasisId: "basis-3",
  }),
);

const paged = signals.resource.paged({
  params: resourceParams<{ workspaceId: string }>(),
  normalizeParams: ({ workspaceId }) =>
    resourceParamIdentity({ workspaceId }, workspaceId),
  itemIdentity: (item: { id: string; title: string }) => item.id,
  reconcile: resourceCollectionShape<
    { items: Array<{ id: string; title: string }>; cursor: string | null; visibleCount: number },
    { id: string; title: string },
    {},
    {
      visibleCount: {
        read(
          value: {
            items: Array<{ id: string; title: string }>;
            cursor: string | null;
            visibleCount: number;
          },
        ): number;
        write(
          value: {
            items: Array<{ id: string; title: string }>;
            cursor: string | null;
            visibleCount: number;
          },
          visibleCount: number,
        ): {
          items: Array<{ id: string; title: string }>;
          cursor: string | null;
          visibleCount: number;
        };
      };
    },
    "pageWindow"
  >({
    items: (value: {
      items: Array<{ id: string; title: string }>;
      cursor: string | null;
      visibleCount: number;
    }) => value.items,
    replaceItems: (
      value: {
        items: Array<{ id: string; title: string }>;
        cursor: string | null;
        visibleCount: number;
      },
      nextItems: readonly { id: string; title: string }[],
    ) => ({ ...value, items: [...nextItems] }),
    summaries: resourceValueSummaries.pageWindow({
      visibleCount: {
        read: (value: {
          items: Array<{ id: string; title: string }>;
          cursor: string | null;
          visibleCount: number;
        }) => value.visibleCount,
        write: (
          value: {
            items: Array<{ id: string; title: string }>;
            cursor: string | null;
            visibleCount: number;
          },
          visibleCount: number,
        ) => ({ ...value, visibleCount }),
      },
    }),
  }),
  accumulatePage: (
    existing: {
      items: Array<{ id: string; title: string }>;
      cursor: string | null;
      visibleCount: number;
    },
    next: {
      items: Array<{ id: string; title: string }>;
      cursor: string | null;
      visibleCount: number;
    },
  ) => ({
    items: [...existing.items, ...next.items],
    cursor: next.cursor,
    visibleCount: next.visibleCount,
  }),
  load: ({ workspaceId }) => ({
    items: [{ id: String(workspaceId), title: String(workspaceId) }],
    cursor: null,
    visibleCount: 1,
  }),
});

const pagedLine = paged.line({ workspaceId: "demo" });
const pagedItems = pagedLine.value();
const pagedReconciliation = pagedLine.reconciliation();
const pagedPatch = pagedLine.patch(
  resourcePatch.item({
    itemId: "demo",
    nextItem: { id: "demo", title: "Paged Updated" },
  }),
);
const pagedSummaryPatch = pagedLine.patch(
  resourcePatch.summary({
    summary: "visibleCount",
    value: 2,
  }),
);

void detailValue;
void externalDetailLine.value();
void detailSignal;
void detailDescriptor;
void detailHistory;
void detailVerificationPackage.committedValue;
void detailVerificationPackage.externalCompatibility.kind;
void detailVerificationPackage.processing.kind;
void detailVerificationPackage.upload.kind;
void detailBranch;
void detailBasisHistory.advances;
void detailHistoryAvailability.restoreExact;
void detailHistoryAvailability.replayExact;
void detailReplayExact.kind;
if (detailReplayExact.kind === "replayed") {
  void detailReplayExact.reloadStatus.operation;
}
void detailRestoreExact.kind;
if (detailRestoreExact.kind === "restored") {
  void detailRestoreExact.reloadStatus.operation;
}
void detailRequest;
void detailDownload;
void detailDiagnostics;
void detailDiagnosticsSummary.current.status;
void detailInvalidate;
void detailFamilyInvalidate;
void detailFamilyInvalidateAll;
void detailRefresh;
void detailRevalidate;
void detailStatus;
void detailFreshness;
void detailAwaitedSettlement.resultKind;
void detailLabel;
void detailGraph;
void collectionItems;
void collectionReconciliation;
void collectionPatchItem;
void collectionPatchAspect;
void collectionPatchReplace;
void collectionPatchSummary;
void collectionDeliveredPatch;
void collectionExternalBasisRefresh;
void pagedItems;
void pagedReconciliation;
void pagedPatch;
void pagedSummaryPatch;

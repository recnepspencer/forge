import {
  createSignals,
  resourceBinaryDescriptor,
  resourceBinaryValue,
  resourceCollectionShape,
  resourceDelivery,
  resourceDownload,
  resourceItemAspects,
  resourceParamIdentity,
  resourceParams,
  resourcePatch,
  resourceProcessingJob,
  resourceProcessingResult,
  resourceUploadTransport,
  resourceUploadResult,
  resourceValueSummaries,
} from "../../index.js";

const signals = await createSignals({ deployment: "mainThreadCompatibility" });

const detail = signals.resource.detail({
  params: resourceParams<{ productId: string }>(),
  normalizeParams: ({ productId }) =>
    resourceParamIdentity({ productId }, productId),
  load: ({ productId }) => ({ id: productId }),
});

const detailLine = detail.line({ productId: "p1" });
const compatibilityDelivery = signals.resource.compatibility.delivery;

compatibilityDelivery.basisRefresh({
  packetId: "pkt-external-refresh",
  basisId: "basis-1",
  // @ts-expect-error external basis refresh requires an explicit nextBasisId
  nextBasisId: null,
});

signals.resource.compatibility.detail({
  version: "forge-resource-external-v1",
  family: "detail",
  definitionId: "bad-detail-contract",
  requestContract: "native-v1",
  // @ts-expect-error detail external definitions only admit reconciliationContract "none"
  reconciliationContract: "collection-v1",
  declaration: {
    params: resourceParams<{ productId: string }>(),
    normalizeParams: ({ productId }) =>
      resourceParamIdentity({ productId }, productId),
    load: ({ productId }) => ({ id: productId }),
  },
});

signals.resource.compatibility.collection({
  version: "forge-resource-external-v1",
  // @ts-expect-error collection external definitions must keep the family discriminant honest
  family: "detail",
  definitionId: "bad-collection-family",
  requestContract: "native-v1",
  reconciliationContract: "none",
  declaration: {
    params: resourceParams<{ workspaceId: string }>(),
    normalizeParams: ({ workspaceId }) =>
      resourceParamIdentity({ workspaceId }, workspaceId),
    itemIdentity: (item: { id: string }) => item.id,
    load: ({ workspaceId }) => [{ id: workspaceId }],
  },
});

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

signals.resource.detail({
  params: resourceParams<{ receiptId: string }>(),
  normalizeParams: ({ receiptId }) =>
    resourceParamIdentity({ receiptId }, receiptId),
  uploadTransport: resourceUploadTransport.signed({
    method: "PUT",
    finalizeRequired: true,
  }),
  load: ({ receiptId }) =>
    // @ts-expect-error resourceBinaryValue must not wrap upload-result truth
    resourceBinaryValue({
      value: resourceUploadResult.uploaded({
        uploadId: `upload:${receiptId}`,
        finalizeRequired: true,
        awaitingProcessing: false,
      }),
    }),
});


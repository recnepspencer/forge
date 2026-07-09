import { createDeferred } from "../runtime_fixture/async/deferred.mjs";

function createHostileApp(mod, resource, restoreState) {
  const requestContext = mod.resourceRequestContext({ basisId: "basis-1" });
  const retryDeferreds = [createDeferred(), createDeferred()];
  const transferDeferreds = [createDeferred(), createDeferred()];
  let retryLoadCount = 0;
  let transferLoadCount = 0;
  const collectionDeclaration = {
    params: mod.resourceParams(),
    requestContext,
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load: (_params, request) => ({
      items: [{
        id: "demo:1",
        title: restoreState.active
          ? "Restored Snapshot"
          : `Load:${request.context.basisId}`,
      }],
    }),
  };
  return {
    detail: resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) =>
        mod.resourceBinaryValue({
          value: { id, state: restoreState.active ? "restored" : "live" },
          descriptors: [
            mod.resourceBinaryDescriptor.file({
              id: `${id}:download`,
              fileName: `${id}.bin`,
              download: restoreState.active
                ? mod.resourceDownload.ready({
                    url: `https://downloads.example/${id}`,
                    method: "GET",
                  })
                : mod.resourceDownload.unavailable({
                    reason: "notReady",
                    detail: "restore not applied",
                  }),
            }),
          ],
        }),
    }).line({ id: "detail-1" }),
    retryDetail: resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.retryOnce(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => {
        retryLoadCount += 1;
        if (retryLoadCount === 1) {
          return { id, status: "stable-1" };
        }
        if (retryLoadCount === 2) {
          return retryDeferreds[0].promise;
        }
        if (retryLoadCount === 3) {
          return retryDeferreds[1].promise;
        }
        return { id, status: restoreState.active ? "restored-stable" : "stable-2" };
      },
    }).line({ id: "retry-1" }),
    transferDetail: resource.detail({
      params: mod.resourceParams(),
      policy: mod.resourcePolicyProfiles.timeoutFast(),
      processingJob: mod.resourceProcessingJob.poll(),
      uploadTransport: mod.resourceUploadTransport.signed({
        method: "POST",
        finalizeRequired: true,
      }),
      normalizeParams: ({ receiptId }) =>
        mod.resourceParamIdentity({ receiptId }, receiptId),
      load: ({ receiptId }) => {
        if (restoreState.active) {
          return { id: receiptId, status: "restored-ready" };
        }
        transferLoadCount += 1;
        if (transferLoadCount === 1) {
          return mod.resourceUploadResult.uploaded({
            uploadId: `upload:${receiptId}`,
            finalizeRequired: true,
            awaitingProcessing: true,
            message: "processing upload",
          });
        }
        if (transferLoadCount === 2) {
          return transferDeferreds[0].promise;
        }
        if (transferLoadCount === 3) {
          return transferDeferreds[1].promise;
        }
        return { id: receiptId, status: "ready" };
      },
    }).line({ receiptId: "receipt-1" }),
    nativeCollection: resource.collection(collectionDeclaration).line({
      workspaceId: "demo",
    }),
    externalCollection: resource.compatibility.collection({
      version: "WORTH-resource-external-v1",
      family: "collection",
      definitionId: "suite0-external-collection",
      requestContract: "native-v1",
      reconciliationContract: "collection-v1",
      declaration: collectionDeclaration,
    }).line({ workspaceId: "demo" }),
    paged: resource.paged({
      params: mod.resourceParams(),
      requestContext,
      normalizeParams: ({ workspaceId }) =>
        mod.resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      reconcile: mod.resourceCollectionShape({
        items: (value) => value.items,
        replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
        summaries: mod.resourceValueSummaries.pageWindow({
          visibleCount: {
            read: (value) => value.visibleCount,
            write: (value, visibleCount) => ({ ...value, visibleCount }),
          },
        }),
      }),
      accumulatePage: (_existing, next) => next,
      load: (_params, request) => ({
        items: [{ id: "demo:page-1", title: `Page:${request.context.basisId}` }],
        cursor: null,
        visibleCount: restoreState.active ? 9 : 1,
      }),
    }).line({ workspaceId: "demo" }),
    controls: {
      externalDelivery: resource.compatibility.delivery,
      retryDeferreds,
      transferDeferreds,
    },
  };
}

async function advanceAsyncSettlement() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

async function runHostileScript(lines, mod) {
  lines.retryDetail.refresh();
  lines.controls.retryDeferreds[0].reject(new Error("temporary retry failure"));
  await lines.controls.retryDeferreds[0].promise.catch(() => {});
  await Promise.resolve();
  lines.controls.retryDeferreds[1].resolve({
    id: "retry-1",
    status: "stable-2",
  });
  await lines.controls.retryDeferreds[1].promise;
  await advanceAsyncSettlement();

  lines.transferDetail.refresh();
  lines.transferDetail.refresh();
  lines.controls.transferDeferreds[0].resolve({
    id: "receipt-1",
    status: "stale-superseded",
  });
  await lines.controls.transferDeferreds[0].promise;
  await Promise.resolve();
  await advanceAsyncSettlement();
  lines.controls.transferDeferreds[1].resolve({
    id: "receipt-1",
    status: "stale-timed-out",
  });
  await lines.controls.transferDeferreds[1].promise;
  await Promise.resolve();
  lines.transferDetail.refresh();

  lines.nativeCollection.patch(
    mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Locally Patched",
    }),
  );
  lines.externalCollection.patch(
    mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Locally Patched",
    }),
  );
  const staleExternal = lines.externalCollection.deliver(
    lines.controls.externalDelivery.patch({
      packetId: "pkt-stale",
      basisId: "basis-2",
      nextBasisId: "basis-3",
      patch: mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Should Reject",
      }),
    }),
  );
  lines.nativeCollection.deliver(
    mod.resourceDelivery.replace({
      packetId: "pkt-native-b2",
      basisId: "basis-1",
      nextBasisId: "basis-2",
      nextValue: { items: [{ id: "demo:1", title: "Delivered Basis 2" }] },
    }),
  );
  lines.nativeCollection.deliver(
    mod.resourceDelivery.patch({
      packetId: "pkt-native-b3",
      basisId: "basis-2",
      nextBasisId: "basis-3",
      patch: mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Delivered Basis 3",
      }),
    }),
  );
  const duplicateNative = lines.nativeCollection.deliver(
    mod.resourceDelivery.patch({
      packetId: "pkt-native-b3",
      basisId: "basis-3",
      nextBasisId: "basis-4",
      patch: mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Duplicate Native Packet",
      }),
    }),
  );
  lines.externalCollection.deliver(
    lines.controls.externalDelivery.basisRefresh({
      packetId: "pkt-external-b2",
      basisId: "basis-1",
      nextBasisId: "basis-2",
    }),
  );
  lines.externalCollection.deliver(
    lines.controls.externalDelivery.patch({
      packetId: "pkt-external-b3",
      basisId: "basis-2",
      nextBasisId: "basis-3",
      patch: mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Delivered Basis 3",
      }),
    }),
  );
  lines.paged.patch(
    mod.resourcePatch.summary({
      summary: "visibleCount",
      value: 2,
    }),
  );
  lines.paged.invalidate();
  lines.detail.refresh();
  lines.nativeCollection.refresh();
  lines.externalCollection.refresh();
  lines.paged.refresh();
  return {
    duplicateNative,
    staleExternal,
  };
}

function freeHostileApp(lines) {
  for (const key of [
    "detail",
    "retryDetail",
    "transferDetail",
    "nativeCollection",
    "externalCollection",
    "paged",
  ]) {
    lines[key].free();
  }
}

export {
  createHostileApp,
  freeHostileApp,
  runHostileScript,
};

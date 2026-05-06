function createLineRequestState(requestDescriptor) {
  let basisId = requestDescriptor.context.basisId;
  const baseContext = requestDescriptor.context;
  const baseDescriptor = Object.freeze({
    family: requestDescriptor.family,
    canonicalParams: requestDescriptor.canonicalParams,
    target: requestDescriptor.target,
    baseUrl: requestDescriptor.baseUrl,
    method: requestDescriptor.method,
    body: requestDescriptor.body,
    auth: requestDescriptor.auth,
    continuation: requestDescriptor.continuation,
    processingJob: requestDescriptor.processingJob,
    uploadTransport: requestDescriptor.uploadTransport,
    sources: requestDescriptor.sources,
  });

  return Object.freeze({
    currentBasisId() {
      return basisId;
    },
    readDescriptor() {
      return createDescriptor(baseDescriptor, baseContext, basisId);
    },
    advanceBasis(nextBasisId) {
      basisId = nextBasisId;
      return basisId;
    },
    stageDescriptor(nextBasisId) {
      return Object.freeze({
        basisId,
        descriptor: createDescriptor(baseDescriptor, baseContext, nextBasisId),
        commit() {
          basisId = nextBasisId;
          return basisId;
        },
      });
    },
  });
}

function createDescriptor(baseDescriptor, baseContext, basisId) {
  return Object.freeze({
    ...baseDescriptor,
    context: Object.freeze({ ...baseContext, basisId }),
  });
}

export { createLineRequestState };

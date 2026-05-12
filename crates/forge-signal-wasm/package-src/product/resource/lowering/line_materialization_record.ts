function createLineMaterializationRecord(
  lineIdentity,
  requestDescriptor,
  requestState,
  binding,
  history,
  lifecycleHistory,
  delivery,
  patch,
  lineScope,
  lifecycle,
  reload,
  release,
  rematerialize,
  resourceLineEpoch,
) {
  return Object.freeze({
    lineIdentity,
    requestDescriptor,
    requestState,
    binding,
    history,
    lifecycleHistory,
    delivery,
    patch,
    lineScope,
    lifecycle,
    reload,
    release,
    rematerialize,
    resourceLineEpoch,
  });
}

export { createLineMaterializationRecord };

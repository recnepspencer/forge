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
  migrateIdentity,
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
    migrateIdentity,
    resourceLineEpoch,
  });
}

export { createLineMaterializationRecord };

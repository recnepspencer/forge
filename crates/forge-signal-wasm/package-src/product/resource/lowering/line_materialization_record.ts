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
  });
}

export { createLineMaterializationRecord };

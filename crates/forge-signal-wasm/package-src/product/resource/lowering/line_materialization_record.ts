function createLineMaterializationRecord(
  lineIdentity,
  requestDescriptor,
  binding,
  history,
  lifecycleHistory,
  patch,
  lineScope,
  lifecycle,
  reload,
  release,
) {
  return Object.freeze({
    lineIdentity,
    requestDescriptor,
    binding,
    history,
    lifecycleHistory,
    patch,
    lineScope,
    lifecycle,
    reload,
    release,
  });
}

export { createLineMaterializationRecord };

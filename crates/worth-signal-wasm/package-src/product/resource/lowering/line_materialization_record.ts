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
  effectBranchDagFactory,
  effectProjectionCoordinator,
) {
  let effectBranchDag = null;
  let nextEffectAdmissionSequence = 1;
  const record = {
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
    effectProjectionCoordinator,
    issueEffectAdmissionSequence() {
      return nextEffectAdmissionSequence++;
    },
    unregisterEffectProjection() {
      effectProjectionCoordinator.unregisterLine(lineIdentity.runtimeLineId);
    },
  };
  Object.defineProperty(record, "effectBranchDag", {
    enumerable: true,
    get() {
      effectBranchDag ??= effectBranchDagFactory(record);
      return effectBranchDag;
    },
  });
  return Object.freeze(record);
}

export { createLineMaterializationRecord };

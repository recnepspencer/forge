function createBasisRefreshDeliveryEffectSummary(
  packetValue,
  resolvedNextBasisId,
  supersededOperation,
) {
  return createLineDeliveryEffectSummary({
    deliveryKind: "basisRefresh",
    deliveryScope: "basis",
    packetValue,
    resolvedNextBasisId,
    supersededOperation,
    patchKind: null,
    patchScope: null,
    patchedItemId: null,
    patchedField: null,
    patchedRegion: null,
    patchedPath: null,
    patchedAspect: null,
    patchedSummary: null,
    valueChanged: false,
    regionProof: null,
  });
}

function createInvalidateDeliveryEffectSummary(
  packetValue,
  resolvedNextBasisId,
  supersededOperation,
) {
  return createLineDeliveryEffectSummary({
    deliveryKind: "invalidate",
    deliveryScope: "invalidate",
    packetValue,
    resolvedNextBasisId,
    supersededOperation,
    patchKind: null,
    patchScope: null,
    patchedItemId: null,
    patchedField: null,
    patchedRegion: null,
    patchedPath: null,
    patchedAspect: null,
    patchedSummary: null,
    valueChanged: false,
    regionProof: null,
  });
}

function createPatchDeliveryEffectSummary(
  packetValue,
  patchValue,
  patchDiagnostics,
  resolvedNextBasisId,
  supersededOperation,
) {
  return createLineDeliveryEffectSummary({
    deliveryKind: packetValue.kind,
    deliveryScope: patchDiagnostics.scope,
    packetValue,
    resolvedNextBasisId,
    supersededOperation,
    patchKind: patchValue.kind,
    patchScope: patchDiagnostics.scope,
    patchedItemId: patchDiagnostics.itemId,
    patchedField: patchDiagnostics.field,
    patchedRegion: patchDiagnostics.region,
    patchedPath: patchDiagnostics.path,
    patchedAspect: patchDiagnostics.aspect,
    patchedSummary: patchDiagnostics.summary,
    valueChanged: patchDiagnostics.valueChanged,
    regionProof: patchDiagnostics.regionProof,
    jsonPathProof: patchDiagnostics.jsonPathProof,
  });
}

function createLineDeliveryEffectSummary(options) {
  return Object.freeze({
    deliveryKind: options.deliveryKind,
    deliveryScope: options.deliveryScope,
    packetId: options.packetValue.packetId,
    basisId: options.packetValue.basisId,
    nextBasisId: options.resolvedNextBasisId,
    supersededOperation: options.supersededOperation,
    patchKind: options.patchKind,
    patchScope: options.patchScope,
    patchedItemId: options.patchedItemId,
    patchedField: options.patchedField,
    patchedRegion: options.patchedRegion,
    patchedPath: options.patchedPath,
    patchedAspect: options.patchedAspect,
    patchedSummary: options.patchedSummary,
    valueChanged: options.valueChanged,
    regionProof: options.regionProof ?? null,
    jsonPathProof: options.jsonPathProof ?? null,
  });
}

export {
  createBasisRefreshDeliveryEffectSummary,
  createInvalidateDeliveryEffectSummary,
  createPatchDeliveryEffectSummary,
};

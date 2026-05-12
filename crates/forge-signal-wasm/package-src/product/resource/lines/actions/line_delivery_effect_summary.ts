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
    patchedAspect: null,
    patchedSummary: null,
    valueChanged: false,
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
    patchedAspect: null,
    patchedSummary: null,
    valueChanged: false,
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
    patchedAspect: patchDiagnostics.aspect,
    patchedSummary: patchDiagnostics.summary,
    valueChanged: patchDiagnostics.valueChanged,
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
    patchedAspect: options.patchedAspect,
    patchedSummary: options.patchedSummary,
    valueChanged: options.valueChanged,
    jsonPathProof: options.jsonPathProof ?? null,
  });
}

export {
  createBasisRefreshDeliveryEffectSummary,
  createInvalidateDeliveryEffectSummary,
  createPatchDeliveryEffectSummary,
};

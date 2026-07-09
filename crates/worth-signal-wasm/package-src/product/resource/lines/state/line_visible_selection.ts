function createInitialVisibleSelection(requestDescriptor, hasVisibleValue) {
  return Object.freeze({
    kind: hasVisibleValue ? "committed" : "unavailable",
    source: "initialLoad",
    effectId: null,
    branchId: requestDescriptor.context.branchId,
    snapshotId: null,
    basisId: requestDescriptor.context.basisId,
    detail: hasVisibleValue
      ? "resource line visible truth is the committed initial load"
      : "resource line visible truth is unavailable because initial load has no value",
  });
}

function createPatchedVisibleSelection(previousSelection, effectEnvelope) {
  const optimistic = effectEnvelope.optimistic;
  if (optimistic.kind === "applied") {
    return Object.freeze({
      kind: "speculative",
      source: "localPatch",
      effectId: effectEnvelope.effectId,
      branchId: optimistic.branchId,
      snapshotId: optimistic.snapshotId,
      basisId: effectEnvelope.request.basisId,
      rollbackKind: optimistic.rollback.kind,
      detail:
        "resource line visible truth is the selected speculative branch effect",
    });
  }
  if (optimistic.kind === "unavailable") {
    return Object.freeze({
      kind: "committed",
      source: "optimismUnavailable",
      effectId: effectEnvelope.effectId,
      branchId: effectEnvelope.request.branchId,
      snapshotId: null,
      basisId: effectEnvelope.request.basisId,
      unavailableReason: optimistic.reason,
      detail:
        "resource line visible truth is committed directly because speculative branch visibility was unavailable",
    });
  }
  return Object.freeze({
    kind: "committed",
    source: "localPatch",
    effectId: effectEnvelope.effectId,
    branchId: effectEnvelope.request.branchId,
    snapshotId: null,
    basisId: effectEnvelope.request.basisId,
    detail: optimistic.detail,
  });
}

function createReloadFulfilledVisibleSelection(
  previousSelection,
  requestDiagnostics,
  operation,
  visibleValueChanged,
) {
  if (!visibleValueChanged && previousSelection.kind !== "unavailable") {
    return previousSelection;
  }
  return Object.freeze({
    kind: "committed",
    source: operation,
    effectId: null,
    branchId: requestDiagnostics.context.branchId,
    snapshotId: null,
    basisId: requestDiagnostics.context.basisId,
    detail: `resource line visible truth is the committed ${operation} result`,
  });
}

function createDeliveredVisibleSelection(effectEnvelope) {
  const confirmation = effectEnvelope.optimistic.confirmation;
  return Object.freeze({
    kind: "confirmed",
    source: "delivery",
    effectId: effectEnvelope.effectId,
    branchId: effectEnvelope.request.branchId,
    snapshotId: null,
    basisId: effectEnvelope.request.basisId,
    confirmationKind: confirmation.kind,
    previousEffectId: confirmation.previousEffectId ?? null,
    detail: confirmation.detail,
  });
}

function createRestoredVisibleSelection(
  previousSelection,
  rollback,
  effectEnvelope = null,
) {
  const effectId = effectEnvelope?.effectId ?? previousSelection.effectId ?? null;
  if (
    rollback === null
    || (
      rollback.kind !== "exactBranchRestoreAvailable"
      && rollback.kind !== "compactInverseAvailable"
    )
  ) {
    return Object.freeze({
      kind: "restored",
      source: "historyRestore",
      effectId,
      branchId: previousSelection.branchId ?? null,
      snapshotId: previousSelection.snapshotId ?? null,
      basisId: previousSelection.basisId ?? null,
      rollbackKind: null,
      detail:
        "resource line visible truth was restored through exact line history restore",
    });
  }
  return Object.freeze({
    kind: "restored",
    source: rollback.kind === "compactInverseAvailable"
      ? "compactInverse"
      : "exactBranchRestore",
    effectId,
    branchId: rollback.branchId,
    snapshotId: rollback.snapshotId,
    basisId: previousSelection.basisId ?? null,
    rollbackKind: rollback.kind,
    detail: rollback.detail,
  });
}

export {
  createDeliveredVisibleSelection,
  createInitialVisibleSelection,
  createPatchedVisibleSelection,
  createReloadFulfilledVisibleSelection,
  createRestoredVisibleSelection,
};

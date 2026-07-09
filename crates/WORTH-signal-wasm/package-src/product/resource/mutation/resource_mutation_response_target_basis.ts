import { readLineBindingState } from "../lines/state/line_binding_state.js";

function createMutationResponseTargetBasisSnapshots(declaration, mutationParams) {
  return createMutationResponseTargetBasisSnapshotsForTargets(
    declaration.targets,
    mutationParams,
  );
}

function createMutationResponseTargetBasisSnapshotsForTargets(
  targets,
  mutationParams,
) {
  return Object.freeze(
    targets.map((target) =>
      createMutationResponseTargetBasisSnapshot(target, mutationParams)),
  );
}

function createMutationResponseTargetBasisSnapshot(target, mutationParams) {
  const targetParams = target.params(mutationParams);
  const lineIdentity = target.readTargetLineIdentity(targetParams);
  const materialization =
    lineIdentity.residency === "resident"
      ? target.lookupResidentTargetMaterialization(targetParams)
      : null;
  const diagnostics =
    materialization === null
      ? null
      : readLineBindingState(materialization.binding).diagnostics;
  const basisId = materialization?.requestState.currentBasisId() ?? null;
  const visibleValueVersion = diagnostics?.visibleValueVersion ?? null;
  return Object.freeze({
    targetId: target.targetId,
    familyKind: target.family.kind,
    familyId: target.family.familyId,
    canonicalKey: lineIdentity.canonicalKey,
    runtimeLineId: lineIdentity.runtimeLineId,
    residency: lineIdentity.residency,
    basisId,
    visibleValueVersion,
    digest: [
      target.targetId,
      target.family.kind,
      target.family.familyId,
      lineIdentity.canonicalKey,
      lineIdentity.runtimeLineId ?? "none",
      lineIdentity.residency,
      basisId ?? "none",
      visibleValueVersion ?? "none",
    ].join("|"),
  });
}

function readMutationResponseTargetStaleness(
  lineIdentity,
  targetMaterialization,
  submittedTarget,
) {
  if (submittedTarget === null || submittedTarget === undefined) {
    return null;
  }
  if (submittedTarget.canonicalKey !== lineIdentity.canonicalKey) {
    return createTargetStaleness(
      submittedTarget,
      lineIdentity,
      null,
      null,
      "canonicalKeyChanged",
    );
  }
  if (submittedTarget.runtimeLineId !== lineIdentity.runtimeLineId) {
    return createTargetStaleness(
      submittedTarget,
      lineIdentity,
      null,
      null,
      "runtimeLineIdChanged",
    );
  }
  const diagnostics = readLineBindingState(targetMaterialization.binding).diagnostics;
  const currentBasisId = targetMaterialization.requestState.currentBasisId();
  if (submittedTarget.basisId !== currentBasisId) {
    return createTargetStaleness(
      submittedTarget,
      lineIdentity,
      currentBasisId,
      diagnostics.visibleValueVersion,
      "basisChanged",
    );
  }
  if (submittedTarget.visibleValueVersion !== diagnostics.visibleValueVersion) {
    return createTargetStaleness(
      submittedTarget,
      lineIdentity,
      currentBasisId,
      diagnostics.visibleValueVersion,
      "visibleValueVersionChanged",
    );
  }
  return null;
}

function createTargetStaleness(
  submittedTarget,
  lineIdentity,
  currentBasisId,
  currentVisibleValueVersion,
  reason,
) {
  return Object.freeze({
    kind: "staleTarget",
    reason,
    submittedBasisId: submittedTarget.basisId,
    currentBasisId,
    submittedVisibleValueVersion: submittedTarget.visibleValueVersion,
    currentVisibleValueVersion,
    submittedCanonicalKey: submittedTarget.canonicalKey,
    currentCanonicalKey: lineIdentity.canonicalKey,
    detail: [
      `mutation response target ${submittedTarget.targetId}`,
      `was submitted for ${submittedTarget.canonicalKey}`,
      `but ${reason} before response reconciliation`,
    ].join(" "),
  });
}

export {
  createMutationResponseTargetBasisSnapshots,
  createMutationResponseTargetBasisSnapshotsForTargets,
  readMutationResponseTargetStaleness,
};

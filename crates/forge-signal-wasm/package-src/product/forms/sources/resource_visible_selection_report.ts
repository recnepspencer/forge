import { stableValueDigest } from "../values/value_paths.js";

export function readFormResourceVisibleSelectionReport(visibleSelection) {
  const branchProof = normalizeBranchProof(visibleSelection);
  const rebaseProof = normalizeRebaseProof(visibleSelection);
  switch (visibleSelection.kind) {
    case "unavailable":
      return freezeVisibleSelection({
        kind: "unavailable",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId,
        branchId: visibleSelection.branchId,
        snapshotId: visibleSelection.snapshotId,
        basisId: visibleSelection.basisId,
        unavailableReason: null,
        rollbackKind: null,
        confirmationKind: null,
        previousEffectId: null,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    case "committed":
      return freezeVisibleSelection({
        kind: "committed",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId,
        branchId: visibleSelection.branchId,
        snapshotId: visibleSelection.snapshotId,
        basisId: visibleSelection.basisId,
        unavailableReason: visibleSelection.unavailableReason ?? null,
        rollbackKind: null,
        confirmationKind: null,
        previousEffectId: null,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    case "speculative":
      return freezeVisibleSelection({
        kind: "speculative",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId,
        branchId: visibleSelection.branchId,
        snapshotId: visibleSelection.snapshotId,
        basisId: visibleSelection.basisId,
        unavailableReason: null,
        rollbackKind: visibleSelection.rollbackKind,
        confirmationKind: null,
        previousEffectId: null,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    case "confirmed":
      return freezeVisibleSelection({
        kind: "confirmed",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId,
        branchId: visibleSelection.branchId,
        snapshotId: visibleSelection.snapshotId,
        basisId: visibleSelection.basisId,
        unavailableReason: null,
        rollbackKind: null,
        confirmationKind: visibleSelection.confirmationKind,
        previousEffectId: visibleSelection.previousEffectId,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    case "restored":
      return freezeVisibleSelection({
        kind: "restored",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId,
        branchId: visibleSelection.branchId,
        snapshotId: visibleSelection.snapshotId,
        basisId: visibleSelection.basisId,
        unavailableReason: null,
        rollbackKind: visibleSelection.rollbackKind,
        confirmationKind: null,
        previousEffectId: null,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    case "merged":
      return freezeVisibleSelection({
        kind: "merged",
        source: visibleSelection.source,
        effectId: visibleSelection.effectId ?? null,
        branchId: visibleSelection.branchId ?? null,
        snapshotId: visibleSelection.snapshotId ?? null,
        basisId: visibleSelection.basisId ?? null,
        unavailableReason: null,
        rollbackKind: null,
        confirmationKind: null,
        previousEffectId: null,
        detail: visibleSelection.detail,
        branchProof,
        rebaseProof,
      });
    default:
      throw new TypeError(`unsupported resource visible-selection kind "${visibleSelection.kind}"`);
  }
}

function normalizeBranchProof(visibleSelection) {
  const admitted = visibleSelection.branchId !== null
    && (
      visibleSelection.kind === "speculative"
      || visibleSelection.kind === "confirmed"
      || visibleSelection.kind === "restored"
      || visibleSelection.kind === "merged"
    );
  return Object.freeze({
    admitted,
    reason: admitted
      ? null
      : "resource line visible selection does not carry admitted native branch-visible proof",
  });
}

function normalizeRebaseProof(visibleSelection) {
  const admitted = visibleSelection.kind === "merged" && visibleSelection.branchId !== null;
  return Object.freeze({
    admitted,
    reason: admitted
      ? null
      : "resource line visible selection does not carry admitted merge/rebase-visible proof",
  });
}

function freezeVisibleSelection(report) {
  return Object.freeze({
    ...report,
    digest: stableValueDigest(report),
  });
}

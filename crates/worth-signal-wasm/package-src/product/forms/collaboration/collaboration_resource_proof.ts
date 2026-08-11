import { FormDeclarationError } from "../form_errors.js";
import { stableValueDigest } from "../values/value_paths.js";

export function deriveCollaborationResourceProof(declaration, resourceSource) {
  if (declaration.mode !== "branchPerActor" && declaration.mode !== "optimisticMerge") {
    return noResourceProof();
  }
  if (resourceSource === null) {
    return collaborationResourceProof({
      required: true,
      admitted: false,
      sourceKind: null,
      visibleSelectionKind: null,
      branchId: null,
      reason: `declared collaboration mode "${declaration.mode}" requires a resource line form source`,
    });
  }
  const visibleSelectionKind = resourceSource.visibleSelection.kind;
  const branchId = resourceSource.visibleSelection.branchId ?? null;
  if (resourceSource.visibleSelection.branchProof.admitted !== true || branchId === null) {
    return collaborationResourceProof({
      required: true,
      admitted: false,
      sourceKind: resourceSource.sourceKind,
      visibleSelectionKind,
      branchId,
      reason: `declared collaboration mode "${declaration.mode}" requires admitted native branch-visible resource proof`,
    });
  }
  return collaborationResourceProof({
    required: true,
    admitted: true,
    sourceKind: resourceSource.sourceKind,
    visibleSelectionKind,
    branchId,
    reason: null,
  });
}

export function noResourceProof() {
  return collaborationResourceProof({
    required: false,
    admitted: true,
    sourceKind: null,
    visibleSelectionKind: null,
    branchId: null,
    reason: null,
  });
}

export function pinCollaborationBranchId(resourceProof, reportedBranchId) {
  if (resourceProof.required && !resourceProof.admitted) {
    // Proof-unavailable modes ignore host branch fiction entirely.
    return null;
  }
  if (!(resourceProof.required && resourceProof.admitted)) {
    return reportedBranchId;
  }
  if (
    reportedBranchId != null
    && reportedBranchId !== resourceProof.branchId
  ) {
    throw new FormDeclarationError(
      "collaboration branchId must match admitted resource branch proof",
      {
        branchId: reportedBranchId,
        resourceBranchId: resourceProof.branchId,
      },
    );
  }
  return resourceProof.branchId;
}

export function readCollaborationBranchId(resourceProof, currentBranchId) {
  // Resource-backed modes either expose admitted native branch identity or null.
  // Host reports must not invent a branch while proof is unavailable, and must
  // not replace the native id once proof is admitted.
  if (resourceProof.required) {
    return resourceProof.admitted ? resourceProof.branchId : null;
  }
  return currentBranchId ?? resourceProof.branchId;
}

function collaborationResourceProof(proof) {
  return Object.freeze({
    ...proof,
    digest: stableValueDigest(proof),
  });
}

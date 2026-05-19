import { resourceEffects } from "../../resource/effects/resource_effect_profile.js";
import { resourceMutationResponses } from "../../resource/mutation/resource_mutation_response_closeout_matrix.js";
import { readFormResourceMutationResponseReport } from "./resource_mutation_response_report.js";
import { readFormResourceSettlementReport } from "./resource_settlement_report.js";
import { readFormResourceVisibleSelectionReport } from "./resource_visible_selection_report.js";
import { stableValueDigest } from "../values/value_paths.js";

export function readResourceLineProof(line, request, summary, status, freshness, mutationResponse) {
  const history = line.history();
  const verificationPackage = history.verificationPackage();
  const mutationResponsePlanCount = verificationPackage.mutationResponse?.planCount ?? 1;
  const effectProfile = request.effects === null
    ? null
    : Object.freeze({
      name: request.effects.name,
      optimism: request.effects.optimism,
      confirmation: request.effects.confirmation,
      rollback: request.effects.rollback,
      rebase: request.effects.rebase,
      preimage: request.effects.preimage,
    });
  const visibleSelection = readFormResourceVisibleSelectionReport(summary.current.visibleSelection);
  const mutationResponseReport = readFormResourceMutationResponseReport(
    mutationResponse,
    mutationResponsePlanCount,
  );
  return Object.freeze({
    effectProfile: Object.freeze({
      profile: effectProfile,
      closeoutMatrixDigest: effectProfile === null
        ? null
        : stableValueDigest(resourceEffects.closeoutMatrix(request.effects)),
    }),
    rollback: normalizeRollbackDigest(summary.current.visibleSelection, effectProfile),
    visibleSelection,
    settlement: readFormResourceSettlementReport(
      status,
      freshness,
      visibleSelection,
      mutationResponseReport,
    ),
    history: Object.freeze({
      branch: history.branch,
      availability: history.availability,
    }),
    mutationResponse: mutationResponseReport,
    verification: Object.freeze({
      packageDigest: stableValueDigest(verificationPackage),
      externalCompatibility: verificationPackage.externalCompatibility,
      mutationResponseCloseoutMatrixDigest: mutationResponse === null
        ? null
        : stableValueDigest(resourceMutationResponses.closeoutMatrix()),
    }),
  });
}

function normalizeRollbackDigest(visibleSelection, effectProfile) {
  if (
    (visibleSelection.kind !== "speculative" && visibleSelection.kind !== "restored") ||
    visibleSelection.rollbackKind === null ||
    visibleSelection.rollbackKind === undefined
  ) {
    return null;
  }
  switch (visibleSelection.rollbackKind) {
    case "exactBranchRestore":
    case "exactBranchRestoreAvailable":
      return Object.freeze({
        kind: "exactBranchRestoreAvailable",
        mode: "SameRuntimeBranchExact",
        branchId: visibleSelection.branchId ?? null,
        snapshotId: visibleSelection.snapshotId ?? null,
        reason: null,
        detail: "resource line visible selection can roll back through exact same-runtime branch restore",
      });
    case "inversePatch":
    case "compactInverseAvailable":
      return Object.freeze({
        kind: "compactInverseAvailable",
        mode: "CompactInversePatch",
        branchId: visibleSelection.branchId ?? null,
        snapshotId: visibleSelection.snapshotId ?? null,
        reason: null,
        detail: "resource line visible selection can roll back through compact inverse patch proof",
      });
    case "unavailable":
      return Object.freeze({
        kind: "unavailable",
        mode: null,
        branchId: visibleSelection.branchId ?? null,
        snapshotId: visibleSelection.snapshotId ?? null,
        reason: effectProfile?.rollback ?? "unavailable",
        detail: "resource line visible selection does not expose an available rollback path",
      });
    case "notApplicable":
      return Object.freeze({
        kind: "notApplicable",
        mode: null,
        branchId: visibleSelection.branchId ?? null,
        snapshotId: visibleSelection.snapshotId ?? null,
        reason: effectProfile?.rollback ?? null,
        detail: "resource line visible selection is not under speculative rollback posture",
      });
    default:
      throw new TypeError(`unsupported resource visible-selection rollback kind "${visibleSelection.rollbackKind}"`);
  }
}

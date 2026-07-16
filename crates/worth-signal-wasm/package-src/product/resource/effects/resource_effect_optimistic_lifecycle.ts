import { createResourceEffectServerConfirmation } from "./resource_effect_server_confirmation.js";
import { createResourceEffectRollback } from "./resource_effect_rollback.js";

function createResourceEffectOptimisticLifecycle(effectPlan, currentLocus, patch, resolvedBranchPosture = effectPlan.branchPosture) {
  const branchPosture = resolvedBranchPosture;
  switch (branchPosture.kind) {
    case "effectOwnedBranch":
      return Object.freeze({
        kind: "applied",
        admissionKind: effectPlan.admissionKind,
        branchPosture: branchPosture.kind,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        rollback: Object.freeze({
          kind: "effectBranchRetirementAvailable",
          branchId: branchPosture.branchId,
          dependencyBasisBranchId: branchPosture.dependencyBasisBranchId,
          mode: "EffectBranchRetirement",
        }),
        confirmation: "pendingServer",
        detail: "resource effect is isolated in an effect-owned branch and projected through derived visible truth",
      });
    case "effectOwnedBranchPlanned":
      throw new TypeError(
        "resource effect optimistic lifecycle requires completed branch acquisition",
      );
    case "optimisticUnavailable":
      return Object.freeze({
        kind: "unavailable",
        admissionKind: effectPlan.admissionKind,
        branchPosture: branchPosture.kind,
        reason: branchPosture.reason,
        detail: branchPosture.detail,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        inverseAvailable: branchPosture.inverseAvailable,
        rollback: createResourceEffectRollback(effectPlan),
      });
    case "committedOnly":
      return Object.freeze({
        kind: "committed",
        admissionKind: effectPlan.admissionKind,
        branchPosture: branchPosture.kind,
        reason: branchPosture.reason,
        detail: branchPosture.detail,
        rollback: createResourceEffectRollback(effectPlan),
        confirmation: createResourceEffectServerConfirmation(
          effectPlan,
          currentLocus,
          patch,
        ),
      });
    default:
      throw new TypeError(
        `resource effect optimistic lifecycle cannot classify branch posture "${branchPosture.kind}"`,
      );
  }
}

export { createResourceEffectOptimisticLifecycle };

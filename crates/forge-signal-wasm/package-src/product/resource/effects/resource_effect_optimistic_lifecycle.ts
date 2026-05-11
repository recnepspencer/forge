import { createResourceEffectServerConfirmation } from "./resource_effect_server_confirmation.js";
import { createResourceEffectRollback } from "./resource_effect_rollback.js";

function createResourceEffectOptimisticLifecycle(effectPlan, currentLocus, patch) {
  const branchPosture = effectPlan.branchPosture;
  switch (branchPosture.kind) {
    case "speculativeBranch":
      return Object.freeze({
        kind: "applied",
        admissionKind: effectPlan.admissionKind,
        branchPosture: branchPosture.kind,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        restoreMode: branchPosture.restoreMode,
        rollback: createResourceEffectRollback(effectPlan),
        confirmation: "pendingServer",
        detail:
          "resource effect was applied under a branch-native speculative posture and awaits server confirmation",
      });
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

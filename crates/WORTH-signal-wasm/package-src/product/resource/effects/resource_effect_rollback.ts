function createResourceEffectRollback(effectPlan) {
  const branchPosture = effectPlan.branchPosture;
  switch (branchPosture.kind) {
    case "speculativeBranch":
      if (branchPosture.rollbackMode === "CompactInversePatch") {
        return Object.freeze({
          kind: "compactInverseAvailable",
          mode: branchPosture.rollbackMode,
          branchId: branchPosture.branchId,
          snapshotId: branchPosture.snapshotId,
          inverse: branchPosture.inverse,
          detail:
            "resource effect rollback can apply the compact inverse captured before speculative mutation",
        });
      }
      return Object.freeze({
        kind: "exactBranchRestoreAvailable",
        mode: branchPosture.restoreMode,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        detail:
          "resource effect rollback can restore the exact branch snapshot captured before speculative application",
      });
    case "optimisticUnavailable":
      return Object.freeze({
        kind: "unavailable",
        reason: branchPosture.reason,
        detail: branchPosture.detail,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        inverseAvailable: branchPosture.inverseAvailable,
      });
    case "committedOnly":
      return Object.freeze({
        kind: "notApplicable",
        reason: branchPosture.reason,
        detail:
          "committed-only resource effects do not carry speculative rollback state",
      });
    default:
      throw new TypeError(
        `resource effect rollback cannot classify branch posture "${branchPosture.kind}"`,
      );
  }
}

export { createResourceEffectRollback };

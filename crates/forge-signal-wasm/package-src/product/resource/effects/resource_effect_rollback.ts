function createResourceEffectRollback(effectPlan) {
  const branchPosture = effectPlan.branchPosture;
  switch (branchPosture.kind) {
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

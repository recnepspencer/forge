function createResourceEffectBranchLifecycle(effectPlan) {
  const branchPosture = effectPlan.branchPosture;
  switch (branchPosture.kind) {
    case "speculativeBranch":
      return Object.freeze({
        kind: "selectedExistingBranch",
        acquisition: "currentRuntimeBranch",
        creation: "notCreatedByResourceRuntime",
        reuse: "currentBranchReuse",
        ownership: "signalsRuntimeOwned",
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        restoreMode: branchPosture.restoreMode,
        disposal: Object.freeze({
          kind: "notOwnedByResourceRuntime",
          detail:
            "resource effect selected an existing Signals branch and must not dispose branch state it did not create",
        }),
        leakDenial: Object.freeze({
          kind: "noResourceOwnedBranch",
          detail:
            "resource effect did not create package-local speculative branch state that could survive disposal",
        }),
      });
    case "optimisticUnavailable":
      return Object.freeze({
        kind: "unavailable",
        creation: "deniedBeforeBranchCreation",
        reason: branchPosture.reason,
        detail: branchPosture.detail,
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        disposal: Object.freeze({
          kind: "notApplicable",
          detail:
            "resource effect did not acquire a speculative branch, so there is no branch disposal action",
        }),
        leakDenial: Object.freeze({
          kind: "optimismDeniedBeforeResourceOwnedBranch",
          detail:
            "resource effect denied branch speculation before creating resource-owned speculative branch state",
        }),
      });
    case "committedOnly":
      return Object.freeze({
        kind: "notApplicable",
        creation: "notApplicable",
        reason: branchPosture.reason,
        detail: branchPosture.detail,
        disposal: Object.freeze({
          kind: "notApplicable",
          detail:
            "committed-only resource effects do not acquire speculative branch state",
        }),
        leakDenial: Object.freeze({
          kind: "notApplicable",
          detail:
            "committed-only resource effects do not create speculative branch state",
        }),
      });
    default:
      throw new TypeError(
        `resource effect branch lifecycle cannot classify branch posture "${branchPosture.kind}"`,
      );
  }
}

export { createResourceEffectBranchLifecycle };

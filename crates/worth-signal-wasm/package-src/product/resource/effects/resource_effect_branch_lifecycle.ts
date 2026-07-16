function createResourceEffectBranchLifecycle(effectPlan, branchPosture = effectPlan.branchPosture) {
  switch (branchPosture.kind) {
    case "effectOwnedBranch":
      return Object.freeze({
        kind: "effectOwnedBranch",
        acquisition: "explicitForkPlan",
        creation: "createdByResourceRuntime",
        reuse: "forbidden",
        ownership: "resourceEffectOwned",
        branchId: branchPosture.branchId,
        snapshotId: branchPosture.snapshotId,
        dependencyBasisBranchId: branchPosture.dependencyBasisBranchId,
        nativeAncestryProof: branchPosture.nativeAncestryProof,
        semanticDependencyProof: branchPosture.semanticDependencyProof,
        disposal: Object.freeze({
          kind: "retireOwnedBranch",
          detail: "effect closeout retires the effect-owned native branch",
        }),
        leakDenial: Object.freeze({
          kind: "retirementReceiptRequired",
          detail: "terminal effect state requires native branch retirement proof",
        }),
      });
    case "effectOwnedBranchPlanned":
      throw new TypeError(
        "resource effect envelope cannot be issued before its effect-owned branch is acquired",
      );
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

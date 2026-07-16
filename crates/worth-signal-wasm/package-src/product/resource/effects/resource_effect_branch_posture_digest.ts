function createEnvelopeBranchPosture(plannedPosture, acquisition) {
  if (acquisition === null) {
    return plannedPosture;
  }
  return Object.freeze({
    kind: "effectOwnedBranch",
    profileName: plannedPosture.profileName,
    optimism: plannedPosture.optimism,
    rollback: plannedPosture.rollback,
    rollbackMode: "EffectBranchRetirement",
    branchId: Number(acquisition.branch.branch.id),
    snapshotId: acquisition.branch.appliedBasis.snapshotId,
    restoreMode: null,
    dependencyBasisBranchId:
      acquisition.dependencyBasisBranch === null
        ? null
        : Number(acquisition.dependencyBasisBranch.branch.id),
    nativeAncestryProof: acquisition.nativeAncestryProof,
    semanticDependencyProof: acquisition.semanticDependencyProof,
    proofBreadth: plannedPosture.proofBreadth + 2,
  });
}

export { createEnvelopeBranchPosture };

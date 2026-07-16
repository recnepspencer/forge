import { requireResourceEffectBranchAcquisitionPlan } from "./resource_effect_branch_acquisition_plan.js";

async function executeResourceEffectBranchAcquisition(
  history,
  rawPlan,
  authoredSignalIds,
  effectValue,
) {
  const plan = requireResourceEffectBranchAcquisitionPlan(rawPlan);
  requireBranchCommandSurface(history);
  let nativeParentBasis = plan.canonicalBasis;
  let dependencyBasisBranch = null;
  let effectBranch = null;

  try {
    if (plan.dependencyBasis.kind === "derivedDependencyBasis") {
      const basisFork = await history.fork_branch({
        name: `resource-dependency-basis:${plan.effectId}`,
        parentBranchId: plan.canonicalBasis.branchId,
        expectedParentBasis: plan.canonicalBasis,
      });
      dependencyBasisBranch = Object.freeze({
        branch: basisFork.branch,
        createdBasis: basisFork.createdBasis,
        appliedBasis: basisFork.createdBasis,
      });
      const basisApplied = await history.apply_transaction_to_branch({
        branchId: basisFork.branch.id,
        expectedBasis: basisFork.createdBasis,
        transactionOps: setSignals(authoredSignalIds, plan.dependencyBasis.value),
      });
      dependencyBasisBranch = Object.freeze({
        ...dependencyBasisBranch,
        appliedBasis: basisApplied.afterBasis,
      });
      nativeParentBasis = basisApplied.afterBasis;
    }

    const effectFork = await history.fork_branch({
      name: plan.effectBranchName,
      parentBranchId: nativeParentBasis.branchId,
      expectedParentBasis: nativeParentBasis,
    });
    effectBranch = Object.freeze({
      lifecycle: "Forked",
      branch: effectFork.branch,
      parentBasis: nativeParentBasis,
      createdBasis: effectFork.createdBasis,
      appliedBasis: effectFork.createdBasis,
    });
    const applied = await history.apply_transaction_to_branch({
      branchId: effectFork.branch.id,
      expectedBasis: effectFork.createdBasis,
      transactionOps: setSignals(authoredSignalIds, effectValue),
    });
    return Object.freeze({
      lifecycle: "Pending",
      transitions: Object.freeze(["Planned", "Forked", "Applied", "Pending"]),
      branch: Object.freeze({
        ...effectBranch,
        lifecycle: "Applied",
        appliedBasis: applied.afterBasis,
        transaction: applied,
      }),
      dependencyBasisBranch,
      dependencyProof: plan.dependencySet,
      nativeAncestryProof: Object.freeze({
        parentBranchId: nativeParentBasis.branchId,
        parentAuthoredStateDigest: nativeParentBasis.authoredStateDigest,
        authority: "nativeForkAncestry",
      }),
      semanticDependencyProof: Object.freeze({
        effectIds: plan.dependencySet.dependencyIds,
        authority: "resourceEffectDependencySet",
        proofDigest: plan.dependencySet.proofDigest,
      }),
    });
  } catch (error) {
    await retireFailedAcquisition(history, effectBranch, dependencyBasisBranch);
    throw error;
  }
}

async function retireFailedAcquisition(
  history,
  effectBranch,
  dependencyBasisBranch,
) {
  const retirements = [];
  if (effectBranch !== null) {
    retirements.push({
      branchId: effectBranch.branch.id,
      expectedBasis: effectBranch.appliedBasis,
      reason: "superseded",
    });
  }
  if (dependencyBasisBranch !== null) {
    retirements.push({
      branchId: dependencyBasisBranch.branch.id,
      expectedBasis: dependencyBasisBranch.appliedBasis,
      reason: "dependencyCancellation",
    });
  }
  if (retirements.length > 0) {
    await history.retire_branches({ retirements });
  }
}

async function retireAcquiredEffectBranches(history, acquisition, reason) {
  const retirements = acquiredEffectRetirementRequests(acquisition, reason);
  const receipt = await history.retire_branches({ retirements });
  return effectRetirementReceipt(receipt.retirements);
}

async function retireAcquiredEffectBranchBatch(history, entries) {
  const groups = entries.map(({ acquisition, reason }) =>
    acquiredEffectRetirementRequests(acquisition, reason));
  const receipt = await history.retire_branches({
    retirements: groups.flat(),
  });
  let offset = 0;
  return Object.freeze(groups.map((group) => {
    const settled = effectRetirementReceipt(
      receipt.retirements.slice(offset, offset + group.length),
    );
    offset += group.length;
    return settled;
  }));
}

function acquiredEffectRetirementRequests(acquisition, reason) {
  const retirements = [{
    branchId: acquisition.branch.branch.id,
    expectedBasis: acquisition.branch.appliedBasis,
    reason,
  }];
  if (acquisition.dependencyBasisBranch !== null) {
    retirements.push({
      branchId: acquisition.dependencyBasisBranch.branch.id,
      expectedBasis: acquisition.dependencyBasisBranch.appliedBasis,
      reason: "dependencyCancellation",
    });
  }
  return retirements;
}

function effectRetirementReceipt(retirements) {
  return Object.freeze({
    retiredEffect: retirements[0],
    retiredDependencyBasis: retirements[1] ?? null,
  });
}

function requireBranchCommandSurface(history) {
  for (const method of [
    "current_branch",
    "worker_branch_basis",
    "fork_branch",
    "apply_transaction_to_branch",
    "retire_branch",
    "retire_branches",
    "closeout_effect_branch",
  ]) {
    if (typeof history?.[method] !== "function") {
      const error = new TypeError(
        `resource effect branch acquisition unavailable: history.${method}(...) is required`,
      );
      error.name = "ResourceEffectBranchUnavailable";
      error.code = "workerBranchCommandUnavailable";
      throw error;
    }
  }
}

function setSignals(ids, value) {
  return ids.map((id) => Object.freeze({ kind: "set", id, value }));
}

export {
  executeResourceEffectBranchAcquisition,
  requireBranchCommandSurface,
  retireAcquiredEffectBranchBatch,
  retireAcquiredEffectBranches,
};

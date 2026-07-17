import { deepFreeze } from "../support/canonical.js";

export function createWorkerSignalProjectionDriver(runtime) {
  const bindingByTruthBranch = new Map();
  return deepFreeze({
    async initialize(plan) {
      const nativeBranch = await runtime.currentBranch();
      const branchId = normalizeBranchId(nativeBranch.id);
      const basis = await runtime.workerBranchBasis(branchId);
      const binding = { branchId, basis };
      bindingByTruthBranch.set(plan.branchId, binding);
      return publicBinding(binding);
    },
    async fork({ branch, parentBranch }) {
      const parent = requireBinding(bindingByTruthBranch, parentBranch.id);
      const fork = await runtime.forkBranch({
        name: `local-truth:${branch.name}`,
        parentBranchId: parent.branchId,
        expectedParentBasis: parent.basis,
      });
      const binding = { branchId: normalizeBranchId(fork.branch.id), basis: fork.createdBasis };
      bindingByTruthBranch.set(branch.id, binding);
      return publicBinding(binding);
    },
    async apply(plan) {
      return apply(runtime, bindingByTruthBranch, plan);
    },
    async destroy({ branchId }) {
      const binding = requireBinding(bindingByTruthBranch, branchId);
      await runtime.retireBranch({
        branchId: binding.branchId,
        expectedBasis: binding.basis,
        reason: "projectionRebuild",
      });
      bindingByTruthBranch.delete(branchId);
    },
    async rebuild(plan) {
      const existing = bindingByTruthBranch.get(plan.branchId);
      if (existing) {
        try {
          await runtime.retireBranch({
            branchId: existing.branchId,
            expectedBasis: existing.basis,
            reason: "projectionRebuild",
          });
        } catch {
          // A missing disposable projection is a rebuild input, not a truth failure.
        }
      }
      const parent = await runtime.currentBranch();
      const parentId = normalizeBranchId(parent.id);
      const parentBasis = await runtime.workerBranchBasis(parentId);
      const fork = await runtime.forkBranch({
        name: `local-truth:rebuild:${plan.branchId}`,
        parentBranchId: parentId,
        expectedParentBasis: parentBasis,
      });
      bindingByTruthBranch.set(plan.branchId, {
        branchId: normalizeBranchId(fork.branch.id),
        basis: fork.createdBasis,
      });
      return apply(runtime, bindingByTruthBranch, plan);
    },
  });
}

async function apply(runtime, bindings, plan) {
  const binding = requireBinding(bindings, plan.branchId);
  if (plan.updates.length === 0) {
    return publicBinding(binding);
  }
  const transactionOps = plan.updates.map((update) => ({
    kind: "set",
    id: update.signalId,
    value: update.value,
    aspects: update.aspects,
  }));
  // Targeting the active branch is a typed kernel denial (ActiveBranchTarget):
  // the ambient transaction path owns that branch. When a consumer has
  // switched onto this projection's native branch, honor that authority split
  // by applying ambiently, then re-reading the advanced basis.
  const active = await runtime.currentBranch();
  let nextBasis;
  if (normalizeBranchId(active.id) === binding.branchId) {
    await runtime.applyTransaction(transactionOps);
    nextBasis = await runtime.workerBranchBasis(binding.branchId);
  } else {
    const receipt = await runtime.applyTransactionToBranch({
      branchId: binding.branchId,
      expectedBasis: binding.basis,
      transactionOps,
    });
    nextBasis = receipt.afterBasis;
  }
  const next = { branchId: binding.branchId, basis: nextBasis };
  bindings.set(plan.branchId, next);
  return publicBinding(next);
}

function requireBinding(bindings, branchId) {
  const binding = bindings.get(branchId);
  if (!binding) {
    throw new Error(`worker Signal projection for local truth branch ${branchId} is unavailable`);
  }
  return binding;
}

function publicBinding(binding) {
  return deepFreeze({
    signalBranchId: Number(binding.branchId),
    signalBasisDigest: binding.basis.authoredStateDigest,
  });
}

function normalizeBranchId(value) {
  return typeof value === "bigint" ? value : BigInt(value);
}

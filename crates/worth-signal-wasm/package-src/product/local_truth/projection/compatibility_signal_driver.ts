import { RAW_SIGNALS } from "../../symbols.js";
import { deepFreeze } from "../support/canonical.js";

export function createCompatibilitySignalProjectionDriver(signals) {
  const history = signals.history();
  const rawSignals = signals[RAW_SIGNALS];
  const bindingByTruthBranch = new Map();

  return deepFreeze({
    async initialize(plan) {
      const nativeBranch = await history.current_branch();
      const basis = await history.worker_branch_basis(nativeBranch.id);
      const binding = { branchId: nativeBranch.id, basis };
      bindingByTruthBranch.set(plan.branchId, binding);
      return publicBinding(binding);
    },
    async fork({ branch, parentBranch }) {
      const parent = requireBinding(bindingByTruthBranch, parentBranch.id);
      const receipt = await history.fork_branch({
        name: `local-truth:${branch.name}`,
        parentBranchId: parent.branchId,
        expectedParentBasis: parent.basis,
      });
      const binding = deepFreeze({ branchId: receipt.branch.id, basis: receipt.createdBasis });
      bindingByTruthBranch.set(branch.id, binding);
      return publicBinding(binding);
    },
    async apply(plan) {
      return applyPlan(history, rawSignals, bindingByTruthBranch, plan);
    },
    async destroy({ branchId }) {
      const binding = requireBinding(bindingByTruthBranch, branchId);
      await history.retire_branch({
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
          await history.retire_branch({
            branchId: existing.branchId,
            expectedBasis: existing.basis,
            reason: "projectionRebuild",
          });
        } catch {
          // Reacquisition remains authoritative even when the disposable branch was already lost.
        }
      }
      const nativeParent = await history.current_branch();
      const parentBasis = await history.worker_branch_basis(nativeParent.id);
      const fork = await history.fork_branch({
        name: `local-truth:rebuild:${plan.branchId}`,
        parentBranchId: nativeParent.id,
        expectedParentBasis: parentBasis,
      });
      bindingByTruthBranch.set(plan.branchId, {
        branchId: fork.branch.id,
        basis: fork.createdBasis,
      });
      return applyPlan(history, rawSignals, bindingByTruthBranch, plan);
    },
  });
}

async function applyPlan(history, rawSignals, bindings, plan) {
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
  const active = await history.current_branch();
  let nextBasis;
  if (String(active.id) === String(binding.branchId)) {
    rawSignals.applyTransaction(transactionOps);
    nextBasis = await history.worker_branch_basis(binding.branchId);
  } else {
    const receipt = await history.apply_transaction_to_branch({
      branchId: binding.branchId,
      expectedBasis: binding.basis,
      transactionOps,
    });
    nextBasis = receipt.afterBasis;
  }
  const next = deepFreeze({ branchId: binding.branchId, basis: nextBasis });
  bindings.set(plan.branchId, next);
  return publicBinding(next);
}

function requireBinding(bindings, branchId) {
  const binding = bindings.get(branchId);
  if (!binding) {
    throw new Error(`Signal projection binding for local truth branch ${branchId} is unavailable`);
  }
  return binding;
}

function publicBinding(binding) {
  return deepFreeze({
    signalBranchId: binding.branchId,
    signalBasisDigest: binding.basis.authoredStateDigest,
  });
}

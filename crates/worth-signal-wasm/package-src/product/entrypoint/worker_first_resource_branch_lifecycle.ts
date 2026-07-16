export function createWorkerFirstResourceBranchLifecycle({
  ready,
  requireActive,
  settlePendingPublications,
  settlePendingMutations,
  bridge,
  refreshBranchCache,
}) {
  async function settle(operation) {
    requireActive(operation);
    await ready();
    await settlePendingPublications();
    await settlePendingMutations();
  }

  async function runMutation(method, request) {
    await settle(method);
    const receipt = await bridge[method](request);
    await refreshBranchCache();
    return receipt;
  }

  return Object.freeze({
    async basis(branchId) {
      await settle("workerBranchBasis");
      return bridge.workerBranchBasis(branchId);
    },
    fork: (request) => runMutation("forkBranch", request),
    applyTransaction: (request) => runMutation("applyTransactionToBranch", request),
    retire: (request) => runMutation("retireBranch", request),
    retireBatch: (request) => runMutation("retireBranches", request),
    closeoutEffect: (request) => runMutation("closeoutEffectBranch", request),
  });
}

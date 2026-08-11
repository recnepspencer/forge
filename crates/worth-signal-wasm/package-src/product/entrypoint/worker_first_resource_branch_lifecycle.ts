export function createWorkerFirstResourceBranchLifecycle({
  ready,
  requireActive,
  settlePendingPublications,
  settlePendingMutations,
  bridge,
  refreshBranchCache,
  readCurrentTipBranchId,
  markActiveTipCatalogChanged,
  readmitReadyAuthoredOntoActiveTip,
}) {
  async function settle(operation) {
    requireActive(operation);
    await ready();
    await settlePendingPublications();
    await settlePendingMutations();
  }

  async function runMutation(method, request) {
    await settle(method);
    const previousTipBranchId = readCurrentTipBranchId();
    const receipt = await bridge[method](request);
    await refreshBranchCache();
    // Authored catalogs track the active tip branch. Head advances on the same
    // branch keep published graphs; only a tip branch identity change orphans.
    const nextTipBranchId = readCurrentTipBranchId();
    if (previousTipBranchId !== nextTipBranchId) {
      markActiveTipCatalogChanged();
      await readmitReadyAuthoredOntoActiveTip();
    }
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

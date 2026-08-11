export function resolveRootBridgeOptions(options = {}) {
  const bridgeOptions = {};
  if (options.workerUrl !== undefined) {
    bridgeOptions.workerUrl = options.workerUrl;
  } else if (options.assets?.workerUrl != null) {
    bridgeOptions.workerUrl = options.assets.workerUrl;
  }
  if (options.assets?.wasmUrl != null) {
    bridgeOptions.wasmUrl = options.assets.wasmUrl;
  } else if (options.wasmUrl !== undefined) {
    bridgeOptions.wasmUrl = options.wasmUrl;
  }
  return bridgeOptions;
}

export async function refreshWorkerFirstRootBranchCache(bridge, assign) {
  let currentBranch = await bridge.currentBranch();
  if (currentBranch !== null && currentBranch.head_snapshot_id === null) {
    try {
      currentBranch = {
        ...currentBranch,
        head_snapshot_id: await bridge.branchSnapshotId(currentBranch.id),
      };
    } catch {
      // Branch snapshot id is best-effort cache enrichment.
    }
  }
  assign(currentBranch, await bridge.branches());
}

import { refreshWorkerFirstRootAfterHistoryMutation } from "./worker_first_root_history_lifecycle.js";
import { refreshWorkerFirstRootBranchCache } from "./worker_first_root_session_bridge_options.js";

export function invalidateWorkerFirstActiveImport(state, message) {
  if (state.activeImportController === null) {
    for (const dependent of state.activeImportDependents) {
      dependent.invalidate(message);
    }
    state.activeImportDependents.clear();
    return;
  }
  state.activeImportController.invalidate(message);
  for (const dependent of state.activeImportDependents) {
    dependent.invalidate(message);
  }
  state.activeImportController = null;
  state.activeImportDependents.clear();
  state.activeImportContext = null;
}

export function requireWorkerFirstRootActive(terminated, operation) {
  if (!terminated) {
    return;
  }
  throw new TypeError(
    `worker-first root session ${operation}() cannot be used after free()`,
  );
}

export function requireWorkerFirstControllerActive(controller, operation) {
  if (!controller.isInvalidated()) {
    return;
  }
  throw controller.invalidatedError(operation);
}

export async function bootstrapWorkerFirstRootBridge(deps) {
  await deps.bridge.bootstrapRecord();
  await deps.bridge.workerRuntimeShellLock();
  await deps.hostCapabilities.bootstrap();
  await deps.refreshBranchCache();
}

export async function refreshWorkerFirstRootSessionAfterHistoryMutation(deps, operation, activeImportContext) {
  await refreshWorkerFirstRootAfterHistoryMutation(deps, operation, activeImportContext);
}

export async function refreshWorkerFirstRootSessionBranchCache(bridge, assign) {
  await refreshWorkerFirstRootBranchCache(bridge, assign);
}

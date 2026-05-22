import { createWorkerFirstHostDependencyId } from "./sessions/support/authored/worker_first_host_dependency_records.js";
import { recordHostDependencyRefreshFailed } from "./worker_first_host_capability_events.js";

export function scheduleWorkerFirstHostDependencyRefresh(deps) {
  void refreshWorkerFirstHostDependencyOrRecord(deps);
}

export async function refreshWorkerFirstHostDependencyOrThrow(deps) {
  recordHostDependencyRefreshAttempt(deps);
  try {
    await refreshWorkerFirstHostDependency(deps);
  } catch (error) {
    recordHostDependencyRefreshFailed(
      deps.performanceSummary,
      deps.diagnosticsRecorder,
      deps.descriptor,
      deps.invalidationMode,
      error,
    );
    throw error;
  }
}

async function refreshWorkerFirstHostDependencyOrRecord(deps) {
  recordHostDependencyRefreshAttempt(deps);
  try {
    await refreshWorkerFirstHostDependency(deps);
  } catch (error) {
    recordHostDependencyRefreshFailed(
      deps.performanceSummary,
      deps.diagnosticsRecorder,
      deps.descriptor,
      deps.invalidationMode,
      error,
    );
  }
}

function refreshWorkerFirstHostDependency(deps) {
  return deps.rootSession.refreshHostCapabilityReadables([
    createWorkerFirstHostDependencyId(deps.descriptor),
  ]);
}

function recordHostDependencyRefreshAttempt(deps) {
  deps.performanceSummary.hostCapabilityDependencyRefreshCount =
    (deps.performanceSummary.hostCapabilityDependencyRefreshCount ?? 0) + 1;
}

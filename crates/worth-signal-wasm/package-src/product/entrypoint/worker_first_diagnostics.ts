const WORKER_FIRST_HOST_CAPABILITY_REPORT = Object.freeze({
  posture: "workerFirstUnavailable",
  reason: "workerFirstHostCapabilityEventReplayNotImplemented",
  message:
    "worker-first diagnostics host capability event replay is not implemented yet",
});

const WORKER_FIRST_HOST_CAPABILITY_EVENT = Object.freeze({
  kind: "unavailable",
  reason: WORKER_FIRST_HOST_CAPABILITY_REPORT.reason,
  message: WORKER_FIRST_HOST_CAPABILITY_REPORT.message,
});

export function createWorkerFirstDiagnosticsFacade(workerFirstSession) {
  return Object.freeze({
    why(id) {
      return workerFirstSession.bridge.why(id);
    },
    health() {
      return workerFirstSession.bridge.health();
    },
    async summaryNow() {
      return (await workerFirstSession.bridge.readDiagnosticsSummary()).summary;
    },
    async historyNow() {
      return (await workerFirstSession.bridge.readDiagnosticsHistory()).history;
    },
    latestFlow() {
      return workerFirstSession.bridge.latestFlow();
    },
    latestObservation() {
      return workerFirstSession.bridge.latestObservation();
    },
    latestHostCapabilityEvent() {
      return WORKER_FIRST_HOST_CAPABILITY_EVENT;
    },
    recentHostCapabilityEvents() {
      return Object.freeze([WORKER_FIRST_HOST_CAPABILITY_EVENT]);
    },
    performanceSummary() {
      return workerFirstSession.bridge.performanceSummary();
    },
    async hostCapabilityReport() {
      return WORKER_FIRST_HOST_CAPABILITY_REPORT;
    },
    latestFailure() {
      return workerFirstSession.bridge.latestFailure();
    },
    latestRollback() {
      return workerFirstSession.bridge.latestRollback();
    },
    latestInvalidationPlanningEstimate() {
      return workerFirstSession.bridge.latestInvalidationPlanningEstimate();
    },
    latestInvalidationTraceRecords() {
      return workerFirstSession.bridge.latestInvalidationTraceRecords();
    },
    recentHistory() {
      return workerFirstSession.bridge.recentHistory();
    },
  });
}

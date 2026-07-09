import { buildHostCapabilityDiagnosticsReport } from "./host_capability_reports.js";

function requireDiagnosticsMethod(rawDiagnostics, methodName) {
  const method = rawDiagnostics?.[methodName];
  if (typeof method !== "function") {
    throw new TypeError(`signals.diagnostics() is missing required method ${methodName}()`);
  }
  return method.bind(rawDiagnostics);
}

export function wrapDiagnostics(rawDiagnostics, hostCapabilities) {
  const free = typeof rawDiagnostics?.free === "function"
    ? rawDiagnostics.free.bind(rawDiagnostics)
    : () => {};
  const dispose = typeof rawDiagnostics?.[Symbol.dispose] === "function"
    ? rawDiagnostics[Symbol.dispose].bind(rawDiagnostics)
    : null;

  return {
    subscribe: requireDiagnosticsMethod(rawDiagnostics, "subscribe"),
    why: requireDiagnosticsMethod(rawDiagnostics, "why"),
    health: requireDiagnosticsMethod(rawDiagnostics, "health"),
    summaryNow: requireDiagnosticsMethod(rawDiagnostics, "summaryNow"),
    historyNow: requireDiagnosticsMethod(rawDiagnostics, "historyNow"),
    latestFlow: requireDiagnosticsMethod(rawDiagnostics, "latestFlow"),
    latestObservation: requireDiagnosticsMethod(rawDiagnostics, "latestObservation"),
    latestHostCapabilityEvent() {
      return hostCapabilities.latestDiagnosticsEvent();
    },
    recentHostCapabilityEvents() {
      return hostCapabilities.recentDiagnosticsEvents();
    },
    performanceSummary() {
      const summary = requireDiagnosticsMethod(rawDiagnostics, "performanceSummary")();
      return {
        ...summary,
        ...hostCapabilities.performanceSummary(),
      };
    },
    hostCapabilityReport() {
      const summary = this.performanceSummary();
      const recentEvents = hostCapabilities.recentDiagnosticsEvents();
      return buildHostCapabilityDiagnosticsReport(summary, recentEvents);
    },
    latestFailure: requireDiagnosticsMethod(rawDiagnostics, "latestFailure"),
    latestRollback: requireDiagnosticsMethod(rawDiagnostics, "latestRollback"),
    latestFrontierExecution: requireDiagnosticsMethod(rawDiagnostics, "latestFrontierExecution"),
    latestInvalidationTraceRecords: requireDiagnosticsMethod(rawDiagnostics, "latestInvalidationTraceRecords"),
    recentHistory: requireDiagnosticsMethod(rawDiagnostics, "recentHistory"),
    free,
    [Symbol.dispose]() {
      if (dispose) {
        dispose();
        return;
      }
      free();
    },
  };
}

import type {
  DiagnosticsExecutionHistorySummary,
  DiagnosticsFailureSummary,
  DiagnosticsFlowSummary,
  DiagnosticsFrontierExecutionSummary,
  DiagnosticsGraphSummary,
  DiagnosticsInvalidationTraceRecord,
  DiagnosticsRollbackDiagnostic,
  HealthSummary,
  WhySummary,
} from "../types.d.ts";

export class SignalDiagnostics {
  inner: any;

  constructor(inner: any) {
    this.inner = inner;
  }

  why(id: string): WhySummary {
    return this.inner.why(id) as WhySummary;
  }

  health(): HealthSummary {
    return this.inner.health() as HealthSummary;
  }

  summaryNow(): DiagnosticsGraphSummary {
    return this.inner.summary_now() as DiagnosticsGraphSummary;
  }

  historyNow(): DiagnosticsExecutionHistorySummary {
    return this.inner.history_now() as DiagnosticsExecutionHistorySummary;
  }

  latestFlow(): DiagnosticsFlowSummary | null {
    return this.inner.latest_flow() as DiagnosticsFlowSummary | null;
  }

  latestFailure(): DiagnosticsFailureSummary | null {
    return this.inner.latest_failure() as DiagnosticsFailureSummary | null;
  }

  latestRollback(): DiagnosticsRollbackDiagnostic | null {
    return this.inner.latest_rollback() as DiagnosticsRollbackDiagnostic | null;
  }

  latestFrontierExecution(): DiagnosticsFrontierExecutionSummary | null {
    return this.inner.latest_frontier_execution() as DiagnosticsFrontierExecutionSummary | null;
  }

  latestInvalidationTraceRecords(): DiagnosticsInvalidationTraceRecord[] {
    return this.inner.latest_invalidation_trace_records() as DiagnosticsInvalidationTraceRecord[];
  }

  recentHistory(): DiagnosticsExecutionHistorySummary[] {
    return this.inner.recent_history() as DiagnosticsExecutionHistorySummary[];
  }
}

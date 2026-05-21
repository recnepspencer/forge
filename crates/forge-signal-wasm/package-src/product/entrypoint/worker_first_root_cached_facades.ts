import { freezeObject } from "../graph_support.js";
import { buildHostCapabilityTransportReport } from "../host_capability_reports.js";

export function createRootDiagnosticsFacade(rootSession) {
  return freezeObject({
    why(id) {
      const context = rootSession.currentImportContext();
      if (!context.whyById.has(id)) {
        throw new TypeError(
          `worker-first root diagnostics().why(${JSON.stringify(id)}) requires an id from the active imported graph`,
        );
      }
      return context.whyById.get(id);
    },
    health() {
      return rootSession.currentImportContext().health;
    },
    summaryNow() {
      return rootSession.currentImportContext().diagnosticsSummary;
    },
    historyNow() {
      return rootSession.currentImportContext().diagnosticsHistory;
    },
    latestFlow() {
      return rootSession.currentImportContext().latestFlow;
    },
    latestObservation() {
      return rootSession.currentImportContext().latestObservation;
    },
    latestHostCapabilityEvent() {
      return rootSession.latestHostCapabilityEvent();
    },
    recentHostCapabilityEvents() {
      return rootSession.recentHostCapabilityEvents();
    },
    performanceSummary() {
      return rootSession.currentImportContext().performanceSummary;
    },
    latestFailure() {
      return rootSession.currentImportContext().latestFailure;
    },
    latestRollback() {
      return rootSession.currentImportContext().latestRollback;
    },
    latestFrontierExecution() {
      return rootSession.currentImportContext().latestFrontierExecution;
    },
    latestInvalidationTraceRecords() {
      return rootSession.currentImportContext().latestInvalidationTraceRecords;
    },
    recentHistory() {
      return rootSession.currentImportContext().recentHistory;
    },
    hostCapabilityReport() {
      return rootSession.hostCapabilityReport();
    },
  });
}

export function createRootAdaptersFacade(rootSession) {
  return freezeObject({
    exportDefinitions() {
      return rootSession.currentImportContext().runtimeDefinitionEnvelope;
    },
    exportRuntimeEnvelope() {
      return rootSession.currentImportContext().runtimeEnvelopeArtifact;
    },
    async replaceRuntimeEnvelope(envelope) {
      return rootSession.replaceRuntimeEnvelope(envelope);
    },
    async restoreExactRuntimeEnvelope(envelope) {
      return rootSession.restoreExactRuntimeEnvelope(envelope);
    },
    runtimeProofReport() {
      return rootSession.currentImportContext().runtimeProofReport;
    },
    hostCapabilityTransportReport(envelope) {
      const definitions =
        envelope?.definitions ?? rootSession.currentImportContext().runtimeDefinitionEnvelope;
      return buildHostCapabilityTransportReport(definitions?.unavailableCallbacks);
    },
    free() {},
    [Symbol.dispose]() {},
  });
}

export function createRootSpecialistFacade(rootSession) {
  return freezeObject({
    evaluateDirty() {
      return rootSession.evaluateDirty();
    },
    evaluate_dirty() {
      return rootSession.evaluateDirty();
    },
    graphSummary() {
      return rootSession.currentImportContext().diagnosticsSummary;
    },
    graph_summary() {
      return rootSession.currentImportContext().diagnosticsSummary;
    },
    readVersions(ids) {
      return readVersionSummaries(rootSession, ids, "specialist.readVersions");
    },
    read_versions(ids) {
      return readVersionSummaries(rootSession, ids, "specialist.read_versions");
    },
    free() {},
    [Symbol.dispose]() {},
  });
}

export function readRootSignalValue(rootSession, target) {
  const id = normalizeRootReadableTargetId(target);
  return rootSession.readSignalValue(id);
}

function normalizeRootReadableTargetId(target) {
  if (typeof target === "string" && target.length > 0) {
    return target;
  }
  if (
    target &&
    typeof target === "object" &&
    typeof target.id === "string" &&
    target.id.length > 0
  ) {
    return target.id;
  }
  if (
    typeof target === "function" &&
    typeof target.id === "string" &&
    target.id.length > 0
  ) {
    return target.id;
  }
  throw new TypeError(
    "worker-first root read(...) requires a worker-first signal handle or canonical signal id",
  );
}

function readVersionSummaries(rootSession, ids, operation) {
  if (!Array.isArray(ids)) {
    throw new TypeError(`${operation}(...) requires an array of signal ids`);
  }
  const context = rootSession.currentImportContext();
  return ids.map((id) => {
    if (typeof id !== "string" || !context.versionById.has(id)) {
      throw new TypeError(
        `${operation}(...) requires signal ids from the active imported graph`,
      );
    }
    return context.versionById.get(id);
  });
}

function createAvailableHistoryArtifact() {
  return Object.freeze({
    kind: "available",
  });
}

function createDeferredReactiveHistoryAvailability() {
  const detail =
    "resource line history explainability is deferred for reactive summary signals; call line.summary() or line.diagnosticsSummary() for direct history availability reads";
  return Object.freeze({
    replay: createUnavailableHistoryArtifact("unsupportedByRuntime", detail),
    replayExact: createUnavailableReplayAvailability("unsupportedByRuntime", detail),
    lineage: createUnavailableHistoryArtifact("unsupportedByRuntime", detail),
    branch: createUnavailableHistoryArtifact("unsupportedByRuntime", detail),
    restoreExact: createUnavailableRestoreAvailability("unsupportedByRuntime", detail),
  });
}

function readHistoryRuntimeErrorDetail(prefix, error) {
  const message =
    error instanceof Error
      ? error.message
      : typeof error === "string"
        ? error
        : "unknown runtime error";
  return `${prefix}: ${message}`;
}

function createUnavailableHistoryArtifact(reason, detail) {
  return Object.freeze({
    kind: "unavailable",
    reason,
    detail,
  });
}

function createAvailableReplayAvailability(mode, signalId) {
  return Object.freeze({
    kind: "available",
    mode,
    signalId,
  });
}

function createUnavailableReplayAvailability(reason, detail) {
  return Object.freeze({
    kind: "unavailable",
    reason,
    detail,
  });
}

function createAvailableRestoreAvailability(mode, branchId, snapshotId) {
  return Object.freeze({
    kind: "available",
    mode,
    branchId,
    snapshotId,
  });
}

function createUnavailableRestoreAvailability(reason, detail) {
  return Object.freeze({
    kind: "unavailable",
    reason,
    detail,
  });
}

export {
  createAvailableHistoryArtifact,
  createDeferredReactiveHistoryAvailability,
  createAvailableReplayAvailability,
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableReplayAvailability,
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
};

function createAvailableHistoryArtifact() {
  return Object.freeze({
    kind: "available",
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
  createAvailableReplayAvailability,
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableReplayAvailability,
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
};

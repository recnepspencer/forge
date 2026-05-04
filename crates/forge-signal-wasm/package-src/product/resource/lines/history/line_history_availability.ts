function createAvailableHistoryArtifact() {
  return Object.freeze({
    kind: "available",
  });
}

function createUnavailableHistoryArtifact(reason, detail) {
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
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableRestoreAvailability,
};

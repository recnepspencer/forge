import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

const ROUTE_RESTORE_BOUNDARY = Symbol("worth.router.restore-boundary");

function createRouteRestoreBoundary(snapshotEnvelope) {
  const normalizedSnapshotEnvelope = requireSnapshotEnvelopeArtifact(
    snapshotEnvelope,
    "signals.router.restoreBoundary(...)",
  );
  const guarantees = Object.freeze({
    routeTruth: "restoredExactGraphTruth",
    outletComposition: "restoredAdmittedOutletComposition",
    graphOwnedState: "restoredWithinSnapshotBoundary",
  });
  const verification = Object.freeze({
    routeRestoreBoundaryDigest: createCanonicalDigest("route-restore-boundary", {
      restoreMode: normalizedSnapshotEnvelope.snapshotEnvelopeRestoreMode,
      restoreToken: normalizedSnapshotEnvelope.snapshotEnvelopeRestoreToken,
      portableWire: normalizedSnapshotEnvelope.snapshotEnvelopePortableWire,
      snapshotId: normalizedSnapshotEnvelope.snapshot?.meta?.snapshot_id ?? null,
      guarantees,
    }),
  });
  return Object.freeze({
    [ROUTE_RESTORE_BOUNDARY]: true,
    kind: "routeRestoreBoundary",
    snapshotEnvelope() {
      return normalizedSnapshotEnvelope;
    },
    guarantees() {
      return guarantees;
    },
    verification() {
      return verification;
    },
  });
}

function isRouteRestoreBoundary(value) {
  return Boolean(value && value[ROUTE_RESTORE_BOUNDARY] === true);
}

function normalizeOptionalRouteRestoreBoundary(value, operation) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireRouteRestoreBoundary(value, operation);
}

function requireRouteRestoreBoundary(value, operation) {
  if (isRouteRestoreBoundary(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires a restore boundary created by signals.router.restoreBoundary(...)`,
  );
}

function restoreRouteHistoryBoundary(history, restoreBoundary, target) {
  const normalizedHistory = requireRouteRestoreHistory(
    history,
    `${target.restoreSourceKind}.restore(...)`,
  );
  const restoreResult = normalizedHistory.restore_exact_snapshot(
    restoreBoundary.snapshotEnvelope(),
  );
  if (isPromiseLike(restoreResult)) {
    return Promise.resolve(restoreResult).then(() =>
      createRouteHistoryRestoreResult(target, restoreBoundary)
    );
  }
  return createRouteHistoryRestoreResult(target, restoreBoundary);
}

function createRouteHistoryRestoreResult(target, restoreBoundary) {
  const verification = Object.freeze({
    routeHistoryRestoreDigest: createCanonicalDigest("route-history-restore", {
      restoreSourceKind: target.restoreSourceKind,
      routeId: target.routeId,
      href: target.href,
      restoredEntryDigest: target.restoredEntryDigest,
      restoreBoundaryDigest: restoreBoundary.verification().routeRestoreBoundaryDigest,
    }),
  });
  return Object.freeze({
    kind: "routeHistoryRestoreResult",
    restoreSourceKind: target.restoreSourceKind,
    routeId: target.routeId,
    href: target.href,
    restoredEntryDigest: target.restoredEntryDigest,
    restoreBoundary,
    verification() {
      return verification;
    },
  });
}

function createRouteHistoryReplayResult(history, target) {
  const normalizedHistory = requireRouteReplayHistory(
    history,
    `${target.replaySourceKind}.replay(...)`,
  );
  if (
    target.runtimeRouteSourceId === null
    && target.runtimeContinuitySourceId === null
  ) {
    throw new TypeError(
      `${target.replaySourceKind}.replay(...) requires replay source ids on the recorded route history artifact`,
    );
  }
  const routeReplay = target.runtimeRouteSourceId === null
    ? null
    : normalizedHistory.replay_for(target.runtimeRouteSourceId);
  const continuityReplay = target.runtimeContinuitySourceId === null
    ? null
    : normalizedHistory.replay_for(target.runtimeContinuitySourceId);
  const verification = Object.freeze({
    routeHistoryReplayDigest: createCanonicalDigest("route-history-replay", {
      replaySourceKind: target.replaySourceKind,
      routeId: target.routeId,
      href: target.href,
      replayedEntryDigest: target.replayedEntryDigest,
      runtimeRouteSourceId: target.runtimeRouteSourceId,
      runtimeContinuitySourceId: target.runtimeContinuitySourceId,
      routeReplayFrameKinds: routeReplay?.frames.map((frame) => frame.kind) ?? [],
      continuityReplayFrameKinds:
        continuityReplay?.frames.map((frame) => frame.kind) ?? [],
    }),
  });
  return Object.freeze({
    kind: "routeHistoryReplayResult",
    replaySourceKind: target.replaySourceKind,
    routeId: target.routeId,
    href: target.href,
    replayedEntryDigest: target.replayedEntryDigest,
    routeReplay,
    continuityReplay,
    verification() {
      return verification;
    },
  });
}

function requireSnapshotEnvelopeArtifact(value, operation) {
  if (
    value
    && typeof value === "object"
    && typeof value.snapshotEnvelopeRestoreToken === "string"
    && typeof value.snapshotEnvelopePortableWire === "string"
    && value.snapshotEnvelopeRestoreMode === "SameRuntimeExact"
  ) {
    return value;
  }
  throw new TypeError(
    `${operation} requires an artifact returned by signals.history().snapshot() or signals.history().branch_snapshot_envelope(...)`,
  );
}

function requireRouteRestoreHistory(value, operation) {
  if (value && typeof value.restore_exact_snapshot === "function") {
    return value;
  }
  throw new TypeError(
    `${operation} requires a history facade with restore_exact_snapshot(...)`,
  );
}

function requireRouteReplayHistory(value, operation) {
  if (value && typeof value.replay_for === "function") {
    return value;
  }
  throw new TypeError(
    `${operation} requires a history facade with replay_for(...)`,
  );
}

function isPromiseLike(value) {
  return value !== null
    && (typeof value === "object" || typeof value === "function")
    && typeof value.then === "function";
}

export {
  createRouteRestoreBoundary,
  createRouteHistoryReplayResult,
  isRouteRestoreBoundary,
  normalizeOptionalRouteRestoreBoundary,
  restoreRouteHistoryBoundary,
};

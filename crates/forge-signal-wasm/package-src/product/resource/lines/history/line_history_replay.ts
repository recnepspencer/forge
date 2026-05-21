import {
  createUnavailableReplayAvailability,
  readHistoryRuntimeErrorDetail,
} from "./line_history_availability.js";
import { executeLineReload } from "../actions/line_reload_execution.js";

function executeLineHistoryExactReplay(materialization, historyRead) {
  const availability = historyRead.availability.replayExact;
  const basis = historyRead.basis;
  if (availability.kind !== "available") {
    return Object.freeze({
      kind: "unavailable",
      reason: availability.reason,
      detail: availability.detail,
      basisCurrentId: basis.currentBasisId,
      basisAdvanceCount: basis.advanceCount,
    });
  }
  try {
    const history = materialization.history;
    if (typeof history.replay_signal_by_id !== "function") {
      const unavailable = createUnavailableReplayAvailability(
        "unsupportedByRuntime",
        "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      );
      return Object.freeze({
        kind: "unavailable",
        reason: unavailable.reason,
        detail: unavailable.detail,
        basisCurrentId: basis.currentBasisId,
        basisAdvanceCount: basis.advanceCount,
      });
    }
    const replayResult = history.replay_signal_by_id(availability.signalId);
    if (isPromiseLike(replayResult)) {
      return replayResult.then(
        () => createReplayResult(materialization, availability, basis),
        (error) => createReplayUnavailableResult(
          "resource line exact replay is unavailable because replay execution failed",
          error,
          basis,
        ),
      );
    }
  } catch (error) {
    return createReplayUnavailableResult(
      "resource line exact replay is unavailable because replay execution failed",
      error,
      basis,
    );
  }
  return createReplayResult(materialization, availability, basis);
}

function createReplayResult(materialization, availability, basis) {
  const reloadStatus = executeLineReload(
    materialization,
    "replay",
    "replayed",
  );
  return Object.freeze({
    kind: "replayed",
    mode: availability.mode,
    signalId: availability.signalId,
    basisCurrentId: basis.currentBasisId,
    basisAdvanceCount: basis.advanceCount,
    reloadStatus,
  });
}

function createReplayUnavailableResult(detailPrefix, error, basis) {
  const unavailable = createUnavailableReplayAvailability(
    "runtimeRejected",
    readHistoryRuntimeErrorDetail(detailPrefix, error),
  );
  return Object.freeze({
    kind: "unavailable",
    reason: unavailable.reason,
    detail: unavailable.detail,
    basisCurrentId: basis.currentBasisId,
    basisAdvanceCount: basis.advanceCount,
  });
}

function isPromiseLike(value) {
  return value !== null && typeof value === "object" && typeof value.then === "function";
}

export { executeLineHistoryExactReplay };

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
    history.replay_signal_by_id(availability.signalId);
  } catch (error) {
    const unavailable = createUnavailableReplayAvailability(
      "runtimeRejected",
      readHistoryRuntimeErrorDetail(
        "resource line exact replay is unavailable because replay execution failed",
        error,
      ),
    );
    return Object.freeze({
      kind: "unavailable",
      reason: unavailable.reason,
      detail: unavailable.detail,
      basisCurrentId: basis.currentBasisId,
      basisAdvanceCount: basis.advanceCount,
    });
  }
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

export { executeLineHistoryExactReplay };

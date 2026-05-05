import {
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
} from "./line_history_availability.js";
import { executeLineReload } from "../actions/line_reload_execution.js";

function executeLineHistoryExactRestore(materialization, historyRead) {
  const availability = historyRead.availability.restoreExact;
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
    if (typeof history.restore_branch_snapshot_by_id === "function") {
      history.restore_branch_snapshot_by_id(
        BigInt(availability.branchId),
        BigInt(availability.snapshotId),
      );
    } else if (
      typeof history.restore_exact_branch_snapshot === "function"
      && typeof history.branch_snapshot === "function"
    ) {
      const snapshot = history.branch_snapshot(BigInt(availability.branchId));
      history.restore_exact_branch_snapshot(
        BigInt(availability.branchId),
        snapshot,
      );
    } else {
      const unavailable = createUnavailableRestoreAvailability(
        "unsupportedByRuntime",
        "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
      );
      return Object.freeze({
        kind: "unavailable",
        reason: unavailable.reason,
        detail: unavailable.detail,
        basisCurrentId: basis.currentBasisId,
        basisAdvanceCount: basis.advanceCount,
      });
    }
  } catch (error) {
    const unavailable = createUnavailableRestoreAvailability(
      "runtimeRejected",
      readHistoryRuntimeErrorDetail(
        "resource line exact branch restore is unavailable because restore execution failed",
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
    "restore",
    "restored",
  );
  return Object.freeze({
    kind: "restored",
    mode: availability.mode,
    branchId: availability.branchId,
    snapshotId: availability.snapshotId,
    basisCurrentId: basis.currentBasisId,
    basisAdvanceCount: basis.advanceCount,
    reloadStatus,
  });
}

export { executeLineHistoryExactRestore };

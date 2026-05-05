import {
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
} from "./line_history_availability.js";
import { recordLineHistoryEntry } from "./record_line_history_entry.js";
import {
  createPendingReloadDiagnostics,
  createReloadFulfilledDiagnostics,
  createReloadRejectedDiagnostics,
  createTimedOutReloadDiagnostics,
} from "../state/line_diagnostics_value.js";
import {
  createFulfilledLineStatus,
  createPendingLineStatus,
  createRejectedLineStatus,
  createTimedOutLineStatus,
} from "../state/line_status_value.js";
import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";

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
  const fallbackRequestDescriptor = readRequestDescriptorFallback(materialization);
  const fallbackPreviousValue = readPreviousValueFallback(materialization);
  const previousDiagnostics = readPreviousDiagnosticsFallback(materialization);
  const previousLifecycleEntries = materialization.lifecycleHistory.entries();
  materialization.resourceLineEpoch.captureAll();
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
  const restoredRequestDescriptor =
    readRequestDescriptorFallback(materialization) ?? fallbackRequestDescriptor;
  if (restoredRequestDescriptor === null) {
    const unavailable = createUnavailableRestoreAvailability(
      "runtimeRejected",
      "resource line exact branch restore is unavailable because request descriptor continuity could not be re-read after restore",
    );
    return Object.freeze({
      kind: "unavailable",
      reason: unavailable.reason,
      detail: unavailable.detail,
      basisCurrentId: basis.currentBasisId,
      basisAdvanceCount: basis.advanceCount,
    });
  }
  const restoredMaterialization = materialization.rematerialize({
    requestDescriptorOverride: restoredRequestDescriptor,
    invalidateNamespace: true,
  });
  const reloadStatus = adoptRestoredMaterialization(
    previousDiagnostics,
    previousLifecycleEntries,
    restoredMaterialization,
    restoredRequestDescriptor,
    fallbackPreviousValue,
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

function readPreviousDiagnosticsFallback(materialization) {
  try {
    return materialization.binding.diagnosticsSignal();
  } catch {
    return null;
  }
}

function adoptRestoredMaterialization(
  previousDiagnostics,
  previousLifecycleEntries,
  restoredMaterialization,
  restoredRequestDescriptor,
  previousValue,
) {
  const effectivePreviousDiagnostics =
    previousDiagnostics ?? restoredMaterialization.binding.diagnosticsSignal();
  cloneLifecycleHistory(
    previousLifecycleEntries,
    restoredMaterialization.lifecycleHistory,
  );
  const status = restoredMaterialization.binding.statusSignal();
  if (status.kind === "fulfilled") {
    const diagnostics = createRestoreReloadDiagnostics(
      createReloadFulfilledDiagnostics(
        effectivePreviousDiagnostics,
        "restore",
        restoredMaterialization.binding.processingSignal(),
        restoredMaterialization.binding.uploadSignal(),
        restoredMaterialization.binding.downloadSignal(),
        !areLineValuesSemanticallyEqual(
          restoredMaterialization.binding.valueSignal(),
          previousValue,
        ),
        0,
      ),
      restoredRequestDescriptor,
    );
    restoredMaterialization.binding.statusSignal.set(
      createFulfilledLineStatus("restore"),
    );
    restoredMaterialization.binding.diagnosticsSignal.set(diagnostics);
    recordLineHistoryEntry(
      restoredMaterialization.lifecycleHistory,
      restoredMaterialization.binding,
      "restored",
    );
    return restoredMaterialization.binding.statusSignal();
  }
  if (status.kind === "pending") {
    restoredMaterialization.binding.statusSignal.set(
      createPendingLineStatus("restore", previousValue !== null),
    );
    restoredMaterialization.binding.diagnosticsSignal.set(
      createRestoreReloadDiagnostics(
        createPendingReloadDiagnostics(
          effectivePreviousDiagnostics,
          "restore",
          null,
        ),
        restoredRequestDescriptor,
      ),
    );
    recordLineHistoryEntry(
      restoredMaterialization.lifecycleHistory,
      restoredMaterialization.binding,
      "pending",
    );
    return restoredMaterialization.binding.statusSignal();
  }
  if (status.kind === "timedOut") {
    restoredMaterialization.binding.statusSignal.set(
      createTimedOutLineStatus("restore", previousValue !== null),
    );
    restoredMaterialization.binding.diagnosticsSignal.set(
      createRestoreReloadDiagnostics(
        createTimedOutReloadDiagnostics(
          effectivePreviousDiagnostics,
          "restore",
          0,
        ),
        restoredRequestDescriptor,
      ),
    );
    recordLineHistoryEntry(
      restoredMaterialization.lifecycleHistory,
      restoredMaterialization.binding,
      "timedOut",
    );
    return restoredMaterialization.binding.statusSignal();
  }
  restoredMaterialization.binding.statusSignal.set(
    createRejectedLineStatus(
      "restore",
      status.message,
      previousValue !== null,
    ),
  );
  restoredMaterialization.binding.diagnosticsSignal.set(
    createRestoreReloadDiagnostics(
      createReloadRejectedDiagnostics(
        effectivePreviousDiagnostics,
        "restore",
        status.message,
        0,
      ),
      restoredRequestDescriptor,
    ),
  );
  recordLineHistoryEntry(
    restoredMaterialization.lifecycleHistory,
    restoredMaterialization.binding,
    "rejected",
  );
  return restoredMaterialization.binding.statusSignal();
}

function cloneLifecycleHistory(entries, lifecycleHistory) {
  if (lifecycleHistory.entries().length !== 0) {
    return;
  }
  for (const entry of entries) {
    lifecycleHistory.append({ ...entry, sequence: 0 });
  }
}

function createRestoreReloadDiagnostics(nextDiagnostics, requestDescriptor) {
  return Object.freeze({
    ...nextDiagnostics,
    request: Object.freeze({
      auth: requestDescriptor.auth,
      context: Object.freeze({
        headerNames: Object.freeze(
          Object.keys(requestDescriptor.context.headers).sort(),
        ),
        correlationId: requestDescriptor.context.correlationId,
        branchId: requestDescriptor.context.branchId,
        basisId: requestDescriptor.context.basisId,
      }),
      continuation: requestDescriptor.continuation,
      processingJob: requestDescriptor.processingJob,
      uploadTransport: requestDescriptor.uploadTransport,
    }),
    basis: Object.freeze({
      ...nextDiagnostics.basis,
      currentBasisId: requestDescriptor.context.basisId,
    }),
  });
}

function readRequestDescriptorFallback(materialization) {
  try {
    return materialization.requestState.readDescriptor();
  } catch {
    return null;
  }
}

function readPreviousValueFallback(materialization) {
  try {
    return materialization.binding.valueSignal();
  } catch {
    return null;
  }
}

export { executeLineHistoryExactRestore };

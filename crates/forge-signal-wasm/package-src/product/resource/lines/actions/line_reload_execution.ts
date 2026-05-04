import { bindReloadLineValue } from "../../lowering/runtime_line_binding.js";
import {
  createFreshnessFromPolicy,
  createPendingFreshness,
  createRejectedFreshness,
  createTimedOutFreshness,
} from "../state/line_freshness_value.js";
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
import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";

function executeLineReload(materialization, operation) {
  const reload = materialization.reload;
  const previousValue = materialization.binding.valueSignal();
  const supersededOperation = materialization.lifecycle.supersedePendingReload();
  if (supersededOperation !== null) {
    recordLineHistoryEntry(
      materialization.lifecycleHistory,
      materialization.binding,
      "superseded",
      { supersededOperation },
    );
  }
  try {
    const bindingResult = bindReloadLineValue(
      reload.load,
      reload.params,
      reload.familyKind,
      reload.requestDescriptor,
      previousValue,
      reload.policy.retryLimit,
    );
    if (bindingResult.kind === "pending") {
      const pendingToken = materialization.lifecycle.beginPendingReload(
        operation,
      );
      const timeoutMs = reload.policy.timeoutMs;
      const hasVisibleValue = previousValue !== null;
      const status = createPendingLineStatus(operation, hasVisibleValue);
      const freshness = createPendingFreshness(operation);
      const diagnostics = createPendingReloadDiagnostics(
        materialization.binding.diagnosticsSignal(),
        operation,
        supersededOperation,
      );
      materialization.binding.statusSignal.set(status);
      materialization.binding.freshnessSignal.set(freshness);
      materialization.binding.diagnosticsSignal.set(diagnostics);
      recordLineHistoryEntry(
        materialization.lifecycleHistory,
        materialization.binding,
        "pending",
      );
      let timedOut = false;
      if (typeof timeoutMs === "number" && timeoutMs >= 0) {
        setTimeout(() => {
          if (!materialization.lifecycle.completePendingReload(pendingToken)) {
            return;
          }
          timedOut = true;
          applyTimedOutReload(
            materialization,
            operation,
            bindingResult.retryTracker.count(),
          );
        }, timeoutMs);
      }
      bindingResult.promise.then(
        (loaded) => {
          if (timedOut) {
            return;
          }
          if (!materialization.lifecycle.completePendingReload(pendingToken)) {
            return;
          }
          applyFulfilledReload(
            materialization,
            reload,
            operation,
            previousValue,
            loaded.loaded,
            loaded.retryAttempts,
          );
        },
        (error) => {
          if (timedOut) {
            return;
          }
          if (!materialization.lifecycle.completePendingReload(pendingToken)) {
            return;
          }
          applyRejectedReload(materialization, operation, error);
        },
      );
      return status;
    }
    return applyFulfilledReload(
      materialization,
      reload,
      operation,
      previousValue,
      bindingResult.loaded,
      bindingResult.retryAttempts,
    );
  } catch (error) {
    return applyRejectedReload(materialization, operation, error);
  }
}

function applyFulfilledReload(
  materialization,
  reload,
  operation,
  previousValue,
  loaded,
  retryAttempts = 0,
) {
  const visibleValueChanged = loaded.hasVisibleValue && loaded.value !== previousValue;
  materialization.binding.valueSignal.set(loaded.value);
  materialization.binding.processingSignal.set(loaded.processing);
  materialization.binding.uploadSignal.set(loaded.upload);
  const status = createFulfilledLineStatus(operation);
  const freshness = createFreshnessFromPolicy(reload.policy);
  const diagnostics = createReloadFulfilledDiagnostics(
    materialization.binding.diagnosticsSignal(),
    operation,
    loaded.processing,
    loaded.upload,
    visibleValueChanged,
    retryAttempts,
  );
  materialization.binding.statusSignal.set(status);
  materialization.binding.freshnessSignal.set(freshness);
  materialization.binding.diagnosticsSignal.set(diagnostics);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "fulfilled",
  );
  return status;
}

function applyRejectedReload(materialization, operation, error) {
  const failure = normalizeReloadFailure(error);
  const hasVisibleValue = materialization.binding.valueSignal() !== null;
  const message =
    failure.error instanceof Error
      ? failure.error.message
      : "resource refresh failed";
  const status = createRejectedLineStatus(operation, message, hasVisibleValue);
  const freshness = createRejectedFreshness(operation);
  const diagnostics = createReloadRejectedDiagnostics(
    materialization.binding.diagnosticsSignal(),
    operation,
    message,
    failure.retryAttempts,
  );
  materialization.binding.statusSignal.set(status);
  materialization.binding.freshnessSignal.set(freshness);
  materialization.binding.diagnosticsSignal.set(diagnostics);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "rejected",
  );
  return status;
}

function applyTimedOutReload(materialization, operation, retryAttempts = 0) {
  const hasVisibleValue = materialization.binding.valueSignal() !== null;
  const status = createTimedOutLineStatus(operation, hasVisibleValue);
  const freshness = createTimedOutFreshness(operation);
  const diagnostics = createTimedOutReloadDiagnostics(
    materialization.binding.diagnosticsSignal(),
    operation,
    retryAttempts,
  );
  materialization.binding.statusSignal.set(status);
  materialization.binding.freshnessSignal.set(freshness);
  materialization.binding.diagnosticsSignal.set(diagnostics);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "timedOut",
  );
  return status;
}

function normalizeReloadFailure(error) {
  if (
    error
    && typeof error === "object"
    && "error" in error
    && "retryAttempts" in error
  ) {
    return error;
  }
  return Object.freeze({
    error,
    retryAttempts: 0,
  });
}

export { executeLineReload };

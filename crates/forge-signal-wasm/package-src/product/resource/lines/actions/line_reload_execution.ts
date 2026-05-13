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
import { areLineValuesSemanticallyEqual } from "../state/line_value_semantic_equality.js";
import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";
import {
  prepareMutationResponsePlanIfDeclared,
  recordMutationResponsePlanIfPresent,
} from "../../mutation/resource_mutation_response_execution.js";
import {
  createMutationResponseTargetBasisSnapshots,
} from "../../mutation/resource_mutation_response_target_basis.js";
import {
  createSubmittedMutationResponseIdentityMigration,
} from "../../mutation/identity/resource_mutation_response_identity_migration.js";

function executeLineReload(materialization, operation, options = "fulfilled") {
  const normalizedOptions =
    typeof options === "string"
      ? Object.freeze({
          fulfilledEvent: options,
          seedDiagnostics: null,
          requestDescriptorOverride: null,
          finalizeFulfilledDiagnostics: null,
          onFulfilled: null,
        })
      : Object.freeze({
          fulfilledEvent: options.fulfilledEvent ?? "fulfilled",
          seedDiagnostics: options.seedDiagnostics ?? null,
          requestDescriptorOverride: options.requestDescriptorOverride ?? null,
          previousValueOverride:
            "previousValueOverride" in options
              ? options.previousValueOverride ?? null
              : undefined,
          finalizeFulfilledDiagnostics:
            options.finalizeFulfilledDiagnostics ?? null,
          onFulfilled: options.onFulfilled ?? null,
        });
  const reload = materialization.reload;
  const previousValue =
    normalizedOptions.previousValueOverride === undefined
      ? materialization.binding.valueSignal()
      : normalizedOptions.previousValueOverride;
  const supersededOperation = materialization.lifecycle.supersedePendingReload();
  if (supersededOperation !== null) {
    recordLineHistoryEntry(
      materialization.lifecycleHistory,
      materialization.binding,
      "superseded",
      { supersededOperation },
    );
  }
  if (normalizedOptions.seedDiagnostics !== null) {
    materialization.binding.diagnosticsSignal.set(
      normalizedOptions.seedDiagnostics(
        materialization.binding.diagnosticsSignal(),
        supersededOperation,
      ),
    );
  }
  try {
    const requestDescriptor =
      normalizedOptions.requestDescriptorOverride
      ?? reload.requestState.readDescriptor();
    const submittedTargets =
      reload.mutationResponseDeclaration === null
        ? null
        : createMutationResponseTargetBasisSnapshots(
            reload.mutationResponseDeclaration,
            reload.params,
          );
    const submittedIdentityMigration =
      reload.mutationResponseDeclaration?.identityMigration === undefined
      || reload.mutationResponseDeclaration.identityMigration === null
        ? null
        : createSubmittedMutationResponseIdentityMigration(
            reload.mutationResponseDeclaration.identityMigration,
            reload.params,
          );
    const bindingResult = bindReloadLineValue(
      reload.load,
      reload.params,
      reload.familyKind,
      requestDescriptor,
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
            requestDescriptor,
            operation,
            previousValue,
            loaded.loaded,
            loaded.retryAttempts,
            normalizedOptions.fulfilledEvent,
            normalizedOptions.finalizeFulfilledDiagnostics,
            normalizedOptions.onFulfilled,
            submittedTargets,
            submittedIdentityMigration,
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
      requestDescriptor,
      operation,
      previousValue,
      bindingResult.loaded,
      bindingResult.retryAttempts,
      normalizedOptions.fulfilledEvent,
      normalizedOptions.finalizeFulfilledDiagnostics,
      normalizedOptions.onFulfilled,
      submittedTargets,
      submittedIdentityMigration,
    );
  } catch (error) {
    return applyRejectedReload(materialization, operation, error);
  }
}

function applyFulfilledReload(
  materialization,
  reload,
  requestDescriptor,
  operation,
  previousValue,
  loaded,
  retryAttempts = 0,
  fulfilledEvent = "fulfilled",
  finalizeFulfilledDiagnostics = null,
  onFulfilled = null,
  submittedTargets = null,
  submittedIdentityMigration = null,
) {
  const visibleValueChanged =
    loaded.hasVisibleValue
    && !areLineValuesSemanticallyEqual(loaded.value, previousValue);
  const status = createFulfilledLineStatus(operation);
  const freshness = createFreshnessFromPolicy(reload.policy);
  const nextDiagnostics = createReloadFulfilledDiagnostics(
    materialization.binding.diagnosticsSignal(),
    operation,
    loaded.processing,
    loaded.upload,
    loaded.download,
    visibleValueChanged,
    retryAttempts,
  );
  const finalizedDiagnostics =
    finalizeFulfilledDiagnostics === null
      ? nextDiagnostics
      : finalizeFulfilledDiagnostics(nextDiagnostics);
  const preparedMutationResponse = prepareMutationResponsePlanIfDeclared(
    materialization.lineIdentity,
    requestDescriptor,
    finalizedDiagnostics,
    reload.mutationResponseDeclaration,
    loaded.value,
    submittedTargets,
    submittedIdentityMigration,
  );
  materialization.binding.valueSignal.set(loaded.value);
  materialization.binding.processingSignal.set(loaded.processing);
  materialization.binding.uploadSignal.set(loaded.upload);
  materialization.binding.downloadSignal.set(loaded.download);
  materialization.binding.statusSignal.set(status);
  materialization.binding.freshnessSignal.set(freshness);
  materialization.binding.diagnosticsSignal.set(
    preparedMutationResponse.diagnostics,
  );
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    fulfilledEvent,
  );
  recordMutationResponsePlanIfPresent(
    materialization.lifecycleHistory,
    materialization.binding,
    preparedMutationResponse.plan,
  );
  if (onFulfilled !== null) {
    onFulfilled();
  }
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

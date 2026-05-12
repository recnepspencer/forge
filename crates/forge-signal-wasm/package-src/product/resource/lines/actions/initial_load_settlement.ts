import { bindReloadLineValue } from "../../lowering/runtime_line_binding.js";
import { createLineDownload } from "../state/line_download_value.js";
import { createReadyLineProcessing } from "../state/line_processing_value.js";
import { createReadyLineUpload } from "../state/line_upload_value.js";
import {
  createFreshnessFromPolicy,
  createPendingFreshness,
  createRejectedFreshness,
  createTimedOutFreshness,
} from "../state/line_freshness_value.js";
import {
  createInitialLineDiagnostics,
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
import {
  prepareMutationResponsePlanIfDeclared,
  recordMutationResponsePlanIfPresent,
} from "../../mutation/resource_mutation_response_execution.js";

function createInitialLineBinding(
  load,
  params,
  lineScope,
  familyKind,
  policy,
  requestDescriptor,
  lifecycle,
  lifecycleHistory,
  mutationResponsePlanning = null,
) {
  const valueSignal = lineScope.input(null, {
    debugName: `${familyKind}ResourceValue`,
  });
  const readableValueSignal = lineScope.computed(() => valueSignal(), {
    debugName: `${familyKind}ResourceLine`,
  });
  const initialProcessing = createReadyLineProcessing(
    requestDescriptor.processingJob.kind,
  );
  const initialUpload = createReadyLineUpload(
    requestDescriptor.uploadTransport.kind,
  );
  const initialDownload = createLineDownload();
  const processingSignal = lineScope.input(initialProcessing, {
    debugName: `${familyKind}ResourceProcessing`,
  });
  const uploadSignal = lineScope.input(initialUpload, {
    debugName: `${familyKind}ResourceUpload`,
  });
  const downloadSignal = lineScope.input(initialDownload, {
    debugName: `${familyKind}ResourceDownload`,
  });
  const statusSignal = lineScope.input(
    createPendingLineStatus("initialLoad", false),
    {
      debugName: `${familyKind}ResourceStatus`,
    },
  );
  const freshnessSignal = lineScope.input(
    createPendingFreshness("initialLoad"),
    {
      debugName: `${familyKind}ResourceFreshness`,
    },
  );
  const binding = Object.freeze({
    valueSignal,
    readableValueSignal,
    processingSignal,
    uploadSignal,
    downloadSignal,
    statusSignal,
    freshnessSignal,
    diagnosticsSignal: lineScope.input(
      createInitialLineDiagnostics(
        policy,
        requestDescriptor,
        initialProcessing,
        initialUpload,
        initialDownload,
        false,
      ),
      {
        debugName: `${familyKind}ResourceDiagnostics`,
      },
    ),
  });

  let resolvedBindingResult;
  try {
    resolvedBindingResult = bindReloadLineValue(
      load,
      params,
      familyKind,
      requestDescriptor,
      null,
      policy.retryLimit,
    );
  } catch (error) {
    throw normalizeReloadFailure(error).error;
  }
  if (resolvedBindingResult.kind === "settled") {
    applyFulfilledInitialLoad(
      lifecycleHistory,
      binding,
      policy,
      mutationResponsePlanning,
      resolvedBindingResult.loaded,
      resolvedBindingResult.retryAttempts,
      false,
    );
    return binding;
  }

  const pendingToken = lifecycle.beginPendingReload("initialLoad");
  binding.diagnosticsSignal.set(
    createPendingReloadDiagnostics(
      binding.diagnosticsSignal(),
      "initialLoad",
      null,
    ),
  );
  const timeoutMs = policy.timeoutMs;
  let timedOut = false;
  if (typeof timeoutMs === "number" && timeoutMs >= 0) {
    setTimeout(() => {
      if (!lifecycle.completePendingReload(pendingToken)) {
        return;
      }
      timedOut = true;
      applyTimedOutInitialLoad(
        lifecycleHistory,
        binding,
        resolvedBindingResult.retryTracker.count(),
      );
    }, timeoutMs);
  }
  resolvedBindingResult.promise.then(
    (settled) => {
      if (timedOut) {
        return;
      }
      if (!lifecycle.completePendingReload(pendingToken)) {
        return;
      }
      applyFulfilledInitialLoad(
        lifecycleHistory,
        binding,
        policy,
        mutationResponsePlanning,
        settled.loaded,
        settled.retryAttempts,
      );
    },
    (error) => {
      if (timedOut) {
        return;
      }
      if (!lifecycle.completePendingReload(pendingToken)) {
        return;
      }
      applyRejectedInitialLoad(lifecycleHistory, binding, error);
    },
  );
  return binding;
}

function applyFulfilledInitialLoad(
  lifecycleHistory,
  binding,
  policy,
  mutationResponsePlanning,
  loaded,
  retryAttempts,
  shouldRecordHistory = true,
) {
  const status = createFulfilledLineStatus("initialLoad");
  const freshness = createFreshnessFromPolicy(policy);
  const nextDiagnostics = createReloadFulfilledDiagnostics(
    binding.diagnosticsSignal(),
    "initialLoad",
    loaded.processing,
    loaded.upload,
    loaded.download,
    loaded.hasVisibleValue,
    retryAttempts,
  );
  const preparedMutationResponse =
    mutationResponsePlanning === null
      ? Object.freeze({
          plan: null,
          diagnostics: nextDiagnostics,
        })
      : prepareMutationResponsePlanIfDeclared(
          mutationResponsePlanning.lineIdentity,
          mutationResponsePlanning.requestDescriptor,
          nextDiagnostics,
          mutationResponsePlanning.declaration,
          loaded.value,
        );
  binding.valueSignal.set(loaded.value);
  binding.processingSignal.set(loaded.processing);
  binding.uploadSignal.set(loaded.upload);
  binding.downloadSignal.set(loaded.download);
  binding.statusSignal.set(status);
  binding.freshnessSignal.set(freshness);
  binding.diagnosticsSignal.set(preparedMutationResponse.diagnostics);
  if (shouldRecordHistory) {
    recordLineHistoryEntry(lifecycleHistory, binding, "fulfilled");
    recordMutationResponsePlanIfPresent(
      lifecycleHistory,
      binding,
      preparedMutationResponse.plan,
    );
  }
}

function applyRejectedInitialLoad(lifecycleHistory, binding, error) {
  const failure = normalizeReloadFailure(error);
  const message =
    failure.error instanceof Error
      ? failure.error.message
      : "resource initial load failed";
  binding.statusSignal.set(createRejectedLineStatus("initialLoad", message, false));
  binding.freshnessSignal.set(createRejectedFreshness("initialLoad"));
  binding.diagnosticsSignal.set(
    createReloadRejectedDiagnostics(
      binding.diagnosticsSignal(),
      "initialLoad",
      message,
      failure.retryAttempts,
    ),
  );
  recordLineHistoryEntry(lifecycleHistory, binding, "rejected");
}

function applyTimedOutInitialLoad(lifecycleHistory, binding, retryAttempts) {
  binding.statusSignal.set(createTimedOutLineStatus("initialLoad", false));
  binding.freshnessSignal.set(createTimedOutFreshness("initialLoad"));
  binding.diagnosticsSignal.set(
    createTimedOutReloadDiagnostics(
      binding.diagnosticsSignal(),
      "initialLoad",
      retryAttempts,
    ),
  );
  recordLineHistoryEntry(lifecycleHistory, binding, "timedOut");
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

export { createInitialLineBinding };

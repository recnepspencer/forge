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
import {
  createLineBindingState,
  patchLineBindingState,
  readLineBindingState,
  replaceLineBindingState,
} from "../state/line_binding_state.js";

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
  const valueSignal = wrapInternalLineMutableSignal(lineScope.input(null, {
    debugName: `${familyKind}ResourceValue`,
  }), lineScope);
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
  const processingSignal = wrapInternalLineMutableSignal(lineScope.input(initialProcessing, {
    debugName: `${familyKind}ResourceProcessing`,
  }), lineScope);
  const uploadSignal = wrapInternalLineMutableSignal(lineScope.input(initialUpload, {
    debugName: `${familyKind}ResourceUpload`,
  }), lineScope);
  const downloadSignal = wrapInternalLineMutableSignal(lineScope.input(initialDownload, {
    debugName: `${familyKind}ResourceDownload`,
  }), lineScope);
  const statusSignal = wrapInternalLineMutableSignal(lineScope.input(
    createPendingLineStatus("initialLoad", false),
    {
      debugName: `${familyKind}ResourceStatus`,
    },
  ), lineScope);
  const freshnessSignal = wrapInternalLineMutableSignal(lineScope.input(
    createPendingFreshness("initialLoad"),
    {
      debugName: `${familyKind}ResourceFreshness`,
    },
  ), lineScope);
  const initialState = createLineBindingState({
    value: null,
    processing: initialProcessing,
    upload: initialUpload,
    download: initialDownload,
    status: createPendingLineStatus("initialLoad", false),
    freshness: createPendingFreshness("initialLoad"),
    diagnostics: createInitialLineDiagnostics(
      policy,
      requestDescriptor,
      initialProcessing,
      initialUpload,
      initialDownload,
      false,
    ),
  });
  const binding = Object.freeze({
    valueSignal,
    readableValueSignal,
    processingSignal,
    uploadSignal,
    downloadSignal,
    statusSignal,
    freshnessSignal,
    diagnosticsSignal: wrapInternalLineMutableSignal(lineScope.input(
      initialState.current.diagnostics,
      {
        debugName: `${familyKind}ResourceDiagnostics`,
      },
    ), lineScope),
    state: initialState,
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
  patchLineBindingState(binding, {
    diagnostics: createPendingReloadDiagnostics(
      readLineBindingState(binding).diagnostics,
      "initialLoad",
      null,
    ),
  });
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
    readLineBindingState(binding).diagnostics,
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
          mutationResponsePlanning.submittedTargets,
          mutationResponsePlanning.submittedIdentityMigration ?? null,
        );
  replaceLineBindingState(binding, {
    ...readLineBindingState(binding),
    value: loaded.value,
    processing: loaded.processing,
    upload: loaded.upload,
    download: loaded.download,
    status,
    freshness,
    diagnostics: preparedMutationResponse.diagnostics,
  });
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
  patchLineBindingState(binding, {
    status: createRejectedLineStatus("initialLoad", message, false),
    freshness: createRejectedFreshness("initialLoad"),
    diagnostics: createReloadRejectedDiagnostics(
      readLineBindingState(binding).diagnostics,
      "initialLoad",
      message,
      failure.retryAttempts,
    ),
  });
  recordLineHistoryEntry(lifecycleHistory, binding, "rejected");
}

function applyTimedOutInitialLoad(lifecycleHistory, binding, retryAttempts) {
  patchLineBindingState(binding, {
    status: createTimedOutLineStatus("initialLoad", false),
    freshness: createTimedOutFreshness("initialLoad"),
    diagnostics: createTimedOutReloadDiagnostics(
      readLineBindingState(binding).diagnostics,
      "initialLoad",
      retryAttempts,
    ),
  });
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

function wrapInternalLineMutableSignal(handle, lineScope) {
  const signal = function internalLineMutableSignal() {
    return handle();
  };
  signal.get = handle.get?.bind(handle);
  signal.value = handle.value?.bind(handle);
  signal.free = handle.free?.bind(handle);
  signal[Symbol.dispose] = handle[Symbol.dispose]?.bind(handle);
  signal.id = handle.id;
  signal.debugName = handle.debugName ?? null;
  signal.set = wrapInternalLineMutation(handle.set?.bind(handle));
  signal.reset = wrapInternalLineMutation(handle.reset?.bind(handle));
  signal.patch = wrapInternalLineMutation(handle.patch?.bind(handle));
  signal.assign = wrapInternalLineMutation(handle.assign?.bind(handle));
  signal.watch = createInternalLineWatch(handle.id, lineScope);
  return Object.freeze(signal);
}

function createInternalLineWatch(signalId, lineScope) {
  const rawSignalsSymbol = Object.getOwnPropertySymbols(lineScope).find(
    (symbol) => String(symbol) === "Symbol(WorthSignal.rawSignals)",
  );
  if (rawSignalsSymbol === undefined) {
    return undefined;
  }
  const rawSignals = lineScope[rawSignalsSymbol];
  if (!rawSignals || typeof rawSignals.watch !== "function") {
    return undefined;
  }
  return (callback) => rawSignals.watch(signalId, callback);
}

function wrapInternalLineMutation(mutate) {
  if (typeof mutate !== "function") {
    return undefined;
  }
  return (...args) => {
    const result = mutate(...args);
    if (result && typeof result.then === "function") {
      void result.catch(() => {});
    }
    return result;
  };
}

export { createInitialLineBinding };

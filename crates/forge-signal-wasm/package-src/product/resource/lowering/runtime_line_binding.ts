import {
  createAcceptedLineProcessing,
  createInProgressLineProcessing,
  createReadyLineProcessing,
  createUploadAwaitingLineProcessing,
} from "../lines/state/line_processing_value.js";
import {
  createPreparedLineUpload,
  createReadyLineUpload,
  createUploadedLineUpload,
} from "../lines/state/line_upload_value.js";
import { createLineDownload } from "../lines/state/line_download_value.js";
import { isResourceBinaryValue } from "../downloads/resource_binary_value.js";
import {
  isProcessingResult,
  requireDeclaredProcessingJob,
} from "../processing/processing_result.js";
import {
  isUploadResult,
  requireDeclaredUploadTransport,
} from "../uploads/upload_result.js";

function bindSynchronousLineValue(
  load,
  params,
  family,
  requestDescriptor,
  currentValue,
) {
  const value = load(params, requestDescriptor);
  if (value && typeof value.then === "function") {
    throw new TypeError(
      `${family} synchronous-only resource binding does not admit promise-backed load(...)`,
    );
  }
  if (isProcessingResult(value)) {
    requireDeclaredProcessingJob(requestDescriptor.processingJob, family);
    return Object.freeze({
      value: currentValue,
      hasVisibleValue: false,
      processing: createDeferredLineProcessing(
        value,
        requestDescriptor.processingJob.kind,
      ),
      upload: createReadyLineUpload(requestDescriptor.uploadTransport.kind),
      download: createLineDownload(),
    });
  }
  if (isUploadResult(value)) {
    requireDeclaredUploadTransport(requestDescriptor.uploadTransport, family);
    const processing = createUploadBackedProcessing(
      value,
      requestDescriptor.processingJob.kind,
    );
    return Object.freeze({
      value: currentValue,
      hasVisibleValue: false,
      processing,
      upload: createDeferredLineUpload(
        value,
        requestDescriptor.uploadTransport.kind,
      ),
      download: createLineDownload(),
    });
  }
  const binaryValue = unwrapBinaryValue(value);
  return Object.freeze({
    value: binaryValue.value,
    hasVisibleValue: true,
    processing: createReadyLineProcessing(requestDescriptor.processingJob.kind),
    upload: createReadyLineUpload(requestDescriptor.uploadTransport.kind),
    download: createLineDownload(binaryValue.descriptors),
  });
}

function bindReloadLineValue(
  load,
  params,
  family,
  requestDescriptor,
  currentValue,
  retryLimit = 0,
) {
  const retryTracker = createRetryTracker();
  while (true) {
    try {
      const value = load(params, requestDescriptor);
      if (value && typeof value.then === "function") {
        return Object.freeze({
          kind: "pending",
          retryTracker,
          promise: settleReloadPromise(
            Promise.resolve(value),
            load,
            params,
            family,
            requestDescriptor,
            currentValue,
            retryLimit,
            retryTracker,
          ),
        });
      }
      return Object.freeze({
        kind: "settled",
        retryAttempts: retryTracker.count(),
        loaded: resolveLoadedLineValue(
          value,
          family,
          requestDescriptor,
          currentValue,
        ),
      });
    } catch (error) {
      if (retryTracker.count() >= retryLimit) {
        throw createReloadFailure(error, retryTracker.count());
      }
      retryTracker.increment();
    }
  }
}

async function settleReloadPromise(
  promise,
  load,
  params,
  family,
  requestDescriptor,
  currentValue,
  retryLimit,
  retryTracker,
) {
  let currentPromise = Promise.resolve(promise);
  while (true) {
    try {
      const settledValue = await currentPromise;
      return Object.freeze({
        retryAttempts: retryTracker.count(),
        loaded: resolveLoadedLineValue(
          settledValue,
          family,
          requestDescriptor,
          currentValue,
        ),
      });
    } catch (error) {
      if (retryTracker.count() >= retryLimit) {
        throw createReloadFailure(error, retryTracker.count());
      }
      retryTracker.increment();
      let retryValue;
      try {
        retryValue = load(params, requestDescriptor);
      } catch (retryError) {
        if (retryTracker.count() >= retryLimit) {
          throw createReloadFailure(retryError, retryTracker.count());
        }
        continue;
      }
      if (!retryValue || typeof retryValue.then !== "function") {
        return Object.freeze({
          retryAttempts: retryTracker.count(),
          loaded: resolveLoadedLineValue(
            retryValue,
            family,
            requestDescriptor,
            currentValue,
          ),
        });
      }
      currentPromise = Promise.resolve(retryValue);
    }
  }
}

function createRetryTracker() {
  let retryAttempts = 0;
  return Object.freeze({
    count() {
      return retryAttempts;
    },
    increment() {
      retryAttempts += 1;
    },
  });
}

function createReloadFailure(error, retryAttempts) {
  return Object.freeze({
    error,
    retryAttempts,
  });
}

function resolveLoadedLineValue(
  value,
  family,
  requestDescriptor,
  currentValue,
) {
  if (value && typeof value.then === "function") {
    throw new TypeError(
      `${family} resources do not admit nested promise-backed load(...) results`,
    );
  }
  if (isProcessingResult(value)) {
    requireDeclaredProcessingJob(requestDescriptor.processingJob, family);
    return Object.freeze({
      value: currentValue,
      hasVisibleValue: false,
      processing: createDeferredLineProcessing(
        value,
        requestDescriptor.processingJob.kind,
      ),
      upload: createReadyLineUpload(requestDescriptor.uploadTransport.kind),
      download: createLineDownload(),
    });
  }
  if (isUploadResult(value)) {
    requireDeclaredUploadTransport(requestDescriptor.uploadTransport, family);
    const processing = createUploadBackedProcessing(
      value,
      requestDescriptor.processingJob.kind,
    );
    return Object.freeze({
      value: currentValue,
      hasVisibleValue: false,
      processing,
      upload: createDeferredLineUpload(
        value,
        requestDescriptor.uploadTransport.kind,
      ),
      download: createLineDownload(),
    });
  }
  const binaryValue = unwrapBinaryValue(value);
  return Object.freeze({
    value: binaryValue.value,
    hasVisibleValue: true,
    processing: createReadyLineProcessing(requestDescriptor.processingJob.kind),
    upload: createReadyLineUpload(requestDescriptor.uploadTransport.kind),
    download: createLineDownload(binaryValue.descriptors),
  });
}

function unwrapBinaryValue(value) {
  if (!isResourceBinaryValue(value)) {
    return Object.freeze({
      value,
      descriptors: Object.freeze([]),
    });
  }
  if (isProcessingResult(value.value) || isUploadResult(value.value)) {
    throw new TypeError(
      "resourceBinaryValue(...) cannot wrap resourceProcessingResult.*() or resourceUploadResult.*()",
    );
  }
  return value;
}

function createDeferredLineProcessing(result, completionKind) {
  if (result.kind === "accepted") {
    return createAcceptedLineProcessing(result, completionKind);
  }
  return createInProgressLineProcessing(result, completionKind);
}

function createDeferredLineUpload(result, transportKind) {
  if (result.kind === "prepared") {
    return createPreparedLineUpload(result, transportKind);
  }
  return createUploadedLineUpload(result, transportKind);
}

function createUploadBackedProcessing(result, completionKind) {
  if (completionKind === "none" || result.awaitingProcessing !== true) {
    return createReadyLineProcessing(completionKind);
  }
  return createUploadAwaitingLineProcessing(result, completionKind);
}

export { bindReloadLineValue, bindSynchronousLineValue };

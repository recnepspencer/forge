import { nextGeneratedAuthoringSignalId } from "./scopes.js";
import { PRIVATE_AUTHORING_ID } from "./symbols.js";
const ACTIVE_RUNTIME_CALLBACK_READS_KEY = "__WorthSignalActiveRuntimeCallbackReads";
const ACTIVE_RUNTIME_CALLBACK_READER_KEY = "__WorthSignalActiveRuntimeCallbackReader";
const ACTIVE_COMPUTED_CALLBACK_FRAMES = [];

function isFunction(value) {
  return typeof value === "function";
}

export function buildComputedCallbackError(code, message) {
  const error = new TypeError(message);
  error.code = code;
  return error;
}

export function denySignalReadsDuringCallbackAuthoring(signalId) {
  throw buildComputedCallbackError(
    "computeCallbackSignalReadDenied",
    `callback computed collector frame was missing when callback attempted to read \`${signalId}\``,
  );
}

export function denySignalMutationDuringCallbackAuthoring() {
  throw buildComputedCallbackError(
    "computeCallbackMutationDenied",
    "callback computed authoring cannot mutate signals or transactions while the callback is being invoked",
  );
}

export function denySignalReadFromForeignRuntime(signalId) {
  throw buildComputedCallbackError(
    "computeCallbackForeignRuntimeReadDenied",
    `callback computed attempted to read \`${signalId}\` from a different Signals runtime`,
  );
}

export function denyUnavailableRuntimeCallbackRead(signalId) {
  throw buildComputedCallbackError(
    "computeCallbackRuntimeReadUnavailable",
    `callback computed attempted to read \`${signalId}\` outside the active runtime callback value map`,
  );
}

export function activeComputedCallbackFrame() {
  return ACTIVE_COMPUTED_CALLBACK_FRAMES[ACTIVE_COMPUTED_CALLBACK_FRAMES.length - 1] ?? null;
}

export function activeRuntimeCallbackReads() {
  const reads = globalThis[ACTIVE_RUNTIME_CALLBACK_READS_KEY];
  if (!reads || typeof reads !== "object") {
    return null;
  }
  return reads;
}

export function activeRuntimeCallbackReader() {
  const reader = globalThis[ACTIVE_RUNTIME_CALLBACK_READER_KEY];
  if (typeof reader !== "function") {
    return null;
  }
  return reader;
}

export function recordHostCapabilityRead(rawSignals, descriptor) {
  const frame = activeComputedCallbackFrame();
  if (!frame) {
    return;
  }
  if (frame.rawSignals !== rawSignals) {
    denySignalReadFromForeignRuntime(descriptor.registrationId);
  }
  const key = `${descriptor.family}:${descriptor.registrationId}:${descriptor.compatibility}`;
  if (!frame.hostCapabilityReadKeys.has(key)) {
    frame.hostCapabilityReadKeys.add(key);
    frame.hostCapabilityReads.push({
      family: descriptor.family,
      registrationId: descriptor.registrationId,
      compatibility: descriptor.compatibility,
    });
  }
}

export function withComputedCallbackFrame(rawSignals, callback) {
  return function wrappedComputedCallback() {
    const frame = {
      rawSignals,
      reads: new Set(),
      runtimeReadIds: new Set(),
      hostCapabilityReadKeys: new Set(),
      hostCapabilityReads: [],
    };
    ACTIVE_COMPUTED_CALLBACK_FRAMES.push(frame);
    try {
      return {
        __WorthSignalCallbackCapture: true,
        value: callback(),
        reads: [...frame.reads],
        hostCapabilityReads: frame.hostCapabilityReads,
        runtimeReadBreadth: frame.runtimeReadIds.size,
      };
    } finally {
      const popped = ACTIVE_COMPUTED_CALLBACK_FRAMES.pop();
      if (popped !== frame) {
        throw buildComputedCallbackError(
          "computeCallbackCollectorCorrupted",
          "callback computed collector stack was corrupted during evaluation",
        );
      }
    }
  };
}

function parseCallbackAuthoringArgs(rawSignals, family, idOrCompute, computeOrOptions, maybeOptions) {
  if (isFunction(idOrCompute)) {
    if (
      computeOrOptions !== undefined
      && (computeOrOptions === null
        || typeof computeOrOptions !== "object"
        || Array.isArray(computeOrOptions))
    ) {
      throw new TypeError(`${family} callback options must be an object when provided`);
    }
    if (maybeOptions !== undefined) {
      throw new TypeError(`${family} callback form does not accept a third argument`);
    }
    return {
      id: computeOrOptions?.[PRIVATE_AUTHORING_ID]
        ?? computeOrOptions?.id
        ?? nextGeneratedAuthoringSignalId(rawSignals, family),
      callback: idOrCompute,
    };
  }

  if (typeof idOrCompute === "string" && isFunction(computeOrOptions)) {
    if (maybeOptions !== undefined) {
      throw new TypeError(`${family} callback form does not accept options after an explicit id`);
    }
    return {
      id: idOrCompute,
      callback: computeOrOptions,
    };
  }

  return null;
}

export function parseComputedCallbackArgs(rawSignals, idOrCompute, computeOrOptions, maybeOptions) {
  return parseCallbackAuthoringArgs(
    rawSignals,
    "computed",
    idOrCompute,
    computeOrOptions,
    maybeOptions,
  );
}

export function parseOutputCallbackArgs(rawSignals, idOrCompute, computeOrOptions, maybeOptions) {
  return parseCallbackAuthoringArgs(
    rawSignals,
    "output",
    idOrCompute,
    computeOrOptions,
    maybeOptions,
  );
}

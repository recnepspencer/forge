import {
  activeComputedCallbackFrame,
  denySignalMutationDuringCallbackAuthoring,
} from "./callback_frames.js";
import { unwrapInputSignalTarget } from "./handles.js";

function attachSerializedField(value, field, serialized) {
  if (!value || typeof value !== "object" || typeof serialized !== "string") {
    return value;
  }
  Object.defineProperty(value, field, {
    value: serialized,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return value;
}

export function wrapTransaction(rawTx, rawSignals) {
  return Object.freeze({
    set(input, value) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.set(unwrapInputSignalTarget(input, rawSignals, "transaction.set"), value);
    },
    setWithAspects(input, value, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithAspects(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithAspects"),
        value,
        aspects,
      );
    },
    setWithRegions(input, value, changedRegions) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegions(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithRegions"),
        value,
        changedRegions,
      );
    },
    setWithRegionsAndAspects(input, value, changedRegions, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegionsAndAspects(
        unwrapInputSignalTarget(input, rawSignals, "transaction.setWithRegionsAndAspects"),
        value,
        changedRegions,
        aspects,
      );
    },
    free() {
      rawTx.free();
    },
    [Symbol.dispose]() {
      if (typeof rawTx[Symbol.dispose] === "function") {
        rawTx[Symbol.dispose]();
        return;
      }
      rawTx.free();
    },
  });
}

export function wrapAdapters(rawAdapters) {
  return Object.freeze({
    exportDefinitions() {
      return rawAdapters.export_definitions();
    },
    exportRuntimeEnvelope() {
      return attachSerializedField(
        rawAdapters.export_runtime_envelope(),
        "runtimeEnvelopeRestoreToken",
        rawAdapters.export_runtime_envelope_wire(),
      );
    },
    replaceRuntimeEnvelope(envelope) {
      if (typeof envelope?.runtimeEnvelopeRestoreToken === "string") {
        return rawAdapters.replace_runtime_envelope_wire(envelope.runtimeEnvelopeRestoreToken);
      }
      return rawAdapters.replace_runtime_envelope(envelope);
    },
    runtimeProofReport() {
      return rawAdapters.runtime_proof_report();
    },
    free() {
      rawAdapters.free();
    },
    [Symbol.dispose]() {
      if (typeof rawAdapters[Symbol.dispose] === "function") {
        rawAdapters[Symbol.dispose]();
        return;
      }
      rawAdapters.free();
    },
  });
}

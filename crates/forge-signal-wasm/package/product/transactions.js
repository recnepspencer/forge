import {
  activeComputedCallbackFrame,
  denySignalMutationDuringCallbackAuthoring,
} from "./callback_frames.js";
import { unwrapSignalTarget } from "./handles.js";

export function wrapTransaction(rawTx) {
  return Object.freeze({
    set(input, value) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.set(unwrapSignalTarget(input), value);
    },
    setWithAspects(input, value, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithAspects(unwrapSignalTarget(input), value, aspects);
    },
    setWithRegions(input, value, changedRegions) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegions(unwrapSignalTarget(input), value, changedRegions);
    },
    setWithRegionsAndAspects(input, value, changedRegions, aspects) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      rawTx.setWithRegionsAndAspects(unwrapSignalTarget(input), value, changedRegions, aspects);
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
      return rawAdapters.export_runtime_envelope();
    },
    replaceRuntimeEnvelope(envelope) {
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

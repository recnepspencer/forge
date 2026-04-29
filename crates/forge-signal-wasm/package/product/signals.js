import { createRawSignals } from "../raw_surface.js";
import {
  parseComputedCallbackArgs,
  parseOutputCallbackArgs,
  withComputedCallbackFrame,
} from "./callback_frames.js";
import { unwrapSignalTarget, wrapInputSignal, wrapReadableSignal } from "./handles.js";
import { wrapAdapters, wrapTransaction } from "./transactions.js";

export function wrapSignals(rawSignals) {
  return {
    input(id, initial, options) {
      return wrapInputSignal(rawSignals.input(id, initial, options), rawSignals);
    },
    computedSpec(id, spec) {
      return wrapReadableSignal(rawSignals.computedSpec(id, spec), rawSignals);
    },
    computed(idOrCompute, specOrCompute, maybeOptions) {
      const callbackArgs = parseComputedCallbackArgs(
        rawSignals,
        idOrCompute,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        const callback = withComputedCallbackFrame(rawSignals, callbackArgs.callback);
        return wrapReadableSignal(
          rawSignals.computedCallback(callbackArgs.id, callback),
          rawSignals,
        );
      }
      if (typeof idOrCompute !== "string") {
        throw new TypeError("computed expects either (id, spec) or a callback form");
      }
      if (maybeOptions !== undefined) {
        throw new TypeError("computed does not accept a third argument for spec authoring");
      }
      return wrapReadableSignal(rawSignals.computedSpec(idOrCompute, specOrCompute), rawSignals);
    },
    outputSpec(id, spec) {
      return wrapReadableSignal(rawSignals.outputSpec(id, spec), rawSignals);
    },
    output(idOrSpec, specOrCompute, maybeOptions) {
      const callbackArgs = parseOutputCallbackArgs(
        rawSignals,
        idOrSpec,
        specOrCompute,
        maybeOptions,
      );
      if (callbackArgs) {
        return rawSignals.outputCallback(callbackArgs.id, callbackArgs.callback);
      }
      if (typeof idOrSpec !== "string") {
        throw new TypeError("output expects either (id, spec) or a callback form");
      }
      if (maybeOptions !== undefined) {
        throw new TypeError("output does not accept a third argument for spec authoring");
      }
      return wrapReadableSignal(rawSignals.outputSpec(idOrSpec, specOrCompute), rawSignals);
    },
    outputCallback(id, callback) {
      return rawSignals.outputCallback(id, callback);
    },
    watch(target, callback) {
      return rawSignals.watch(unwrapSignalTarget(target), callback);
    },
    effect(target, callback) {
      return rawSignals.effect(unwrapSignalTarget(target), callback);
    },
    transaction(callback) {
      return rawSignals.transaction((rawTx) => callback(wrapTransaction(rawTx)));
    },
    batch(callback) {
      return rawSignals.batch((rawTx) => callback(wrapTransaction(rawTx)));
    },
    nuke: rawSignals.nuke.bind(rawSignals),
    diagnostics: rawSignals.diagnostics.bind(rawSignals),
    history: rawSignals.history.bind(rawSignals),
    specialist: rawSignals.specialist.bind(rawSignals),
    adapters() {
      return wrapAdapters(rawSignals.adapters());
    },
    compatibilityApp: rawSignals.compatibilityApp.bind(rawSignals),
    compatibilityRuntime: rawSignals.compatibilityRuntime.bind(rawSignals),
    free: rawSignals.free.bind(rawSignals),
    [Symbol.dispose]() {
      if (typeof rawSignals[Symbol.dispose] === "function") {
        rawSignals[Symbol.dispose]();
        return;
      }
      rawSignals.free();
    },
  };
}

export function createCallableSignals() {
  return wrapSignals(createRawSignals());
}

export function createSignals() {
  return createCallableSignals();
}

import { unwrapSignalTarget } from "../handles.js";

export const RAW_OBSERVATION_HANDLE = Symbol("worth.rawObservationHandle");

export function createCompatibilityObservation(rawSignals, family, target, callback) {
  const deferred = createDeferredCallback(callback, family === "watch");
  const rawTarget = unwrapSignalTarget(target, rawSignals, `signals.${family}`);
  const rawHandle = rawSignals[family](rawTarget, deferred.callback);
  return Object.freeze({
    free() {
      deferred.dispose();
      rawHandle.free();
    },
    [Symbol.dispose]() {
      deferred.dispose();
      if (typeof rawHandle[Symbol.dispose] === "function") {
        rawHandle[Symbol.dispose]();
      } else {
        rawHandle.free();
      }
    },
    [RAW_OBSERVATION_HANDLE]: rawHandle,
  });
}

function createDeferredCallback(callback, forwardsNotice) {
  let active = true;
  return Object.freeze({
    callback(notice) {
      queueMicrotask(() => {
        if (!active) return;
        if (forwardsNotice) callback(notice);
        else callback();
      });
    },
    dispose() {
      active = false;
    },
  });
}

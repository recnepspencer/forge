import {
  activeComputedCallbackFrame,
  activeRuntimeCallbackReader,
  activeRuntimeCallbackReads,
  denySignalMutationDuringCallbackAuthoring,
  denySignalReadFromForeignRuntime,
  denyUnavailableRuntimeCallbackRead,
} from "./callback_frames.js";
import { RAW_SIGNAL_HANDLE, RAW_SIGNALS } from "./symbols.js";

export function unwrapSignalTarget(target) {
  if (typeof target === "string") {
    return target;
  }
  if (target && typeof target === "function" && RAW_SIGNAL_HANDLE in target) {
    return target[RAW_SIGNAL_HANDLE];
  }
  return target;
}

export function wrapReadableSignal(rawHandle, rawSignals) {
  const signal = function signal() {
    const frame = activeComputedCallbackFrame();
    if (frame) {
      if (frame.rawSignals !== rawSignals) {
        denySignalReadFromForeignRuntime(rawHandle.id);
      }
      frame.reads.add(rawHandle.id);
      const runtimeReads = activeRuntimeCallbackReads();
      if (runtimeReads) {
        frame.runtimeReadIds.add(rawHandle.id);
        if (Object.prototype.hasOwnProperty.call(runtimeReads, rawHandle.id)) {
          return runtimeReads[rawHandle.id];
        }
        const runtimeReader = activeRuntimeCallbackReader();
        if (runtimeReader) {
          return runtimeReader(rawHandle.id);
        }
        denyUnavailableRuntimeCallbackRead(rawHandle.id);
      }
    }
    return rawHandle.get();
  };

  Object.defineProperties(signal, {
    id: {
      enumerable: true,
      get() {
        return rawHandle.id;
      },
    },
    get: {
      enumerable: false,
      value() {
        const frame = activeComputedCallbackFrame();
        if (frame) {
          if (frame.rawSignals !== rawSignals) {
            denySignalReadFromForeignRuntime(rawHandle.id);
          }
          frame.reads.add(rawHandle.id);
          const runtimeReads = activeRuntimeCallbackReads();
          if (runtimeReads) {
            frame.runtimeReadIds.add(rawHandle.id);
            if (Object.prototype.hasOwnProperty.call(runtimeReads, rawHandle.id)) {
              return runtimeReads[rawHandle.id];
            }
            const runtimeReader = activeRuntimeCallbackReader();
            if (runtimeReader) {
              return runtimeReader(rawHandle.id);
            }
            denyUnavailableRuntimeCallbackRead(rawHandle.id);
          }
        }
        return rawHandle.get();
      },
    },
    peek: {
      enumerable: false,
      value() {
        return rawHandle.peek();
      },
    },
    free: {
      enumerable: false,
      value() {
        return rawHandle.free();
      },
    },
    [Symbol.dispose]: {
      enumerable: false,
      value() {
        if (typeof rawHandle[Symbol.dispose] === "function") {
          rawHandle[Symbol.dispose]();
          return;
        }
        rawHandle.free();
      },
    },
    [RAW_SIGNAL_HANDLE]: {
      enumerable: false,
      value: rawHandle,
    },
    [RAW_SIGNALS]: {
      enumerable: false,
      value: rawSignals,
    },
  });

  return signal;
}

export function wrapInputSignal(rawHandle, rawSignals) {
  const signal = wrapReadableSignal(rawHandle, rawSignals);
  Object.defineProperty(signal, "set", {
    enumerable: false,
    value(nextValue) {
      if (activeComputedCallbackFrame()) {
        denySignalMutationDuringCallbackAuthoring();
      }
      return rawSignals.transaction((tx) => {
        tx.set(rawHandle, nextValue);
      });
    },
  });
  return signal;
}

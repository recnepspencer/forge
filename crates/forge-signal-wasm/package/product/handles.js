import {
  activeComputedCallbackFrame,
  activeRuntimeCallbackReader,
  activeRuntimeCallbackReads,
  denySignalMutationDuringCallbackAuthoring,
  denySignalReadFromForeignRuntime,
  denyUnavailableRuntimeCallbackRead,
} from "./callback_frames.js";
import { PRODUCT_SIGNAL_KIND, RAW_SIGNAL_HANDLE, RAW_SIGNALS } from "./symbols.js";

function describeHandleKind(target) {
  if (!target || typeof target !== "function") {
    return null;
  }
  return target[PRODUCT_SIGNAL_KIND] ?? null;
}

function signalIdForError(target) {
  return typeof target?.id === "string" ? target.id : "<unknown>";
}

function isProductSignalHandle(target) {
  return (
    target
    && typeof target === "function"
    && RAW_SIGNAL_HANDLE in target
    && RAW_SIGNALS in target
    && PRODUCT_SIGNAL_KIND in target
  );
}

function invalidTargetError(operation) {
  return new TypeError(
    `${operation} expects a string id or a product signal handle created by this package`,
  );
}

function foreignRuntimeError(operation, signalId) {
  return new TypeError(
    `${operation} cannot use signal \`${signalId}\` from a different Signals runtime`,
  );
}

function nonInputMutationError(operation, kind, signalId) {
  return new TypeError(
    `${operation} expects an input handle, but received a ${kind} handle for \`${signalId}\``,
  );
}

function requireProductSignalHandle(target, rawSignals, operation) {
  if (!isProductSignalHandle(target)) {
    throw invalidTargetError(operation);
  }
  if (target[RAW_SIGNALS] !== rawSignals) {
    throw foreignRuntimeError(operation, signalIdForError(target));
  }
  return target;
}

export function unwrapSignalTarget(target, rawSignals, operation = "signal operation") {
  if (typeof target === "string") {
    return target;
  }
  return requireProductSignalHandle(target, rawSignals, operation)[RAW_SIGNAL_HANDLE];
}

export function unwrapInputSignalTarget(target, rawSignals, operation = "signal mutation") {
  const handle = requireProductSignalHandle(target, rawSignals, operation);
  const kind = describeHandleKind(handle);
  if (kind !== "input") {
    throw nonInputMutationError(operation, kind ?? "unknown", signalIdForError(handle));
  }
  return handle[RAW_SIGNAL_HANDLE];
}

export function wrapReadableSignal(rawHandle, rawSignals, kind = "signal") {
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
    [PRODUCT_SIGNAL_KIND]: {
      enumerable: false,
      value: kind,
    },
  });

  return signal;
}

export function wrapInputSignal(rawHandle, rawSignals) {
  const signal = wrapReadableSignal(rawHandle, rawSignals, "input");
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

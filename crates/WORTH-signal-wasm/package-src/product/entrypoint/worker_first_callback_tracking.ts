import {
  activeComputedCallbackFrame,
  activeRuntimeCallbackReader,
  activeRuntimeCallbackReads,
  denySignalMutationDuringCallbackAuthoring,
  denySignalReadFromForeignRuntime,
  denyUnavailableRuntimeCallbackRead,
} from "../callback_frames.js";

export function readWorkerFirstTrackedSignal(runtimeMarker, signalId, read) {
  const frame = activeComputedCallbackFrame();
  if (frame) {
    if (frame.rawSignals !== runtimeMarker) {
      denySignalReadFromForeignRuntime(signalId);
    }
    frame.reads.add(signalId);
    const runtimeReads = activeRuntimeCallbackReads();
    if (runtimeReads) {
      frame.runtimeReadIds.add(signalId);
      if (Object.prototype.hasOwnProperty.call(runtimeReads, signalId)) {
        return runtimeReads[signalId];
      }
      const runtimeReader = activeRuntimeCallbackReader();
      if (runtimeReader) {
        return runtimeReader(signalId);
      }
      denyUnavailableRuntimeCallbackRead(signalId);
    }
  }
  return read();
}

export function denyWorkerFirstMutationDuringCallbackAuthoring() {
  if (activeComputedCallbackFrame()) {
    denySignalMutationDuringCallbackAuthoring();
  }
}

import { withComputedCallbackFrame } from "../../../../callback_frames.js";
import { nextGeneratedStandaloneSignalId } from "./worker_first_authored_input_state.js";
import { createWorkerFirstHostDependencyRecords } from "./worker_first_host_dependency_records.js";

export function captureWorkerFirstAuthoredCallback(runtimeMarker, callback, family, hasKnownSignalId) {
  const capture = withComputedCallbackFrame(runtimeMarker, callback)();
  if (!capture || capture.__WORTHSignalCallbackCapture !== true) {
    throw new TypeError(
      `worker-first ${family}Async(...) callback authoring did not produce a tracked callback capture`,
    );
  }
  for (const readId of capture.reads) {
    if (!hasKnownSignalId(readId)) {
      throw new TypeError(
        `worker-first ${family}Async(...) can read only currently available worker-first signals; \`${readId}\` is not currently available`,
      );
    }
  }
  const hostDependencies = createWorkerFirstHostDependencyRecords(capture.hostCapabilityReads);
  return Object.freeze({
    ...capture,
    hostDependencies,
    hostDependencyIds: hostDependencies.map((dependency) => dependency.dependencyId),
  });
}

export function createWorkerFirstAuthoredCallbackState(
  family,
  callback,
  hiddenInputId,
  capture,
) {
  return {
    family,
    callback,
    hiddenInputId,
    dependencyIds: capture.reads,
    hostDependencyIds: capture.hostDependencyIds,
    hostDependencies: capture.hostDependencies,
  };
}

export function nextWorkerFirstCallbackBackingInputId(counters, family, visibleId) {
  const suffix = nextGeneratedStandaloneSignalId(counters, "callbackBacking", family);
  return `${suffix}.${visibleId}`;
}

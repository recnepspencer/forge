import { withComputedCallbackFrame } from "../../../callback_frames.js";

export function captureWorkerFirstAuthoredCallback(runtimeMarker, callback, family, hasKnownSignalId) {
  const capture = withComputedCallbackFrame(runtimeMarker, callback)();
  if (!capture || capture.__forgeSignalCallbackCapture !== true) {
    throw new TypeError(
      `worker-first ${family}Async(...) callback authoring did not produce a tracked callback capture`,
    );
  }
  if (capture.hostCapabilityReads.length > 0) {
    const families = [...new Set(capture.hostCapabilityReads.map((entry) => entry.family))].join(", ");
    throw new TypeError(
      `worker-first ${family}Async(...) does not support host capability reads inside callback authoring yet; found ${families || "host capability"} reads`,
    );
  }
  for (const readId of capture.reads) {
    if (!hasKnownSignalId(readId)) {
      throw new TypeError(
        `worker-first ${family}Async(...) can read only currently available worker-first signals; \`${readId}\` is not currently available`,
      );
    }
  }
  return capture;
}

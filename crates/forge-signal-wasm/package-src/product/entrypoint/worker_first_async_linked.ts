import { withComputedCallbackFrame } from "../callback_frames.js";
import { freezeObject } from "../graph_support.js";
import {
  cloneLinkedSignalValue,
  createLinkedPrevious,
  normalizeLinkedDefinition,
} from "../linked_definition.js";
import { PRODUCT_SIGNAL_KIND } from "../symbols.js";
import {
  denyWorkerFirstMutationDuringCallbackAuthoring,
  readWorkerFirstTrackedSignal,
} from "./worker_first_callback_tracking.js";

export async function createWorkerFirstAsyncLinkedHandle(
  rootSession,
  id,
  sourceOrDefinition,
  options,
) {
  const { source, computation, debugName } = normalizeLinkedDefinition(sourceOrDefinition, options);
  const initialSourceValue = evaluateLinkedSource(rootSession, source);
  const initialValue = computation(initialSourceValue, null);
  await rootSession.createStandaloneInput(id, initialValue, {});

  let latestSourceValue = cloneLinkedSignalValue(initialSourceValue);
  const read = () => readWorkerFirstTrackedSignal(rootSession, id, () => rootSession.readSignalValue(id));

  const handle = function workerFirstAsyncLinkedSignal() {
    return read();
  };
  handle.get = read;
  handle.value = read;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = debugName;
  handle[PRODUCT_SIGNAL_KIND] = "input";
  handle.set = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "set", value });
  };
  handle.patch = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "patch", value });
  };
  handle.assign = (fields) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "patch", value: fields });
  };
  handle.reset = async () => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return applyLinkedCommit("reset");
  };
  handle.relink = async () => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return applyLinkedCommit("relink");
  };
  return freezeObject(handle);

  async function applyLinkedCommit(mode) {
    const nextSourceValue = evaluateLinkedSource(rootSession, source);
    const previous = createLinkedPrevious(
      mode === "reset" ? rootSession.readAuthoredInputBaseline(id) : rootSession.readSignalValue(id),
      latestSourceValue,
    );
    const nextValue = computation(nextSourceValue, previous);
    const result = await rootSession.applyAuthoredInputMutation(id, {
      kind: "set",
      value: nextValue,
    });
    rootSession.writeAuthoredInputBaseline(id, nextValue);
    latestSourceValue = cloneLinkedSignalValue(nextSourceValue);
    return result;
  }
}

function evaluateLinkedSource(rootSession, source) {
  const capture = withComputedCallbackFrame(rootSession, source)();
  if (!capture || capture.__forgeSignalCallbackCapture !== true) {
    throw new TypeError("worker-first linkedAsync(...) source callback did not produce a tracked callback capture");
  }
  return cloneLinkedSignalValue(capture.value);
}

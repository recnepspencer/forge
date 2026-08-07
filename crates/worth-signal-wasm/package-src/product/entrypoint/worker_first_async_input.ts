import { freezeObject } from "../graph_support.js";
import { PRODUCT_SIGNAL_KIND } from "../symbols.js";
import {
  denyWorkerFirstMutationDuringCallbackAuthoring,
  readWorkerFirstTrackedSignal,
} from "./worker_first_callback_tracking.js";
import { brandWorkerFirstRootHandle } from "./worker_first_handle_ownership.js";

export function createWorkerFirstAsyncInputHandle(rootSession, id, debugName = null) {
  const read = () => readWorkerFirstTrackedSignal(rootSession, id, () => rootSession.readSignalValue(id));
  const handle = function workerFirstAsyncInputSignal() {
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
  handle.reset = () => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "reset" });
  };
  handle.patch = (value) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "patch", value });
  };
  handle.assign = (fields) => {
    denyWorkerFirstMutationDuringCallbackAuthoring();
    return rootSession.applyAuthoredInputMutation(id, { kind: "patch", value: fields });
  };
  return freezeObject(brandWorkerFirstRootHandle(handle, rootSession));
}

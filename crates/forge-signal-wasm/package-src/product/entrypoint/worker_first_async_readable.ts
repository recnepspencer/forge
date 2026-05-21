import { freezeObject } from "../graph_support.js";
import { PRODUCT_SIGNAL_KIND } from "../symbols.js";
import { readWorkerFirstTrackedSignal } from "./worker_first_callback_tracking.js";

export function createWorkerFirstAsyncReadableHandle(rootSession, id, family, debugName = null) {
  const read = () => readWorkerFirstTrackedSignal(rootSession, id, () => rootSession.readSignalValue(id));
  const handle = function workerFirstAsyncReadableSignal() {
    return read();
  };
  handle.get = read;
  handle.value = read;
  handle.free = () => {};
  handle[Symbol.dispose] = () => {};
  handle.id = id;
  handle.debugName = debugName;
  handle[PRODUCT_SIGNAL_KIND] = family;
  return freezeObject(handle);
}

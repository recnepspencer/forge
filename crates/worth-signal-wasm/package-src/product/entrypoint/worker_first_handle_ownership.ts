import { WORKER_FIRST_ROOT_SESSION } from "../symbols.js";

export function brandWorkerFirstRootHandle(handle, rootSession) {
  Object.defineProperty(handle, WORKER_FIRST_ROOT_SESSION, {
    enumerable: false,
    value: rootSession,
  });
  return handle;
}

export function assertWorkerFirstHandleOwnership(rootSession, target, operation) {
  if (target == null || (typeof target !== "object" && typeof target !== "function")) {
    return;
  }
  const owner = target[WORKER_FIRST_ROOT_SESSION];
  if (owner != null && owner !== rootSession) {
    throw new TypeError(
      `${operation} rejects a signal handle owned by another worker-first runtime`,
    );
  }
}

import { freezeObject } from "../graph_support.js";
import {
  DEBUG_NAME,
  GRAPH_LOCAL_ID,
  GRAPH_OWNER_ID,
  GRAPH_SCOPE_DESCRIPTOR,
  GRAPH_SCOPE_ID,
  PRODUCT_SIGNAL_KIND,
} from "../symbols.js";

export function decorateWorkerFirstScopedHandle(handle, descriptor, signalIdentity = null) {
  if (typeof handle !== "function") {
    return handle;
  }
  const decorated = function workerFirstScopedHandle() {
    return handle();
  };
  copyHandleMethod(decorated, handle, "get");
  copyHandleMethod(decorated, handle, "value");
  copyHandleMethod(decorated, handle, "set");
  copyHandleMethod(decorated, handle, "reset");
  copyHandleMethod(decorated, handle, "relink");
  copyHandleMethod(decorated, handle, "patch");
  copyHandleMethod(decorated, handle, "assign");
  copyHandleMethod(decorated, handle, "free");
  copyHandleMethod(decorated, handle, Symbol.dispose);
  decorated.id = handle.id;
  decorated.debugName = handle.debugName ?? null;
  decorated[PRODUCT_SIGNAL_KIND] = handle[PRODUCT_SIGNAL_KIND];
  Object.defineProperties(decorated, {
    [GRAPH_SCOPE_ID]: {
      enumerable: false,
      value: descriptor.id,
    },
    [GRAPH_OWNER_ID]: {
      enumerable: false,
      value: descriptor.graphOwnerId,
    },
    [GRAPH_SCOPE_DESCRIPTOR]: {
      enumerable: false,
      value: descriptor,
    },
    [GRAPH_LOCAL_ID]: {
      enumerable: false,
      value: signalIdentity?.localId ?? null,
    },
    [DEBUG_NAME]: {
      enumerable: false,
      value: handle[DEBUG_NAME] ?? null,
    },
  });
  if (signalIdentity !== null) {
    Object.defineProperty(decorated, "signalIdentity", {
      enumerable: false,
      value: () => signalIdentity,
    });
  }
  return freezeObject(decorated);
}

function copyHandleMethod(target, source, key) {
  if (typeof source[key] === "function") {
    target[key] = source[key].bind(source);
  }
}

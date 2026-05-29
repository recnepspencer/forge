import {
  DEBUG_NAME,
  GRAPH_LOCAL_ID,
  GRAPH_OWNER_ID,
  GRAPH_SCOPE_DESCRIPTOR,
  GRAPH_SCOPE_ID,
} from "./symbols.js";

export function tagScopedHandle(
  handle,
  descriptor,
  scopeId,
  graphOwnerId,
  signalIdentity,
  localId = null,
) {
  Object.defineProperties(handle, {
    [GRAPH_SCOPE_ID]: {
      enumerable: false,
      value: scopeId,
    },
    [GRAPH_OWNER_ID]: {
      enumerable: false,
      value: graphOwnerId,
    },
    [GRAPH_SCOPE_DESCRIPTOR]: {
      enumerable: false,
      value: descriptor,
    },
    [GRAPH_LOCAL_ID]: {
      enumerable: false,
      value: localId,
    },
  });
  const inheritedDebugName = handle[DEBUG_NAME] ?? null;
  if (inheritedDebugName !== null) {
    Object.defineProperty(handle, DEBUG_NAME, {
      enumerable: false,
      value: inheritedDebugName,
    });
  }
  if (signalIdentity) {
    Object.defineProperty(handle, "signalIdentity", {
      enumerable: false,
      value: () => signalIdentity,
    });
  }
  return handle;
}

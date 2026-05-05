const API_ROUTE_TARGET_METADATA = Symbol(
  "forgeSignal.apiRouteTargetMetadata",
);

function attachApiRouteTargetMetadata(declaration) {
  const decorated = { ...declaration };
  Object.defineProperty(decorated, API_ROUTE_TARGET_METADATA, {
    value: Object.freeze({
      requestPathFromCanonicalKey: true,
    }),
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return Object.freeze(decorated);
}

function readApiRouteTargetMetadata(declaration) {
  if (!declaration || typeof declaration !== "object") {
    return null;
  }
  return declaration[API_ROUTE_TARGET_METADATA] ?? null;
}

export {
  attachApiRouteTargetMetadata,
  readApiRouteTargetMetadata,
};

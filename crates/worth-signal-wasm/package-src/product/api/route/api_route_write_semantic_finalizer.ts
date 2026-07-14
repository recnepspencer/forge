function requireApiRouteWriteSemanticFinalizer(route, semanticFinalizer) {
  if (
    semanticFinalizer !== "create"
    && semanticFinalizer !== "update"
    && semanticFinalizer !== "remove"
  ) {
    throw new TypeError(
      `api.url("${route}") does not recognize write semantic finalizer "${String(semanticFinalizer)}"`,
    );
  }
  return semanticFinalizer;
}

function readDefaultApiRouteWriteMethod(semanticFinalizer) {
  switch (semanticFinalizer) {
    case "create":
      return "POST";
    case "update":
      return "PUT";
    case "remove":
      return "DELETE";
    default:
      return requireApiRouteWriteSemanticFinalizer(
        "unknown route",
        semanticFinalizer,
      );
  }
}

function resolveApiRouteWriteMethod(route, requestShapeState, semanticFinalizer) {
  return requestShapeState.method
    ?? readDefaultApiRouteWriteMethod(
      requireApiRouteWriteSemanticFinalizer(route, semanticFinalizer),
    );
}

export {
  readDefaultApiRouteWriteMethod,
  requireApiRouteWriteSemanticFinalizer,
  resolveApiRouteWriteMethod,
};

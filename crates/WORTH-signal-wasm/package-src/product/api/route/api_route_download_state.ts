function createApiRouteDownloadsState() {
  return Object.freeze({
    declaration: undefined,
  });
}

function withApiRouteDownloads(state, route, declaration) {
  if (typeof declaration !== "function") {
    throw new TypeError(
      `api.url("${route}").downloads(...) requires a declaration function`,
    );
  }
  if (state.declaration !== undefined) {
    throw new TypeError(
      `api.url("${route}").downloads(...) may only be declared once`,
    );
  }
  return Object.freeze({
    declaration,
  });
}

export { createApiRouteDownloadsState, withApiRouteDownloads };

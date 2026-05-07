import { mergeApiDeclaration, normalizeApiLayer } from "./api_request_defaults.js";
import { createApiRouteBuilder } from "./route/api_route_builder.js";

function createApiFactory(signalNamespace) {
  const state = { nextScopeId: 1 };
  return function api(options = {}) {
    const rootLayer = normalizeApiLayer("apiRoot", options);
    return createApiNamespace(signalNamespace, [rootLayer], state);
  };
}

function createApiNamespace(signalNamespace, layers, state) {
  return Object.freeze({
    scope(options = {}) {
      const layer = normalizeApiLayer(`apiScope[${state.nextScopeId}]`, options);
      state.nextScopeId += 1;
      return createApiNamespace(signalNamespace, [...layers, layer], state);
    },
    url(route) {
      return createApiRouteBuilder(signalNamespace, layers, route);
    },
    detail(declaration) {
      return signalNamespace.resource.detail(
        mergeApiDeclaration(layers, declaration),
      );
    },
    collection(declaration) {
      return signalNamespace.resource.collection(
        mergeApiDeclaration(layers, declaration),
      );
    },
    paged(declaration) {
      return signalNamespace.resource.paged(
        mergeApiDeclaration(layers, declaration),
      );
    },
  });
}

export { createApiFactory };

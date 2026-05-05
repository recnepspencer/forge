import { resourceParamIdentity } from "../resource/params/param_identity_factory.js";
import { resourceParams } from "../resource/params/declared_resource_params.js";
import { mergeApiDeclaration } from "./api_request_defaults.js";
import {
  createRouteBoundParams,
  createRouteCanonicalKey,
  parseApiRoutePattern,
} from "./api_route_pattern.js";

function createApiRouteBuilder(signalNamespace, layers, route) {
  const pattern = parseApiRoutePattern(route);
  return Object.freeze({
    detail(declaration) {
      return signalNamespace.resource.detail(
        mergeApiDeclaration(layers, lowerRouteDeclaration(pattern, declaration)),
      );
    },
    list(declaration) {
      return signalNamespace.resource.collection(
        mergeApiDeclaration(layers, lowerRouteDeclaration(pattern, declaration)),
      );
    },
    paged(declaration) {
      return signalNamespace.resource.paged(
        mergeApiDeclaration(layers, lowerRouteDeclaration(pattern, declaration)),
      );
    },
  });
}

function lowerRouteDeclaration(pattern, declaration) {
  if (!declaration || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new TypeError(
      `api.url("${pattern.route}") finalizers require a declaration object`,
    );
  }
  if ("params" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}") owns params(...) in the route-first lane`,
    );
  }
  if ("normalizeParams" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}") owns normalizeParams(...) in the route-first lane`,
    );
  }
  return Object.freeze({
    ...declaration,
    params: resourceParams(),
    normalizeParams(rawParams) {
      const params = createRouteBoundParams(pattern, rawParams);
      return resourceParamIdentity(
        params,
        createRouteCanonicalKey(pattern, params),
      );
    },
  });
}

export { createApiRouteBuilder };

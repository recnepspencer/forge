import { resourceParamIdentity } from "../resource/params/param_identity_factory.js";
import { resourceParams } from "../resource/params/declared_resource_params.js";
import { attachApiFamilyPatchHelpers } from "./api_family_patch_helpers.js";
import { mergeApiDeclaration } from "./api_request_defaults.js";
import { attachApiRouteTargetMetadata } from "./api_route_target_metadata.js";
import {
  createRouteBoundParams,
  createRouteCanonicalKey,
  parseApiRoutePattern,
} from "./api_route_pattern.js";
import {
  createApiRouteRequestParamsState,
  withDeclaredApiRouteRequestParams,
} from "./api_route_request_params.js";

function createApiRouteBuilder(signalNamespace, layers, route) {
  const pattern = parseApiRoutePattern(route);
  return createConfiguredApiRouteBuilder(
    signalNamespace,
    layers,
    pattern,
    createApiRouteRequestParamsState(),
  );
}

function createConfiguredApiRouteBuilder(
  signalNamespace,
  layers,
  pattern,
  requestParamsState,
) {
  const builder = {
    params() {
      if (pattern.pathParamNames.includes("params")) {
        throw new TypeError(
          `api.url("${pattern.route}") cannot declare request params because path param "params" would collide with the request params lane`,
        );
      }
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        withDeclaredApiRouteRequestParams(),
      );
    },
    detail(declaration) {
      return signalNamespace.resource.detail(
        mergeApiDeclaration(
          layers,
          lowerRouteDeclaration(pattern, requestParamsState, declaration),
        ),
      );
    },
    list(declaration) {
      const lowered = mergeApiDeclaration(
        layers,
        lowerRouteDeclaration(pattern, requestParamsState, declaration),
      );
      return attachApiFamilyPatchHelpers(
        "collection",
        signalNamespace.resource.collection(lowered),
        lowered,
      );
    },
    paged(declaration) {
      const lowered = mergeApiDeclaration(
        layers,
        lowerRouteDeclaration(pattern, requestParamsState, declaration),
      );
      return attachApiFamilyPatchHelpers(
        "paged",
        signalNamespace.resource.paged(lowered),
        lowered,
      );
    },
  };
  if (requestParamsState.declared) {
    delete builder.params;
  }
  return Object.freeze(builder);
}

function lowerRouteDeclaration(pattern, requestParamsState, declaration) {
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
  return attachApiRouteTargetMetadata(Object.freeze({
    ...declaration,
    params: resourceParams(),
    normalizeParams(rawParams) {
      const params = createRouteBoundParams(pattern, requestParamsState, rawParams);
      return resourceParamIdentity(
        params,
        createRouteCanonicalKey(pattern, params),
      );
    },
  }));
}

export { createApiRouteBuilder };

import { resourceParamIdentity } from "../../resource/params/param_identity_factory.js";
import { createApiRouteMutationResponseDeclaration } from "./api_route_mutation_response.js";
import { attachApiRouteTargetMetadata } from "./api_route_target_metadata.js";
import {
  createRouteBoundParams,
  createRouteCanonicalKey,
} from "./api_route_pattern.js";

function createResponseMutationRouteDeclaration(options) {
  return attachApiRouteTargetMetadata(Object.freeze({
    ...options.lowered,
    method: options.method,
    ...(options.bodyRequired
      ? { requestBody: (params) => params.body }
      : {}),
    responseLensProof: options.response.lensProof,
    ...createDetailResponseReconcileFields(options.response),
    mutationResponse: createApiRouteMutationResponseDeclaration(
      options.pattern.route,
      options.method,
      options.response,
      options.reconciles,
      options.diagnostics,
    ),
    normalizeParams(rawParams) {
      const params = createRouteBoundParams(
        options.pattern,
        options.requestParamsState,
        rawParams,
        options.bodyRequired ? "required" : "forbidden",
      );
      return resourceParamIdentity(
        params,
        createRouteCanonicalKey(options.pattern, params, options.bodyRequired),
      );
    },
  }), (rawParams) =>
    createRouteCanonicalKey(
      options.pattern,
      createRouteBoundParams(
        options.pattern,
        options.requestParamsState,
        rawParams,
        options.bodyRequired ? "required" : "forbidden",
      ),
      false,
    ));
}

function createDetailResponseReconcileFields(response) {
  if (response.kind !== "detail") {
    return {};
  }
  return {
    detailFields: response.fields ?? undefined,
    detailRegions: response.regions ?? undefined,
    detailJsonPaths: response.jsonPaths ?? undefined,
  };
}

export { createResponseMutationRouteDeclaration };

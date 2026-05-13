import { resourceParamIdentity } from "../../resource/params/param_identity_factory.js";
import { resourceParams } from "../../resource/params/declared_resource_params.js";
import { applyOwnedBinaryDownloads } from "./api_route_binary_download_finalization.js";
import { createApiRouteItemsReconcile } from "./api_route_items_reconcile.js";
import { createResponseMutationRouteDeclaration } from "./api_route_response_mutation_finalization.js";
import { attachApiRouteTargetMetadata } from "./api_route_target_metadata.js";
import {
  createRouteBoundParams,
  createRouteCanonicalKey,
} from "./api_route_pattern.js";

function lowerReadRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  requestShapeState,
  transferState,
  downloadsState,
) {
  if (!declaration || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new TypeError(
      `api.url("${pattern.route}") finalizers require a declaration object`,
    );
  }
  requireOwnedApiRouteReadFields(pattern.route, declaration);
  const lowered = applyOwnedTransferState(
    pattern.route,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  return attachApiRouteTargetMetadata(Object.freeze({
    ...lowered,
    params: resourceParams(),
    normalizeParams(rawParams) {
      const params = createRouteBoundParams(
        pattern,
        requestParamsState,
        rawParams,
        requestShapeState.bodyDeclared ? "required" : "forbidden",
      );
      return resourceParamIdentity(
        params,
        createRouteCanonicalKey(
          pattern,
          params,
          requestShapeState.bodyDeclared,
        ),
      );
    },
  }), (rawParams) =>
    createRouteCanonicalKey(
      pattern,
      createRouteBoundParams(
        pattern,
        requestParamsState,
        rawParams,
        requestShapeState.bodyDeclared ? "required" : "forbidden",
      ),
      requestShapeState.bodyDeclared,
    ));
}

function lowerWriteRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  method,
  requestShapeState,
  transferState,
  downloadsState,
) {
  const lowered = lowerRouteDeclarationBase(
    pattern,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  if ("reconciles" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").create/update(...) owns reconciles(...) only in the mutation response lane`,
    );
  }
  if ("identity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").create/update(...) owns identity(...) only in the mutation response lane`,
    );
  }
  return attachApiRouteTargetMetadata(Object.freeze({
    ...lowered,
    method,
    requestBody: (params) => params.body,
    normalizeParams(rawParams) {
      const params = createRouteBoundParams(
        pattern,
        requestParamsState,
        rawParams,
        "required",
      );
      return resourceParamIdentity(
        params,
        createRouteCanonicalKey(pattern, params, true),
      );
    },
  }), (rawParams) =>
    createRouteCanonicalKey(
      pattern,
      createRouteBoundParams(
        pattern,
        requestParamsState,
        rawParams,
        "required",
      ),
      false,
    ));
}

function lowerDirectArrayRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  requestShapeState,
  directItemsState,
  transferState,
  downloadsState,
) {
  const lowered = lowerReadRouteDeclaration(
    pattern,
    requestParamsState,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  if ("itemIdentity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").${directItemsState.source}(...) owns itemIdentity(...) in the direct-array lane`,
    );
  }
  if ("reconcile" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").${directItemsState.source}(...) owns reconcile(...) in the direct-array lane`,
    );
  }
  return Object.freeze({
    ...lowered,
    itemIdentity: directItemsState.itemIdentity,
    reconcile: createApiRouteItemsReconcile(pattern.route, directItemsState),
  });
}

function lowerResponseMutationRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  method,
  requestShapeState,
  directItemsState,
  transferState,
  downloadsState,
) {
  const bodyRequired = method === "DELETE"
    ? requestShapeState.bodyDeclared
    : true;
  const lowered = lowerRouteDeclarationBase(
    pattern,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  const {
    reconciles,
    atomicity,
    diagnostics,
    identity,
    ...loweredWithoutMutationResponse
  } = lowered;
  if ("itemIdentity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...) owns response identity in the mutation response lane`,
    );
  }
  if ("reconcile" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...) owns mutation response planning in the mutation response lane`,
    );
  }
  return createResponseMutationRouteDeclaration({
    pattern,
    requestParamsState,
    lowered: loweredWithoutMutationResponse,
    method,
    bodyRequired,
    response: directItemsState.response,
    reconciles,
    atomicity,
    diagnostics,
    identity,
  });
}

function lowerResponseDetailRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  requestShapeState,
  directItemsState,
  transferState,
  downloadsState,
) {
  const lowered = lowerReadRouteDeclaration(
    pattern,
    requestParamsState,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  if ("reconciles" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...).detail(...) owns reconciles(...) only in the mutation response lane`,
    );
  }
  if ("identity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...).detail(...) owns identity(...) only in the mutation response lane`,
    );
  }
  if ("itemIdentity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...) owns response identity in the single response lane`,
    );
  }
  if ("reconcile" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").response(...) owns response reconciliation in the single response lane`,
    );
  }
  return Object.freeze({
    ...lowered,
    responseLensProof: directItemsState.response.lensProof,
    ...(directItemsState.response.kind === "detail"
      ? {
          detailFields: directItemsState.response.fields ?? undefined,
          detailRegions: directItemsState.response.regions ?? undefined,
          detailJsonPaths: directItemsState.response.jsonPaths ?? undefined,
        }
      : {}),
  });
}

function lowerRemoveRouteDeclaration(
  pattern,
  requestParamsState,
  declaration,
  requestShapeState,
  transferState,
  downloadsState,
) {
  const lowered = lowerReadRouteDeclaration(
    pattern,
    requestParamsState,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  if ("reconciles" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").remove(...) owns reconciles(...) only in the mutation response lane`,
    );
  }
  if ("identity" in declaration) {
    throw new TypeError(
      `api.url("${pattern.route}").remove(...) owns identity(...) only in the mutation response lane`,
    );
  }
  return attachApiRouteTargetMetadata(Object.freeze({
    ...lowered,
    method: "DELETE",
  }), (rawParams) =>
    createRouteCanonicalKey(
      pattern,
      createRouteBoundParams(pattern, requestParamsState, rawParams),
    ));
}

function lowerRouteDeclarationBase(
  pattern,
  declaration,
  requestShapeState,
  transferState,
  downloadsState,
) {
  if (!declaration || typeof declaration !== "object" || Array.isArray(declaration)) {
    throw new TypeError(
      `api.url("${pattern.route}") finalizers require a declaration object`,
    );
  }
  requireOwnedApiRouteReadFields(pattern.route, declaration);
  const lowered = applyOwnedTransferState(
    pattern.route,
    declaration,
    requestShapeState,
    transferState,
    downloadsState,
  );
  return Object.freeze({
    ...lowered,
    params: resourceParams(),
  });
}

function applyOwnedTransferState(
  route,
  declaration,
  requestShapeState,
  transferState,
  downloadsState,
) {
  const lowered = applyOwnedRequestShape(
    route,
    applyOwnedBinaryDownloads(route, declaration, downloadsState),
    requestShapeState,
  );
  if (transferState.processingJob !== undefined) {
    if ("processingJob" in declaration) {
      throw new TypeError(
        `api.url("${route}").processing(...) owns processingJob(...) in the route-first lane`,
      );
    }
    lowered.processingJob = transferState.processingJob;
  }
  if (transferState.uploadTransport !== undefined) {
    if ("uploadTransport" in declaration) {
      throw new TypeError(
        `api.url("${route}") upload builders own uploadTransport(...) in the route-first lane`,
      );
    }
    lowered.uploadTransport = transferState.uploadTransport;
  }
  return lowered;
}

function applyOwnedRequestShape(route, declaration, requestShapeState) {
  const lowered = { ...declaration };
  if (requestShapeState.headers !== undefined) {
    if ("headers" in declaration) {
      throw new TypeError(
        `api.url("${route}").headers(...) owns headers(...) in the route-first lane`,
      );
    }
    lowered.headers = requestShapeState.headers;
  }
  if (requestShapeState.effects !== undefined) {
    if ("effects" in declaration) {
      throw new TypeError(
        `api.url("${route}").effects(...) owns effects(...) in the route-first lane`,
      );
    }
    lowered.effects = requestShapeState.effects;
  }
  if (requestShapeState.method !== undefined) {
    lowered.method = requestShapeState.method;
  }
  if (requestShapeState.bodyDeclared) {
    lowered.requestBody = (params) => params.body;
  }
  return lowered;
}

function requireOwnedApiRouteReadFields(route, declaration) {
  if ("params" in declaration) {
    throw new TypeError(
      `api.url("${route}") owns params(...) in the route-first lane`,
    );
  }
  if ("normalizeParams" in declaration) {
    throw new TypeError(
      `api.url("${route}") owns normalizeParams(...) in the route-first lane`,
    );
  }
  if ("method" in declaration) {
    throw new TypeError(
      `api.url("${route}") owns request method selection in the route-first lane`,
    );
  }
  if ("requestBody" in declaration) {
    throw new TypeError(
      `api.url("${route}") owns requestBody(...) in the route-first lane`,
    );
  }
}

export {
  lowerDirectArrayRouteDeclaration,
  lowerResponseDetailRouteDeclaration,
  lowerResponseMutationRouteDeclaration,
  lowerReadRouteDeclaration,
  lowerRemoveRouteDeclaration,
  lowerWriteRouteDeclaration,
};

import { resourceParamIdentity } from "../../resource/params/param_identity_factory.js";
import { resourceParams } from "../../resource/params/declared_resource_params.js";
import {
  isResourceBinaryValue,
  resourceBinaryValue,
} from "../../resource/downloads/resource_binary_value.js";
import { isProcessingResult } from "../../resource/processing/processing_result.js";
import { isUploadResult } from "../../resource/uploads/upload_result.js";
import { createApiRouteBodyCanonicalSuffix } from "./api_route_body_identity.js";
import { apiRouteDownloadsBuilder } from "./api_route_download_builder.js";
import { createApiRouteItemsReconcile } from "./api_route_items_reconcile.js";
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
      true,
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
  if (requestShapeState.method !== undefined) {
    lowered.method = requestShapeState.method;
  }
  if (requestShapeState.bodyDeclared) {
    lowered.requestBody = (params) => params.body;
  }
  return lowered;
}

function applyOwnedBinaryDownloads(route, declaration, downloadsState) {
  const builderDownloads = downloadsState.declaration;
  if (builderDownloads !== undefined) {
    if ("downloads" in declaration && declaration.downloads !== undefined) {
      throw new TypeError(
        `api.url("${route}").downloads(...) owns downloads(...) in the pleasant lane`,
      );
    }
    return lowerOwnedBinaryDownloads(route, declaration, builderDownloads);
  }
  if (!("downloads" in declaration) || declaration.downloads === undefined) {
    return { ...declaration };
  }
  return lowerOwnedBinaryDownloads(route, declaration, declaration.downloads);
}

function lowerOwnedBinaryDownloads(route, declaration, downloads) {
  if (typeof declaration.downloads !== "function") {
    if (typeof downloads !== "function") {
      throw new TypeError(
        `api.url("${route}") downloads(...) must be declared as a function`,
      );
    }
  }
  const { load, ...rest } = declaration;
  delete rest.downloads;
  return {
    ...rest,
    load(params, request) {
      const loaded = load(params, request);
      if (
        loaded
        && typeof loaded === "object"
        && typeof loaded.then === "function"
      ) {
        return loaded.then((value) =>
          lowerOwnedBinaryDownloadValue(route, params, value, downloads));
      }
      return lowerOwnedBinaryDownloadValue(route, params, loaded, downloads);
    },
  };
}

function lowerOwnedBinaryDownloadValue(route, params, value, downloads) {
  if (isProcessingResult(value) || isUploadResult(value)) {
    return value;
  }
  if (isResourceBinaryValue(value)) {
    throw new TypeError(
      `api.url("${route}") downloads(...) owns resourceBinaryValue(...) in the pleasant lane`,
    );
  }
  return resourceBinaryValue({
    value,
    descriptors: downloads(params, value, apiRouteDownloadsBuilder),
  });
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
  lowerReadRouteDeclaration,
  lowerRemoveRouteDeclaration,
  lowerWriteRouteDeclaration,
};

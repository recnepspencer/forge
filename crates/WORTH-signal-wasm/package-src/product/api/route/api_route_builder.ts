import { createApiRouteDownloadsState, withApiRouteDownloads } from "./api_route_download_state.js";
import { attachCollectionApiRouteFamily, attachDetailApiRouteFamily } from "./api_route_builder_resource_family.js";
import { createApiRouteItemsState, extendApiRouteItemsAspect, extendApiRouteItemsSummary, requireApiRouteItemsReconcileState, requireApiRouteResponseItemsState, requireApiRouteItemsState } from "./api_route_items_reconcile.js";
import { mergeApiDeclaration } from "../api_request_defaults.js";
import { lowerDirectArrayRouteDeclaration, lowerReadRouteDeclaration, lowerRemoveRouteDeclaration, lowerResponseDetailRouteDeclaration, lowerResponseMutationRouteDeclaration, lowerWriteRouteDeclaration } from "./api_route_finalization.js";
import { parseApiRoutePattern } from "./api_route_pattern.js";
import { createApiRouteRequestParamsState, withDeclaredApiRouteRequestParams } from "./api_route_request_params.js";
import { createApiRouteRequestShapeState, withApiRouteBody, withApiRouteEffects, withApiRouteHeaders, withApiRouteVerb } from "./api_route_request_shape_state.js";
import { createApiRouteTransferState, withApiRouteMultipartUpload, withApiRouteProcessing, withApiRouteSignedUpload } from "./api_route_transfer_state.js";
import { applyApiRouteBuilderVisibility } from "./api_route_builder_visibility.js";
import { normalizeCommandMutationDeclaration, normalizeSemanticMutationDeclaration } from "./api_route_semantic_mutation_surface.js";

function createApiRouteBuilder(signalNamespace, layers, route) {
  const pattern = parseApiRoutePattern(route);
  return createConfiguredApiRouteBuilder(
    signalNamespace,
    layers,
    pattern,
    createApiRouteRequestParamsState(),
    createApiRouteRequestShapeState(),
    createApiRouteItemsState(),
    createApiRouteTransferState(),
    createApiRouteDownloadsState(),
  );
}

function createConfiguredApiRouteBuilder(
  signalNamespace,
  layers,
  pattern,
  requestParamsState,
  requestShapeState,
  directItemsState,
  transferState,
  downloadsState,
) {
  const lowerDetailDeclaration = (declaration) => mergeApiDeclaration(
    layers,
    directItemsState.response !== null
      ? lowerResponseDetailRouteDeclaration(
          pattern,
          requestParamsState,
          declaration,
          requestShapeState,
          directItemsState,
          transferState,
          downloadsState,
        )
      : lowerReadRouteDeclaration(
          pattern,
          requestParamsState,
          declaration,
          requestShapeState,
          transferState,
          downloadsState,
        ),
  );
  const lowerMutationDeclaration = (declaration, semanticFinalizer, methodOverride, authoringSurface) => mergeApiDeclaration(
    layers,
    directItemsState.source === "response"
      ? lowerResponseMutationRouteDeclaration(
          pattern,
          requestParamsState,
          declaration,
          semanticFinalizer,
          authoringSurface,
          methodOverride === undefined ? requestShapeState : { ...requestShapeState, method: methodOverride },
          directItemsState,
          transferState,
          downloadsState,
        )
      : semanticFinalizer === "remove"
        ? lowerRemoveRouteDeclaration(
            pattern,
            requestParamsState,
            declaration,
            methodOverride === undefined ? requestShapeState : { ...requestShapeState, method: methodOverride },
            semanticFinalizer,
            transferState,
            downloadsState,
          )
        : lowerWriteRouteDeclaration(
            pattern,
            requestParamsState,
            declaration,
            semanticFinalizer,
            methodOverride === undefined ? requestShapeState : { ...requestShapeState, method: methodOverride },
            transferState,
            downloadsState,
          ),
  );
  const lowerCollectionDeclaration = (declaration) => mergeApiDeclaration(
    layers,
    directItemsState.declared
      ? lowerDirectArrayRouteDeclaration(
          pattern,
          requestParamsState,
          declaration,
          requestShapeState,
          directItemsState,
          transferState,
          downloadsState,
        )
      : lowerReadRouteDeclaration(
          pattern,
          requestParamsState,
          declaration,
          requestShapeState,
          transferState,
          downloadsState,
        ),
  );
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
        requestShapeState,
        directItemsState,
        transferState,
        downloadsState,
      );
    },
    items(itemIdentity) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        requireApiRouteItemsState(itemIdentity, pattern.route),
        transferState,
        downloadsState,
      );
    },
    response(responseContract) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        requireApiRouteResponseItemsState(responseContract, pattern.route),
        transferState,
        downloadsState,
      );
    },
    verb(method) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        withApiRouteVerb(requestShapeState, pattern.route, method),
        directItemsState,
        transferState,
        downloadsState,
      );
    },
    body() {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        withApiRouteBody(requestShapeState, pattern.route),
        directItemsState,
        transferState,
        downloadsState,
      );
    },
    headers(headers) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        withApiRouteHeaders(requestShapeState, pattern.route, headers),
        directItemsState,
        transferState,
        downloadsState,
      );
    },
    effects(effects) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        withApiRouteEffects(requestShapeState, pattern.route, effects),
        directItemsState,
        transferState,
        downloadsState,
      );
    },
    downloads(declaration) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        directItemsState,
        transferState,
        withApiRouteDownloads(downloadsState, pattern.route, declaration),
      );
    },
    signedUpload(options) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        directItemsState,
        withApiRouteSignedUpload(transferState, pattern.route, options),
        downloadsState,
      );
    },
    multipartUpload(options) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        directItemsState,
        withApiRouteMultipartUpload(transferState, pattern.route, options),
        downloadsState,
      );
    },
    processing(kind, options) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        directItemsState,
        withApiRouteProcessing(transferState, pattern.route, kind, options),
        downloadsState,
      );
    },
    reconcile(items, replaceItems) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        requireApiRouteItemsReconcileState(
          directItemsState,
          pattern.route,
          items,
          replaceItems,
        ),
        transferState,
        downloadsState,
      );
    },
    aspect(name, read, write) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        extendApiRouteItemsAspect(
          directItemsState,
          pattern.route,
          name,
          read,
          write,
        ),
        transferState,
        downloadsState,
      );
    },
    summary(name, read, write) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        extendApiRouteItemsSummary(
          directItemsState,
          pattern.route,
          name,
          read,
          write,
          "line",
        ),
        transferState,
        downloadsState,
      );
    },
    pageWindowSummary(name, read, write) {
      return createConfiguredApiRouteBuilder(
        signalNamespace,
        layers,
        pattern,
        requestParamsState,
        requestShapeState,
        extendApiRouteItemsSummary(
          directItemsState,
          pattern.route,
          name,
          read,
          write,
          "pageWindow",
        ),
        transferState,
        downloadsState,
      );
    },
    detail(declaration) {
      return attachDetailApiRouteFamily(signalNamespace, lowerDetailDeclaration(declaration));
    },
    create(declaration) {
      return attachDetailApiRouteFamily(
        signalNamespace,
        lowerMutationDeclaration(declaration, "create", undefined, "create"),
      );
    },
    update(declaration) {
      return attachDetailApiRouteFamily(
        signalNamespace,
        lowerMutationDeclaration(declaration, "update", undefined, "update"),
      );
    },
    remove(declaration) {
      return attachDetailApiRouteFamily(
        signalNamespace,
        lowerMutationDeclaration(declaration, "remove", undefined, "remove"),
      );
    },
    mutation(declaration) {
      const normalized = normalizeSemanticMutationDeclaration(pattern.route, declaration);
      return attachDetailApiRouteFamily(
        signalNamespace,
        lowerMutationDeclaration(
          normalized.declaration,
          normalized.semanticFinalizer,
          normalized.method,
          "mutation",
        ),
      );
    },
    command(declaration) {
      const normalized = normalizeCommandMutationDeclaration(pattern.route, declaration);
      return attachDetailApiRouteFamily(
        signalNamespace,
        lowerMutationDeclaration(
          normalized.declaration,
          normalized.semanticFinalizer,
          normalized.method,
          "command",
        ),
      );
    },
    list(declaration) {
      return attachCollectionApiRouteFamily(
        "collection",
        signalNamespace,
        lowerCollectionDeclaration(declaration),
      );
    },
    paged(declaration) {
      return attachCollectionApiRouteFamily(
        "paged",
        signalNamespace,
        lowerCollectionDeclaration(declaration),
      );
    },
  };
  if (requestParamsState.declared) {
    delete builder.params;
  }
  applyApiRouteBuilderVisibility(
    builder,
    requestShapeState,
    directItemsState,
    transferState,
    downloadsState,
  );
  return Object.freeze(builder);
}

export { createApiRouteBuilder };

import { hasAdvancedApiRouteRequestShape } from "./api_route_request_shape_state.js";

function applyApiRouteBuilderVisibility(
  builder,
  requestShapeState,
  directItemsState,
  transferState,
  downloadsState,
) {
  if (requestShapeState.headers !== undefined) {
    delete builder.headers;
  }
  if (requestShapeState.bodyDeclared) {
    delete builder.body;
  }
  if (requestShapeState.method !== undefined) {
    delete builder.verb;
  }
  if (directItemsState.declared) {
    delete builder.items;
    delete builder.response;
    if (directItemsState.reconcileMode === "responseDetail") {
      attachSingleResponseCollectionFinalizerDenials(builder, "detail");
      attachSingleResponseWriteFinalizerDenials(builder, "detail");
    } else if (directItemsState.reconcileMode === "responseSummary") {
      attachSingleResponseCollectionFinalizerDenials(builder, "summary");
      attachSingleResponseWriteFinalizerDenials(builder, "summary");
    } else if (directItemsState.source === "response") {
      attachResponseDetailFinalizerDenials(builder);
    } else {
      delete builder.detail;
      delete builder.create;
      delete builder.update;
      delete builder.remove;
    }
    if (
      directItemsState.reconcileMode === "custom" ||
      directItemsState.reconcileMode === "responseCollection" ||
      directItemsState.reconcileMode === "responseDetail" ||
      directItemsState.reconcileMode === "responseSummary"
    ) {
      delete builder.reconcile;
    }
    if (
      directItemsState.reconcileMode === "responseCollection" ||
      directItemsState.reconcileMode === "responseDetail" ||
      directItemsState.reconcileMode === "responseSummary"
    ) {
      delete builder.aspect;
      delete builder.summary;
      delete builder.pageWindowSummary;
    }
  } else {
    delete builder.reconcile;
    delete builder.aspect;
    delete builder.summary;
    delete builder.pageWindowSummary;
  }
  if (transferState.uploadTransport !== undefined) {
    delete builder.signedUpload;
    delete builder.multipartUpload;
  }
  if (transferState.processingJob !== undefined) {
    delete builder.processing;
  }
  if (downloadsState.declaration !== undefined) {
    delete builder.downloads;
  }
  if (
    hasAdvancedApiRouteRequestShape(requestShapeState) &&
    directItemsState.source !== "response"
  ) {
    delete builder.create;
    delete builder.update;
    delete builder.remove;
  }
}

function attachSingleResponseCollectionFinalizerDenials(builder, kind) {
  builder.list = () => denySingleResponseCollectionFinalizer(kind);
  builder.paged = () => denySingleResponseCollectionFinalizer(kind);
}

function attachSingleResponseWriteFinalizerDenials(builder, kind) {
  builder.create = () => denySingleResponseWriteFinalizer(kind);
  builder.update = () => denySingleResponseWriteFinalizer(kind);
  builder.remove = () => denySingleResponseWriteFinalizer(kind);
}

function attachResponseDetailFinalizerDenials(builder) {
  builder.detail = denyResponseDetailFinalizer;
  builder.create = denyResponseDetailFinalizer;
  builder.update = denyResponseDetailFinalizer;
  builder.remove = denyResponseDetailFinalizer;
}

function denyResponseDetailFinalizer() {
  throw new TypeError(
    "api.url(...).response(...) is a collection response lane; use list(...) or paged(...) until detail response lenses support detail-field effect loci",
  );
}

function denySingleResponseCollectionFinalizer(kind) {
  throw new TypeError(
    `api.url(...).response(resource.response.${kind}<T>()) is a ${kind} response lane; use detail(...) instead of list(...) or paged(...)`,
  );
}

function denySingleResponseWriteFinalizer(kind) {
  throw new TypeError(
    `api.url(...).response(resource.response.${kind}<T>()) supports detail(...) broad replacement only; create/update/remove await ${kind} mutation response lenses`,
  );
}

export { applyApiRouteBuilderVisibility };

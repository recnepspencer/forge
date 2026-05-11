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
      attachResponseDetailCollectionFinalizerDenials(builder);
      attachResponseDetailWriteFinalizerDenials(builder);
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
      directItemsState.reconcileMode === "responseDetail"
    ) {
      delete builder.reconcile;
    }
    if (
      directItemsState.reconcileMode === "responseCollection" ||
      directItemsState.reconcileMode === "responseDetail"
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

function attachResponseDetailCollectionFinalizerDenials(builder) {
  builder.list = denyResponseDetailCollectionFinalizer;
  builder.paged = denyResponseDetailCollectionFinalizer;
}

function attachResponseDetailWriteFinalizerDenials(builder) {
  builder.create = denyResponseDetailWriteFinalizer;
  builder.update = denyResponseDetailWriteFinalizer;
  builder.remove = denyResponseDetailWriteFinalizer;
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

function denyResponseDetailCollectionFinalizer() {
  throw new TypeError(
    "api.url(...).response(resource.response.detail<T>()) is a detail response lane; use detail(...) instead of list(...) or paged(...)",
  );
}

function denyResponseDetailWriteFinalizer() {
  throw new TypeError(
    "api.url(...).response(resource.response.detail<T>()) supports detail(...) broad replacement only; create/update/remove await detail mutation response lenses",
  );
}

export { applyApiRouteBuilderVisibility };

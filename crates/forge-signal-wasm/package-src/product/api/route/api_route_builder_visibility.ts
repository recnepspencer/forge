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
    delete builder.detail;
    delete builder.create;
    delete builder.update;
    delete builder.remove;
    if (directItemsState.reconcileMode === "custom") {
      delete builder.reconcile;
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
  if (hasAdvancedApiRouteRequestShape(requestShapeState)) {
    delete builder.create;
    delete builder.update;
    delete builder.remove;
  }
}

export { applyApiRouteBuilderVisibility };

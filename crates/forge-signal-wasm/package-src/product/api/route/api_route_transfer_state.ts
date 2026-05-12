import { resourceProcessingJob } from "../../resource/processing/resource_processing_job.js";
import { resourceUploadTransport } from "../../resource/uploads/resource_upload_transport.js";

function createApiRouteTransferState() {
  return Object.freeze({
    processingJob: undefined,
    uploadTransport: undefined,
  });
}

function withApiRouteSignedUpload(state, route, options) {
  requireApiRouteUploadUnset(state, route, "signedUpload");
  return Object.freeze({
    ...state,
    uploadTransport: resourceUploadTransport.signed(options ?? {}),
  });
}

function withApiRouteMultipartUpload(state, route, options) {
  requireApiRouteUploadUnset(state, route, "multipartUpload");
  return Object.freeze({
    ...state,
    uploadTransport: resourceUploadTransport.directMultipart(options ?? {}),
  });
}

function withApiRouteProcessing(state, route, kind, options) {
  requireApiRouteProcessingUnset(state, route);
  return Object.freeze({
    ...state,
    processingJob: createApiRouteProcessingPosture(route, kind, options),
  });
}

function createApiRouteProcessingPosture(route, kind, options) {
  if (kind === "poll") {
    return resourceProcessingJob.poll();
  }
  if (kind === "callback") {
    return resourceProcessingJob.callback(requireApiRouteOptions(route, "processing", kind, options));
  }
  if (kind === "webhook") {
    return resourceProcessingJob.webhook(requireApiRouteOptions(route, "processing", kind, options));
  }
  throw new TypeError(
    `api.url("${route}").processing(...) kind must be "poll", "callback", or "webhook"`,
  );
}

function requireApiRouteUploadUnset(state, route, methodName) {
  if (state.uploadTransport !== undefined) {
    throw new TypeError(
      `api.url("${route}") already owns upload transport in this route lane before ${methodName}(...)`,
    );
  }
}

function requireApiRouteProcessingUnset(state, route) {
  if (state.processingJob !== undefined) {
    throw new TypeError(
      `api.url("${route}") already owns processing(...) in this route lane`,
    );
  }
}

function requireApiRouteOptions(route, methodName, kind, options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `api.url("${route}").${methodName}("${kind}", ...) requires an options object`,
    );
  }
  return options;
}

export {
  createApiRouteTransferState,
  withApiRouteMultipartUpload,
  withApiRouteProcessing,
  withApiRouteSignedUpload,
};

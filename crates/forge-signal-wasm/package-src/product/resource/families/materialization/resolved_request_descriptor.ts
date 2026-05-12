import { readApiRouteTargetMetadata } from "../../../api/route/api_route_target_metadata.js";
import { resolveResourceEffectProfile } from "../../effects/effect_profile_resolution.js";
import { resolveResourceProcessingJobPosture } from "../../processing/processing_job_resolution.js";
import { composeBaseUrlWithRoute } from "../../requests/base_url_resolution.js";
import {
  resolveResourceAuthPosture,
  resolveResourceBaseUrlPosture,
  resolveResourceContinuationPosture,
  resolveResourceRequestContext,
} from "../../requests/request_posture_resolution.js";
import { createResourceRequestDescriptor } from "../../requests/request_descriptor.js";
import {
  requireResourceRequestMethod,
  RESOURCE_REQUEST_METHODS,
} from "../../requests/resource_request_method.js";
import { resolveResourceUploadTransportPosture } from "../../uploads/upload_transport_resolution.js";

function createResolvedRequestDescriptor(lineIdentity, familyRecord, params) {
  const baseUrl = resolveResourceBaseUrlPosture(
    familyRecord.baseUrl,
    params,
    familyRecord.identity.kind,
  );
  const auth = resolveResourceAuthPosture(
    familyRecord.auth,
    params,
    familyRecord.identity.kind,
  );
  const context = resolveResourceRequestContext(
    familyRecord.requestContext,
    params,
    familyRecord.identity.kind,
  );
  const continuation = resolveResourceContinuationPosture(
    familyRecord.continuation,
    params,
    familyRecord.identity.kind,
  );
  const processingJob = resolveResourceProcessingJobPosture(
    familyRecord.processingJob,
    params,
    familyRecord.identity.kind,
  );
  const uploadTransport = resolveResourceUploadTransportPosture(
    familyRecord.uploadTransport,
    params,
    familyRecord.identity.kind,
  );
  const effects = resolveResourceEffectProfile(
    familyRecord.effects,
    params,
    familyRecord.identity.kind,
  );
  const target = createResolvedRequestTarget(
    familyRecord.requestTarget,
    lineIdentity.canonicalParams.params,
    baseUrl.value,
  );
  return createResourceRequestDescriptor(
    lineIdentity,
    target,
    baseUrl.value,
    resolveResourceRequestMethod(familyRecord),
    resolveResourceRequestBody(familyRecord, params),
    auth.value,
    context.value,
    continuation.value,
    processingJob.value,
    uploadTransport.value,
    effects.value,
    Object.freeze({
      baseUrl: baseUrl.source,
      auth: auth.source,
      context: context.source,
      continuation: continuation.source,
      processingJob: processingJob.source,
      uploadTransport: uploadTransport.source,
      effects: effects.source,
    }),
  );
}

function createRequestTargetRecord(declaration) {
  const metadata = readApiRouteTargetMetadata(declaration);
  if (metadata === null) {
    return null;
  }
  return Object.freeze({
    requestPath: metadata.requestPath,
  });
}

function createResolvedRequestTarget(requestTarget, params, baseUrl) {
  if (requestTarget === null) {
    return Object.freeze({
      baseUrl,
      requestPath: null,
      url: null,
    });
  }
  const requestPath = requestTarget.requestPath(params);
  return Object.freeze({
    baseUrl,
    requestPath,
    url: composeBaseUrlWithRoute(baseUrl, requestPath),
  });
}

function resolveResourceRequestMethod(familyRecord) {
  if (familyRecord.method === undefined) {
    return RESOURCE_REQUEST_METHODS.get;
  }
  return requireResourceRequestMethod(
    familyRecord.method,
    familyRecord.identity.kind,
  );
}

function resolveResourceRequestBody(familyRecord, params) {
  if (familyRecord.requestBody === undefined) {
    return null;
  }
  const body = familyRecord.requestBody(params);
  return body === undefined ? null : body;
}

export { createRequestTargetRecord, createResolvedRequestDescriptor };

import { requireDeclaredResourceParams } from "../../params/declared_resource_params.js";
import { requireResourceAuthPosture } from "../../requests/auth_posture.js";
import { requireResourceContinuationPosture } from "../../requests/continuation_posture.js";
import { requireResourceRequestContext } from "../../requests/request_context.js";
import { requireResourceRequestMethod } from "../../requests/resource_request_method.js";
import { requireResourceProcessingJobPosture } from "../../processing/processing_job_posture.js";
import { requireResourceUploadTransportPosture } from "../../uploads/upload_transport_posture.js";

function requireResourceDeclarationBase(kind, declaration) {
  if (
    !declaration ||
    typeof declaration !== "object" ||
    Array.isArray(declaration)
  ) {
    throw new TypeError(`${kind} resource declaration must be an object`);
  }
  requireDeclaredResourceParams(declaration.params, kind);
  if (typeof declaration.normalizeParams !== "function") {
    throw new TypeError(`${kind} resources require normalizeParams(...)`);
  }
  if (typeof declaration.load !== "function") {
    throw new TypeError(`${kind} resources require load(...)`);
  }
  if (declaration.method !== undefined) {
    requireResourceRequestMethod(declaration.method, kind);
  }
  if (
    declaration.requestBody !== undefined
    && typeof declaration.requestBody !== "function"
  ) {
    throw new TypeError(`${kind} resources require requestBody(...) to be a function when provided`);
  }
  if (
    declaration.auth !== undefined &&
    typeof declaration.auth !== "function"
  ) {
    requireResourceAuthPosture(declaration.auth, kind);
  }
  if (
    declaration.requestContext !== undefined &&
    typeof declaration.requestContext !== "function"
  ) {
    requireResourceRequestContext(declaration.requestContext, kind);
  }
  if (
    declaration.continuation !== undefined &&
    typeof declaration.continuation !== "function"
  ) {
    requireResourceContinuationPosture(declaration.continuation, kind);
  }
  if (
    declaration.processingJob !== undefined &&
    typeof declaration.processingJob !== "function"
  ) {
    requireResourceProcessingJobPosture(declaration.processingJob, kind);
  }
  if (
    declaration.uploadTransport !== undefined &&
    typeof declaration.uploadTransport !== "function"
  ) {
    requireResourceUploadTransportPosture(declaration.uploadTransport, kind);
  }
  return declaration;
}

export { requireResourceDeclarationBase };

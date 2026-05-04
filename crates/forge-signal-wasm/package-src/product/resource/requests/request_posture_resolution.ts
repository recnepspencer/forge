import { requireResourceAuthPosture } from "./auth_posture.js";
import { requireResourceContinuationPosture } from "./continuation_posture.js";
import { resourceAuth } from "./resource_auth.js";
import { resourceContinuation } from "./resource_continuation.js";
import {
  requireResourceRequestContext,
  resourceRequestContext,
} from "./request_context.js";

function resolveResourceAuthPosture(input, params, family) {
  if (input === undefined) {
    return resourceAuth.anonymous();
  }
  const value = typeof input === "function" ? input(params) : input;
  return requireResourceAuthPosture(value, family);
}

function resolveResourceRequestContext(input, params, family) {
  if (input === undefined) {
    return resourceRequestContext();
  }
  const value = typeof input === "function" ? input(params) : input;
  return requireResourceRequestContext(value, family);
}

function resolveResourceContinuationPosture(input, params, family) {
  if (input === undefined) {
    return resourceContinuation.none();
  }
  const value = typeof input === "function" ? input(params) : input;
  return requireResourceContinuationPosture(value, family);
}

export {
  resolveResourceAuthPosture,
  resolveResourceContinuationPosture,
  resolveResourceRequestContext,
};

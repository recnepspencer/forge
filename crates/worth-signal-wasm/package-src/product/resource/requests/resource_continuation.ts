import { createResourceContinuationPosture } from "./continuation_posture.js";

const resourceContinuation = Object.freeze({
  none() {
    return createResourceContinuationPosture("none", {});
  },
  redirect(options = {}) {
    return createResourceContinuationPosture("redirect", {
      returnTo: normalizeOptionalString(options.returnTo, "returnTo"),
    });
  },
  callback(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceContinuation.callback(...) requires an options object",
      );
    }
    return createResourceContinuationPosture("callback", {
      callbackId: requireString(options.callbackId, "callbackId"),
      returnTo: normalizeOptionalString(options.returnTo, "returnTo"),
    });
  },
  webhook(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceContinuation.webhook(...) requires an options object",
      );
    }
    return createResourceContinuationPosture("webhook", {
      correlationKey: requireString(options.correlationKey, "correlationKey"),
      provider: normalizeOptionalString(options.provider, "provider"),
    });
  },
});

function normalizeOptionalString(value, fieldName) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireString(value, fieldName);
}

function requireString(value, fieldName) {
  if (typeof value !== "string") {
    throw new TypeError(
      `resourceContinuation ${fieldName} must be a string`,
    );
  }
  return value;
}

export { resourceContinuation };

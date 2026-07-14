import { createResourceProcessingJobPosture } from "./processing_job_posture.js";

const resourceProcessingJob = Object.freeze({
  none() {
    return createResourceProcessingJobPosture("none", {});
  },
  poll() {
    return createResourceProcessingJobPosture("poll", {});
  },
  callback(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceProcessingJob.callback(...) requires an options object",
      );
    }
    return createResourceProcessingJobPosture("callback", {
      callbackId: requireString(options.callbackId, "callbackId"),
    });
  },
  webhook(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceProcessingJob.webhook(...) requires an options object",
      );
    }
    return createResourceProcessingJobPosture("webhook", {
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
      `resourceProcessingJob ${fieldName} must be a string`,
    );
  }
  return value;
}

export { resourceProcessingJob };

const RESOURCE_PROCESSING_RESULT_BRAND = Symbol(
  "forgeSignal.resourceProcessingResult",
);

const resourceProcessingResult = Object.freeze({
  accepted(options) {
    return createProcessingResult("accepted", options);
  },
  processing(options) {
    return createProcessingResult("processing", options);
  },
});

function createProcessingResult(kind, options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `resourceProcessingResult.${kind}(...) requires an options object`,
    );
  }
  return Object.freeze({
    kind,
    jobId: requireString(options.jobId, "jobId"),
    message: normalizeOptionalString(options.message, "message"),
    [RESOURCE_PROCESSING_RESULT_BRAND]: "resourceProcessingResult",
  });
}

function isProcessingResult(value) {
  return (
    !!value &&
    value[RESOURCE_PROCESSING_RESULT_BRAND] === "resourceProcessingResult"
  );
}

function requireDeclaredProcessingJob(value, family) {
  if (value.kind === "none") {
    throw new TypeError(
      `${family} resources do not admit resourceProcessingResult.*() without processingJob declared on the family`,
    );
  }
}

function normalizeOptionalString(value, fieldName) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireString(value, fieldName);
}

function requireString(value, fieldName) {
  if (typeof value !== "string") {
    throw new TypeError(
      `resourceProcessingResult ${fieldName} must be a string`,
    );
  }
  return value;
}

export {
  isProcessingResult,
  requireDeclaredProcessingJob,
  resourceProcessingResult,
};

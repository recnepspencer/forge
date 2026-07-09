const RESOURCE_REQUEST_CONTEXT_BRAND = Symbol(
  "WORTHSignal.resourceRequestContext",
);

function resourceRequestContext(options = {}) {
  const basisId = resolveBasisId(options);
  return Object.freeze({
    headers: snapshotHeaders(options.headers),
    correlationId: normalizeOptionalString(
      options.correlationId,
      "correlationId",
    ),
    branchId: normalizeOptionalBranchId(options.branchId),
    basisId,
    [RESOURCE_REQUEST_CONTEXT_BRAND]: "resourceRequestContext",
  });
}

function requireResourceRequestContext(value, family) {
  if (
    !value ||
    value[RESOURCE_REQUEST_CONTEXT_BRAND] !== "resourceRequestContext"
  ) {
    throw new TypeError(
      `${family} resources require requestContext created with resourceRequestContext(...)`,
    );
  }
  return value;
}

function snapshotHeaders(headers) {
  if (headers === undefined) {
    return Object.freeze({});
  }
  if (!headers || typeof headers !== "object" || Array.isArray(headers)) {
    throw new TypeError(
      "resourceRequestContext headers must be a plain object of string values",
    );
  }
  const snapshot = {};
  for (const [name, value] of Object.entries(headers)) {
    if (typeof value !== "string") {
      throw new TypeError(
        "resourceRequestContext headers must be a plain object of string values",
      );
    }
    snapshot[name] = value;
  }
  return Object.freeze(snapshot);
}

function normalizeOptionalString(value, fieldName) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "string") {
    throw new TypeError(`resourceRequestContext ${fieldName} must be a string`);
  }
  return value;
}

function normalizeOptionalBranchId(value) {
  if (value === undefined || value === null) {
    return null;
  }
  if (typeof value !== "string" && typeof value !== "number") {
    throw new TypeError(
      "resourceRequestContext branchId must be a string or number",
    );
  }
  return value;
}

function resolveBasisId(options) {
  if (
    options.basis !== undefined &&
    options.basisId !== undefined &&
    options.basis !== options.basisId
  ) {
    throw new TypeError(
      "resourceRequestContext basis and basisId must match when both are provided",
    );
  }
  return normalizeOptionalString(
    options.basisId ?? options.basis,
    "basisId",
  );
}

export { requireResourceRequestContext, resourceRequestContext };

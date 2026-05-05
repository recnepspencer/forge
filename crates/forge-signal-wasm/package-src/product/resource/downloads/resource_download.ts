const RESOURCE_DOWNLOAD_BRAND = Symbol("forgeSignal.resourceDownload");

const resourceDownload = Object.freeze({
  ready(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError("resourceDownload.ready(...) requires an options object");
    }
    return Object.freeze({
      kind: "ready",
      url: requireString(options.url, "url"),
      method: requireMethod(options.method),
      headers: snapshotStringMap(options.headers, "headers"),
      expiresAt: normalizeOptionalString(options.expiresAt, "expiresAt"),
      [RESOURCE_DOWNLOAD_BRAND]: "resourceDownload",
    });
  },
  unavailable(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceDownload.unavailable(...) requires an options object",
      );
    }
    return Object.freeze({
      kind: "unavailable",
      reason: requireUnavailableReason(options.reason),
      detail: requireString(options.detail, "detail"),
      [RESOURCE_DOWNLOAD_BRAND]: "resourceDownload",
    });
  },
  incompatible(options) {
    if (!options || typeof options !== "object" || Array.isArray(options)) {
      throw new TypeError(
        "resourceDownload.incompatible(...) requires an options object",
      );
    }
    return Object.freeze({
      kind: "incompatible",
      reason: requireIncompatibleReason(options.reason),
      detail: requireString(options.detail, "detail"),
      [RESOURCE_DOWNLOAD_BRAND]: "resourceDownload",
    });
  },
});

function requireResourceDownload(value) {
  if (!value || value[RESOURCE_DOWNLOAD_BRAND] !== "resourceDownload") {
    throw new TypeError(
      "resource binary descriptors require download created with resourceDownload.ready(...), resourceDownload.unavailable(...), or resourceDownload.incompatible(...)",
    );
  }
  return value;
}

function requireMethod(value) {
  if (value !== "GET" && value !== "POST") {
    throw new TypeError('resourceDownload method must be "GET" or "POST"');
  }
  return value;
}

function requireUnavailableReason(value) {
  if (value !== "notReady" && value !== "unavailable") {
    throw new TypeError(
      'resourceDownload unavailable reason must be "notReady" or "unavailable"',
    );
  }
  return value;
}

function requireIncompatibleReason(value) {
  if (value !== "staleDescriptor" && value !== "transportBoundary") {
    throw new TypeError(
      'resourceDownload incompatible reason must be "staleDescriptor" or "transportBoundary"',
    );
  }
  return value;
}

function snapshotStringMap(value, fieldName) {
  if (value === undefined) {
    return Object.freeze({});
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`resourceDownload ${fieldName} must be an object`);
  }
  const snapshot = {};
  for (const [key, entry] of Object.entries(value)) {
    snapshot[key] = requireString(entry, `${fieldName}.${key}`);
  }
  return Object.freeze(snapshot);
}

function normalizeOptionalString(value, fieldName) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireString(value, fieldName);
}

function requireString(value, fieldName) {
  if (typeof value !== "string") {
    throw new TypeError(`resourceDownload ${fieldName} must be a string`);
  }
  return value;
}

export { requireResourceDownload, resourceDownload };

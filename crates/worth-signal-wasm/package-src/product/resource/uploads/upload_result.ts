const RESOURCE_UPLOAD_RESULT_BRAND = Symbol("WorthSignal.resourceUploadResult");

const resourceUploadResult = Object.freeze({
  prepared(options) {
    return createUploadResult("prepared", options);
  },
  uploaded(options) {
    return createUploadResult("uploaded", options);
  },
});

function createUploadResult(kind, options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `resourceUploadResult.${kind}(...) requires an options object`,
    );
  }
  return Object.freeze({
    kind,
    uploadId: requireString(options.uploadId, "uploadId"),
    descriptor:
      kind === "prepared" ? createUploadDescriptor(options.descriptor) : null,
    finalizeRequired: requireBoolean(
      options.finalizeRequired,
      "finalizeRequired",
    ),
    awaitingProcessing: kind === "uploaded"
      ? requireBoolean(options.awaitingProcessing, "awaitingProcessing")
      : false,
    message: normalizeOptionalString(options.message, "message"),
    [RESOURCE_UPLOAD_RESULT_BRAND]: "resourceUploadResult",
  });
}

function createUploadDescriptor(options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "resourceUploadResult.prepared(...) descriptor must be an object",
    );
  }
  return Object.freeze({
    kind: requireDescriptorKind(options.kind),
    url: requireString(options.url, "url"),
    method: requireDescriptorMethod(options.method),
    headers: snapshotStringMap(options.headers, "headers"),
    fields: snapshotStringMap(options.fields, "fields"),
    objectKey: normalizeOptionalString(options.objectKey, "objectKey"),
    expiresAt: normalizeOptionalString(options.expiresAt, "expiresAt"),
  });
}

function isUploadResult(value) {
  return !!value && value[RESOURCE_UPLOAD_RESULT_BRAND] === "resourceUploadResult";
}

function requireDeclaredUploadTransport(value, family) {
  if (value.kind === "none") {
    throw new TypeError(
      `${family} resources do not admit resourceUploadResult.*() without uploadTransport declared on the family`,
    );
  }
}

function requireDescriptorKind(value) {
  if (value !== "signed" && value !== "directMultipart") {
    throw new TypeError(
      'resourceUploadResult descriptor kind must be "signed" or "directMultipart"',
    );
  }
  return value;
}

function requireDescriptorMethod(value) {
  if (value !== "PUT" && value !== "POST") {
    throw new TypeError(
      'resourceUploadResult descriptor method must be "PUT" or "POST"',
    );
  }
  return value;
}

function snapshotStringMap(value, fieldName) {
  if (value === undefined) {
    return Object.freeze({});
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(`resourceUploadResult ${fieldName} must be an object`);
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
    throw new TypeError(`resourceUploadResult ${fieldName} must be a string`);
  }
  return value;
}

function requireBoolean(value, fieldName) {
  if (typeof value !== "boolean") {
    throw new TypeError(`resourceUploadResult ${fieldName} must be a boolean`);
  }
  return value;
}

export {
  isUploadResult,
  requireDeclaredUploadTransport,
  resourceUploadResult,
};

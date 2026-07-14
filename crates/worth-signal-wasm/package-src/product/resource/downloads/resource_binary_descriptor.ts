import { requireResourceDownload } from "./resource_download.js";

const RESOURCE_BINARY_DESCRIPTOR_BRAND = Symbol(
  "WorthSignal.resourceBinaryDescriptor",
);

const resourceBinaryDescriptor = Object.freeze({
  file(options) {
    return createBinaryDescriptor("file", options);
  },
  media(options) {
    return createBinaryDescriptor("media", options);
  },
  export(options) {
    return createBinaryDescriptor("export", options);
  },
});

function createBinaryDescriptor(kind, options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `resourceBinaryDescriptor.${kind}(...) requires an options object`,
    );
  }
  return Object.freeze({
    kind,
    id: requireString(options.id, "id"),
    label: normalizeOptionalString(options.label, "label"),
    fileName: normalizeOptionalString(options.fileName, "fileName"),
    mediaType: normalizeOptionalString(options.mediaType, "mediaType"),
    byteLength: normalizeOptionalByteLength(options.byteLength),
    download: requireResourceDownload(options.download),
    [RESOURCE_BINARY_DESCRIPTOR_BRAND]: "resourceBinaryDescriptor",
  });
}

function requireBinaryDescriptors(value) {
  if (value === undefined) {
    return Object.freeze([]);
  }
  if (!Array.isArray(value)) {
    throw new TypeError("resourceBinaryValue(...) descriptors must be an array");
  }
  return Object.freeze(value.map(requireBinaryDescriptor));
}

function requireBinaryDescriptor(value) {
  if (
    !value
    || value[RESOURCE_BINARY_DESCRIPTOR_BRAND] !== "resourceBinaryDescriptor"
  ) {
    throw new TypeError(
      "resourceBinaryValue(...) descriptors must be created with resourceBinaryDescriptor.*(...)",
    );
  }
  return value;
}

function requireString(value, fieldName) {
  if (typeof value !== "string") {
    throw new TypeError(
      `resourceBinaryDescriptor ${fieldName} must be a string`,
    );
  }
  return value;
}

function normalizeOptionalString(value, fieldName) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireString(value, fieldName);
}

function normalizeOptionalByteLength(value) {
  if (value === undefined || value === null) {
    return null;
  }
  if (
    typeof value !== "number"
    || !Number.isFinite(value)
    || value < 0
    || !Number.isInteger(value)
  ) {
    throw new TypeError(
      "resourceBinaryDescriptor byteLength must be a non-negative integer",
    );
  }
  return value;
}

export { requireBinaryDescriptors, resourceBinaryDescriptor };

export function readSource(source) {
  return readSourceValue(sourceSourceValue(source));
}

export function readSourceSchemaVersion(source) {
  if (!isSourceDescriptor(source) || source.schemaVersion === undefined) {
    return null;
  }
  return normalizeSchemaVersion(readSourceValue(source.schemaVersion));
}

export function readSourceDraftMigration(source) {
  return isSourceDescriptor(source) && typeof source.migrateDraft === "function"
    ? source.migrateDraft
    : null;
}

function sourceSourceValue(source) {
  return isSourceDescriptor(source) ? source.value : source;
}

function readSourceValue(source) {
  if (typeof source === "function") {
    return source();
  }
  if (source && typeof source.get === "function") {
    return source.get();
  }
  return source;
}

function isSourceDescriptor(source) {
  return source !== null && typeof source === "object" && "value" in source;
}

function normalizeSchemaVersion(value) {
  if (value == null) {
    return null;
  }
  return String(value);
}

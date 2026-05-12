const RESOURCE_DETAIL_FIELDS = Symbol("forgeSignal.resourceDetailFields");

function resourceDetailFields(definitions) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceDetailFields(...) requires a definition object");
  }
  const normalized = {};
  for (const [field, definition] of Object.entries(definitions)) {
    if (typeof field !== "string" || field.length === 0) {
      throw new TypeError("resourceDetailFields(...) field names must be non-empty strings");
    }
    if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
      throw new TypeError(`resourceDetailFields(...) field "${field}" must be an object`);
    }
    if (typeof definition.read !== "function") {
      throw new TypeError(`resourceDetailFields(...) field "${field}" requires read(...)`);
    }
    if (typeof definition.write !== "function") {
      throw new TypeError(`resourceDetailFields(...) field "${field}" requires write(...)`);
    }
    normalized[field] = Object.freeze({
      read: definition.read,
      write: definition.write,
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalized),
    [RESOURCE_DETAIL_FIELDS]: "resourceDetailFields",
  });
}

function requireResourceDetailFields(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_FIELDS] !== "resourceDetailFields"
  ) {
    const label =
      kind === undefined
        ? "resourceDetailFields(...)"
        : `${kind} requires detail fields created with resourceDetailFields(...)`;
    throw new TypeError(label);
  }
  return value;
}

export { requireResourceDetailFields, resourceDetailFields };

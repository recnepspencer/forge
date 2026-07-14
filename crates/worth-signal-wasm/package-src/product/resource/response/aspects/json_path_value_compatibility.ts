function requireJsonCompatibleValue(value, aspect, seen) {
  if (
    value === null ||
    typeof value === "string" ||
    typeof value === "boolean"
  ) {
    return;
  }
  if (typeof value === "number") {
    if (!Number.isFinite(value)) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects non-finite JSON numbers`,
      );
    }
    return;
  }
  if (typeof value !== "object") {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires JSON-compatible values`,
    );
  }
  if (seen.has(value)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects cyclic JSON values`,
    );
  }
  seen.add(value);
  if (Array.isArray(value)) {
    requireDenseJsonArray(value, aspect);
    for (let index = 0; index < value.length; index += 1) {
      const nestedValue = readJsonValueDataProperty(value, index, aspect);
      requireJsonCompatibleValue(nestedValue, aspect, seen);
    }
    return;
  }
  requirePlainJsonObject(value, aspect);
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))) {
    requireJsonObjectPropertyName(aspect, key);
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON property "${key}"`,
      );
    }
    if (!descriptor.enumerable) {
      continue;
    }
    requireJsonCompatibleValue(descriptor.value, aspect, seen);
  }
}

function requirePlainJsonObject(value, aspect) {
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects non-plain JSON objects`,
    );
  }
}

function requireDenseJsonArray(value, aspect) {
  for (let index = 0; index < value.length; index += 1) {
    if (!Object.prototype.hasOwnProperty.call(value, index)) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects sparse JSON arrays`,
      );
    }
  }
}

function readJsonValueDataProperty(container, segment, aspect) {
  const descriptor = Object.getOwnPropertyDescriptor(container, segment);
  if (descriptor === undefined) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires existing JSON path array index "${segment}"`,
    );
  }
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON path segment "${segment}"`,
    );
  }
  return descriptor.value;
}

function requireJsonObjectPropertyName(aspect, key) {
  if (typeof key !== "string" || key.length === 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" path segments must be non-empty strings or non-negative array indexes`,
    );
  }
  if (key === "__proto__" || key === "constructor" || key === "prototype") {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects unsafe path segment "${key}"`,
    );
  }
}

export {
  requireDenseJsonArray,
  requireJsonCompatibleValue,
  requireJsonObjectPropertyName,
  requirePlainJsonObject,
};

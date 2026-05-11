import { resourceItemAspects } from "../reconciliation/resource_item_aspects.js";

function jsonPathAspects() {
  return function defineJsonPathAspects(definitions) {
    if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
      throw new TypeError(
        "resource.response.jsonPathAspects<T>()(...) requires an aspect path object",
      );
    }
    const aspects = {};
    for (const [aspect, definition] of Object.entries(definitions)) {
      const path = requireJsonPathAspectDefinition(aspect, definition);
      aspects[aspect] = createJsonPathAspect(aspect, path);
    }
    return resourceItemAspects(aspects);
  };
}

function createJsonPathAspect(aspect, path) {
  return Object.freeze({
    read(item) {
      const root = requireJsonPathItem(item, aspect)[path.field];
      return readRequiredJsonPath(root, path.segments, aspect);
    },
    write(item, value) {
      requireJsonCompatibleValue(value, aspect, new WeakSet());
      const objectItem = requireJsonPathItem(item, aspect);
      const root = objectItem[path.field];
      const nextRoot = writeRequiredJsonPath(root, path.segments, value, aspect);
      return {
        ...objectItem,
        [path.field]: nextRoot,
      };
    },
    locus: "jsonItemAspect",
  });
}

function requireJsonPathAspectDefinition(aspect, definition) {
  if (typeof aspect !== "string" || aspect.length === 0) {
    throw new TypeError(
      "resource.response.jsonPathAspects<T>()(...) aspect names must be non-empty strings",
    );
  }
  if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires a path definition object`,
    );
  }
  const field = requireJsonPathField(aspect, definition.field);
  if (!Array.isArray(definition.path) || definition.path.length === 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires a non-empty path array`,
    );
  }
  return Object.freeze({
    field,
    segments: Object.freeze(
      definition.path.map((segment) =>
        requireJsonPathSegment(aspect, segment),
      ),
    ),
  });
}

function requireJsonPathField(aspect, field) {
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires a non-empty field name`,
    );
  }
  return field;
}

function requireJsonPathSegment(aspect, segment) {
  if (typeof segment === "number") {
    return requireJsonArrayPathSegment(aspect, segment);
  }
  if (typeof segment !== "string" || segment.length === 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" path segments must be non-empty strings or non-negative array indexes`,
    );
  }
  if (
    segment === "__proto__" ||
    segment === "constructor" ||
    segment === "prototype"
  ) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects unsafe path segment "${segment}"`,
    );
  }
  return segment;
}

function requireJsonArrayPathSegment(aspect, segment) {
  if (!Number.isSafeInteger(segment) || segment < 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" array path indexes must be non-negative safe integers`,
    );
  }
  return segment;
}

function requireJsonPathItem(item, aspect) {
  if (!item || typeof item !== "object" || Array.isArray(item)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires object items`,
    );
  }
  return item;
}

function readRequiredJsonPath(root, segments, aspect) {
  let current = requireJsonPathContainer(root, aspect, segments[0]);
  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index];
    const nextValue = readJsonPathDataProperty(current, segment, aspect);
    if (index === segments.length - 1) {
      requireJsonCompatibleValue(nextValue, aspect, new WeakSet());
      return nextValue;
    }
    current = requireJsonPathContainer(nextValue, aspect, segments[index + 1]);
  }
  return current;
}

function writeRequiredJsonPath(root, segments, value, aspect) {
  requireJsonPathContainer(root, aspect, segments[0]);
  return writeJsonPathSegment(root, segments, 0, value, aspect);
}

function writeJsonPathSegment(current, segments, index, value, aspect) {
  const segment = segments[index];
  const currentContainer = requireJsonPathContainer(current, aspect, segment);
  const currentSegmentValue = readJsonPathDataProperty(
    currentContainer,
    segment,
    aspect,
  );
  const nextSegmentValue =
    index === segments.length - 1
      ? value
      : writeJsonPathSegment(
          currentSegmentValue,
          segments,
          index + 1,
          value,
          aspect,
        );
  if (Array.isArray(currentContainer)) {
    const nextArray = [...currentContainer];
    nextArray[segment] = nextSegmentValue;
    return nextArray;
  }
  return {
    ...currentContainer,
    [segment]: nextSegmentValue,
  };
}

function requireJsonPathContainer(value, aspect, segment) {
  if (typeof segment === "number") {
    if (!Array.isArray(value)) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires array JSON containers before index "${segment}"`,
      );
    }
    requireDenseJsonArray(value, aspect);
    return value;
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires object JSON containers before segment "${segment}"`,
    );
  }
  return value;
}

function requireJsonPathSegmentExists(container, segment, aspect) {
  const descriptor = Object.getOwnPropertyDescriptor(container, segment);
  if (descriptor === undefined) {
    const segmentKind = typeof segment === "number" ? "array index" : "segment";
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires existing JSON path ${segmentKind} "${segment}"`,
    );
  }
  if (!descriptor.enumerable) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires enumerable JSON path segment "${segment}"`,
    );
  }
  return descriptor;
}

function readJsonPathDataProperty(container, segment, aspect) {
  const descriptor = requireJsonPathSegmentExists(container, segment, aspect);
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON path segment "${segment}"`,
    );
  }
  return descriptor.value;
}

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
      const nestedValue = readJsonPathDataProperty(value, index, aspect);
      requireJsonCompatibleValue(nestedValue, aspect, seen);
    }
    return;
  }
  for (const [key, descriptor] of Object.entries(Object.getOwnPropertyDescriptors(value))) {
    requireJsonPathSegment(aspect, key);
    if (!descriptor.enumerable) {
      continue;
    }
    if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects accessor JSON property "${key}"`,
      );
    }
    const nestedValue = descriptor.value;
    requireJsonCompatibleValue(nestedValue, aspect, seen);
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

export { jsonPathAspects };

import { resourceItemAspects } from "../reconciliation/resource_item_aspects.js";

function objectAspects() {
  return function defineObjectAspects(fields) {
    return createObjectAspects(
      fields,
      "resource.response.objectAspects<T>()(...)",
      "itemAspect",
    );
  };
}

function jsonObjectAspects() {
  return function defineJsonObjectAspects(fields) {
    return createObjectAspects(
      fields,
      "resource.response.jsonObjectAspects<T>()(...)",
      "jsonItemAspect",
    );
  };
}

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

function createObjectAspects(fields, kind, locus) {
  if (!fields || typeof fields !== "object" || Array.isArray(fields)) {
    throw new TypeError(`${kind} requires an aspect field object`);
  }
  const definitions = {};
  for (const [aspect, field] of Object.entries(fields)) {
    const objectAspectField = requireObjectAspectField(kind, aspect, field);
    definitions[aspect] = Object.freeze({
      read(item) {
        return requireObjectAspectItem(kind, item, aspect)[objectAspectField];
      },
      write(item, value) {
        return {
          ...requireObjectAspectItem(kind, item, aspect),
          [objectAspectField]: value,
        };
      },
      locus,
    });
  }
  return resourceItemAspects(definitions);
}

function createJsonPathAspect(aspect, path) {
  return Object.freeze({
    read(item) {
      const root = requireObjectAspectItem(
        "resource.response.jsonPathAspects<T>()(...)",
        item,
        aspect,
      )[path.field];
      return readRequiredJsonPath(root, path.segments, aspect);
    },
    write(item, value) {
      requireJsonCompatibleValue(value, aspect, new WeakSet());
      const objectItem = requireObjectAspectItem(
        "resource.response.jsonPathAspects<T>()(...)",
        item,
        aspect,
      );
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
  const field = requireObjectAspectField(
    "resource.response.jsonPathAspects<T>()(...)",
    aspect,
    definition.field,
  );
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

function requireObjectAspectField(kind, aspect, field) {
  if (typeof aspect !== "string" || aspect.length === 0) {
    throw new TypeError(
      `${kind} aspect names must be non-empty strings`,
    );
  }
  if (typeof field !== "string" || field.length === 0) {
    throw new TypeError(
      `${kind} aspect "${aspect}" requires a non-empty field name`,
    );
  }
  return field;
}

function requireJsonPathSegment(aspect, segment) {
  if (typeof segment !== "string" || segment.length === 0) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" path segments must be non-empty strings`,
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

function requireObjectAspectItem(kind, item, aspect) {
  if (!item || typeof item !== "object" || Array.isArray(item)) {
    throw new TypeError(
      `${kind} aspect "${aspect}" requires object items`,
    );
  }
  return item;
}

function readRequiredJsonPath(root, segments, aspect) {
  let current = requireJsonPathObject(root, aspect, segments[0]);
  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index];
    if (!Object.prototype.propertyIsEnumerable.call(current, segment)) {
      throw new TypeError(
        `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires existing JSON path segment "${segment}"`,
      );
    }
    const nextValue = current[segment];
    if (index === segments.length - 1) {
      requireJsonCompatibleValue(nextValue, aspect, new WeakSet());
      return nextValue;
    }
    current = requireJsonPathObject(nextValue, aspect, segments[index + 1]);
  }
  return current;
}

function writeRequiredJsonPath(root, segments, value, aspect) {
  requireJsonPathObject(root, aspect, segments[0]);
  return writeJsonPathSegment(root, segments, 0, value, aspect);
}

function writeJsonPathSegment(current, segments, index, value, aspect) {
  const segment = segments[index];
  const currentObject = requireJsonPathObject(current, aspect, segment);
  if (!Object.prototype.propertyIsEnumerable.call(currentObject, segment)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires existing JSON path segment "${segment}"`,
    );
  }
  const nextSegmentValue =
    index === segments.length - 1
      ? value
      : writeJsonPathSegment(
          currentObject[segment],
          segments,
          index + 1,
          value,
          aspect,
        );
  return {
    ...currentObject,
    [segment]: nextSegmentValue,
  };
}

function requireJsonPathObject(value, aspect, segment) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" requires object JSON containers before segment "${segment}"`,
    );
  }
  return value;
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
    for (let index = 0; index < value.length; index += 1) {
      if (!Object.prototype.hasOwnProperty.call(value, index)) {
        throw new TypeError(
          `resource.response.jsonPathAspects<T>()(...) aspect "${aspect}" rejects sparse JSON arrays`,
        );
      }
      requireJsonCompatibleValue(value[index], aspect, seen);
    }
    return;
  }
  for (const [key, nestedValue] of Object.entries(value)) {
    requireJsonPathSegment(aspect, key);
    requireJsonCompatibleValue(nestedValue, aspect, seen);
  }
}

export { jsonObjectAspects, jsonPathAspects, objectAspects };

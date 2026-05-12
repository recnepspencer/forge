import { writeJsonContainerSegment } from "../response/aspects/json_path_container_clone.js";
import {
  requireDenseJsonArray,
  requireJsonCompatibleValue,
  requirePlainJsonObject,
} from "../response/aspects/json_path_value_compatibility.js";
import {
  createResourceDetailJsonPathProof,
  requireResourceDetailJsonPathProof,
} from "../response/detail_json_path_proof.js";

const RESOURCE_DETAIL_JSON_PATHS = Symbol("forgeSignal.resourceDetailJsonPaths");

function resourceDetailJsonPaths(definitions) {
  if (!definitions || typeof definitions !== "object" || Array.isArray(definitions)) {
    throw new TypeError("resourceDetailJsonPaths(...) requires a definition object");
  }
  const normalizedDefinitions = {};
  for (const [pathName, definition] of Object.entries(definitions)) {
    const path = requireDetailJsonPathDefinition(pathName, definition);
    normalizedDefinitions[pathName] = Object.freeze({
      read(value) {
        return path.presence === "optional"
          ? readOptionalJsonPath(value, path.segments, pathName)
          : readRequiredJsonPath(value, path.segments, pathName);
      },
      write(value, nextPathValue) {
        requireJsonCompatibleValue(nextPathValue, pathName, new WeakSet());
        return path.presence === "optional"
          ? writeOptionalJsonPath(value, path.segments, nextPathValue, pathName)
          : writeRequiredJsonPath(value, path.segments, nextPathValue, pathName);
      },
      jsonPathProof: createResourceDetailJsonPathProof(pathName, path),
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalizedDefinitions),
    [RESOURCE_DETAIL_JSON_PATHS]: "resourceDetailJsonPaths",
  });
}

function requireResourceDetailJsonPaths(value, kind) {
  if (
    !value ||
    typeof value !== "object" ||
    value[RESOURCE_DETAIL_JSON_PATHS] !== "resourceDetailJsonPaths"
  ) {
    const label =
      kind === undefined
        ? "resourceDetailJsonPaths(...)"
        : `${kind} requires detail JSON paths created with resourceDetailJsonPaths(...)`;
    throw new TypeError(label);
  }
  const normalizedDefinitions = {};
  for (const [pathName, definition] of Object.entries(value.definitions ?? {})) {
    const jsonPathProof = requireResourceDetailJsonPathProof(
      definition?.jsonPathProof,
      pathName,
    );
    if (!definition || typeof definition.read !== "function" || typeof definition.write !== "function") {
      throw new TypeError(
        `${kind ?? "resourceDetailJsonPaths(...)"} requires valid detail JSON path definitions`,
      );
    }
    normalizedDefinitions[pathName] = Object.freeze({
      read: definition.read,
      write: definition.write,
      jsonPathProof,
    });
  }
  return Object.freeze({
    definitions: Object.freeze(normalizedDefinitions),
    [RESOURCE_DETAIL_JSON_PATHS]: "resourceDetailJsonPaths",
  });
}

function requireDetailJsonPathDefinition(pathName, definition) {
  if (typeof pathName !== "string" || pathName.length === 0) {
    throw new TypeError(
      "resourceDetailJsonPaths(...) path names must be non-empty strings",
    );
  }
  if (!definition || typeof definition !== "object" || Array.isArray(definition)) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires a path definition object`,
    );
  }
  const presence = requireDetailJsonPathPresence(pathName, definition.presence);
  if (!Array.isArray(definition.path) || definition.path.length === 0) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires a non-empty path array`,
    );
  }
  return Object.freeze({
    presence,
    segments: Object.freeze(
      definition.path.map((segment) => requireDetailJsonPathSegment(pathName, segment)),
    ),
  });
}

function requireDetailJsonPathPresence(pathName, presence) {
  if (presence === undefined || presence === "required") {
    return "required";
  }
  if (presence === "optional") {
    return "optional";
  }
  throw new TypeError(
    `resourceDetailJsonPaths(...) path "${pathName}" has unsupported path presence policy "${presence}"`,
  );
}

function requireDetailJsonPathSegment(pathName, segment) {
  if (typeof segment === "number") {
    if (!Number.isSafeInteger(segment) || segment < 0) {
      throw new TypeError(
        `resourceDetailJsonPaths(...) path "${pathName}" array indexes must be non-negative safe integers`,
      );
    }
    return segment;
  }
  if (typeof segment !== "string" || segment.length === 0) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" segments must be non-empty strings or non-negative array indexes`,
    );
  }
  if (
    segment === "__proto__" ||
    segment === "constructor" ||
    segment === "prototype"
  ) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" rejects unsafe path segment "${segment}"`,
    );
  }
  return segment;
}

function readRequiredJsonPath(root, segments, pathName) {
  let current = requireDetailJsonPathContainer(root, pathName, segments[0]);
  for (let index = 0; index < segments.length; index += 1) {
    const segment = segments[index];
    const nextValue = readJsonPathDataProperty(current, segment, pathName);
    if (index === segments.length - 1) {
      requireJsonCompatibleValue(nextValue, pathName, new WeakSet());
      return nextValue;
    }
    current = requireDetailJsonPathContainer(nextValue, pathName, segments[index + 1]);
  }
  return current;
}

function writeRequiredJsonPath(root, segments, nextValue, pathName) {
  requireDetailJsonPathContainer(root, pathName, segments[0]);
  return writeJsonPathSegment(root, segments, 0, nextValue, pathName);
}

function readOptionalJsonPath(root, segments, pathName) {
  const parent = readJsonPathParentContainer(root, segments, pathName);
  const terminal = segments[segments.length - 1];
  if (typeof terminal === "number") {
    const value = readJsonPathDataProperty(parent, terminal, pathName);
    requireJsonCompatibleValue(value, pathName, new WeakSet());
    return value;
  }
  const descriptor = readOptionalJsonPathDataProperty(parent, terminal, pathName);
  if (descriptor === null) {
    return null;
  }
  requireJsonCompatibleValue(descriptor.value, pathName, new WeakSet());
  return descriptor.value;
}

function writeOptionalJsonPath(root, segments, nextValue, pathName) {
  const parent = readJsonPathParentContainer(root, segments, pathName);
  const terminal = segments[segments.length - 1];
  if (typeof terminal === "number") {
    readJsonPathDataProperty(parent, terminal, pathName);
  } else {
    readOptionalJsonPathDataProperty(parent, terminal, pathName);
  }
  return writeOptionalJsonPathSegment(root, segments, 0, nextValue, pathName);
}

function readJsonPathParentContainer(root, segments, pathName) {
  const firstSegment = segments[0];
  if (segments.length === 1) {
    return requireDetailJsonPathContainer(root, pathName, firstSegment);
  }
  let current = requireDetailJsonPathContainer(root, pathName, firstSegment);
  for (let index = 0; index < segments.length - 1; index += 1) {
    const segment = segments[index];
    const nextValue = readJsonPathDataProperty(current, segment, pathName);
    if (index === segments.length - 2) {
      return requireDetailJsonPathContainer(nextValue, pathName, segments[index + 1]);
    }
    current = requireDetailJsonPathContainer(nextValue, pathName, segments[index + 1]);
  }
  return current;
}

function writeJsonPathSegment(current, segments, index, nextValue, pathName) {
  const segment = segments[index];
  const currentContainer = requireDetailJsonPathContainer(current, pathName, segment);
  const currentSegmentValue = readJsonPathDataProperty(
    currentContainer,
    segment,
    pathName,
  );
  const nextSegmentValue =
    index === segments.length - 1
      ? nextValue
      : writeJsonPathSegment(
          currentSegmentValue,
          segments,
          index + 1,
          nextValue,
          pathName,
        );
  return writeJsonContainerSegment(
    currentContainer,
    segment,
    nextSegmentValue,
    pathName,
  );
}

function writeOptionalJsonPathSegment(current, segments, index, nextValue, pathName) {
  const segment = segments[index];
  const currentContainer = requireDetailJsonPathContainer(current, pathName, segment);
  const nextSegmentValue =
    index === segments.length - 1
      ? nextValue
      : writeOptionalJsonPathSegment(
          readJsonPathDataProperty(currentContainer, segment, pathName),
          segments,
          index + 1,
          nextValue,
          pathName,
        );
  return writeJsonContainerSegment(
    currentContainer,
    segment,
    nextSegmentValue,
    pathName,
  );
}

function requireDetailJsonPathContainer(value, pathName, segment) {
  if (typeof segment === "number") {
    if (!Array.isArray(value)) {
      throw new TypeError(
        `resourceDetailJsonPaths(...) path "${pathName}" requires array JSON containers before index "${segment}"`,
      );
    }
    requireDenseJsonArray(value, pathName);
    return value;
  }
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires object JSON containers before segment "${segment}"`,
    );
  }
  requirePlainJsonObject(value, pathName);
  return value;
}

function requireJsonPathSegmentExists(container, segment, pathName) {
  const descriptor = Object.getOwnPropertyDescriptor(container, segment);
  if (descriptor === undefined) {
    const segmentKind = typeof segment === "number" ? "array index" : "segment";
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires existing JSON path ${segmentKind} "${segment}"`,
    );
  }
  if (!descriptor.enumerable) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires enumerable JSON path segment "${segment}"`,
    );
  }
  return descriptor;
}

function readJsonPathDataProperty(container, segment, pathName) {
  const descriptor = requireJsonPathSegmentExists(container, segment, pathName);
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" rejects accessor JSON path segment "${segment}"`,
    );
  }
  return descriptor.value;
}

function readOptionalJsonPathDataProperty(container, segment, pathName) {
  const descriptor = Object.getOwnPropertyDescriptor(container, segment);
  if (descriptor === undefined) {
    return null;
  }
  if (!descriptor.enumerable) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" requires enumerable JSON path segment "${segment}"`,
    );
  }
  if (!Object.prototype.hasOwnProperty.call(descriptor, "value")) {
    throw new TypeError(
      `resourceDetailJsonPaths(...) path "${pathName}" rejects accessor JSON path segment "${segment}"`,
    );
  }
  return descriptor;
}

export { requireResourceDetailJsonPaths, resourceDetailJsonPaths };

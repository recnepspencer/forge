import { FormDeclarationError } from "../form_errors.js";
import { cloneFormValue, isPlainObject } from "./value_semantics.js";

export {
  cloneFormValue,
  deepEqual,
  isPlainObject,
  mergeDraft,
  stableValueDigest,
} from "./value_semantics.js";

const UNSAFE_SEGMENTS = new Set(["__proto__", "prototype", "constructor"]);

export function parseFieldPath(path) {
  if (Array.isArray(path)) {
    return validatePathSegments(path);
  }
  if (typeof path !== "string") {
    throw new FormDeclarationError("form field path must be a string or segment array", { path });
  }
  return validatePathSegments(path.split("."));
}

function validatePathSegments(segments) {
  if (segments.length === 0) {
    throw new FormDeclarationError("form field path must not be empty");
  }
  return segments.map((segment) => {
    if (typeof segment !== "string" && typeof segment !== "number") {
      throw new FormDeclarationError("form field path segments must be strings or numbers", { segment });
    }
    const normalized = String(segment);
    if (normalized.length === 0) {
      throw new FormDeclarationError("form field path segments must not be empty");
    }
    if (UNSAFE_SEGMENTS.has(normalized)) {
      throw new FormDeclarationError("form field path contains an unsafe object segment", {
        segment: normalized,
      });
    }
    return normalized;
  });
}

export function fieldPathKey(segments) {
  return segments.join(".");
}

export function readPath(value, segments) {
  let cursor = value;
  for (const segment of segments) {
    if (cursor == null) {
      return undefined;
    }
    cursor = cursor[segment];
  }
  return cursor;
}

export function writePath(value, segments, nextValue) {
  return writePathAt(value, segments, 0, nextValue);
}

function writePathAt(value, segments, index, nextValue) {
  if (index >= segments.length) {
    return cloneFormValue(nextValue);
  }
  const segment = segments[index];
  const nextContainer = Array.isArray(value)
    ? value.slice()
    : isPlainObject(value)
      ? { ...value }
      : {};
  nextContainer[segment] = writePathAt(
    value == null ? undefined : value[segment],
    segments,
    index + 1,
    nextValue,
  );
  return nextContainer;
}

export function deletePath(value, segments) {
  return deletePathAt(value, segments, 0).value;
}

function deletePathAt(value, segments, index) {
  if (index >= segments.length) {
    return { value: undefined, empty: true };
  }
  if (value == null || (typeof value !== "object" && !Array.isArray(value))) {
    return { value, empty: false };
  }
  const segment = segments[index];
  if (!Object.prototype.hasOwnProperty.call(value, segment)) {
    return { value, empty: false };
  }
  const nextContainer = Array.isArray(value) ? value.slice() : { ...value };
  const deleted = deletePathAt(nextContainer[segment], segments, index + 1);
  if (deleted.empty) {
    delete nextContainer[segment];
  } else {
    nextContainer[segment] = deleted.value;
  }
  return {
    value: nextContainer,
    empty: Object.keys(nextContainer).length === 0,
  };
}

export function isPlainObject(value) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

export function cloneFormValue(value) {
  if (typeof globalThis.structuredClone === "function") {
    try {
      return globalThis.structuredClone(value);
    } catch {
      return value;
    }
  }
  if (Array.isArray(value)) {
    return value.map((entry) => cloneFormValue(entry));
  }
  if (isPlainObject(value)) {
    return Object.fromEntries(
      Object.entries(value).map(([key, entry]) => [key, cloneFormValue(entry)]),
    );
  }
  return value;
}

export function mergeDraft(source, draft) {
  if (Array.isArray(source) || Array.isArray(draft)) {
    return draft === undefined ? cloneFormValue(source) : cloneFormValue(draft);
  }
  if (!isPlainObject(source) || !isPlainObject(draft)) {
    return draft === undefined ? cloneFormValue(source) : cloneFormValue(draft);
  }
  const merged = { ...cloneFormValue(source) };
  for (const [key, value] of Object.entries(draft)) {
    merged[key] = mergeDraft(source[key], value);
  }
  return merged;
}

export function deepEqual(left, right) {
  if (Object.is(left, right)) {
    return true;
  }
  if (Array.isArray(left) || Array.isArray(right)) {
    if (!Array.isArray(left) || !Array.isArray(right) || left.length !== right.length) {
      return false;
    }
    return left.every((entry, index) => deepEqual(entry, right[index]));
  }
  if (isPlainObject(left) || isPlainObject(right)) {
    if (!isPlainObject(left) || !isPlainObject(right)) {
      return false;
    }
    const leftKeys = Object.keys(left);
    const rightKeys = Object.keys(right);
    if (leftKeys.length !== rightKeys.length) {
      return false;
    }
    return leftKeys.every((key) => (
      Object.prototype.hasOwnProperty.call(right, key) && deepEqual(left[key], right[key])
    ));
  }
  return false;
}

export function stableValueDigest(value) {
  return JSON.stringify(canonicalize(value));
}

function canonicalize(value) {
  if (Array.isArray(value)) {
    return value.map((entry) => canonicalize(entry));
  }
  if (isPlainObject(value)) {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  }
  if (value instanceof Date) {
    return {
      forgeFormValueType: "Date",
      value: Number.isNaN(value.getTime()) ? "Invalid Date" : value.toISOString(),
    };
  }
  if (value instanceof Map) {
    return {
      forgeFormValueType: "Map",
      entries: [...value.entries()]
        .map(([key, entry]) => [canonicalize(key), canonicalize(entry)])
        .sort((left, right) => JSON.stringify(left[0]).localeCompare(JSON.stringify(right[0]))),
    };
  }
  if (value instanceof Set) {
    return {
      forgeFormValueType: "Set",
      entries: [...value.values()]
        .map((entry) => canonicalize(entry))
        .sort((left, right) => JSON.stringify(left).localeCompare(JSON.stringify(right))),
    };
  }
  return value;
}

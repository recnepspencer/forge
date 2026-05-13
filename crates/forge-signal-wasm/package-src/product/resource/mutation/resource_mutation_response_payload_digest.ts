function createMutationResponsePayloadDigest(value) {
  return ["mutation-response-payload", canonicalStringify(value)].join("|");
}

function canonicalStringify(value) {
  return JSON.stringify(canonicalize(value, new Set(), "$response"));
}

function canonicalize(value, seen, path) {
  if (typeof value === "bigint" || typeof value === "function" || typeof value === "symbol") {
    throw new TypeError(
      `mutation response payload digest cannot classify ${typeof value} at ${path}`,
    );
  }
  if (Array.isArray(value)) {
    return canonicalizeArray(value, seen, path);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  return canonicalizeObject(value, seen, path);
}

function canonicalizeArray(value, seen, path) {
  if (seen.has(value)) {
    throw new TypeError(
      `mutation response payload digest cannot classify a cyclic array at ${path}`,
    );
  }
  seen.add(value);
  try {
    const canonicalArray = [];
    for (let index = 0; index < value.length; index += 1) {
      const descriptor = Object.getOwnPropertyDescriptor(value, String(index));
      if (descriptor === undefined) {
        throw new TypeError(
          `mutation response payload digest cannot classify a sparse array slot at ${path}[${index}]`,
        );
      }
      if ("get" in descriptor || "set" in descriptor) {
        throw new TypeError(
          `mutation response payload digest cannot inspect accessor-backed array slot at ${path}[${index}]`,
        );
      }
      canonicalArray.push(canonicalize(descriptor.value, seen, `${path}[${index}]`));
    }
    return canonicalArray;
  } finally {
    seen.delete(value);
  }
}

function canonicalizeObject(value, seen, path) {
  const prototype = Object.getPrototypeOf(value);
  if (prototype !== Object.prototype && prototype !== null) {
    throw new TypeError(
      `mutation response payload digest requires plain objects or arrays at ${path}`,
    );
  }
  if (seen.has(value)) {
    throw new TypeError(
      `mutation response payload digest cannot classify a cyclic object at ${path}`,
    );
  }
  seen.add(value);
  const result = {};
  try {
    for (const key of Object.keys(value).sort()) {
      const descriptor = Object.getOwnPropertyDescriptor(value, key);
      if (descriptor === undefined) {
        continue;
      }
      if ("get" in descriptor || "set" in descriptor) {
        throw new TypeError(
          `mutation response payload digest cannot inspect accessor-backed property "${key}" at ${path}`,
        );
      }
      result[key] = canonicalize(descriptor.value, seen, `${path}.${key}`);
    }
    return result;
  } finally {
    seen.delete(value);
  }
}

export { createMutationResponsePayloadDigest };

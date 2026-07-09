function createApiRouteBodyCanonicalSuffix(route, value) {
  return `#body=${stableSerializeApiRouteBody(route, value)}`;
}

function stableSerializeApiRouteBody(route, value) {
  return serializeApiRouteBodyValue(route, value);
}

function serializeApiRouteBodyValue(route, value) {
  if (value === null) {
    return "null";
  }
  const valueType = typeof value;
  if (valueType === "string") {
    return JSON.stringify(value);
  }
  if (valueType === "number") {
    if (!Number.isFinite(value)) {
      throw new TypeError(
        `${route} line(...) body must not contain non-finite numbers`,
      );
    }
    return String(value);
  }
  if (valueType === "boolean") {
    return value ? "true" : "false";
  }
  if (Array.isArray(value)) {
    return `[${value.map((entry) => serializeApiRouteBodyValue(route, entry)).join(",")}]`;
  }
  if (valueType === "object") {
    const keys = Object.keys(value).sort();
    return `{${keys.map((key) => `${JSON.stringify(key)}:${serializeApiRouteBodyValue(route, value[key])}`).join(",")}}`;
  }
  throw new TypeError(
    `${route} line(...) body must be JSON-like data made from plain objects, arrays, strings, numbers, booleans, and null`,
  );
}

export {
  createApiRouteBodyCanonicalSuffix,
  stableSerializeApiRouteBody,
};

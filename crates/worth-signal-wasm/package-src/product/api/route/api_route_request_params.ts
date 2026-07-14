function createApiRouteRequestParamsState() {
  return Object.freeze({
    declared: false,
  });
}

function withDeclaredApiRouteRequestParams() {
  return Object.freeze({
    declared: true,
  });
}

function createRouteRequestParams(rawValue, route) {
  if (!rawValue || typeof rawValue !== "object" || Array.isArray(rawValue)) {
    throw new TypeError(
      `${route} line(...) requires params to be a plain object when request params are declared`,
    );
  }
  const params = {};
  for (const [name, value] of Object.entries(rawValue)) {
    const normalized = normalizeRouteRequestParamValue(route, name, value);
    if (normalized !== undefined) {
      params[name] = normalized;
    }
  }
  return Object.freeze(params);
}

function createRouteQueryString(route, requestParams) {
  const names = Object.keys(requestParams).sort();
  if (names.length === 0) {
    return "";
  }
  const pairs = [];
  for (const name of names) {
    appendRouteQueryPairs(route, pairs, name, requestParams[name]);
  }
  if (pairs.length === 0) {
    return "";
  }
  return `?${pairs.join("&")}`;
}

function appendRouteQueryPairs(route, pairs, name, value) {
  const encodedName = encodeURIComponent(name);
  if (Array.isArray(value)) {
    for (const entry of value) {
      pairs.push(`${encodedName}=${encodeRouteRequestParamScalar(route, name, entry)}`);
    }
    return;
  }
  pairs.push(`${encodedName}=${encodeRouteRequestParamScalar(route, name, value)}`);
}

function normalizeRouteRequestParamValue(route, name, value) {
  if (value === undefined) {
    return undefined;
  }
  if (Array.isArray(value)) {
    return Object.freeze(
      value.map((entry) => requireRouteRequestParamScalar(route, name, entry)),
    );
  }
  return requireRouteRequestParamScalar(route, name, value);
}

function encodeRouteRequestParamScalar(route, name, value) {
  return encodeURIComponent(String(requireRouteRequestParamScalar(route, name, value)));
}

function requireRouteRequestParamScalar(route, name, value) {
  if (
    typeof value === "string"
    || typeof value === "number"
    || typeof value === "boolean"
  ) {
    return value;
  }
  throw new TypeError(
    `${route} request param "${name}" must be a string, number, boolean, or array of those values`,
  );
}

export {
  createApiRouteRequestParamsState,
  createRouteQueryString,
  createRouteRequestParams,
  withDeclaredApiRouteRequestParams,
};

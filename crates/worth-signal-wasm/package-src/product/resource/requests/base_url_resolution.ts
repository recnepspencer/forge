function resolveResourceBaseUrl(input, params, family, taggedReader) {
  if (input === undefined) {
    return Object.freeze({
      value: null,
      source: null,
    });
  }
  const tagged = taggedReader(input, params);
  if (tagged !== null) {
    return Object.freeze({
      value: requireResourceBaseUrl(tagged.value, family),
      source: tagged.source,
    });
  }
  const value = typeof input === "function" ? input(params) : input;
  return Object.freeze({
    value: requireResourceBaseUrl(value, family),
    source: Object.freeze({
      sources: Object.freeze(["endpoint.baseUrl"]),
    }),
  });
}

function requireResourceBaseUrl(value, family) {
  if (typeof value !== "string") {
    throw new TypeError(`${family} baseUrl must resolve to a string`);
  }
  if (value.length === 0) {
    throw new TypeError(`${family} baseUrl must not be empty`);
  }
  if (value.includes("?") || value.includes("#")) {
    throw new TypeError(`${family} baseUrl must not include query or fragment components`);
  }
  if (isAbsoluteHttpUrl(value)) {
    validateBaseUrlPath(extractAbsoluteUrlPathname(value), family);
    return trimTrailingSlash(value);
  }
  if (!value.startsWith("/")) {
    throw new TypeError(`${family} baseUrl must start with / or use http(s)://`);
  }
  validateBaseUrlPath(value, family);
  return trimTrailingSlash(value);
}

function composeResourceBaseUrl(previous, next, sourceLabel) {
  if (next === null) {
    return previous;
  }
  if (previous === null) {
    return next;
  }
  if (isAbsoluteHttpUrl(next)) {
    throw new TypeError(
      `${sourceLabel} cannot compose an absolute baseUrl over inherited baseUrl "${previous}"`,
    );
  }
  if (next === "/") {
    return previous;
  }
  return `${previous}${next}`;
}

function composeBaseUrlWithRoute(baseUrl, routePath) {
  if (routePath === null) {
    return null;
  }
  if (baseUrl === null) {
    return routePath;
  }
  if (routePath === "/") {
    return `${baseUrl}/`;
  }
  return `${baseUrl}${routePath}`;
}

function trimTrailingSlash(value) {
  if (value === "/") {
    return value;
  }
  return value.endsWith("/") ? value.slice(0, -1) : value;
}

function isAbsoluteHttpUrl(value) {
  return value.startsWith("http://") || value.startsWith("https://");
}

function extractAbsoluteUrlPathname(value) {
  try {
    return new URL(value).pathname;
  } catch {
    throw new TypeError(`resource baseUrl must be a valid http(s):// URL`);
  }
}

function validateBaseUrlPath(pathname, family) {
  if (pathname === "/") {
    return;
  }
  for (const segment of pathname.slice(1).split("/")) {
    if (segment.length === 0) {
      throw new TypeError(
        `${family} baseUrl must not contain empty path segments`,
      );
    }
  }
}

export {
  composeBaseUrlWithRoute,
  composeResourceBaseUrl,
  requireResourceBaseUrl,
  resolveResourceBaseUrl,
};

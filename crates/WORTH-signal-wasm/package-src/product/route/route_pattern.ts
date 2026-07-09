function parseRoutePattern(route, sourceLabel = "route(...)") {
  if (typeof route !== "string" || route.length === 0) {
    throw new TypeError(`${sourceLabel} requires a non-empty route string`);
  }
  if (!route.startsWith("/")) {
    throw new TypeError(`${sourceLabel} routes must start with /`);
  }
  if (route === "/") {
    return Object.freeze({
      route,
      tokens: Object.freeze([]),
      pathParamNames: Object.freeze([]),
    });
  }
  const pathParamNames = [];
  const seen = new Set();
  const tokens = [];
  const segments = route.slice(1).split("/");
  for (const segment of segments) {
    if (segment.length === 0) {
      throw new TypeError(
        `${sourceLabel} route "${route}" must not contain empty path segments`,
      );
    }
    if (!segment.startsWith(":")) {
      tokens.push(Object.freeze({ kind: "literal", value: segment }));
      continue;
    }
    const name = segment.slice(1);
    validatePathParamName(route, segment, name, sourceLabel);
    if (seen.has(name)) {
      throw new TypeError(
        `${sourceLabel} route "${route}" must not repeat path param "${name}"`,
      );
    }
    seen.add(name);
    pathParamNames.push(name);
    tokens.push(Object.freeze({ kind: "param", name }));
  }
  return Object.freeze({
    route,
    tokens: Object.freeze(tokens),
    pathParamNames: Object.freeze(pathParamNames),
  });
}

function createRouteRequestPath(pattern, params, sourceLabel = "route(...)", options = {}) {
  if (!params || typeof params !== "object" || Array.isArray(params)) {
    throw new TypeError(
      `${sourceLabel} requires params to be an object containing the declared path params`,
    );
  }
  if (pattern.tokens.length === 0) {
    assertNoUndeclaredPathParams(pattern, params, sourceLabel, options.admittedKeys);
    return "/";
  }
  const path = pattern.tokens.map((token) =>
    renderRouteToken(pattern.route, token, params, sourceLabel),
  );
  assertNoUndeclaredPathParams(pattern, params, sourceLabel, options.admittedKeys);
  return `/${path.join("/")}`;
}

function matchRoutePath(pattern, pathname) {
  if (typeof pathname !== "string" || !pathname.startsWith("/")) {
    return null;
  }
  if (pattern.tokens.length === 0) {
    return pathname === "/" ? Object.freeze({}) : null;
  }
  const segments = pathname.slice(1).split("/");
  if (segments.length !== pattern.tokens.length) {
    return null;
  }
  const params = {};
  for (let index = 0; index < pattern.tokens.length; index += 1) {
    const token = pattern.tokens[index];
    const segment = segments[index];
    if (segment.length === 0) {
      return null;
    }
    if (token.kind === "literal") {
      if (segment !== token.value) {
        return null;
      }
      continue;
    }
    try {
      params[token.name] = decodeURIComponent(segment);
    } catch {
      return null;
    }
  }
  return Object.freeze(params);
}

function matchRoutePathPrefix(pattern, pathname) {
  if (typeof pathname !== "string" || !pathname.startsWith("/")) {
    return null;
  }
  if (pattern.tokens.length === 0) {
    return Object.freeze({});
  }
  const segments = pathname === "/" ? [] : pathname.slice(1).split("/");
  if (segments.length < pattern.tokens.length) {
    return null;
  }
  const params = {};
  for (let index = 0; index < pattern.tokens.length; index += 1) {
    const token = pattern.tokens[index];
    const segment = segments[index];
    if (segment === undefined || segment.length === 0) {
      return null;
    }
    if (token.kind === "literal") {
      if (segment !== token.value) {
        return null;
      }
      continue;
    }
    try {
      params[token.name] = decodeURIComponent(segment);
    } catch {
      return null;
    }
  }
  return Object.freeze(params);
}

function createRoutePatternProjectionShape(pattern) {
  if (pattern.tokens.length === 0) {
    return "/";
  }
  return pattern.tokens.map((token) =>
    token.kind === "literal" ? `literal:${token.value}` : "param:*",
  ).join("/");
}

function assertNoUndeclaredPathParams(pattern, params, sourceLabel, admittedKeys = []) {
  const admittedKeySet = new Set(admittedKeys);
  for (const name of Object.keys(params)) {
    if (admittedKeySet.has(name)) {
      continue;
    }
    if (!pattern.pathParamNames.includes(name)) {
      throw new TypeError(
        `${sourceLabel} does not admit undeclared path param "${name}"`,
      );
    }
  }
}

function renderRouteToken(route, token, params, sourceLabel) {
  if (token.kind === "literal") {
    return token.value;
  }
  if (!(token.name in params)) {
    throw new TypeError(
      `${sourceLabel} route "${route}" is missing required path param "${token.name}"`,
    );
  }
  return encodeRouteParamValue(route, token.name, params[token.name], sourceLabel);
}

function encodeRouteParamValue(route, name, value, sourceLabel) {
  if (typeof value !== "string" && typeof value !== "number") {
    throw new TypeError(
      `${sourceLabel} route "${route}" path param "${name}" must be a string or number`,
    );
  }
  return encodeURIComponent(String(value));
}

function validatePathParamName(route, segment, name, sourceLabel) {
  if (!isValidParamStart(name[0])) {
    throw new TypeError(
      `${sourceLabel} route segment "${segment}" in "${route}" must use :paramName placeholders`,
    );
  }
  for (let index = 1; index < name.length; index += 1) {
    if (!isValidParamPart(name[index])) {
      throw new TypeError(
        `${sourceLabel} route segment "${segment}" in "${route}" must use :paramName placeholders`,
      );
    }
  }
}

function isValidParamStart(character) {
  return isAsciiLetter(character) || character === "_";
}

function isValidParamPart(character) {
  return isAsciiLetter(character) || isAsciiDigit(character) || character === "_";
}

function isAsciiLetter(character) {
  if (typeof character !== "string" || character.length !== 1) {
    return false;
  }
  const code = character.charCodeAt(0);
  return (
    (code >= 65 && code <= 90) ||
    (code >= 97 && code <= 122)
  );
}

function isAsciiDigit(character) {
  if (typeof character !== "string" || character.length !== 1) {
    return false;
  }
  const code = character.charCodeAt(0);
  return code >= 48 && code <= 57;
}

export {
  createRoutePatternProjectionShape,
  createRouteRequestPath,
  matchRoutePath,
  matchRoutePathPrefix,
  parseRoutePattern,
};

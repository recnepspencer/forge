function parseApiRoutePattern(route) {
  if (typeof route !== "string" || route.length === 0) {
    throw new TypeError("api.url(...) requires a non-empty route string");
  }
  if (!route.startsWith("/")) {
    throw new TypeError("api.url(...) routes must start with /");
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
        `api.url(...) route "${route}" must not contain empty path segments`,
      );
    }
    if (segment.startsWith(":")) {
      const name = segment.slice(1);
      validatePathParamName(route, segment, name);
      if (seen.has(name)) {
        throw new TypeError(
          `api.url(...) route "${route}" must not repeat path param "${name}"`,
        );
      }
      seen.add(name);
      pathParamNames.push(name);
      tokens.push(Object.freeze({ kind: "param", name }));
      continue;
    }
    tokens.push(Object.freeze({ kind: "literal", value: segment }));
  }
  return Object.freeze({
    route,
    tokens: Object.freeze(tokens),
    pathParamNames: Object.freeze(pathParamNames),
  });
}

function createRouteBoundParams(pattern, rawParams) {
  if (!rawParams || typeof rawParams !== "object" || Array.isArray(rawParams)) {
    throw new TypeError(
      `${pattern.route} line(...) requires an object containing exactly the declared path params`,
    );
  }
  const params = {};
  for (const name of pattern.pathParamNames) {
    if (!(name in rawParams)) {
      throw new TypeError(
        `${pattern.route} line(...) is missing required path param "${name}"`,
      );
    }
    params[name] = rawParams[name];
  }
  for (const name of Object.keys(rawParams)) {
    if (!pattern.pathParamNames.includes(name)) {
      throw new TypeError(
        `${pattern.route} line(...) does not admit undeclared param "${name}" before params(...) exists`,
      );
    }
  }
  return Object.freeze(params);
}

function createRouteCanonicalKey(pattern, params) {
  if (pattern.tokens.length === 0) {
    return "/";
  }
  return `/${pattern.tokens.map((token) => renderRouteToken(pattern.route, token, params)).join("/")}`;
}

function encodeRouteParamValue(route, name, value) {
  if (
    typeof value !== "string"
    && typeof value !== "number"
    && typeof value !== "boolean"
  ) {
    throw new TypeError(
      `${route} path param "${name}" must be a string, number, or boolean`,
    );
  }
  return encodeURIComponent(String(value));
}

function renderRouteToken(route, token, params) {
  if (token.kind === "literal") {
    return token.value;
  }
  return encodeRouteParamValue(route, token.name, params[token.name]);
}

function validatePathParamName(route, segment, name) {
  if (!isValidParamStart(name[0])) {
    throw new TypeError(
      `api.url(...) route segment "${segment}" in "${route}" must use :paramName placeholders`,
    );
  }
  for (let index = 1; index < name.length; index += 1) {
    if (!isValidParamPart(name[index])) {
      throw new TypeError(
        `api.url(...) route segment "${segment}" in "${route}" must use :paramName placeholders`,
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
    (code >= 65 && code <= 90)
    || (code >= 97 && code <= 122)
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
  createRouteBoundParams,
  createRouteCanonicalKey,
  parseApiRoutePattern,
};

import {
  createRouteQueryString,
  createRouteRequestParams,
} from "./api_route_request_params.js";
import { createApiRouteBodyCanonicalSuffix } from "./api_route_body_identity.js";
import {
  createRouteRequestPath as createSharedRouteRequestPath,
  parseRoutePattern,
} from "../../route/route_pattern.js";

function parseApiRoutePattern(route) {
  return parseRoutePattern(route, "api.url(...)");
}

function createRouteBoundParams(
  pattern,
  requestParamsState,
  rawParams,
  bodyMode = "forbidden",
) {
  if (!rawParams || typeof rawParams !== "object" || Array.isArray(rawParams)) {
    throw new TypeError(
      `${pattern.route} line(...) requires an object containing exactly the declared path params`,
    );
  }
  const requestParams = requestParamsState.declared
    ? requireDeclaredRouteRequestParams(pattern, rawParams)
    : undefined;
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
    if (requestParamsState.declared && name === "params") {
      continue;
    }
    if (bodyMode !== "forbidden" && name === "body") {
      continue;
    }
    if (!pattern.pathParamNames.includes(name)) {
      throw new TypeError(
        `${pattern.route} line(...) does not admit undeclared path param "${name}"`,
      );
    }
  }
  if (requestParams !== undefined) {
    params.params = requestParams;
  }
  if (bodyMode === "required") {
    if (!("body" in rawParams) || rawParams.body === undefined) {
      throw new TypeError(
        `${pattern.route} line(...) requires an explicit body value for this write declaration`,
      );
    }
    params.body = rawParams.body;
  } else if (bodyMode === "forbidden" && "body" in rawParams) {
    throw new TypeError(
      `${pattern.route} line(...) does not admit a body for this declaration`,
    );
  }
  return Object.freeze(params);
}

function createRouteCanonicalKey(pattern, params, includeBody = false) {
  return `${createRouteRequestPath(pattern, params)}${createRouteQueryString(pattern.route, params.params ?? {})}${includeBody ? createApiRouteBodyCanonicalSuffix(pattern.route, params.body) : ""}`;
}

function createRouteRequestPath(pattern, params) {
  return createSharedRouteRequestPath(
    pattern,
    params,
    `api.url("${pattern.route}").line(...)`,
    { admittedKeys: ["params", "body"] },
  );
}

function requireDeclaredRouteRequestParams(pattern, rawParams) {
  if ("params" in rawParams) {
    return createRouteRequestParams(rawParams.params, pattern.route);
  }
  throw new TypeError(
    `${pattern.route} line(...) requires an explicit params object when request params are declared`,
  );
}

export {
  createRouteBoundParams,
  createRouteCanonicalKey,
  createRouteRequestPath,
  parseApiRoutePattern,
};

import { createRouteDeclaration, isRouteDeclaration } from "../router_declaration.js";
import { ROUTE_LAYOUT_DECLARATION } from "../router_symbols.js";

function createRouteLayoutDeclaration(routeOrDeclaration, optionsOrChildren, maybeChildren) {
  const routeDeclaration = normalizeLayoutRouteDeclaration(routeOrDeclaration);
  const { outletId, children } = normalizeLayoutArguments(
    routeDeclaration.route,
    optionsOrChildren,
    maybeChildren,
  );
  return Object.freeze({
    [ROUTE_LAYOUT_DECLARATION]: true,
    route: routeDeclaration,
    outletId,
    children,
  });
}

function isRouteLayoutDeclaration(value) {
  return Boolean(value && value[ROUTE_LAYOUT_DECLARATION] === true);
}

function normalizeLayoutRouteDeclaration(routeOrDeclaration) {
  if (isRouteDeclaration(routeOrDeclaration)) {
    assertLayoutRouteDoesNotDeclareSearchOrHash(routeOrDeclaration);
    return routeOrDeclaration;
  }
  return createRouteDeclaration(routeOrDeclaration, {});
}

function assertLayoutRouteDoesNotDeclareSearchOrHash(routeDeclaration) {
  if (Object.keys(routeDeclaration.search).length > 0 || routeDeclaration.hash !== null) {
    throw new TypeError(
      `signals.router.layout("${routeDeclaration.route}") does not yet admit declared search or hash state`,
    );
  }
}

function normalizeLayoutArguments(route, optionsOrChildren, maybeChildren) {
  if (maybeChildren === undefined) {
    return {
      outletId: "default",
      children: normalizeLayoutChildren(
        route,
        optionsOrChildren,
        "signals.router.layout(..., children)",
      ),
    };
  }
  const outletId = normalizeOutletId(route, optionsOrChildren);
  return {
    outletId,
    children: normalizeLayoutChildren(
      route,
      maybeChildren,
      "signals.router.layout(..., options, children)",
    ),
  };
}

function normalizeOutletId(route, options) {
  if (options === undefined) {
    return "default";
  }
  if (!isPlainObject(options)) {
    throw new TypeError(
      `signals.router.layout("${route}") options must be an object when provided`,
    );
  }
  const outletId = options.outlet ?? "default";
  if (typeof outletId !== "string" || outletId.trim().length === 0) {
    throw new TypeError(
      `signals.router.layout("${route}") outlet must be a non-empty string when provided`,
    );
  }
  return outletId;
}

function normalizeLayoutChildren(route, children, label) {
  if (!isPlainObject(children)) {
    throw new TypeError(
      `signals.router.layout("${route}") ${label} requires a nested route object`,
    );
  }
  return Object.freeze({ ...children });
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  createRouteLayoutDeclaration,
  isRouteLayoutDeclaration,
};

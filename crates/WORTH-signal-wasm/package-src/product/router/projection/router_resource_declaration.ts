import { ROUTE_RESOURCE_DECLARATION } from "../router_symbols.js";

const ROUTE_RESOURCE_PREFETCH_POSTURES = new Set([
  "hover",
  "focus",
  "viewport",
  "intent",
]);

function createRouteResourceDeclaration(resourceFamily, options = {}) {
  if (!isRouteResourceFamily(resourceFamily)) {
    throw new TypeError(
      "signals.router.resourceLine(...) requires a resource family created by signals.resource.detail/collection/paged(...)",
    );
  }
  if (typeof options.params !== "function") {
    throw new TypeError(
      "signals.router.resourceLine(...) requires a params(route) resolver",
    );
  }
  const prefetch = options.prefetch ?? "intent";
  if (!ROUTE_RESOURCE_PREFETCH_POSTURES.has(prefetch)) {
    throw new TypeError(
      'signals.router.resourceLine(...) prefetch must be "hover", "focus", "viewport", or "intent"',
    );
  }
  return Object.freeze({
    [ROUTE_RESOURCE_DECLARATION]: true,
    family: resourceFamily,
    resolveParams: options.params,
    prefetch,
  });
}

function normalizeRouteResources(route, resources) {
  if (resources === undefined) {
    return Object.freeze({});
  }
  if (!isPlainObject(resources)) {
    throw new TypeError(
      `signals.router.route("${route}") resources must be an object when provided`,
    );
  }
  const normalized = {};
  for (const [key, value] of Object.entries(resources)) {
    if (!isRouteResourceDeclaration(value)) {
      throw new TypeError(
        `signals.router.route("${route}") resources["${key}"] must be declared with signals.router.resourceLine(...)`,
      );
    }
    normalized[key] = value;
  }
  return Object.freeze(normalized);
}

function isRouteResourceDeclaration(value) {
  return Boolean(value && value[ROUTE_RESOURCE_DECLARATION] === true);
}

function isRouteResourceFamily(value) {
  return Boolean(
    value &&
    typeof value.invalidate === "function" &&
    typeof value.invalidateAll === "function" &&
    typeof value.line === "function",
  );
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function isRouteResourcePrefetchPosture(value) {
  return ROUTE_RESOURCE_PREFETCH_POSTURES.has(value);
}

function requireRouteResourcePrefetchPosture(value, context) {
  if (!isRouteResourcePrefetchPosture(value)) {
    throw new TypeError(
      `${context} trigger must be "hover", "focus", "viewport", or "intent"`,
    );
  }
  return value;
}

function routeResourceMatchesWarmupTrigger(resourceDeclaration, trigger) {
  requireRouteResourcePrefetchPosture(trigger, "route resource warmup");
  return trigger === "intent" || resourceDeclaration.prefetch === trigger;
}

export {
  createRouteResourceDeclaration,
  isRouteResourceDeclaration,
  isRouteResourcePrefetchPosture,
  normalizeRouteResources,
  requireRouteResourcePrefetchPosture,
  routeResourceMatchesWarmupTrigger,
};

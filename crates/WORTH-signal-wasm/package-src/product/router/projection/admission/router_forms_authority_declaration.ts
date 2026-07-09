import { ROUTE_FORMS_AUTHORITY_DECLARATION } from "../../router_symbols.js";

const CONTINUITIES = new Set(["preserve", "freeze", "discard", "defer"]);

export function createRouteFormsAuthorityDeclaration(surfaceId, options = {}) {
  if (typeof surfaceId !== "string" || surfaceId.length === 0) {
    throw new TypeError("signals.router.forms(...) requires a non-empty surface id");
  }
  const continuity = options.continuity ?? "preserve";
  if (!CONTINUITIES.has(continuity)) {
    throw new TypeError(
      `signals.router.forms("${surfaceId}") continuity must be preserve, freeze, discard, or defer`,
    );
  }
  return Object.freeze({
    [ROUTE_FORMS_AUTHORITY_DECLARATION]: true,
    surfaceId,
    continuity,
    reason: options.reason === undefined ? null : String(options.reason),
  });
}

export function normalizeRouteFormsAuthorityDeclaration(route, declaration) {
  if (declaration === undefined) {
    return null;
  }
  if (!declaration || declaration[ROUTE_FORMS_AUTHORITY_DECLARATION] !== true) {
    throw new TypeError(
      `signals.router.route("${route}") forms must be declared with signals.router.forms(...)`,
    );
  }
  return declaration;
}

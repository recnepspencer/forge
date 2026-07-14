import { ROUTE_FORMS_AUTHORITY } from "../../router_symbols.js";
import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

export function createRouteFormsAuthorityArtifact(routeDeclaration, admittedRouteCapability) {
  if (routeDeclaration.forms === null) {
    return null;
  }
  const verification = Object.freeze({
    formsAuthorityDigest: createCanonicalDigest("route-forms-authority", {
      routeId: admittedRouteCapability.routeId,
      href: admittedRouteCapability.href,
      scopeKind: "route",
      surfaceId: routeDeclaration.forms.surfaceId,
      continuity: routeDeclaration.forms.continuity,
      reason: routeDeclaration.forms.reason,
    }),
  });
  return Object.freeze({
    [ROUTE_FORMS_AUTHORITY]: true,
    kind: "routeFormsAuthority",
    routeId: admittedRouteCapability.routeId,
    href: admittedRouteCapability.href,
    scopeKind: "route",
    surfaceId: routeDeclaration.forms.surfaceId,
    continuity: routeDeclaration.forms.continuity,
    reason: routeDeclaration.forms.reason,
    verification() {
      return verification;
    },
  });
}

export function isRouteFormsAuthorityArtifact(value) {
  return Boolean(
    value
    && typeof value === "object"
    && value[ROUTE_FORMS_AUTHORITY] === true
    && value.kind === "routeFormsAuthority",
  );
}

export function requireRouteFormsAuthorityArtifact(value) {
  if (isRouteFormsAuthorityArtifact(value)) {
    return value;
  }
  throw new TypeError(
    "form.reportRouteAuthority(...) requires a route forms authority artifact from router admission",
  );
}

import {
  createRouteRequestPath,
  matchRoutePath,
} from "../route/route_pattern.js";
import { createHashString, normalizeHashInput, parseHashState } from "./router_hash_state.js";
import {
  createSearchString,
  normalizeSearchInput,
  parseSearchState,
} from "./router_search_state.js";
import {
  createNavigationIntentBuilder,
  createNavigationPlan,
} from "./navigation/router_navigation_plan.js";
import { ROUTE_LOCATION, ROUTE_REFERENCE } from "./router_symbols.js";
import { createCanonicalRouteArtifact } from "./url_authority/router_canonical_artifact.js";
import { createRouteReferenceVerification } from "./url_authority/router_verification_packages.js";
import {
  coerceRouteMatchHref,
  tryParseAdmittedUrl,
} from "./url_authority/router_url_authority.js";

function createRouteReference(declaration, path, scopeId) {
  const descriptor = createRouteDescriptor(declaration, path, scopeId);
  const referenceVerification = createRouteReferenceVerification(declaration, descriptor);
  const reference = {
    [ROUTE_REFERENCE]: true,
    descriptor() {
      return descriptor;
    },
    verification() {
      return referenceVerification;
    },
    canonical(input = {}) {
      return createCanonicalArtifact(
        declaration,
        descriptor,
        input,
        referenceVerification,
      );
    },
    href(input = {}) {
      return createRouteHref(declaration, input);
    },
    to(input = {}) {
      return createRouteLocation(reference, declaration, descriptor, input, referenceVerification);
    },
    intent(input = {}, options = {}) {
      return createRouteLocation(
        reference,
        declaration,
        descriptor,
        input,
        referenceVerification,
      ).intent(options);
    },
    match(routeAuthority) {
      const parsed = matchRouteLocation(declaration, routeAuthority);
      if (!parsed) {
        return null;
      }
      return createRouteLocation(
        reference,
        declaration,
        descriptor,
        parsed,
        referenceVerification,
      );
    },
  };
  return Object.freeze(reference);
}

function createRouteDescriptor(declaration, path, scopeId) {
  const routeId = scopeId ? `${scopeId}:${path.join(".")}` : path.join(".");
  return Object.freeze({
    routeId,
    scopeId,
    declarationPath: Object.freeze(path.slice()),
    route: declaration.route,
    pathParamNames: declaration.pattern.pathParamNames,
    searchKeys: Object.freeze(Object.keys(declaration.search)),
    hash: declaration.hash,
  });
}

function createRouteLocation(reference, declaration, descriptor, input, referenceVerification) {
  const normalized = normalizeRouteInput(declaration, input);
  const canonical = createCanonicalArtifactFromNormalized(
    declaration,
    descriptor,
    normalized,
    referenceVerification,
  );
  return Object.freeze({
    [ROUTE_LOCATION]: true,
    route: reference,
    routeId: descriptor.routeId,
    params: normalized.params,
    search: normalized.search,
    hash: normalized.hash,
    href: canonical.href,
    descriptor() {
      return descriptor;
    },
    canonical() {
      return canonical;
    },
    intent(options = {}) {
      return createNavigationIntentBuilder(this, options);
    },
    plan(policy = {}) {
      return createNavigationPlan(this, "push", policy);
    },
  });
}

function createRouteHref(declaration, input) {
  return createCanonicalHref(declaration, normalizeRouteInput(declaration, input));
}

function matchRouteLocation(declaration, routeAuthority) {
  const url = parseRouteUrl(coerceRouteMatchHref(routeAuthority));
  if (url === null) {
    return null;
  }
  const params = matchRoutePath(declaration.pattern, url.pathname);
  if (!params) {
    return null;
  }
  const search = parseSearchState(declaration.search, routeLabel(declaration), url.searchParams);
  if (search === null) {
    return null;
  }
  const hash = parseHashState(declaration.hash, url.hash);
  if (hash === null) {
    return null;
  }
  return {
    params,
    search,
    hash,
  };
}

function isRouteLocation(value) {
  return Boolean(value && value[ROUTE_LOCATION] === true);
}

function createCanonicalHref(declaration, input) {
  return createCanonicalHrefParts(declaration, input).href;
}

function createCanonicalArtifact(
  declaration,
  descriptor,
  input,
  referenceVerification,
) {
  return createCanonicalArtifactFromNormalized(
    declaration,
    descriptor,
    normalizeRouteInput(declaration, input),
    referenceVerification,
  );
}

function createCanonicalArtifactFromNormalized(
  declaration,
  descriptor,
  normalized,
  referenceVerification,
) {
  return createCanonicalRouteArtifact(
    descriptor,
    normalized,
    createCanonicalHrefParts(declaration, normalized),
    referenceVerification,
  );
}

function createCanonicalHrefParts(declaration, input) {
  const route = routeLabel(declaration);
  const pathname = createRouteRequestPath(
    declaration.pattern,
    input.params,
    routeOperationLabel(route, "href"),
  );
  const searchString = createSearchString(route, declaration.search, input.search);
  const hashString = createHashString(route, declaration.hash, input.hash);
  return Object.freeze({
    pathname,
    searchString,
    hashString,
    href: `${pathname}${searchString}${hashString}`,
  });
}

function normalizeRouteInput(declaration, input) {
  if (!isPlainObject(input)) {
    throw new TypeError(
      "signals.router route references require an object with params/search/hash fields",
    );
  }
  const route = routeLabel(declaration);
  return Object.freeze({
    params: Object.freeze({ ...(input.params ?? {}) }),
    search: Object.freeze(normalizeSearchInput(route, declaration.search, input.search ?? {})),
    hash: normalizeHashInput(route, declaration.hash, input.hash),
  });
}

function parseRouteUrl(rawHref) {
  return tryParseAdmittedUrl(rawHref);
}

function routeLabel(declaration) {
  return declaration.route;
}

function routeOperationLabel(route, operation) {
  return `signals.router.route("${route}").${operation}(...)`;
}

function isPlainObject(value) {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

export {
  createRouteReference,
  isRouteLocation,
};

import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createBrowserAuthorityBoundaryArtifact,
  normalizeOptionalBrowserAuthorityCoherence,
} from "./router_browser_authority_coherence.js";
import {
  normalizeOptionalCarriedBreadcrumbs,
  normalizeOptionalRestoredBreadcrumbs,
  requireCarriedBreadcrumbsArtifact,
  requireRestoredBreadcrumbsArtifact,
} from "../breadcrumb/router_breadcrumb_provenance.js";
import { normalizeOptionalRouteRestoreBoundary } from "./router_restore_boundary.js";
import {
  createRawLocationAuthority,
  isRawLocationAuthority,
} from "../../url_authority/router_url_authority.js";

const BROWSER_HISTORY_INGRESS = Symbol("forge.router.browser-history-ingress");

function createBrowserHistoryNamespace() {
  return Object.freeze({
    load(location, options) {
      return createRouterBrowserHistoryIngress("load", location, options);
    },
    push(location, options) {
      return createRouterBrowserHistoryIngress("pushstate", location, options);
    },
    replace(location, options) {
      return createRouterBrowserHistoryIngress("replacestate", location, options);
    },
    pop(location, options) {
      return createRouterBrowserHistoryIngress("popstate", location, options);
    },
    manual(location, options) {
      return createRouterBrowserHistoryIngress("manual", location, options);
    },
    external(location, options) {
      return createRouterBrowserHistoryIngress("external", location, options);
    },
  });
}

function createRouterBrowserHistoryIngress(navigationKind, location, options = {}) {
  const rawLocation = normalizeIngressRawLocation(location, navigationKind);
  const normalized = normalizeIngressBindings(options, navigationKind);
  const verification = Object.freeze({
    browserHistoryEnvelopeDigest: createCanonicalDigest("browser-history-ingress", {
      navigationKind,
      rawLocationDigest: rawLocation.verification().rawLocationDigest,
      rawLocationHref: rawLocation.href,
      routeIdentity: normalized.routeIdentity,
      runtimeRouteSourceId: normalized.runtimeRouteSourceId,
      routeValue: normalized.routeValue,
      runtimeContinuitySourceId: normalized.runtimeContinuitySourceId,
      continuityValue: normalized.continuityValue,
      coherenceDigest:
        normalized.coherence?.verification().browserAuthorityCoherenceDigest ?? null,
      carriedBreadcrumbsDigest:
        normalized.carriedBreadcrumbs?.verification().carriedBreadcrumbsDigest ?? null,
      restoredBreadcrumbsDigest:
        normalized.restoredBreadcrumbs?.verification().restoredBreadcrumbsDigest ?? null,
      restoreBoundaryDigest:
        normalized.restoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
    }),
  });
  return Object.freeze({
    [BROWSER_HISTORY_INGRESS]: true,
    kind: "routerBrowserHistoryIngress",
    navigationKind,
    rawLocation,
    routeIdentity: normalized.routeIdentity,
    runtimeRouteSourceId: normalized.runtimeRouteSourceId,
    routeValue: normalized.routeValue,
    runtimeContinuitySourceId: normalized.runtimeContinuitySourceId,
    continuityValue: normalized.continuityValue,
    coherence: normalized.coherence,
    carriedBreadcrumbs: normalized.carriedBreadcrumbs,
    restoredBreadcrumbs: normalized.restoredBreadcrumbs,
    restoreBoundary: normalized.restoreBoundary,
    verification() {
      return verification;
    },
  });
}

function isRouterBrowserHistoryIngress(value) {
  return Boolean(
    value
    && typeof value === "object"
    && value[BROWSER_HISTORY_INGRESS] === true
    && value.kind === "routerBrowserHistoryIngress",
  );
}

function requireRouterBrowserHistoryIngress(value, operation) {
  if (isRouterBrowserHistoryIngress(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires an ingress envelope created by signals.router.browserHistory.*(...)`,
  );
}

function normalizeWorkerBrowserHistoryIngress(ingress, operation) {
  if (isRouterBrowserHistoryIngress(ingress)) {
    if (typeof ingress.routeIdentity !== "string" || ingress.routeIdentity.length === 0) {
      throw new TypeError(`${operation} requires routeIdentity for worker browser-history ingress`);
    }
    return Object.freeze({
      navigationKind: ingress.navigationKind,
      rawLocation: ingress.rawLocation.href,
      routeIdentity: ingress.routeIdentity,
      ...(ingress.runtimeRouteSourceId === null ? {} : { runtimeRouteSourceId: ingress.runtimeRouteSourceId }),
      ...(ingress.routeValue === undefined ? {} : { routeValue: ingress.routeValue }),
      ...(ingress.runtimeContinuitySourceId === null
        ? {}
        : { runtimeContinuitySourceId: ingress.runtimeContinuitySourceId }),
      ...(ingress.continuityValue === undefined ? {} : { continuityValue: ingress.continuityValue }),
      ...(ingress.coherence === null
        ? {}
        : { coherenceDigest: ingress.coherence.verification().browserAuthorityCoherenceDigest }),
      ...(ingress.carriedBreadcrumbs === null
        ? {}
        : { carriedBreadcrumbsDigest: ingress.carriedBreadcrumbs.verification().carriedBreadcrumbsDigest }),
      ...(ingress.restoredBreadcrumbs === null
        ? {}
        : { restoredBreadcrumbsDigest: ingress.restoredBreadcrumbs.verification().restoredBreadcrumbsDigest }),
    });
  }
  return normalizeLegacyWorkerBrowserHistoryIngress(ingress, operation);
}

function createBrowserHistoryAdmissionReport(ingress, routeOutcome) {
  const diagnostics = createIngressDiagnostics(ingress, routeOutcome);
  const verification = Object.freeze({
    browserHistoryEnvelopeDigest: ingress.verification().browserHistoryEnvelopeDigest,
    routeTruthDigest: createCanonicalDigest("browser-history-route-truth", {
      browserHistoryEnvelopeDigest: ingress.verification().browserHistoryEnvelopeDigest,
      routeOutcomeDigest: routeOutcome.verification().routeOutcomeDigest,
      routeId: routeOutcome.routeId,
      href: routeOutcome.href,
      outcomeKind: routeOutcome.kind,
    }),
    continuityDigest: createCanonicalDigest("browser-history-continuity", {
      browserHistoryEnvelopeDigest: ingress.verification().browserHistoryEnvelopeDigest,
      continuityValue: ingress.continuityValue ?? null,
      coherenceDigest:
        ingress.coherence?.verification().browserAuthorityCoherenceDigest ?? null,
      boundaryArtifact: diagnostics.boundaryArtifact,
      outcomeKind: routeOutcome.kind,
      carriedBreadcrumbsDigest:
        ingress.carriedBreadcrumbs?.verification().carriedBreadcrumbsDigest ?? null,
      restoredBreadcrumbsDigest:
        ingress.restoredBreadcrumbs?.verification().restoredBreadcrumbsDigest ?? null,
      restoreBoundaryDigest:
        ingress.restoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
    }),
  });
  return Object.freeze({
    envelopeFamily: "browserHistoryIngress",
    navigationKind: ingress.navigationKind,
    rawLocationHref: ingress.rawLocation.href,
    routeIdentity: ingress.routeIdentity,
    runtimeRouteSourceId: ingress.runtimeRouteSourceId,
    runtimeContinuitySourceId: ingress.runtimeContinuitySourceId,
    coherence() {
      return ingress.coherence;
    },
    carriedBreadcrumbs() {
      return ingress.carriedBreadcrumbs;
    },
    restoredBreadcrumbs() {
      return ingress.restoredBreadcrumbs;
    },
    restoreBoundary() {
      return ingress.restoreBoundary;
    },
    outcome() {
      return routeOutcome;
    },
    diagnostics() {
      return diagnostics;
    },
    verification() {
      return verification;
    },
  });
}

function createIngressDiagnostics(ingress, routeOutcome) {
  return Object.freeze({
    boundarySource: "browserHistoryIngress",
    boundaryArtifact: createBrowserAuthorityBoundaryArtifact(ingress, routeOutcome),
    navigationKind: ingress.navigationKind,
    rawLocationHref: ingress.rawLocation.href,
    routeIdentity: ingress.routeIdentity,
    coherenceKind: ingress.coherence?.coherenceKind ?? null,
    outcomeKind: routeOutcome.kind,
    routeId: routeOutcome.routeId,
    href: routeOutcome.href,
  });
}

function normalizeIngressRawLocation(location, navigationKind) {
  if (typeof location === "string") {
    return createRawLocationAuthority(location, {
      navigationType: rawLocationNavigationTypeFor(navigationKind),
    });
  }
  if (isRawLocationAuthority(location)) {
    return location;
  }
  throw new TypeError(
    "signals.router.browserHistory.*(...) requires a local href string or raw location authority",
  );
}

function rawLocationNavigationTypeFor(navigationKind) {
  switch (navigationKind) {
    case "pushstate":
      return "push";
    case "replacestate":
      return "replace";
    case "popstate":
      return "pop";
    case "external":
      return "external";
    default:
      return navigationKind;
  }
}

function normalizeIngressBindings(options, navigationKind) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...) options must be an object when provided`,
    );
  }
  const {
    routeIdentity = null,
    runtimeRouteSourceId = null,
    routeValue,
    runtimeContinuitySourceId = null,
    continuityValue,
    coherence = null,
    carriedBreadcrumbs = null,
    restoredBreadcrumbs = null,
    restoreBoundary = null,
    ...unknownOptions
  } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  return Object.freeze({
    routeIdentity: normalizeOptionalString(routeIdentity, "routeIdentity"),
    runtimeRouteSourceId: normalizeOptionalString(runtimeRouteSourceId, "runtimeRouteSourceId"),
    routeValue,
    runtimeContinuitySourceId: normalizeOptionalString(
      runtimeContinuitySourceId,
      "runtimeContinuitySourceId",
    ),
    continuityValue,
    coherence: normalizeOptionalBrowserAuthorityCoherence(
      coherence,
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...)`,
    ),
    carriedBreadcrumbs: normalizeOptionalCarriedBreadcrumbs(
      carriedBreadcrumbs,
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...)`,
    ),
    restoredBreadcrumbs: normalizeOptionalRestoredBreadcrumbs(
      restoredBreadcrumbs,
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...)`,
    ),
    restoreBoundary: normalizeOptionalRouteRestoreBoundary(
      restoreBoundary,
      `signals.router.browserHistory.${displayNavigationMethod(navigationKind)}(...)`,
    ),
  });
}

function normalizeLegacyWorkerBrowserHistoryIngress(ingress, operation) {
  if (!ingress || typeof ingress !== "object" || Array.isArray(ingress)) {
    throw new TypeError(`${operation} expects a browser-history ingress object`);
  }
  const {
    navigationKind,
    rawLocation,
    routeIdentity,
    runtimeRouteSourceId,
    routeValue,
    runtimeContinuitySourceId,
    continuityValue,
    coherence,
    carriedBreadcrumbs,
    restoredBreadcrumbs,
    ...unknownOptions
  } = ingress;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(`${operation} does not support: ${unknownKeys.join(", ")}`);
  }
  return Object.freeze({
    navigationKind: requireNavigationKind(navigationKind, operation),
    rawLocation: requireNonEmptyString(rawLocation, `${operation} rawLocation`),
    routeIdentity: requireNonEmptyString(routeIdentity, `${operation} routeIdentity`),
    ...(runtimeRouteSourceId === undefined
      ? {}
      : { runtimeRouteSourceId: requireNonEmptyString(runtimeRouteSourceId, `${operation} runtimeRouteSourceId`) }),
    ...(routeValue === undefined ? {} : { routeValue }),
    ...(runtimeContinuitySourceId === undefined
      ? {}
      : {
          runtimeContinuitySourceId: requireNonEmptyString(
            runtimeContinuitySourceId,
            `${operation} runtimeContinuitySourceId`,
          ),
        }),
    ...(continuityValue === undefined ? {} : { continuityValue }),
    ...(coherence === undefined || coherence === null
      ? {}
      : {
          coherenceDigest: normalizeOptionalBrowserAuthorityCoherence(
            coherence,
            operation,
          ).verification().browserAuthorityCoherenceDigest,
        }),
    ...(carriedBreadcrumbs === undefined || carriedBreadcrumbs === null
      ? {}
      : {
          carriedBreadcrumbsDigest: requireCarriedBreadcrumbsArtifact(
            carriedBreadcrumbs,
            operation,
          ).verification().carriedBreadcrumbsDigest,
        }),
    ...(restoredBreadcrumbs === undefined || restoredBreadcrumbs === null
      ? {}
      : {
          restoredBreadcrumbsDigest: requireRestoredBreadcrumbsArtifact(
            restoredBreadcrumbs,
            operation,
          ).verification().restoredBreadcrumbsDigest,
        }),
  });
}

function requireNavigationKind(value, operation) {
  if (
    value === "load" ||
    value === "pushstate" ||
    value === "replacestate" ||
    value === "popstate" ||
    value === "manual" ||
    value === "external"
  ) {
    return value;
  }
  throw new TypeError(`${operation} navigationKind must be one of load, pushstate, replacestate, popstate, manual, external`);
}

function requireNonEmptyString(value, label) {
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string`);
}

function normalizeOptionalString(value, label) {
  if (value === null || value === undefined) {
    return null;
  }
  return requireNonEmptyString(value, label);
}

function displayNavigationMethod(navigationKind) {
  switch (navigationKind) {
    case "pushstate":
      return "push";
    case "replacestate":
      return "replace";
    case "popstate":
      return "pop";
    default:
      return navigationKind;
  }
}

export {
  createBrowserHistoryAdmissionReport,
  createBrowserHistoryNamespace,
  createRouterBrowserHistoryIngress,
  isRouterBrowserHistoryIngress,
  normalizeWorkerBrowserHistoryIngress,
  requireRouterBrowserHistoryIngress,
};

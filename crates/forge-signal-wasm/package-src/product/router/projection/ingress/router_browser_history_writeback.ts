import { isRouteLocation } from "../../router_location.js";
import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createBrowserAuthorityBoundaryArtifact,
  normalizeOptionalBrowserAuthorityCoherence,
} from "./router_browser_authority_coherence.js";
import {
  normalizeOptionalCarriedBreadcrumbs,
  normalizeOptionalRestoredBreadcrumbs,
} from "../breadcrumb/router_breadcrumb_provenance.js";
import { normalizeOptionalRouteRestoreBoundary } from "./router_restore_boundary.js";
import {
  createRawLocationAuthority,
  isRawLocationAuthority,
} from "../../url_authority/router_url_authority.js";

const BROWSER_HISTORY_WRITEBACK = Symbol("forge.router.browser-history-writeback");

function createBrowserHistoryWritebackNamespace() {
  return Object.freeze({
    push(target, options) {
      return createRouterBrowserHistoryWriteback("pushstate", target, options);
    },
    replace(target, options) {
      return createRouterBrowserHistoryWriteback("replacestate", target, options);
    },
    external(target, options) {
      return createRouterBrowserHistoryWriteback("external", target, options);
    },
  });
}

function createRouterBrowserHistoryWriteback(navigationKind, target, options = {}) {
  const normalizedTarget = normalizeWritebackTarget(target, navigationKind);
  const normalized = normalizeWritebackBindings(options, navigationKind);
  const verification = Object.freeze({
    browserHistoryWritebackDigest: createCanonicalDigest("browser-history-writeback", {
      navigationKind,
      targetHref: normalizedTarget.href,
      targetKind: normalizedTarget.kind,
      ...(normalizedTarget.kind === "local"
        ? {
            rawLocationDigest: normalizedTarget.rawLocation.verification().rawLocationDigest,
          }
        : {}),
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
    [BROWSER_HISTORY_WRITEBACK]: true,
    kind: "routerBrowserHistoryWriteback",
    navigationKind,
    targetKind: normalizedTarget.kind,
    targetHref: normalizedTarget.href,
    rawLocation: normalizedTarget.kind === "local" ? normalizedTarget.rawLocation : null,
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

function isRouterBrowserHistoryWriteback(value) {
  return Boolean(
    value
    && typeof value === "object"
    && value[BROWSER_HISTORY_WRITEBACK] === true
    && value.kind === "routerBrowserHistoryWriteback",
  );
}

function requireRouterBrowserHistoryWriteback(value, operation) {
  if (isRouterBrowserHistoryWriteback(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires a writeback envelope created by signals.router.browserHistory.writeback.*(...)`,
  );
}

function createBrowserHistoryWritebackReport(writeback, routeOutcome) {
  const diagnostics = createWritebackDiagnostics(writeback, routeOutcome);
  const verification = Object.freeze({
    browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
    routeTruthDigest: createCanonicalDigest("browser-history-writeback-route-truth", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      boundaryArtifact: diagnostics.boundaryArtifact,
      routeOutcomeDigest: routeOutcome?.verification().routeOutcomeDigest ?? null,
      routeId: routeOutcome?.routeId ?? null,
      href: routeOutcome?.href ?? null,
      outcomeKind: routeOutcome?.kind ?? null,
      coherenceDigest:
        writeback.coherence?.verification().browserAuthorityCoherenceDigest ?? null,
    }),
    boundaryStoryDigest: createCanonicalDigest("browser-history-writeback-boundary-story", {
      browserHistoryWritebackDigest: writeback.verification().browserHistoryWritebackDigest,
      boundarySource: diagnostics.boundarySource,
      boundaryArtifact: diagnostics.boundaryArtifact,
      targetHref: diagnostics.targetHref,
      routeId: diagnostics.routeId,
      href: diagnostics.href,
      carriedBreadcrumbsDigest:
        writeback.carriedBreadcrumbs?.verification().carriedBreadcrumbsDigest ?? null,
      restoredBreadcrumbsDigest:
        writeback.restoredBreadcrumbs?.verification().restoredBreadcrumbsDigest ?? null,
      restoreBoundaryDigest:
        writeback.restoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
    }),
  });
  return Object.freeze({
    envelopeFamily: "browserHistoryWriteback",
    navigationKind: writeback.navigationKind,
    targetKind: writeback.targetKind,
    targetHref: writeback.targetHref,
    routeIdentity: writeback.routeIdentity,
    runtimeRouteSourceId: writeback.runtimeRouteSourceId,
    runtimeContinuitySourceId: writeback.runtimeContinuitySourceId,
    coherence() {
      return writeback.coherence;
    },
    carriedBreadcrumbs() {
      return writeback.carriedBreadcrumbs;
    },
    restoredBreadcrumbs() {
      return writeback.restoredBreadcrumbs;
    },
    restoreBoundary() {
      return writeback.restoreBoundary;
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

function createWritebackDiagnostics(writeback, routeOutcome) {
  if (writeback.targetKind === "external") {
    return Object.freeze({
      boundarySource: "browserHistoryWriteback",
      boundaryArtifact: "externalNavigationEscaped",
      navigationKind: writeback.navigationKind,
      targetKind: writeback.targetKind,
      targetHref: writeback.targetHref,
      routeIdentity: writeback.routeIdentity,
      coherenceKind: writeback.coherence?.coherenceKind ?? null,
      outcomeKind: routeOutcome?.kind ?? null,
      routeId: routeOutcome?.routeId ?? null,
      href: routeOutcome?.href ?? null,
    });
  }
  return Object.freeze({
    boundarySource: "browserHistoryWriteback",
    boundaryArtifact: createBrowserAuthorityBoundaryArtifact(writeback, routeOutcome),
    navigationKind: writeback.navigationKind,
    targetKind: writeback.targetKind,
    targetHref: writeback.targetHref,
    routeIdentity: writeback.routeIdentity,
    coherenceKind: writeback.coherence?.coherenceKind ?? null,
    outcomeKind: routeOutcome?.kind ?? null,
    routeId: routeOutcome?.routeId ?? null,
    href: routeOutcome?.href ?? null,
  });
}

function normalizeWritebackTarget(target, navigationKind) {
  if (navigationKind === "external") {
    return Object.freeze({
      kind: "external",
      href: requireExternalTarget(target),
    });
  }
  const rawLocation = normalizeLocalWritebackTarget(target, navigationKind);
  return Object.freeze({
    kind: "local",
    href: rawLocation.href,
    rawLocation,
  });
}

function normalizeLocalWritebackTarget(target, navigationKind) {
  if (typeof target === "string") {
    return createRawLocationAuthority(target, {
      navigationType: navigationKind === "pushstate" ? "push" : "replace",
    });
  }
  if (isRawLocationAuthority(target)) {
    return target;
  }
  if (isRouteLocation(target)) {
    return createRawLocationAuthority(target.href, {
      navigationType: navigationKind === "pushstate" ? "push" : "replace",
    });
  }
  throw new TypeError(
    "signals.router.browserHistory.writeback.push(...) and replace(...) require a local href string, route location, or raw location authority",
  );
}

function requireExternalTarget(target) {
  if (typeof target !== "string" || target.trim().length === 0) {
    throw new TypeError(
      "signals.router.browserHistory.writeback.external(...) requires a non-empty external target string",
    );
  }
  if (target.startsWith("/")) {
    throw new TypeError(
      "signals.router.browserHistory.writeback.external(...) rejects local href strings; use push(...) or replace(...) for local route writeback",
    );
  }
  if (/^[a-zA-Z][a-zA-Z0-9+.-]*:/.test(target) || target.startsWith("//")) {
    return target;
  }
  throw new TypeError(
    "signals.router.browserHistory.writeback.external(...) requires an explicit external URL or scheme target",
  );
}

function normalizeWritebackBindings(options, navigationKind) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...) options must be an object when provided`,
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
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (navigationKind !== "external" && routeIdentity == null) {
    throw new TypeError(
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...) requires routeIdentity for local graph-issued writeback`,
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
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...)`,
    ),
    carriedBreadcrumbs: normalizeOptionalCarriedBreadcrumbs(
      carriedBreadcrumbs,
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...)`,
    ),
    restoredBreadcrumbs: normalizeOptionalRestoredBreadcrumbs(
      restoredBreadcrumbs,
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...)`,
    ),
    restoreBoundary: normalizeOptionalRouteRestoreBoundary(
      restoreBoundary,
      `signals.router.browserHistory.writeback.${displayWritebackMethod(navigationKind)}(...)`,
    ),
  });
}

function normalizeOptionalString(value, label) {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string`);
}

function displayWritebackMethod(navigationKind) {
  return navigationKind === "replacestate" ? "replace" : navigationKind;
}

export {
  createBrowserHistoryWritebackNamespace,
  createBrowserHistoryWritebackReport,
  createRouterBrowserHistoryWriteback,
  isRouterBrowserHistoryWriteback,
  requireRouterBrowserHistoryWriteback,
};

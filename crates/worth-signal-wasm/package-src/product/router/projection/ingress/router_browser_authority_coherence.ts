import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

const ROUTER_BROWSER_AUTHORITY_COHERENCE = Symbol("worth.router.browser-authority-coherence");

function createBrowserAuthorityCoherenceNamespace() {
  return Object.freeze({
    sameTab(options = {}) {
      return createRouterBrowserAuthorityCoherence("sameTab", options.channelId ?? null, options);
    },
    crossTab(channelId, options = {}) {
      return createRouterBrowserAuthorityCoherence("crossTab", channelId, options);
    },
    externalNavigation(options = {}) {
      return createRouterBrowserAuthorityCoherence(
        "externalNavigation",
        options.channelId ?? null,
        options,
      );
    },
  });
}

function createRouterBrowserAuthorityCoherence(coherenceKind, channelId, options = {}) {
  const normalized = normalizeBrowserAuthorityCoherenceOptions(
    coherenceKind,
    channelId,
    options,
  );
  const verification = Object.freeze({
      browserAuthorityCoherenceDigest: createCanonicalDigest("router-browser-authority-coherence", {
        coherenceKind,
        channelId: normalized.channelId,
        sourceTabId: normalized.sourceTabId,
        expectedRouteId: normalized.expectedRouteId,
      }),
  });
  return Object.freeze({
    [ROUTER_BROWSER_AUTHORITY_COHERENCE]: true,
    kind: "routerBrowserAuthorityCoherence",
    coherenceKind,
    channelId: normalized.channelId,
    sourceTabId: normalized.sourceTabId,
    expectedRouteId: normalized.expectedRouteId,
    verification() {
      return verification;
    },
  });
}

function isRouterBrowserAuthorityCoherence(value) {
  return Boolean(
    value
    && typeof value === "object"
    && value[ROUTER_BROWSER_AUTHORITY_COHERENCE] === true
    && value.kind === "routerBrowserAuthorityCoherence",
  );
}

function normalizeOptionalBrowserAuthorityCoherence(value, operation) {
  if (value === null || value === undefined) {
    return null;
  }
  if (isRouterBrowserAuthorityCoherence(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} coherence must be created by signals.router.browserHistory.coherence.*(...)`,
  );
}

function createBrowserAuthorityBoundaryArtifact(report, routeOutcome) {
  if (routeOutcome === null || routeOutcome.kind !== "admitted") {
    return "routeOutcomeNotAdmitted";
  }
  if (
    report.coherence?.expectedRouteId != null
    && typeof routeOutcome.routeId === "string"
    && routeOutcome.routeId !== report.coherence.expectedRouteId
  ) {
    return "routeTruthDriftedFromAuthority";
  }
  return "routeTruthConverged";
}

function normalizeBrowserAuthorityCoherenceOptions(coherenceKind, channelId, options) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `signals.router.browserHistory.coherence.${coherenceKind}(...) options must be an object`,
    );
  }
  const {
    channelId: optionChannelId = null,
    sourceTabId = null,
    expectedRouteId = null,
    ...unknownOptions
  } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `signals.router.browserHistory.coherence.${coherenceKind}(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  if (coherenceKind === "crossTab" && sourceTabId === null) {
    throw new TypeError(
      "signals.router.browserHistory.coherence.crossTab(...) requires sourceTabId",
    );
  }
  return Object.freeze({
    channelId: channelId === null
      ? (optionChannelId === null ? null : requireNonEmptyString(optionChannelId, "channelId"))
      : requireNonEmptyString(channelId, "channelId"),
    sourceTabId: sourceTabId === null
      ? null
      : requireNonEmptyString(sourceTabId, "sourceTabId"),
    expectedRouteId: expectedRouteId === null
      ? null
      : requireNonEmptyString(expectedRouteId, "expectedRouteId"),
  });
}

function requireNonEmptyString(value, label) {
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string`);
}

export {
  createBrowserAuthorityBoundaryArtifact,
  createBrowserAuthorityCoherenceNamespace,
  createRouterBrowserAuthorityCoherence,
  isRouterBrowserAuthorityCoherence,
  normalizeOptionalBrowserAuthorityCoherence,
};

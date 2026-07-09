import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createRawLocationAuthority,
  isRawLocationAuthority,
} from "../../url_authority/router_url_authority.js";

const ROUTER_HYDRATION_HANDOFF = Symbol("WORTH.router.hydration-handoff");

function createHydrationNamespace() {
  return Object.freeze({
    server(location, options) {
      return createRouterHydrationHandoff("server", location, options);
    },
  });
}

function createRouterHydrationHandoff(hydrationKind, location, options = {}) {
  const rawLocation = normalizeHydrationRawLocation(location);
  const normalized = normalizeHydrationOptions(options, hydrationKind);
  const verification = Object.freeze({
    hydrationHandoffDigest: createCanonicalDigest("router-hydration-handoff", {
      hydrationKind,
      rawLocationDigest: rawLocation.verification().rawLocationDigest,
      rawLocationHref: rawLocation.href,
      serverRouteIdentity: normalized.serverRouteIdentity,
      serverHref: normalized.serverHref,
    }),
  });
  return Object.freeze({
    [ROUTER_HYDRATION_HANDOFF]: true,
    kind: "routerHydrationHandoff",
    hydrationKind,
    rawLocation,
    serverRouteIdentity: normalized.serverRouteIdentity,
    serverHref: normalized.serverHref,
    verification() {
      return verification;
    },
  });
}

function isRouterHydrationHandoff(value) {
  return Boolean(
    value
    && typeof value === "object"
    && value[ROUTER_HYDRATION_HANDOFF] === true
    && value.kind === "routerHydrationHandoff",
  );
}

function requireRouterHydrationHandoff(value, operation) {
  if (isRouterHydrationHandoff(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires a hydration handoff created by signals.router.hydration.server(...)`,
  );
}

function createHydrationAdmissionReport(handoff, routeOutcome) {
  const diagnostics = createHydrationDiagnostics(handoff, routeOutcome);
  const verification = Object.freeze({
    hydrationHandoffDigest: handoff.verification().hydrationHandoffDigest,
    routeTruthDigest: createCanonicalDigest("router-hydration-route-truth", {
      hydrationHandoffDigest: handoff.verification().hydrationHandoffDigest,
      serverRouteIdentity: handoff.serverRouteIdentity,
      serverHref: handoff.serverHref,
      boundaryArtifact: diagnostics.boundaryArtifact,
      routeOutcomeDigest: routeOutcome.verification().routeOutcomeDigest,
      routeId: routeOutcome.routeId,
      href: routeOutcome.href,
      outcomeKind: routeOutcome.kind,
    }),
    hydrationBoundaryDigest: createCanonicalDigest("router-hydration-boundary", {
      hydrationHandoffDigest: handoff.verification().hydrationHandoffDigest,
      hydrationKind: handoff.hydrationKind,
      boundaryArtifact: diagnostics.boundaryArtifact,
      rawLocationHref: handoff.rawLocation.href,
      serverRouteIdentity: handoff.serverRouteIdentity,
      serverHref: handoff.serverHref,
      routeId: diagnostics.routeId,
      href: diagnostics.href,
      outcomeKind: diagnostics.outcomeKind,
    }),
  });
  return Object.freeze({
    envelopeFamily: "hydrationHandoff",
    hydrationKind: handoff.hydrationKind,
    rawLocationHref: handoff.rawLocation.href,
    serverRouteIdentity: handoff.serverRouteIdentity,
    serverHref: handoff.serverHref,
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

function createHydrationDiagnostics(handoff, routeOutcome) {
  return Object.freeze({
    boundarySource: "hydrationHandoff",
    boundaryArtifact: classifyHydrationBoundaryArtifact(handoff, routeOutcome),
    hydrationKind: handoff.hydrationKind,
    rawLocationHref: handoff.rawLocation.href,
    serverRouteIdentity: handoff.serverRouteIdentity,
    serverHref: handoff.serverHref,
    outcomeKind: routeOutcome.kind,
    routeId: routeOutcome.routeId ?? null,
    href: routeOutcome.href ?? null,
  });
}

function classifyHydrationBoundaryArtifact(handoff, routeOutcome) {
  if (routeOutcome.kind !== "admitted") {
    return "routeOutcomeNotAdmitted";
  }
  return handoff.serverRouteIdentity === routeOutcome.routeId
    && (handoff.serverHref === null || handoff.serverHref === routeOutcome.href)
    ? "routeTruthMatchedServer"
    : "routeTruthDriftedFromServer";
}

function normalizeHydrationRawLocation(location) {
  if (typeof location === "string") {
    return createRawLocationAuthority(location, { navigationType: "load" });
  }
  if (isRawLocationAuthority(location)) {
    return location;
  }
  throw new TypeError(
    "signals.router.hydration.server(...) requires a local href string or raw location authority",
  );
}

function normalizeHydrationOptions(options, hydrationKind) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `signals.router.hydration.${hydrationKind}(...) options must be an object`,
    );
  }
  const {
    serverRouteIdentity,
    serverHref = null,
    ...unknownOptions
  } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `signals.router.hydration.${hydrationKind}(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  return Object.freeze({
    serverRouteIdentity: requireNonEmptyString(
      serverRouteIdentity,
      `signals.router.hydration.${hydrationKind}(...) serverRouteIdentity`,
    ),
    serverHref: serverHref === null || serverHref === undefined
      ? null
      : requireNonEmptyString(
        serverHref,
        `signals.router.hydration.${hydrationKind}(...) serverHref`,
      ),
  });
}

function requireNonEmptyString(value, label) {
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string`);
}

export {
  createHydrationAdmissionReport,
  createHydrationNamespace,
  createRouterHydrationHandoff,
  isRouterHydrationHandoff,
  requireRouterHydrationHandoff,
};

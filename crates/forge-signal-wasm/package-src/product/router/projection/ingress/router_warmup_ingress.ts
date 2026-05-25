import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createRawLocationAuthority,
  isRawLocationAuthority,
} from "../../url_authority/router_url_authority.js";

const ROUTER_WARMUP_INGRESS = Symbol("forge.router.warmup-ingress");

function createWarmupIngressNamespace() {
  return Object.freeze({
    hover(location, options) {
      return createRouterWarmupIngress("hover", location, options);
    },
    focus(location, options) {
      return createRouterWarmupIngress("focus", location, options);
    },
    viewport(location, options) {
      return createRouterWarmupIngress("viewport", location, options);
    },
    intent(location, options) {
      return createRouterWarmupIngress("intent", location, options);
    },
  });
}

function createRouterWarmupIngress(trigger, location, options = {}) {
  const rawLocation = normalizeWarmupRawLocation(location);
  const normalized = normalizeWarmupOptions(options, trigger);
  const verification = Object.freeze({
    routeWarmupIngressDigest: createCanonicalDigest("route-warmup-ingress", {
      trigger,
      rawLocationDigest: rawLocation.verification().rawLocationDigest,
      rawLocationHref: rawLocation.href,
      sourceId: normalized.sourceId,
      sourceValue: normalized.sourceValue,
      routeIdentity: normalized.routeIdentity,
    }),
  });
  return Object.freeze({
    [ROUTER_WARMUP_INGRESS]: true,
    kind: "routerWarmupIngress",
    trigger,
    rawLocation,
    sourceId: normalized.sourceId,
    sourceValue: normalized.sourceValue,
    routeIdentity: normalized.routeIdentity,
    verification() {
      return verification;
    },
  });
}

function createRouteWarmupReport(ingress, artifact, boundaryArtifact) {
  const verification = Object.freeze({
    routeWarmupIngressDigest: ingress.verification().routeWarmupIngressDigest,
    routeWarmupReportDigest: createCanonicalDigest("route-warmup-report", {
      routeWarmupIngressDigest: ingress.verification().routeWarmupIngressDigest,
      trigger: ingress.trigger,
      href: ingress.rawLocation.href,
      routeIdentity: ingress.routeIdentity,
      warmedResourceNames: artifact?.resourceNames() ?? [],
      skippedResourceNames: artifact?.skippedResourceNames() ?? [],
      artifactDigest: artifact?.verification().routePrefetchDigest ?? null,
      boundaryArtifact,
    }),
  });
  return Object.freeze({
    envelopeFamily: "routeWarmupIngress",
    trigger: ingress.trigger,
    rawLocationHref: ingress.rawLocation.href,
    routeIdentity: ingress.routeIdentity,
    artifact() {
      return artifact;
    },
    diagnostics() {
      return Object.freeze({
        boundarySource: "routeWarmupIngress",
        boundaryArtifact,
        trigger: ingress.trigger,
        rawLocationHref: ingress.rawLocation.href,
        routeIdentity: ingress.routeIdentity,
        warmedResourceNames: Object.freeze(artifact?.resourceNames() ?? []),
        skippedResourceNames: Object.freeze(artifact?.skippedResourceNames() ?? []),
      });
    },
    verification() {
      return verification;
    },
  });
}

function isRouterWarmupIngress(value) {
  return Boolean(
    value &&
    typeof value === "object" &&
    value[ROUTER_WARMUP_INGRESS] === true &&
    value.kind === "routerWarmupIngress",
  );
}

function requireRouterWarmupIngress(value, operation) {
  if (isRouterWarmupIngress(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires an ingress envelope created by signals.router.warmup.*(...)`,
  );
}

function normalizeWarmupRawLocation(location) {
  if (typeof location === "string") {
    return createRawLocationAuthority(location, {
      navigationType: "manual",
    });
  }
  if (isRawLocationAuthority(location)) {
    return location;
  }
  throw new TypeError(
    "signals.router.warmup.*(...) requires a local href string or raw location authority",
  );
}

function normalizeWarmupOptions(options, trigger) {
  if (!options || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      `signals.router.warmup.${trigger}(...) options must be an object when provided`,
    );
  }
  const {
    sourceId = null,
    sourceValue,
    routeIdentity = null,
    ...unknownOptions
  } = options;
  const unknownKeys = Object.keys(unknownOptions);
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `signals.router.warmup.${trigger}(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  return Object.freeze({
    sourceId: normalizeOptionalString(sourceId, "sourceId"),
    sourceValue,
    routeIdentity: normalizeOptionalString(routeIdentity, "routeIdentity"),
  });
}

function normalizeOptionalString(value, label) {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value === "string" && value.trim().length > 0) {
    return value;
  }
  throw new TypeError(`${label} must be a non-empty string when provided`);
}

export {
  createRouteWarmupReport,
  createRouterWarmupIngress,
  createWarmupIngressNamespace,
  isRouterWarmupIngress,
  requireRouterWarmupIngress,
};

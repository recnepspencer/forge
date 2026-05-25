import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

function createRouteBreadcrumbProvenanceArtifact(entry, options = {}) {
  const restoreBoundary = options.restoreBoundary
    ?? (typeof entry.restoreBoundary === "function" ? entry.restoreBoundary() : null);
  const replayAvailability = options.replayAvailability
    ?? (typeof entry.replay === "function" ? "replayHistory" : "unavailable");
  const restoreAvailability = restoreBoundary === null
    ? "unavailable"
    : "restoreBoundary";
  const verification = Object.freeze({
    breadcrumbProvenanceDigest: createCanonicalDigest("route-breadcrumb-provenance", {
      crumbId: entry.crumbId,
      routeId: entry.routeId,
      href: entry.href,
      targetHref: entry.targetHref,
      status: entry.status,
      sourceKind: entry.sourceKind,
      restoreAvailability,
      replayAvailability,
      restoreBoundaryDigest:
        restoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
    }),
  });
  return Object.freeze({
    kind: "routeBreadcrumbProvenance",
    crumbId: entry.crumbId,
    routeId: entry.routeId,
    href: entry.href,
    targetHref: entry.targetHref,
    status: entry.status,
    sourceKind: entry.sourceKind,
    restoreAvailability,
    replayAvailability,
    restoreBoundary() {
      return restoreBoundary;
    },
    verification() {
      return verification;
    },
  });
}

export {
  createRouteBreadcrumbProvenanceArtifact,
};

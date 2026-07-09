import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  ROUTE_CARRIED_BREADCRUMBS,
  ROUTE_RESTORED_BREADCRUMBS,
} from "../../router_symbols.js";

function createCarriedBreadcrumbsArtifact(trail) {
  return createBreadcrumbProvenanceArtifact(
    trail,
    "signals.router.carryBreadcrumbs(...)",
    ROUTE_CARRIED_BREADCRUMBS,
    "routeCarriedBreadcrumbs",
    "carriedBreadcrumbsDigest",
    "route-carried-breadcrumbs",
    false,
  );
}

function createRestoredBreadcrumbsArtifact(trail) {
  return createBreadcrumbProvenanceArtifact(
    trail,
    "signals.router.restoreBreadcrumbs(...)",
    ROUTE_RESTORED_BREADCRUMBS,
    "routeRestoredBreadcrumbs",
    "restoredBreadcrumbsDigest",
    "route-restored-breadcrumbs",
    true,
  );
}

function isCarriedBreadcrumbsArtifact(value) {
  return Boolean(value && value[ROUTE_CARRIED_BREADCRUMBS] === true);
}

function isRestoredBreadcrumbsArtifact(value) {
  return Boolean(value && value[ROUTE_RESTORED_BREADCRUMBS] === true);
}

function requireCarriedBreadcrumbsArtifact(value, operation) {
  if (isCarriedBreadcrumbsArtifact(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires a carried breadcrumb artifact created by signals.router.carryBreadcrumbs(...)`,
  );
}

function requireRestoredBreadcrumbsArtifact(value, operation) {
  if (isRestoredBreadcrumbsArtifact(value)) {
    return value;
  }
  throw new TypeError(
    `${operation} requires a restored breadcrumb artifact created by signals.router.restoreBreadcrumbs(...)`,
  );
}

function normalizeOptionalCarriedBreadcrumbs(value, operation) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireCarriedBreadcrumbsArtifact(value, operation);
}

function normalizeOptionalRestoredBreadcrumbs(value, operation) {
  if (value === undefined || value === null) {
    return null;
  }
  return requireRestoredBreadcrumbsArtifact(value, operation);
}

function normalizeBreadcrumbEntries(value, operation) {
  if (value && typeof value === "object" && Array.isArray(value.entries)) {
    return Object.freeze(value.entries.slice());
  }
  if (Array.isArray(value)) {
    return Object.freeze(value.slice());
  }
  throw new TypeError(
    `${operation} requires a breadcrumbTrail() artifact or array of breadcrumb entries`,
  );
}

function createBreadcrumbProvenanceArtifact(
  trail,
  operation,
  brand,
  kind,
  verificationField,
  digestLabel,
  requireRestoreBoundary,
) {
  const entries = normalizeBreadcrumbEntries(trail, operation);
  if (requireRestoreBoundary) {
    validateRestoredBreadcrumbEntries(entries, operation);
  }
  const verification = Object.freeze({
    [verificationField]: createCanonicalDigest(digestLabel, {
      entryDigests: entries.map((entry) => entry.verification().breadcrumbEntryDigest),
      restoreBoundaryDigests: requireRestoreBoundary
        ? entries.map((entry) => entry.restoreBoundary().verification().routeRestoreBoundaryDigest)
        : [],
    }),
  });
  return Object.freeze({
    [brand]: true,
    kind,
    entries,
    verification() {
      return verification;
    },
  });
}

function validateRestoredBreadcrumbEntries(entries, operation) {
  entries.forEach((entry, index) => {
    if (typeof entry.restoreBoundary !== "function" || entry.restoreBoundary() === null) {
      throw new TypeError(
        `${operation} requires restore-backed breadcrumb entries; entries[${index}] has no restoreBoundary()`,
      );
    }
  });
}

export {
  createCarriedBreadcrumbsArtifact,
  createRestoredBreadcrumbsArtifact,
  isCarriedBreadcrumbsArtifact,
  isRestoredBreadcrumbsArtifact,
  normalizeOptionalCarriedBreadcrumbs,
  normalizeOptionalRestoredBreadcrumbs,
  requireCarriedBreadcrumbsArtifact,
  requireRestoredBreadcrumbsArtifact,
};

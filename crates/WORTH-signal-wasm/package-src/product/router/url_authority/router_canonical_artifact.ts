import { createRouteCanonicalVerification } from "./router_verification_packages.js";

function createCanonicalRouteArtifact(descriptor, normalized, hrefParts, referenceVerification) {
  const pathname = hrefParts.pathname;
  const searchString = hrefParts.searchString;
  const hashString = hrefParts.hashString;
  const href = `${pathname}${searchString}${hashString}`;
  const searchDigest = createCanonicalDigest("search", normalized.search);
  const hashDigest = createCanonicalDigest("hash", normalized.hash ?? null);
  const canonicalUrlDigest = createCanonicalDigest("url", href);
  const equivalenceDigest = createCanonicalDigest("equivalence", {
    routeId: descriptor.routeId,
    href,
  });
  return Object.freeze({
    routeId: descriptor.routeId,
    href,
    pathname,
    search: normalized.search,
    hash: normalized.hash,
    searchDigest,
    hashDigest,
    canonicalUrlDigest,
    equivalenceDigest,
    descriptor() {
      return descriptor;
    },
    verification() {
      return createRouteCanonicalVerification(referenceVerification, this);
    },
  });
}

function createCanonicalDigest(label, value) {
  return `WORTH-router:${label}:${JSON.stringify(value)}`;
}

export { createCanonicalRouteArtifact };

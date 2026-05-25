import {
  CANONICAL_URL_AUTHORITY,
  RAW_LOCATION_AUTHORITY,
} from "../router_symbols.js";
import {
  createCanonicalDigest,
  createCanonicalVerification,
  createRawLocationVerification,
} from "./router_verification_packages.js";

const ROUTE_MATCH_BASE_URL = new URL("https://forge-signal.test");
const RAW_LOCATION_NAVIGATION_TYPES = Object.freeze([
  "load",
  "push",
  "replace",
  "pop",
  "manual",
  "external",
]);

function createRawLocationAuthority(rawHref, options = {}) {
  const rawPathname = readRawPathname(rawHref);
  if (rawPathname === null || containsRejectedDotSegment(rawPathname)) {
    throw new TypeError("signals.router.raw(...) requires a local href without dot segments");
  }
  const url = createAdmittedUrl(rawHref);
  const rawSearch = readRawSearch(rawHref);
  const rawHashFragment = readRawHashFragment(rawHref);
  const navigationType = normalizeRawLocationNavigationType(options.navigationType);
  const rawSearchParams = Object.freeze(parseSearchParamEntries(rawSearch));
  const authority = Object.freeze({
    [RAW_LOCATION_AUTHORITY]: true,
    href: rawHref,
    pathname: rawPathname,
    searchParams: rawSearchParams,
    hashFragment: rawHashFragment,
    navigationType,
    canonical() {
      return createCanonicalUrlAuthorityFromParsed(
        rawHref,
        url,
        rawSearchParams,
        rawHashFragment,
      );
    },
    verification() {
      return createRawLocationVerification(this, this.canonical());
    },
  });
  return authority;
}

function createCanonicalUrlAuthority(rawHref, options = {}) {
  return createRawLocationAuthority(rawHref, options).canonical();
}

function createCanonicalUrlAuthorityFromParsed(
  rawHref,
  url,
  rawSearchParams,
  rawHashFragment,
) {
  const canonicalSearchParams = Object.freeze(
    sortSearchParamEntries(rawSearchParams).map(freezeSearchParamEntry),
  );
  const canonicalHashFragment = normalizeCanonicalHashFragment(rawHashFragment);
  const pathname = url.pathname;
  const searchString = createCanonicalSearchString(canonicalSearchParams);
  const hashString = createCanonicalHashString(canonicalHashFragment);
  const href = `${pathname}${searchString}${hashString}`;
  const canonicalAuthority = Object.freeze({
    [CANONICAL_URL_AUTHORITY]: true,
    href,
    pathname,
    searchParams: canonicalSearchParams,
    hashFragment: canonicalHashFragment,
    canonicalUrlDigest: createCanonicalDigest("url", href),
    equivalenceDigest: createCanonicalDigest("url-equivalence", {
      pathname,
      searchParams: canonicalSearchParams,
      hashFragment: canonicalHashFragment ?? null,
    }),
    searchDigest: createCanonicalDigest("raw-search", canonicalSearchParams),
    hashDigest: createCanonicalDigest("raw-hash", canonicalHashFragment ?? null),
    verification() {
      return createCanonicalVerification(this);
    },
  });
  return canonicalAuthority;
}

function createAdmittedUrl(rawHref) {
  const url = new URL(rawHref, ROUTE_MATCH_BASE_URL);
  if (isAbsoluteLikeUrl(rawHref) && url.origin !== ROUTE_MATCH_BASE_URL.origin) {
    throw new TypeError("signals.router raw location must stay on the local origin");
  }
  return url;
}

function coerceRouteMatchHref(input) {
  if (typeof input === "string") {
    return input;
  }
  if (isCanonicalUrlAuthority(input)) {
    return input.href;
  }
  if (isRawLocationAuthority(input)) {
    return input.canonical().href;
  }
  throw new TypeError(
    "route.match(...) requires a local href string, raw location authority, or canonical url authority",
  );
}

function tryParseAdmittedUrl(rawHref) {
  if (typeof rawHref !== "string" || rawHref.length === 0) {
    throw new TypeError("route.match(...) requires a non-empty href or path string");
  }
  const rawPathname = readRawPathname(rawHref);
  if (rawPathname === null || containsRejectedDotSegment(rawPathname)) {
    return null;
  }
  const url = new URL(rawHref, ROUTE_MATCH_BASE_URL);
  if (isAbsoluteLikeUrl(rawHref) && url.origin !== ROUTE_MATCH_BASE_URL.origin) {
    return null;
  }
  return url;
}

function isRawLocationAuthority(value) {
  return Boolean(value && value[RAW_LOCATION_AUTHORITY] === true);
}

function isCanonicalUrlAuthority(value) {
  return Boolean(value && value[CANONICAL_URL_AUTHORITY] === true);
}

function readRawPathname(rawHref) {
  if (typeof rawHref !== "string" || rawHref.length === 0) {
    return null;
  }
  if (isNetworkPathReference(rawHref)) {
    return splitPathnameAndSuffix(extractNetworkPathPath(rawHref));
  }
  if (rawHref.startsWith("/")) {
    return splitPathnameAndSuffix(rawHref);
  }
  if (isAbsoluteUrl(rawHref)) {
    return splitPathnameAndSuffix(extractAbsoluteUrlPath(rawHref));
  }
  return null;
}

function extractAbsoluteUrlPath(rawHref) {
  const schemeBoundary = rawHref.indexOf("://");
  const authorityStart = schemeBoundary + 3;
  const pathStart = rawHref.indexOf("/", authorityStart);
  return pathStart === -1 ? "/" : rawHref.slice(pathStart);
}

function extractNetworkPathPath(rawHref) {
  const authorityStart = 2;
  const pathStart = rawHref.indexOf("/", authorityStart);
  return pathStart === -1 ? "/" : rawHref.slice(pathStart);
}

function splitPathnameAndSuffix(rawPathLike) {
  const queryIndex = rawPathLike.indexOf("?");
  const hashIndex = rawPathLike.indexOf("#");
  const boundary = firstNonNegativeIndex(queryIndex, hashIndex);
  return boundary === -1 ? rawPathLike : rawPathLike.slice(0, boundary);
}

function containsRejectedDotSegment(rawPathname) {
  if (rawPathname === "/") {
    return false;
  }
  const segments = rawPathname.slice(1).split("/");
  for (const segment of segments) {
    if (segment.length === 0) {
      continue;
    }
    try {
      const decoded = decodeURIComponent(segment);
      if (decoded === "." || decoded === "..") {
        return true;
      }
    } catch {
      return true;
    }
  }
  return false;
}

function readRawSearch(rawHref) {
  const queryIndex = rawHref.indexOf("?");
  if (queryIndex === -1) {
    return "";
  }
  const hashIndex = rawHref.indexOf("#", queryIndex);
  return hashIndex === -1
    ? rawHref.slice(queryIndex + 1)
    : rawHref.slice(queryIndex + 1, hashIndex);
}

function readRawHashFragment(rawHref) {
  const hashIndex = rawHref.indexOf("#");
  if (hashIndex === -1) {
    return undefined;
  }
  return rawHref.slice(hashIndex + 1);
}

function parseSearchParamEntries(rawSearch) {
  if (rawSearch.length === 0) {
    return [];
  }
  return rawSearch.split("&").map((entry) => parseSearchParamEntry(entry));
}

function parseSearchParamEntry(entry) {
  const equalsIndex = entry.indexOf("=");
  const rawKey = equalsIndex === -1 ? entry : entry.slice(0, equalsIndex);
  const rawValue = equalsIndex === -1 ? "" : entry.slice(equalsIndex + 1);
  return freezeSearchParamEntry({
    key: decodeQueryComponent(rawKey),
    value: decodeQueryComponent(rawValue),
  });
}

function sortSearchParamEntries(entries) {
  return entries
    .slice()
    .sort((left, right) => (
      left.key.localeCompare(right.key) ||
      left.value.localeCompare(right.value)
    ));
}

function freezeSearchParamEntry(entry) {
  return Object.freeze(entry);
}

function decodeQueryComponent(value) {
  return decodeURIComponent(value.replace(/\+/g, "%20"));
}

function normalizeCanonicalHashFragment(rawHashFragment) {
  if (rawHashFragment === undefined) {
    return undefined;
  }
  return decodeURIComponent(rawHashFragment);
}

function createCanonicalSearchString(searchParams) {
  if (searchParams.length === 0) {
    return "";
  }
  return `?${searchParams.map((entry) => (
    `${encodeURIComponent(entry.key)}=${encodeURIComponent(entry.value)}`
  )).join("&")}`;
}

function createCanonicalHashString(hashFragment) {
  return hashFragment === undefined ? "" : `#${encodeURIComponent(hashFragment)}`;
}

function normalizeRawLocationNavigationType(navigationType) {
  if (navigationType === undefined) {
    return "manual";
  }
  if (!RAW_LOCATION_NAVIGATION_TYPES.includes(navigationType)) {
    throw new TypeError(
      `signals.router raw navigationType must be one of ${RAW_LOCATION_NAVIGATION_TYPES.join(", ")}`,
    );
  }
  return navigationType;
}

function firstNonNegativeIndex(left, right) {
  if (left === -1) {
    return right;
  }
  if (right === -1) {
    return left;
  }
  return Math.min(left, right);
}

function isAbsoluteUrl(rawHref) {
  return /^[a-zA-Z][a-zA-Z\d+.-]*:\/\//.test(rawHref);
}

function isNetworkPathReference(rawHref) {
  return rawHref.startsWith("//");
}

function isAbsoluteLikeUrl(rawHref) {
  return isAbsoluteUrl(rawHref) || isNetworkPathReference(rawHref);
}

export {
  coerceRouteMatchHref,
  createCanonicalUrlAuthority,
  createRawLocationAuthority,
  isCanonicalUrlAuthority,
  isRawLocationAuthority,
  tryParseAdmittedUrl,
};

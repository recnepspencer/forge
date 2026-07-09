import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createHistoryFallbackBreadcrumbTrail,
} from "../breadcrumb/router_breadcrumb_artifact.js";
import {
  createRouteBreadcrumbProvenanceArtifact,
} from "../breadcrumb/router_breadcrumb_entry_provenance.js";
import {
  createRouteHistoryReplayResult,
  restoreRouteHistoryBoundary,
} from "./router_restore_boundary.js";

function createCurrentBrowserHistoryBreadcrumbTrail(currentEntry) {
  const declaredTrail = currentEntry?.breadcrumbTrail() ?? null;
  const currentAuthority = currentEntry === null
    ? null
    : Object.freeze({
        restoreBoundary: currentEntry.restoreBoundary(),
        runtimeRouteSourceId: currentEntry.runtimeRouteSourceId ?? null,
        runtimeContinuitySourceId: currentEntry.runtimeContinuitySourceId ?? null,
      });
  return declaredTrail ?? createBrowserHistoryBreadcrumbTrail(
    createHistoryFallbackBreadcrumbTrail(currentEntry === null ? [] : collectBreadcrumbHistoryEntries(currentEntry)).entries,
    currentAuthority,
  );
}

function collectBreadcrumbHistoryEntries(currentEntry) {
  const entries = [];
  let entry = currentEntry;
  while (entry !== null) {
    entries.push(entry);
    entry = entry.previous();
  }
  entries.reverse();
  return entries;
}

function createDeclaredBreadcrumbTrail(
  outcome,
  restoredBreadcrumbs,
  carriedBreadcrumbs,
  breadcrumbAuthority,
) {
  if (
    typeof outcome?.layouts !== "function"
    || typeof outcome?.route !== "function"
  ) {
    return null;
  }
  const entries = [];
  for (const placement of outcome.layouts()) {
    const trail = placement.capability().breadcrumbTrail?.() ?? null;
    if (trail !== null) {
      entries.push(...trail.entries);
    }
  }
  const routeTrail = outcome.route().breadcrumbTrail?.({
    restoredBreadcrumbs,
    carriedBreadcrumbs,
  }) ?? null;
  if (routeTrail !== null) {
    entries.push(...routeTrail.entries);
  }
  return entries.length === 0
    ? null
    : createBrowserHistoryBreadcrumbTrail(entries, breadcrumbAuthority);
}

function createBrowserHistoryBreadcrumbTrail(entries, breadcrumbAuthority = null) {
  const frozenEntries = Object.freeze(
    entries.map((entry) => createBrowserHistoryBreadcrumbEntry(entry, breadcrumbAuthority)),
  );
  return Object.freeze({
    kind: "browserHistoryBreadcrumbTrail",
    entries: frozenEntries,
    verification() {
      return Object.freeze({
        breadcrumbTrailDigest: createCanonicalDigest("browser-history-breadcrumb-trail", {
          entryDigests: frozenEntries.map((entry) => entry.verification().breadcrumbEntryDigest),
        }),
      });
    },
  });
}

function createBrowserHistoryBreadcrumbEntry(entry, breadcrumbAuthority = null) {
  const restoreBoundary = typeof entry.restoreBoundary === "function"
    ? entry.restoreBoundary()
    : breadcrumbAuthority?.restoreBoundary ?? null;
  return Object.freeze({
    ...entry,
    restoreBoundary() {
      return restoreBoundary;
    },
    restore(history) {
      if (typeof entry.restore === "function") {
        return entry.restore(history);
      }
      if (restoreBoundary === null) {
        throw new TypeError(
          "browserHistoryBreadcrumbEntry.restore(...) requires restore-backed breadcrumb provenance",
        );
      }
      return restoreRouteHistoryBoundary(history, restoreBoundary, {
        restoreSourceKind: "breadcrumbEntry",
        routeId: entry.routeId,
        href: entry.targetHref ?? entry.href,
        restoredEntryDigest: entry.verification().breadcrumbEntryDigest,
      });
    },
    replay(history) {
      if (typeof entry.replay === "function") {
        return entry.replay(history);
      }
      if (
        breadcrumbAuthority?.runtimeRouteSourceId === null
        && breadcrumbAuthority?.runtimeContinuitySourceId === null
      ) {
        throw new TypeError(
          "browserHistoryBreadcrumbEntry.replay(...) requires replay-backed breadcrumb provenance",
        );
      }
      return createRouteHistoryReplayResult(history, {
        replaySourceKind: "breadcrumbEntry",
        routeId: entry.routeId,
        href: entry.targetHref ?? entry.href,
        replayedEntryDigest: entry.verification().breadcrumbEntryDigest,
        runtimeRouteSourceId: breadcrumbAuthority?.runtimeRouteSourceId ?? null,
        runtimeContinuitySourceId: breadcrumbAuthority?.runtimeContinuitySourceId ?? null,
      });
    },
    provenance() {
      return createRouteBreadcrumbProvenanceArtifact(this, {
        restoreBoundary,
        replayAvailability:
          typeof entry.replay === "function"
            || breadcrumbAuthority?.runtimeRouteSourceId !== null
            || breadcrumbAuthority?.runtimeContinuitySourceId !== null
            ? "replayHistory"
            : "unavailable",
      });
    },
  });
}

export {
  createCurrentBrowserHistoryBreadcrumbTrail,
  createDeclaredBreadcrumbTrail,
};

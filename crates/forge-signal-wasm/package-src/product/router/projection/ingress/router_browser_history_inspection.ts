import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

function createBrowserHistoryInspectionArtifact({
  latestBoundaryEvent,
  currentRouteTruthEvent,
  currentEntry,
  backProvenance,
  breadcrumbTrail,
}) {
  const authorityEvents = Object.freeze(
    dedupeBoundaryEvents(
      [latestBoundaryEvent, currentRouteTruthEvent, backProvenance.previous].filter(
        (item) => item !== null,
      ),
    ),
  );
  const currentOutletComposition = currentEntry?.outletComposition?.() ?? null;
  const backOutletComposition = backProvenance.previous?.outletComposition?.() ?? null;
  const breadcrumbProvenance = Object.freeze(
    breadcrumbTrail.entries.map((entry) => entry.provenance()),
  );
  const summary = Object.freeze({
    currentEntryAvailable: currentEntry !== null,
    currentEntryRestoreAvailability:
      currentEntry?.restoreBoundary?.() ? "restoreBoundary" : "unavailable",
    currentEntryReplayAvailability: hasReplaySourceIds(currentEntry)
      ? "replayHistory"
      : "unavailable",
    backProvenanceAvailable: backProvenance.available,
    backRestoreAvailability:
      backProvenance.restoreBoundary() === null ? "unavailable" : "restoreBoundary",
    backReplayAvailability: hasReplaySourceIds(backProvenance.previous)
      ? "replayHistory"
      : "unavailable",
    currentOutletCompositionAvailable: currentOutletComposition !== null,
    backOutletCompositionAvailable: backOutletComposition !== null,
    breadcrumbEntryCount: breadcrumbProvenance.length,
    breadcrumbRestoreAvailability: summarizeAvailability(
      breadcrumbProvenance,
      "restoreAvailability",
    ),
    breadcrumbReplayAvailability: summarizeAvailability(
      breadcrumbProvenance,
      "replayAvailability",
    ),
    resolvedBreadcrumbCount: countBreadcrumbStatus(
      breadcrumbProvenance,
      "resolved",
    ),
    recomputedBreadcrumbCount: countBreadcrumbStatus(
      breadcrumbProvenance,
      "recomputed",
    ),
    carriedBreadcrumbCount: countBreadcrumbStatus(
      breadcrumbProvenance,
      "carried",
    ),
    restoredBreadcrumbCount: countBreadcrumbStatus(
      breadcrumbProvenance,
      "restored",
    ),
    fallbackBreadcrumbCount: countBreadcrumbStatus(
      breadcrumbProvenance,
      "fallback",
    ),
    routeDeclarationBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "routeDeclaration",
    ),
    recomputedBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "recomputed",
    ),
    carriedBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "carriedProvenance",
    ),
    restoredBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "restoredProvenance",
    ),
    fallbackBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "fallback",
    ),
    historyFallbackBreadcrumbPresent: hasBreadcrumbSourceKind(
      breadcrumbProvenance,
      "historyFallback",
    ),
    latestBoundaryCoherenceKind: latestBoundaryEvent?.coherenceKind ?? null,
    currentRouteTruthCoherenceKind: currentRouteTruthEvent?.coherenceKind ?? null,
    sameTabCoherencePresent: hasCoherenceKind(authorityEvents, "sameTab"),
    crossTabCoherencePresent: hasCoherenceKind(authorityEvents, "crossTab"),
    externalNavigationCoherencePresent: hasCoherenceKind(
      authorityEvents,
      "externalNavigation",
    ),
    convergedBoundaryEventCount: countBoundaryArtifact(
      authorityEvents,
      "routeTruthConverged",
    ),
    driftedBoundaryEventCount: countBoundaryArtifact(
      authorityEvents,
      "routeTruthDriftedFromAuthority",
    ),
    notAdmittedBoundaryEventCount: countBoundaryArtifact(
      authorityEvents,
      "routeOutcomeNotAdmitted",
    ),
  });
  const verification = Object.freeze({
    historyInspectionDigest: createCanonicalDigest("browser-history-inspection", {
      latestBoundaryEventDigest:
        latestBoundaryEvent?.verification().boundaryEventDigest ?? null,
      currentRouteTruthEventDigest:
        currentRouteTruthEvent?.verification().boundaryEventDigest ?? null,
      currentEntryDigest:
      currentEntry?.verification().routeHistoryEntryDigest ?? null,
      backProvenanceDigest:
        backProvenance.verification().backProvenanceDigest,
      currentOutletCompositionDigest:
        currentOutletComposition?.verification().outletCompositionDigest ?? null,
      backOutletCompositionDigest:
        backOutletComposition?.verification().outletCompositionDigest ?? null,
      breadcrumbTrailDigest:
        breadcrumbTrail.verification().breadcrumbTrailDigest,
      summary,
      breadcrumbProvenanceDigests: breadcrumbProvenance.map(
        (provenance) => provenance.verification().breadcrumbProvenanceDigest,
      ),
    }),
  });
  return Object.freeze({
    kind: "browserHistoryInspection",
    latestBoundaryEvent,
    currentRouteTruthEvent,
    currentEntry,
    backProvenance,
    breadcrumbTrail,
    currentOutletComposition() {
      return currentOutletComposition;
    },
    backOutletComposition() {
      return backOutletComposition;
    },
    breadcrumbProvenance() {
      return breadcrumbProvenance;
    },
    summary() {
      return summary;
    },
    verification() {
      return verification;
    },
  });
}

function summarizeAvailability(items, field) {
  if (items.length === 0) {
    return "none";
  }
  const availableCount = items.filter((item) => item[field] !== "unavailable").length;
  if (availableCount === 0) {
    return "none";
  }
  if (availableCount === items.length) {
    return "all";
  }
  return "partial";
}

function hasReplaySourceIds(entry) {
  if (entry === null || entry === undefined) {
    return false;
  }
  return (entry.runtimeRouteSourceId ?? null) !== null
    || (entry.runtimeContinuitySourceId ?? null) !== null;
}

function countBreadcrumbStatus(items, status) {
  return items.filter((item) => item.status === status).length;
}

function hasBreadcrumbSourceKind(items, sourceKind) {
  return items.some((item) => item.sourceKind === sourceKind);
}

function hasCoherenceKind(items, coherenceKind) {
  return items.some((item) => item.coherenceKind === coherenceKind);
}

function countBoundaryArtifact(items, boundaryArtifact) {
  return items.filter((item) => item.boundaryArtifact === boundaryArtifact).length;
}

function dedupeBoundaryEvents(items) {
  const seen = new Set();
  return items.filter((item) => {
    const digest = item.verification().boundaryEventDigest;
    if (seen.has(digest)) {
      return false;
    }
    seen.add(digest);
    return true;
  });
}

export {
  createBrowserHistoryInspectionArtifact,
};

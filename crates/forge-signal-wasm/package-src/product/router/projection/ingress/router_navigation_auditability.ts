import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

function createNavigationAuditabilityArtifact(historyInspection, hydrationReport = null) {
  const normalizedHydrationReport = normalizeOptionalHydrationAdmissionReport(
    hydrationReport,
    "story.auditability(...)",
  );
  const hydrationDiagnostics = normalizedHydrationReport?.diagnostics() ?? null;
  const currentEntry = historyInspection.currentEntry;
  const currentRouteTruthEvent = historyInspection.currentRouteTruthEvent;
  const latestBoundaryEvent = historyInspection.latestBoundaryEvent;
  const currentVisibleRoute = resolveCurrentVisibleRoute(
    currentEntry,
    currentRouteTruthEvent,
    hydrationDiagnostics,
  );
  const currentRestoreBoundary =
    currentEntry?.restoreBoundary?.() ?? null;
  const currentRestoreAvailability =
    currentRestoreBoundary === null ? "unavailable" : "restoreBoundary";
  const currentReplayAvailability =
    currentEntry !== null && hasReplaySourceIds(currentEntry)
      ? "replayHistory"
      : "unavailable";
  const hydrationMatchesCurrentVisibleRoute =
    hydrationDiagnostics === null
    || hydrationDiagnostics.outcomeKind !== "admitted"
      ? null
      : currentVisibleRoute.routeId === null
        ? false
        : hydrationDiagnostics.routeId === currentVisibleRoute.routeId
          && hydrationDiagnostics.href === currentVisibleRoute.href;
  const summary = Object.freeze({
    hydrationBoundaryPresent: hydrationDiagnostics !== null,
    hydrationBoundaryArtifact: hydrationDiagnostics?.boundaryArtifact ?? null,
    hydrationMatchesCurrentVisibleRoute,
    historyCurrentEntryPresent: currentEntry !== null,
    currentVisibleRouteSource: currentVisibleRoute.sourceKind,
    currentVisibilityExplanation: currentVisibleRoute.explanationKind,
    currentBoundarySource: currentVisibleRoute.boundarySource,
    currentBoundaryArtifact: currentVisibleRoute.boundaryArtifact,
    currentNavigationIntent: currentVisibleRoute.navigationIntent,
    currentCoherenceKind: currentVisibleRoute.coherenceKind,
    currentRouteId: currentVisibleRoute.routeId,
    currentHref: currentVisibleRoute.href,
    currentRestoreAvailability,
    currentReplayAvailability,
    routeHistoryExplainsCurrent: currentVisibleRoute.sourceKind === "routeHistoryEntry",
    restoreBoundaryExplainsCurrent: currentRestoreBoundary !== null,
    latestBoundarySource: latestBoundaryEvent?.boundarySource ?? null,
    latestBoundaryArtifact: latestBoundaryEvent?.boundaryArtifact ?? null,
    latestBoundaryCoherenceKind: latestBoundaryEvent?.coherenceKind ?? null,
    sameTabCoherencePresent: historyInspection.summary().sameTabCoherencePresent,
    crossTabCoherencePresent: historyInspection.summary().crossTabCoherencePresent,
    externalNavigationCoherencePresent:
      historyInspection.summary().externalNavigationCoherencePresent,
    convergedBoundaryEventCount: historyInspection.summary().convergedBoundaryEventCount,
    driftedBoundaryEventCount: historyInspection.summary().driftedBoundaryEventCount,
    notAdmittedBoundaryEventCount:
      historyInspection.summary().notAdmittedBoundaryEventCount,
  });
  const verification = Object.freeze({
    navigationAuditabilityDigest: createCanonicalDigest("router-navigation-auditability", {
      hydrationBoundaryDigest:
        normalizedHydrationReport?.verification().hydrationBoundaryDigest ?? null,
      historyInspectionDigest:
        historyInspection.verification().historyInspectionDigest,
      currentRestoreBoundaryDigest:
        currentRestoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
      summary,
    }),
  });
  return Object.freeze({
    kind: "navigationAuditability",
    hydrationBoundary() {
      return normalizedHydrationReport;
    },
    historyInspection() {
      return historyInspection;
    },
    currentRestoreBoundary() {
      return currentRestoreBoundary;
    },
    summary() {
      return summary;
    },
    verification() {
      return verification;
    },
  });
}

function resolveCurrentVisibleRoute(currentEntry, currentRouteTruthEvent, hydrationDiagnostics) {
  if (currentEntry !== null) {
    return Object.freeze({
      sourceKind: "routeHistoryEntry",
      explanationKind:
        currentEntry.restoreBoundary?.() === null
          ? "routeHistoryEntry"
          : "routeHistoryRestoreBoundary",
      boundarySource: currentRouteTruthEvent?.boundarySource ?? currentEntry.boundarySource,
      boundaryArtifact:
        currentRouteTruthEvent?.boundaryArtifact ?? currentEntry.boundaryArtifact,
      navigationIntent:
        currentRouteTruthEvent?.navigationKind ?? currentEntry.navigationKind,
      coherenceKind: currentRouteTruthEvent?.coherenceKind ?? currentEntry.coherenceKind,
      routeId: currentEntry.routeId,
      href: currentEntry.href,
    });
  }
  if (hydrationDiagnostics !== null && hydrationDiagnostics.outcomeKind === "admitted") {
    return Object.freeze({
      sourceKind: "hydrationAdmission",
      explanationKind: "hydrationBoundary",
      boundarySource: hydrationDiagnostics.boundarySource,
      boundaryArtifact: hydrationDiagnostics.boundaryArtifact,
      navigationIntent: "load",
      coherenceKind: null,
      routeId: hydrationDiagnostics.routeId,
      href: hydrationDiagnostics.href,
    });
  }
  return Object.freeze({
    sourceKind: "none",
    explanationKind: "none",
    boundarySource: null,
    boundaryArtifact: null,
    navigationIntent: null,
    coherenceKind: null,
    routeId: null,
    href: null,
  });
}

function hasReplaySourceIds(entry) {
  return (entry.runtimeRouteSourceId ?? null) !== null
    || (entry.runtimeContinuitySourceId ?? null) !== null;
}

function normalizeOptionalHydrationAdmissionReport(value, operation) {
  if (value === null || value === undefined) {
    return null;
  }
  if (
    typeof value === "object"
    && value.envelopeFamily === "hydrationHandoff"
    && typeof value.outcome === "function"
    && typeof value.diagnostics === "function"
    && typeof value.verification === "function"
  ) {
    return value;
  }
  throw new TypeError(
    `${operation} hydration boundary must come from routes.admitHydrationHandoff(...)`,
  );
}

export {
  createNavigationAuditabilityArtifact,
};

import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createCurrentBrowserHistoryBreadcrumbTrail,
  createDeclaredBreadcrumbTrail,
} from "./router_browser_history_breadcrumb.js";
import {
  createBrowserHistoryInspectionArtifact,
} from "./router_browser_history_inspection.js";
import {
  createNavigationAuditabilityArtifact,
} from "./router_navigation_auditability.js";
import {
  createBrowserHistoryOutletCompositionArtifact,
} from "./router_browser_history_outlet_composition.js";
import {
  createRouteHistoryReplayResult,
  restoreRouteHistoryBoundary,
} from "./router_restore_boundary.js";

function createBrowserHistoryStory(initialReport) {
  const boundaryEvents = [];
  const admittedEntries = [];
  const listeners = new Set();
  let currentRouteTruthBoundaryEvent = null;

  function notifyListeners() {
    for (const listener of Array.from(listeners)) {
      listener();
    }
  }

  const api = Object.freeze({
    record(report) {
      const normalizedReport = requireBrowserHistoryBoundaryReport(
        report,
        "signals.router.browserHistory.story().record(...)",
      );
      const previousEntry = admittedEntries.at(-1) ?? null;
      const event = createBoundaryEventArtifact(
        normalizedReport,
        boundaryEvents.length,
        previousEntry,
      );
      boundaryEvents.push(event);
      if (event.routeTruthEntry !== null) {
        admittedEntries.push(event.routeTruthEntry);
        currentRouteTruthBoundaryEvent = event;
      }
      notifyListeners();
      return event;
    },
    subscribe(listener) {
      if (typeof listener !== "function") {
        throw new TypeError(
          "signals.router.browserHistory.story().subscribe(...) requires a listener function",
        );
      }
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
    events() {
      return Object.freeze([...boundaryEvents]);
    },
    admittedEntries() {
      return Object.freeze([...admittedEntries]);
    },
    current() {
      return admittedEntries.at(-1) ?? null;
    },
    latestBoundaryEvent() {
      return boundaryEvents.at(-1) ?? null;
    },
    currentRouteTruthEvent() {
      return currentRouteTruthBoundaryEvent;
    },
    back() {
      return admittedEntries.length < 2 ? null : admittedEntries.at(-2);
    },
    breadcrumbs() {
      return Object.freeze([...admittedEntries]);
    },
    backProvenance() {
      const current = admittedEntries.at(-1) ?? null;
      const previous = admittedEntries.length < 2 ? null : admittedEntries.at(-2);
      return createBackProvenanceArtifact(current, previous);
    },
    breadcrumbTrail() {
      return createCurrentBrowserHistoryBreadcrumbTrail(
        admittedEntries.at(-1) ?? null,
      );
    },
    inspection() {
      return createBrowserHistoryInspectionArtifact({
        latestBoundaryEvent: api.latestBoundaryEvent(),
        currentRouteTruthEvent: api.currentRouteTruthEvent(),
        currentEntry: api.current(),
        backProvenance: api.backProvenance(),
        breadcrumbTrail: api.breadcrumbTrail(),
      });
    },
    auditability(hydrationReport = null) {
      return createNavigationAuditabilityArtifact(
        api.inspection(),
        hydrationReport,
      );
    },
    verification() {
      return Object.freeze({
        historyStoryDigest: createCanonicalDigest("browser-history-story", {
          eventDigests: boundaryEvents.map((event) => event.verification().boundaryEventDigest),
          admittedEntryDigests: admittedEntries.map((entry) => entry.verification().routeHistoryEntryDigest),
        }),
        latestBoundaryEventDigest:
          boundaryEvents.at(-1)?.verification().boundaryEventDigest ?? null,
        currentRouteTruthEventDigest:
          api.currentRouteTruthEvent()?.verification().boundaryEventDigest ?? null,
        currentEntryDigest: admittedEntries.at(-1)?.verification().routeHistoryEntryDigest ?? null,
        backEntryDigest: admittedEntries.length < 2
          ? null
          : admittedEntries.at(-2)?.verification().routeHistoryEntryDigest ?? null,
      });
    },
  });

  if (initialReport !== undefined) {
    api.record(initialReport);
  }

  return api;
}

function requireBrowserHistoryBoundaryReport(report, operation) {
  if (
    report
    && typeof report === "object"
    && (report.envelopeFamily === "browserHistoryIngress"
      || report.envelopeFamily === "browserHistoryWriteback")
    && typeof report.outcome === "function"
    && typeof report.diagnostics === "function"
    && typeof report.verification === "function"
  ) {
    return report;
  }
  throw new TypeError(
    `${operation} requires a boundary report from routes.admitBrowserHistoryIngress(...) or routes.applyBrowserHistoryWriteback(...)`,
  );
}

function createBoundaryEventArtifact(report, eventIndex, previousEntry) {
  const diagnostics = report.diagnostics();
  const routeTruthEntry = createRouteHistoryEntry(report, diagnostics, eventIndex, previousEntry);
  const routeIdentifier = diagnostics.routeId ?? report.routeIdentity ?? null;
  const verification = Object.freeze({
    boundaryEventDigest: createCanonicalDigest("browser-history-boundary-event", {
      envelopeFamily: report.envelopeFamily,
      navigationKind: report.navigationKind,
      boundarySource: diagnostics.boundarySource,
      boundaryArtifact: diagnostics.boundaryArtifact,
      targetHref:
        "rawLocationHref" in report
          ? report.rawLocationHref
          : report.targetHref,
      routeIdentity: report.routeIdentity,
      routeTruthDigest:
        "routeTruthDigest" in report.verification()
          ? report.verification().routeTruthDigest
          : null,
      routeHistoryEntryDigest: routeTruthEntry?.verification().routeHistoryEntryDigest ?? null,
      eventIndex,
    }),
  });
  return Object.freeze({
    kind: "browserHistoryBoundaryEvent",
    eventIndex,
    envelopeFamily: report.envelopeFamily,
    boundarySource: diagnostics.boundarySource,
    boundaryArtifact: diagnostics.boundaryArtifact,
    navigationKind: report.navigationKind,
    targetHref:
      "rawLocationHref" in report
        ? report.rawLocationHref
        : report.targetHref,
    routeIdentity: report.routeIdentity,
    coherenceKind: diagnostics.coherenceKind ?? null,
    advancedRouteTruth: routeTruthEntry !== null,
    outcomeKind: diagnostics.outcomeKind,
    routeId: routeIdentifier,
    href: diagnostics.href,
    routeTruthEntry,
    verification() {
      return verification;
    },
  });
}

function createRouteHistoryEntry(report, diagnostics, eventIndex, previousEntry) {
  const outcome = report.outcome();
  if (outcome === null || outcome.kind !== "admitted") {
    return null;
  }
  const routeIdentifier = outcome.routeId ?? outcome.routeIdentity ?? report.routeIdentity ?? null;
  const restoreBoundary = report.restoreBoundary?.() ?? null;
  const runtimeRouteSourceId = report.runtimeRouteSourceId ?? null;
  const runtimeContinuitySourceId = report.runtimeContinuitySourceId ?? null;
  const breadcrumbAuthority = Object.freeze({
    restoreBoundary,
    runtimeRouteSourceId,
    runtimeContinuitySourceId,
  });
  const outletComposition = createRouteHistoryOutletComposition(outcome);
  const declaredBreadcrumbTrail = createDeclaredBreadcrumbTrail(
    outcome,
    report.restoredBreadcrumbs?.() ?? null,
    report.carriedBreadcrumbs?.() ?? null,
    breadcrumbAuthority,
  );
  const verification = Object.freeze({
    routeHistoryEntryDigest: createCanonicalDigest("browser-history-route-entry", {
      envelopeFamily: report.envelopeFamily,
      routeTruthDigest:
        "routeTruthDigest" in report.verification()
          ? report.verification().routeTruthDigest
          : null,
      boundarySource: diagnostics.boundarySource,
      boundaryArtifact: diagnostics.boundaryArtifact,
      routeId: routeIdentifier,
      href: outcome.href,
      breadcrumbTrailDigest: declaredBreadcrumbTrail?.verification().breadcrumbTrailDigest ?? null,
      outletCompositionDigest:
        outletComposition?.verification().outletCompositionDigest ?? null,
      previousEntryDigest: previousEntry?.verification().routeHistoryEntryDigest ?? null,
      restoreBoundaryDigest:
        restoreBoundary?.verification().routeRestoreBoundaryDigest ?? null,
      eventIndex,
    }),
  });
  return Object.freeze({
    kind: "routeHistoryEntry",
    eventIndex,
    boundarySource: diagnostics.boundarySource,
    boundaryArtifact: diagnostics.boundaryArtifact,
    navigationKind: report.navigationKind,
    routeId: routeIdentifier,
    href: outcome.href,
    routeIdentity: report.routeIdentity,
    runtimeRouteSourceId,
    runtimeContinuitySourceId,
    coherenceKind: diagnostics.coherenceKind ?? null,
    previous() {
      return previousEntry;
    },
    restoreBoundary() {
      return restoreBoundary;
    },
    restore(history) {
      if (restoreBoundary === null) {
        throw new TypeError(
          "routeHistoryEntry.restore(...) requires a restore boundary on the recorded browser-history event",
        );
      }
      return restoreRouteHistoryBoundary(history, restoreBoundary, {
        restoreSourceKind: "routeHistoryEntry",
        routeId: routeIdentifier,
        href: outcome.href,
        restoredEntryDigest: verification.routeHistoryEntryDigest,
      });
    },
    replay(history) {
      return createRouteHistoryReplayResult(
        history,
        {
          replaySourceKind: "routeHistoryEntry",
          routeId: routeIdentifier,
          href: outcome.href,
          replayedEntryDigest: verification.routeHistoryEntryDigest,
          runtimeRouteSourceId,
          runtimeContinuitySourceId,
        },
      );
    },
    breadcrumbTrail() {
      return declaredBreadcrumbTrail;
    },
    outletComposition() {
      return outletComposition;
    },
    route() {
      return outcome.route();
    },
    verification() {
      return verification;
    },
  });
}

function createRouteHistoryOutletComposition(outcome) {
  if (
    typeof outcome?.layouts !== "function"
    || typeof outcome?.outlet !== "function"
    || typeof outcome?.outlets !== "function"
  ) {
    return null;
  }
  const layouts = outcome.layouts();
  const outlet = outcome.outlet();
  const outlets = outcome.outlets();
  return createBrowserHistoryOutletCompositionArtifact(
    outcome.routeId,
    outcome.href,
    layouts,
    outlet,
    outlets,
  );
}

function createBackProvenanceArtifact(current, previous) {
  const verification = Object.freeze({
    backProvenanceDigest: createCanonicalDigest("browser-history-back-provenance", {
      currentEntryDigest: current?.verification().routeHistoryEntryDigest ?? null,
      previousEntryDigest: previous?.verification().routeHistoryEntryDigest ?? null,
    }),
  });
  return Object.freeze({
    kind: "browserHistoryBackProvenance",
    available: previous !== null,
    current,
    previous,
    restoreBoundary() {
      return previous?.restoreBoundary?.() ?? null;
    },
    restore(history) {
      if (previous === null) {
        throw new TypeError(
          "browserHistoryBackProvenance.restore(...) requires an earlier admitted route history entry",
        );
      }
      return previous.restore(history);
    },
    replay(history) {
      if (previous === null) {
        throw new TypeError(
          "browserHistoryBackProvenance.replay(...) requires an earlier admitted route history entry",
        );
      }
      return previous.replay(history);
    },
    verification() {
      return verification;
    },
  });
}

export {
  createBrowserHistoryStory,
};

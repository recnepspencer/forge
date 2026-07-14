import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-10 navigation auditability explains hydration-only visible route truth honestly", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/detail"),
  });
  const story = signals.router.browserHistory.story();

  try {
    const hydrationReport = await routes.admitHydrationHandoff(
      signals.router.hydration.server("/detail", {
        serverRouteIdentity: "home",
        serverHref: "/",
      }),
    );

    const auditability = story.auditability(hydrationReport);

    assert.deepEqual(auditability.summary(), {
      hydrationBoundaryPresent: true,
      hydrationBoundaryArtifact: "routeTruthDriftedFromServer",
      hydrationMatchesCurrentVisibleRoute: true,
      historyCurrentEntryPresent: false,
      currentVisibleRouteSource: "hydrationAdmission",
      currentVisibilityExplanation: "hydrationBoundary",
      currentBoundarySource: "hydrationHandoff",
      currentBoundaryArtifact: "routeTruthDriftedFromServer",
      currentNavigationIntent: "load",
      currentCoherenceKind: null,
      currentRouteId: "detail",
      currentHref: "/detail",
      currentRestoreAvailability: "unavailable",
      currentReplayAvailability: "unavailable",
      routeHistoryExplainsCurrent: false,
      restoreBoundaryExplainsCurrent: false,
      latestBoundarySource: null,
      latestBoundaryArtifact: null,
      latestBoundaryCoherenceKind: null,
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: false,
      externalNavigationCoherencePresent: false,
      convergedBoundaryEventCount: 0,
      driftedBoundaryEventCount: 0,
      notAdmittedBoundaryEventCount: 0,
    });
    assert.equal(
      auditability.hydrationBoundary()?.diagnostics().boundaryArtifact,
      "routeTruthDriftedFromServer",
    );
    assert.match(
      auditability.verification().navigationAuditabilityDigest,
      /router-navigation-auditability/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-10 navigation auditability prefers route-history truth while preserving hydration and coherence evidence", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routeTruth = signals.input("detailRoute");
  const continuity = signals.input("restored");
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/detail"),
  });
  const story = signals.router.browserHistory.story();

  try {
    const hydrationReport = await routes.admitHydrationHandoff(
      signals.router.hydration.server("/detail", {
        serverRouteIdentity: "home",
        serverHref: "/",
      }),
    );
    const crossTab = signals.router.browserHistory.coherence.crossTab("workspace-main", {
      sourceTabId: "tab-b",
      expectedRouteId: "home",
    });
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.external("/detail", {
          routeIdentity: "home",
          runtimeRouteSourceId: routeTruth.id,
          routeValue: routeTruth(),
          runtimeContinuitySourceId: continuity.id,
          continuityValue: continuity(),
          coherence: crossTab,
        }),
      ),
    );

    const auditability = story.auditability(hydrationReport);

    assert.deepEqual(auditability.summary(), {
      hydrationBoundaryPresent: true,
      hydrationBoundaryArtifact: "routeTruthDriftedFromServer",
      hydrationMatchesCurrentVisibleRoute: true,
      historyCurrentEntryPresent: true,
      currentVisibleRouteSource: "routeHistoryEntry",
      currentVisibilityExplanation: "routeHistoryEntry",
      currentBoundarySource: "browserHistoryIngress",
      currentBoundaryArtifact: "routeTruthDriftedFromAuthority",
      currentNavigationIntent: "external",
      currentCoherenceKind: "crossTab",
      currentRouteId: "detail",
      currentHref: "/detail",
      currentRestoreAvailability: "unavailable",
      currentReplayAvailability: "replayHistory",
      routeHistoryExplainsCurrent: true,
      restoreBoundaryExplainsCurrent: false,
      latestBoundarySource: "browserHistoryIngress",
      latestBoundaryArtifact: "routeTruthDriftedFromAuthority",
      latestBoundaryCoherenceKind: "crossTab",
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: true,
      externalNavigationCoherencePresent: false,
      convergedBoundaryEventCount: 0,
      driftedBoundaryEventCount: 1,
      notAdmittedBoundaryEventCount: 0,
    });
    assert.equal(
      auditability.historyInspection().currentEntry?.coherenceKind,
      "crossTab",
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

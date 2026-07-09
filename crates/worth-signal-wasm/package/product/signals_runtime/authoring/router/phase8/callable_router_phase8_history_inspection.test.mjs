import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-8 browser-history inspection unifies current back and breadcrumb authority", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routeTruth = signals.input("homeRoute");
  const continuity = signals.input("fresh");
  const routes = signals.router.define({
    home: signals.router.route("/", {
      breadcrumb: signals.router.breadcrumb({
        id: "home",
        label: "Home",
      }),
    }),
    result: signals.router.route("/search/results/:resultId", {
      breadcrumb: signals.router.breadcrumb({
        id: "result",
        label: ({ params }) => `Result ${params.resultId}`,
        parent: signals.router.breadcrumbParent({
          carry: true,
          fallback: signals.router.breadcrumbEntry({
            id: "search-fallback",
            label: "Search Results",
            target: "/search",
          }),
        }),
      }),
    }),
  });
  const story = signals.router.browserHistory.story();

  try {
    const homeBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.load("/", {
          routeIdentity: "homeRoute",
          runtimeRouteSourceId: routeTruth.id,
          routeValue: routeTruth(),
          runtimeContinuitySourceId: continuity.id,
          continuityValue: continuity(),
          restoreBoundary: homeBoundary,
        }),
      ),
    );
    const carriedBreadcrumbs = signals.router.carryBreadcrumbs(story.breadcrumbTrail());

    routeTruth.set("resultRoute");
    continuity.set("restored");
    const resultBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    story.record(
      await routes.applyBrowserHistoryWriteback(
        signals.router.browserHistory.writeback.push("/search/results/r1", {
          routeIdentity: "resultRoute",
          runtimeRouteSourceId: routeTruth.id,
          routeValue: routeTruth(),
          runtimeContinuitySourceId: continuity.id,
          continuityValue: continuity(),
          carriedBreadcrumbs,
          restoreBoundary: resultBoundary,
        }),
      ),
    );

    const inspection = story.inspection();

    assert.equal(inspection.currentEntry?.routeId, "result");
    assert.equal(inspection.backProvenance.previous?.routeId, "home");
    assert.equal(inspection.breadcrumbTrail.entries[0].sourceKind, "carriedProvenance");
    assert.deepEqual(
      inspection.breadcrumbProvenance().map((provenance) => provenance.sourceKind),
      ["carriedProvenance", "routeDeclaration"],
    );
    assert.deepEqual(inspection.summary(), {
      currentEntryAvailable: true,
      currentEntryRestoreAvailability: "restoreBoundary",
      currentEntryReplayAvailability: "replayHistory",
      backProvenanceAvailable: true,
      backRestoreAvailability: "restoreBoundary",
      backReplayAvailability: "replayHistory",
      currentOutletCompositionAvailable: true,
      backOutletCompositionAvailable: true,
      breadcrumbEntryCount: 2,
      breadcrumbRestoreAvailability: "all",
      breadcrumbReplayAvailability: "all",
      resolvedBreadcrumbCount: 1,
      recomputedBreadcrumbCount: 0,
      carriedBreadcrumbCount: 1,
      restoredBreadcrumbCount: 0,
      fallbackBreadcrumbCount: 0,
      routeDeclarationBreadcrumbPresent: true,
      recomputedBreadcrumbPresent: false,
      carriedBreadcrumbPresent: true,
      restoredBreadcrumbPresent: false,
      fallbackBreadcrumbPresent: false,
      historyFallbackBreadcrumbPresent: false,
      latestBoundaryCoherenceKind: null,
      currentRouteTruthCoherenceKind: null,
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: false,
      externalNavigationCoherencePresent: false,
      convergedBoundaryEventCount: 2,
      driftedBoundaryEventCount: 0,
      notAdmittedBoundaryEventCount: 0,
    });
    assert.match(
      inspection.verification().historyInspectionDigest,
      /browser-history-inspection/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 browser-history inspection fails closed when restore and replay authority are absent", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    result: signals.router.route("/search/results/:resultId", {
      breadcrumb: signals.router.breadcrumb({
        id: "result",
        label: ({ params }) => `Result ${params.resultId}`,
        parent: signals.router.breadcrumbParent({
          fallback: signals.router.breadcrumbEntry({
            id: "search-fallback",
            label: "Search Results",
            target: "/search",
          }),
        }),
      }),
    }),
  });
  const story = signals.router.browserHistory.story();

  try {
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.load("/search/results/r17", {
          routeIdentity: "result:r17",
        }),
      ),
    );

    assert.deepEqual(story.inspection().summary(), {
      currentEntryAvailable: true,
      currentEntryRestoreAvailability: "unavailable",
      currentEntryReplayAvailability: "unavailable",
      backProvenanceAvailable: false,
      backRestoreAvailability: "unavailable",
      backReplayAvailability: "unavailable",
      currentOutletCompositionAvailable: true,
      backOutletCompositionAvailable: false,
      breadcrumbEntryCount: 2,
      breadcrumbRestoreAvailability: "none",
      breadcrumbReplayAvailability: "none",
      resolvedBreadcrumbCount: 1,
      recomputedBreadcrumbCount: 0,
      carriedBreadcrumbCount: 0,
      restoredBreadcrumbCount: 0,
      fallbackBreadcrumbCount: 1,
      routeDeclarationBreadcrumbPresent: true,
      recomputedBreadcrumbPresent: false,
      carriedBreadcrumbPresent: false,
      restoredBreadcrumbPresent: false,
      fallbackBreadcrumbPresent: true,
      historyFallbackBreadcrumbPresent: false,
      latestBoundaryCoherenceKind: null,
      currentRouteTruthCoherenceKind: null,
      sameTabCoherencePresent: false,
      crossTabCoherencePresent: false,
      externalNavigationCoherencePresent: false,
      convergedBoundaryEventCount: 1,
      driftedBoundaryEventCount: 0,
      notAdmittedBoundaryEventCount: 0,
    });
  } finally {
    signals.free();
    await cleanup();
  }
});

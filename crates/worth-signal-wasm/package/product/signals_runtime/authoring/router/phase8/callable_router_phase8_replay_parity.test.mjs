import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-8 restored breadcrumb provenance outranks carried provenance and preserves replay parity", async () => {
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
    const homeReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/", {
        routeIdentity: "homeRoute",
        runtimeRouteSourceId: routeTruth.id,
        routeValue: routeTruth(),
        runtimeContinuitySourceId: continuity.id,
        continuityValue: continuity(),
        restoreBoundary: homeBoundary,
      }),
    );
    story.record(homeReport);

    const restoredBreadcrumbs = signals.router.restoreBreadcrumbs(
      story.breadcrumbTrail().entries,
    );
    const carriedBreadcrumbs = signals.router.carryBreadcrumbs(story.breadcrumbTrail());

    const resultReport = await routes.applyBrowserHistoryWriteback(
      signals.router.browserHistory.writeback.push("/search/results/r1", {
        routeIdentity: "resultRoute",
        runtimeRouteSourceId: routeTruth.id,
        routeValue: "resultRoute",
        runtimeContinuitySourceId: continuity.id,
        continuityValue: "restored",
        restoredBreadcrumbs,
        carriedBreadcrumbs,
      }),
    );
    story.record(resultReport);

    const trail = story.breadcrumbTrail();
    const restoredEntry = trail.entries[0];
    const currentReplay = story.current()?.replay(signals.history());
    const backReplay = story.backProvenance().replay(signals.history());
    const breadcrumbReplay = restoredEntry.replay(signals.history());
    const expectedRouteReplay = signals.history().replay_for(routeTruth.id);
    const expectedContinuityReplay = signals.history().replay_for(continuity.id);

    assert.equal(restoredEntry.status, "restored");
    assert.equal(restoredEntry.sourceKind, "restoredProvenance");
    assert.equal(restoredEntry.provenance().sourceKind, "restoredProvenance");
    assert.equal(restoredEntry.provenance().restoreAvailability, "restoreBoundary");
    assert.equal(restoredEntry.provenance().replayAvailability, "replayHistory");
    assert.equal(
      resultReport.restoredBreadcrumbs()?.verification().restoredBreadcrumbsDigest,
      restoredBreadcrumbs.verification().restoredBreadcrumbsDigest,
    );
    assert.equal(
      resultReport.carriedBreadcrumbs()?.verification().carriedBreadcrumbsDigest,
      carriedBreadcrumbs.verification().carriedBreadcrumbsDigest,
    );

    assert.equal(currentReplay?.kind, "routeHistoryReplayResult");
    assert.deepEqual(currentReplay?.routeReplay, expectedRouteReplay);
    assert.deepEqual(currentReplay?.continuityReplay, expectedContinuityReplay);
    assert.deepEqual(backReplay.routeReplay, expectedRouteReplay);
    assert.deepEqual(backReplay.continuityReplay, expectedContinuityReplay);
    assert.deepEqual(breadcrumbReplay.routeReplay, expectedRouteReplay);
    assert.deepEqual(breadcrumbReplay.continuityReplay, expectedContinuityReplay);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 restored breadcrumb artifacts fail closed for non-restore-backed entries", async () => {
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

    assert.throws(
      () => signals.router.restoreBreadcrumbs(story.breadcrumbTrail().entries),
      /requires restore-backed breadcrumb entries/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-8 route history entries and back provenance restore exact snapshot boundaries", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routeTruth = signals.input("homeRoute");
  const continuity = signals.input("fresh");
  const routes = signals.router.define({
    home: signals.router.route("/"),
    settings: signals.router.route("/settings"),
  });
  const story = signals.router.browserHistory.story();

  try {
    const homeBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    const homeReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/", {
        routeIdentity: "homeRoute",
        restoreBoundary: homeBoundary,
      }),
    );
    story.record(homeReport);

    routeTruth.set("settingsRoute");
    continuity.set("restored");
    const settingsBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    const settingsReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.push("/settings", {
        routeIdentity: "settingsRoute",
        restoreBoundary: settingsBoundary,
      }),
    );
    story.record(settingsReport);

    routeTruth.set("mutatedRoute");
    continuity.set("mutated");

    const restoreResult = story.backProvenance().restore(signals.history());

    assert.equal(routeTruth(), "homeRoute");
    assert.equal(continuity(), "fresh");
    assert.equal(restoreResult.kind, "routeHistoryRestoreResult");
    assert.equal(restoreResult.restoreSourceKind, "routeHistoryEntry");
    assert.equal(restoreResult.routeId, "home");
    assert.equal(restoreResult.href, "/");
    assert.equal(
      story.backProvenance().restoreBoundary()?.verification().routeRestoreBoundaryDigest,
      homeBoundary.verification().routeRestoreBoundaryDigest,
    );
    assert.equal(
      story.current()?.restoreBoundary()?.verification().routeRestoreBoundaryDigest,
      settingsBoundary.verification().routeRestoreBoundaryDigest,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 route history preserves nested layout outlet composition through restore-backed back provenance", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routeTruth = signals.input("workspace.project.settings");
  const routes = signals.router.define({
    workspace: signals.router.layout(
      signals.router.route("/workspace/:workspaceId"),
      {
        project: signals.router.layout(
          signals.router.route("/workspace/:workspaceId/projects/:projectId"),
          { outlet: "detail" },
          {
            settings: signals.router.route("/workspace/:workspaceId/projects/:projectId/settings"),
          },
        ),
      },
    ),
    home: signals.router.route("/"),
  });
  const story = signals.router.browserHistory.story();

  try {
    const nestedBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.load("/workspace/acme/projects/p1/settings", {
          routeIdentity: routeTruth(),
          restoreBoundary: nestedBoundary,
        }),
      ),
    );

    routeTruth.set("homeRoute");
    const homeBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.push("/", {
          routeIdentity: routeTruth(),
          restoreBoundary: homeBoundary,
        }),
      ),
    );

    const backComposition = story.backProvenance().previous?.outletComposition();
    const inspection = story.inspection();
    const restoreResult = story.backProvenance().restore(signals.history());

    assert.equal(routeTruth(), "workspace.project.settings");
    assert.equal(restoreResult.routeId, "workspace.project.settings");
    assert.deepEqual(backComposition?.summary(), {
      layoutCount: 2,
      outletCount: 2,
      layoutRouteIds: ["workspace", "workspace.project"],
      outletIds: ["default", "detail"],
      occupantRouteIds: ["workspace.project", "workspace.project.settings"],
    });
    assert.equal(
      backComposition?.verification().outletCompositionDigest,
      inspection.backOutletComposition()?.verification().outletCompositionDigest,
    );
    assert.equal(inspection.summary().currentOutletCompositionAvailable, true);
    assert.equal(inspection.summary().backOutletCompositionAvailable, true);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 carried breadcrumb provenance preserves restore-backed crumb return", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routeTruth = signals.input("homeRoute");
  const routes = signals.router.define({
    home: signals.router.route("/"),
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
        restoreBoundary: homeBoundary,
      }),
    );
    story.record(homeReport);
    const carriedBreadcrumbs = signals.router.carryBreadcrumbs(story.breadcrumbTrail());

    routeTruth.set("resultRoute");
    const resultBoundary = signals.router.restoreBoundary(signals.history().snapshot());
    const resultReport = await routes.applyBrowserHistoryWriteback(
      signals.router.browserHistory.writeback.push("/search/results/r1", {
        routeIdentity: "resultRoute",
        carriedBreadcrumbs,
        restoreBoundary: resultBoundary,
      }),
    );
    story.record(resultReport);

    routeTruth.set("mutatedRoute");
    const carriedEntry = story.breadcrumbTrail().entries[0];
    const restoreResult = carriedEntry.restore(signals.history());

    assert.equal(carriedEntry.sourceKind, "carriedProvenance");
    assert.equal(carriedEntry.provenance().sourceKind, "carriedProvenance");
    assert.equal(carriedEntry.provenance().restoreAvailability, "restoreBoundary");
    assert.equal(carriedEntry.provenance().replayAvailability, "replayHistory");
    assert.equal(routeTruth(), "homeRoute");
    assert.equal(restoreResult.routeId, "home");
    assert.equal(
      carriedEntry.restoreBoundary()?.verification().routeRestoreBoundaryDigest,
      homeBoundary.verification().routeRestoreBoundaryDigest,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 restore-backed back and breadcrumb return fail closed without restore boundaries", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/detail", {
      breadcrumb: signals.router.breadcrumb({
        id: "detail",
        label: "Detail",
        parent: signals.router.breadcrumbParent({
          fallback: signals.router.breadcrumbEntry({
            id: "home-fallback",
            label: "Home",
            target: "/",
          }),
        }),
      }),
    }),
  });
  const story = signals.router.browserHistory.story();

  try {
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.load("/", {
          routeIdentity: "homeRoute",
        }),
      ),
    );
    story.record(
      await routes.admitBrowserHistoryIngress(
        signals.router.browserHistory.push("/detail", {
          routeIdentity: "detailRoute",
        }),
      ),
    );

    assert.equal(story.backProvenance().restoreBoundary(), null);
    assert.throws(
      () => story.backProvenance().restore(signals.history()),
      /requires a restore boundary/,
    );
    assert.equal(story.breadcrumbTrail().entries[0].restoreBoundary(), null);
    assert.throws(
      () => story.breadcrumbTrail().entries[0].restore(signals.history()),
      /requires restore-backed breadcrumb provenance/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

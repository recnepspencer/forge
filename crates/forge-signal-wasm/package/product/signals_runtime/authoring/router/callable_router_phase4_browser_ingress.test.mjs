import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-4 browser-history ingress converges compatibility route truth and preserves worker-first facade parity", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createRouterNamespace } = await importProductModule("router/router_namespace.js");

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const workerSignals = await createSignals({ deployment: "workerFirst" });
  const workerRouter = createRouterNamespace();
  const workerScopedRouter = createRouterNamespace("nested");
  const routes = definePhaseFourRoutes(compatibilitySignals);
  const workerRoutes = definePhaseFourRoutes(workerSignals);
  const ingress = compatibilitySignals.router.browserHistory.push("/search?q=forge", {
    routeIdentity: "searchRoute:forge",
    runtimeRouteSourceId: "routeIdentity",
    routeValue: "searchRoute:forge",
    runtimeContinuitySourceId: "routeContinuity",
    continuityValue: "restored",
  });
  const workerIngress = workerRouter.browserHistory.push("/search?q=forge", {
    routeIdentity: "searchRoute:forge",
    runtimeRouteSourceId: "routeIdentity",
    routeValue: "searchRoute:forge",
    runtimeContinuitySourceId: "routeContinuity",
    continuityValue: "restored",
  });

  try {
    assert.equal(compatibilitySignals.router.browserHistory.manual("/").navigationKind, "manual");
    assert.equal(workerRouter.browserHistory.external("/").navigationKind, "external");
    assert.equal(compatibilitySignals.router.browserHistory.push("/search").rawLocation.navigationType, "push");
    assert.equal(workerRouter.browserHistory.replace("/search").rawLocation.navigationType, "replace");
    assert.equal(
      workerScopedRouter.browserHistory.replace("/search?q=forge").navigationKind,
      "replacestate",
    );
    assert.equal(
      compatibilitySignals.scope("nested").router.browserHistory.replace("/search?q=forge")
        .verification().browserHistoryEnvelopeDigest,
      compatibilitySignals.router.browserHistory.replace("/search?q=forge")
        .verification().browserHistoryEnvelopeDigest,
    );
    assert.equal(
      ingress.verification().browserHistoryEnvelopeDigest,
      workerIngress.verification().browserHistoryEnvelopeDigest,
    );

    const report = await routes.admitBrowserHistoryIngress(ingress);
    const workerReport = await workerRoutes.admitBrowserHistoryIngress(workerIngress);
    assert.equal(report.envelopeFamily, "browserHistoryIngress");
    assert.equal(report.navigationKind, "pushstate");
    assert.equal(report.rawLocationHref, "/search?q=forge");
    assert.equal(report.routeIdentity, "searchRoute:forge");
    assert.equal(report.outcome().kind, "admitted");
    assert.equal(report.outcome().route().kind, "admittedRouteCapability");
    assert.equal(report.outcome().routeId, "search");
    assert.equal(report.outcome().href, "/search?q=forge");
    assert.equal(report.diagnostics().outcomeKind, "admitted");
    assert.equal(report.diagnostics().routeId, "search");
    assert.equal(
      report.verification().browserHistoryEnvelopeDigest,
      ingress.verification().browserHistoryEnvelopeDigest,
    );
    assert.match(report.verification().routeTruthDigest, /browser-history-route-truth/);
    assert.match(report.verification().continuityDigest, /browser-history-continuity/);
    assert.equal(workerReport.outcome().kind, "admitted");
    assert.equal(workerReport.outcome().routeId, "search");
    assert.equal(workerReport.outcome().href, "/search?q=forge");
    assert.equal(workerReport.diagnostics().routeId, "search");
    assert.equal(
      workerReport.verification().routeTruthDigest,
      report.verification().routeTruthDigest,
    );
    assert.equal(
      workerReport.verification().continuityDigest,
      report.verification().continuityDigest,
    );
  } finally {
    compatibilitySignals.free();
    await workerSignals.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("phase-4 browser-history ingress fails closed for no-match compatibility truth and non-envelope admission", async () => {
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createRouterNamespace } = await importProductModule("router/router_namespace.js");
  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const workerRouter = createRouterNamespace();
  const routes = compatibilitySignals.router.define({
    home: compatibilitysignals.router.route("/"),
  });
  const unmatchedIngress = compatibilitySignals.router.browserHistory.pop("/missing");
  const workerIngress = workerRouter.browserHistory.push("/search?q=forge");

  try {
    const unmatchedReport = await routes.admitBrowserHistoryIngress(unmatchedIngress);
    assert.equal(unmatchedReport.outcome().kind, "notFound");
    assert.equal(unmatchedReport.diagnostics().routeId, null);
    assert.equal(unmatchedReport.rawLocationHref, "/missing");
    assert.equal(workerIngress.routeIdentity, null);
    assert.equal(workerIngress.rawLocation.navigationType, "push");

    await assert.rejects(
      () => routes.admitBrowserHistoryIngress("/missing"),
      /requires an ingress envelope created by signals\.router\.browserHistory\.\*\(\.\.\.\)/,
    );
  } finally {
    compatibilitySignals.free();
    await cleanup();
  }
});

test("phase-4 browser-history writeback converges local route truth and preserves explicit external navigation boundary", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, importProductModule, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const { createRouterNamespace } = await importProductModule("router/router_namespace.js");
  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const workerSignals = await createSignals({ deployment: "workerFirst" });
  const workerRouter = createRouterNamespace();
  const routes = definePhaseFourRoutes(compatibilitySignals);
  const workerRoutes = definePhaseFourRoutes(workerSignals);
  const localWriteback = compatibilitySignals.router.browserHistory.writeback.replace(
    routes.search.to({ search: { q: "forge" } }),
    {
      routeIdentity: "searchRoute:forge",
      runtimeRouteSourceId: "routeIdentity",
      routeValue: "searchRoute:forge",
      runtimeContinuitySourceId: "routeContinuity",
      continuityValue: "restored",
    },
  );
  const workerLocalWriteback = workerRouter.browserHistory.writeback.replace("/search?q=forge", {
    routeIdentity: "searchRoute:forge",
    runtimeRouteSourceId: "routeIdentity",
    routeValue: "searchRoute:forge",
    runtimeContinuitySourceId: "routeContinuity",
    continuityValue: "restored",
  });
  const externalWriteback = compatibilitySignals.router.browserHistory.writeback.external(
    "https://example.com/docs/router",
    {
      routeIdentity: "searchRoute:forge",
    },
  );

  try {
    assert.equal(localWriteback.targetKind, "local");
    assert.equal(localWriteback.rawLocation?.href, "/search?q=forge");
    assert.equal(workerLocalWriteback.targetKind, "local");
    assert.equal(
      localWriteback.verification().browserHistoryWritebackDigest,
      workerLocalWriteback.verification().browserHistoryWritebackDigest,
    );

    const localReport = await routes.applyBrowserHistoryWriteback(localWriteback);
    const workerReport = await workerRoutes.applyBrowserHistoryWriteback(workerLocalWriteback);
    assert.equal(localReport.envelopeFamily, "browserHistoryWriteback");
    assert.equal(localReport.navigationKind, "replacestate");
    assert.equal(localReport.targetKind, "local");
    assert.equal(localReport.targetHref, "/search?q=forge");
    assert.equal(localReport.outcome()?.kind, "admitted");
    assert.equal(localReport.outcome()?.routeId, "search");
    assert.equal(localReport.diagnostics().boundarySource, "browserHistoryWriteback");
    assert.equal(localReport.diagnostics().boundaryArtifact, "routeTruthConverged");
    assert.match(
      localReport.verification().routeTruthDigest,
      /browser-history-writeback-route-truth/,
    );
    assert.match(
      localReport.verification().boundaryStoryDigest,
      /browser-history-writeback-boundary-story/,
    );
    assert.equal(workerReport.outcome()?.kind, "admitted");
    assert.equal(workerReport.outcome()?.routeId, "search");
    assert.equal(
      workerReport.verification().routeTruthDigest,
      localReport.verification().routeTruthDigest,
    );
    assert.equal(
      workerReport.verification().boundaryStoryDigest,
      localReport.verification().boundaryStoryDigest,
    );

    const externalReport = await routes.applyBrowserHistoryWriteback(externalWriteback);
    assert.equal(externalReport.envelopeFamily, "browserHistoryWriteback");
    assert.equal(externalReport.targetKind, "external");
    assert.equal(externalReport.targetHref, "https://example.com/docs/router");
    assert.equal(externalReport.outcome(), null);
    assert.equal(externalReport.diagnostics().boundaryArtifact, "externalNavigationEscaped");
    assert.equal(externalReport.diagnostics().outcomeKind, null);

    assert.throws(
      () => compatibilitySignals.router.browserHistory.writeback.external("/search?q=forge"),
      /rejects local href strings/,
    );
    assert.throws(
      () => compatibilitySignals.router.browserHistory.writeback.replace("/search?q=forge"),
      /requires routeIdentity for local graph-issued writeback/,
    );
    await assert.rejects(
      () => routes.applyBrowserHistoryWriteback("/search?q=forge"),
      /requires a writeback envelope created by signals\.router\.browserHistory\.writeback\.\*\(\.\.\.\)/,
    );
  } finally {
    compatibilitySignals.free();
    await workerSignals.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("phase-4 browser-history story unifies ingress and writeback into one back and breadcrumb provenance trail", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = compatibilitySignals.router.define({
    home: compatibilitysignals.router.route("/"),
    search: compatibilitySignals.router.route("/search", {
      search: {
        q: compatibilitySignals.router.search.required.string(),
      },
    }),
    settings: compatibilitysignals.router.route("/settings"),
  });
  const story = compatibilitySignals.router.browserHistory.story();

  try {
    const homeIngress = await routes.admitBrowserHistoryIngress(
      compatibilitySignals.router.browserHistory.load("/", {
        routeIdentity: "homeRoute",
      }),
    );
    const searchWriteback = await routes.applyBrowserHistoryWriteback(
      compatibilitySignals.router.browserHistory.writeback.push(
        routes.search.to({ search: { q: "forge" } }),
        {
          routeIdentity: "searchRoute:forge",
        },
      ),
    );
    const settingsIngress = await routes.admitBrowserHistoryIngress(
      compatibilitySignals.router.browserHistory.push("/settings", {
        routeIdentity: "settingsRoute",
      }),
    );
    const externalEscape = await routes.applyBrowserHistoryWriteback(
      compatibilitySignals.router.browserHistory.writeback.external(
        "https://example.com/docs/router",
      ),
    );

    const homeEvent = story.record(homeIngress);
    const searchEvent = story.record(searchWriteback);
    const settingsEvent = story.record(settingsIngress);
    const externalEvent = story.record(externalEscape);

    assert.equal(homeEvent.routeTruthEntry?.routeId, "home");
    assert.equal(searchEvent.routeTruthEntry?.routeId, "search");
    assert.equal(settingsEvent.routeTruthEntry?.routeId, "settings");
    assert.equal(externalEvent.routeTruthEntry, null);
    assert.equal(externalEvent.boundaryArtifact, "externalNavigationEscaped");

    assert.equal(story.current()?.routeId, "settings");
    assert.equal(story.back()?.routeId, "search");
    assert.equal(story.current()?.previous()?.routeId, "search");
    assert.equal(story.latestBoundaryEvent()?.boundaryArtifact, "externalNavigationEscaped");
    assert.equal(story.currentRouteTruthEvent()?.routeTruthEntry?.routeId, "settings");
    assert.equal(story.currentRouteTruthEvent()?.targetHref, "/settings");
    assert.equal(story.currentRouteTruthEvent()?.advancedRouteTruth, true);
    assert.deepEqual(
      story.breadcrumbs().map((entry) => [entry.routeId, entry.href]),
      [
        ["home", "/"],
        ["search", "/search?q=forge"],
        ["settings", "/settings"],
      ],
    );
    assert.equal(story.backProvenance().available, true);
    assert.equal(story.backProvenance().current?.routeId, "settings");
    assert.equal(story.backProvenance().previous?.routeId, "search");
    assert.deepEqual(
      story.breadcrumbTrail().entries.map((entry) => entry.routeId),
      ["home", "search", "settings"],
    );
    assert.equal(story.events().length, 4);
    assert.equal(story.admittedEntries().length, 3);
    assert.match(story.verification().historyStoryDigest, /browser-history-story/);
    assert.match(
      story.backProvenance().verification().backProvenanceDigest,
      /browser-history-back-provenance/,
    );
    assert.match(
      story.breadcrumbTrail().verification().breadcrumbTrailDigest,
      /browser-history-breadcrumb-trail/,
    );

    assert.throws(
      () => story.record({ envelopeFamily: "browserHistoryIngress" }),
      /requires a boundary report from routes\.admitBrowserHistoryIngress/,
    );
  } finally {
    compatibilitySignals.free();
    await cleanup();
  }
});

function definePhaseFourRoutes(signals) {
  return signals.router.define({
    home: signals.router.route("/"),
    search: signals.router.route("/search", {
      search: {
        q: signals.router.search.required.string(),
      },
    }),
    settings: signals.router.route("/settings"),
  });
}

import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-8 declared breadcrumb trail composes layout contribution plus recomputed route ancestry", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    workspace: signals.router.layout(
      signals.router.route("/workspace/:workspaceId", {
        breadcrumb: signals.router.breadcrumb({
          id: "workspace",
          label: ({ params }) => `Workspace ${params.workspaceId}`,
        }),
      }),
      {
        result: signals.router.route("/workspace/:workspaceId/search/results/:resultId", {
          breadcrumb: signals.router.breadcrumb({
            id: "result",
            label: ({ params }) => `Result ${params.resultId}`,
            parent: signals.router.breadcrumbParent({
              recompute: ({ params }) => (
                params.resultId === "durable"
                  ? signals.router.breadcrumbTrail([
                    signals.router.breadcrumbEntry({
                      id: "search-context",
                      label: `Saved Search ${params.workspaceId}`,
                      target: `/workspace/${params.workspaceId}/search`,
                    }),
                  ])
                  : null
              ),
              fallback: signals.router.breadcrumbEntry({
                id: "search-results",
                label: "Search Results",
                target: "/search",
              }),
            }),
          }),
        }),
      },
    ),
  });
  const story = signals.router.browserHistory.story();

  try {
    const report = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/workspace/acme/search/results/durable", {
        routeIdentity: "workspace:acme:result:durable",
      }),
    );
    const event = story.record(report);
    const trail = story.breadcrumbTrail();

    assert.equal(event.routeTruthEntry?.routeId, "workspace.result");
    assert.equal(event.routeTruthEntry?.breadcrumbTrail()?.verification().breadcrumbTrailDigest, trail.verification().breadcrumbTrailDigest);
    assert.equal(trail.entries[1].provenance().sourceKind, "recomputed");
    assert.equal(trail.entries[1].provenance().replayAvailability, "unavailable");
    assert.deepEqual(
      trail.entries.map((entry) => ({
        crumbId: entry.crumbId,
        label: entry.label,
        status: entry.status,
        sourceKind: entry.sourceKind,
        targetHref: entry.targetHref,
      })),
      [
        {
          crumbId: "workspace",
          label: "Workspace acme",
          status: "resolved",
          sourceKind: "routeDeclaration",
          targetHref: "/workspace/acme",
        },
        {
          crumbId: "search-context",
          label: "Saved Search acme",
          status: "recomputed",
          sourceKind: "recomputed",
          targetHref: "/workspace/acme/search",
        },
        {
          crumbId: "result",
          label: "Result durable",
          status: "resolved",
          sourceKind: "routeDeclaration",
          targetHref: "/workspace/acme/search/results/durable",
        },
      ],
    );
    assert.match(
      trail.verification().breadcrumbTrailDigest,
      /browser-history-breadcrumb-trail/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 breadcrumb trail fails closed into explicit fallback for deep links without durable ancestry", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    result: signals.router.route("/search/results/:resultId", {
      breadcrumb: signals.router.breadcrumb({
        id: "result",
        label: ({ params }) => `Result ${params.resultId}`,
        parent: signals.router.breadcrumbParent({
          recompute: () => null,
          fallback: signals.router.breadcrumbEntry({
            id: "search-results",
            label: "Search Results",
            target: "/search",
          }),
        }),
      }),
    }),
  });
  const story = signals.router.browserHistory.story();

  try {
    const report = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/search/results/r17", {
        routeIdentity: "result:r17",
      }),
    );
    story.record(report);
    const trail = story.breadcrumbTrail();

    assert.deepEqual(
      trail.entries.map((entry) => ({
        crumbId: entry.crumbId,
        label: entry.label,
        status: entry.status,
        sourceKind: entry.sourceKind,
        routeId: entry.routeId,
        targetHref: entry.targetHref,
      })),
      [
        {
          crumbId: "search-results",
          label: "Search Results",
          status: "fallback",
          sourceKind: "fallback",
          routeId: "result",
          targetHref: "/search",
        },
        {
          crumbId: "result",
          label: "Result r17",
          status: "resolved",
          sourceKind: "routeDeclaration",
          routeId: "result",
          targetHref: "/search/results/r17",
        },
      ],
    );
    assert.notEqual(trail.entries[0].targetHref, "/search/results/r17");
    assert.equal(story.currentRouteTruthEvent()?.routeTruthEntry?.routeId, "result");
    assert.equal(trail.entries[0].provenance().sourceKind, "fallback");
    assert.equal(trail.entries[0].provenance().restoreAvailability, "unavailable");
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-8 explicit carried breadcrumb provenance wins over fallback and stays opt-in", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    search: signals.router.route("/search", {
      breadcrumb: signals.router.breadcrumb({
        id: "search-results",
        label: "Search Results",
        target: "/search",
      }),
    }),
    resultCarried: signals.router.route("/search/results/:resultId", {
      breadcrumb: signals.router.breadcrumb({
        id: "result",
        label: ({ params }) => `Result ${params.resultId}`,
        parent: signals.router.breadcrumbParent({
          carry: true,
          fallback: signals.router.breadcrumbEntry({
            id: "search-fallback",
            label: "Search Fallback",
            target: "/search",
          }),
        }),
      }),
    }),
    resultNoCarry: signals.router.route("/plain/results/:resultId", {
      breadcrumb: signals.router.breadcrumb({
        id: "result",
        label: ({ params }) => `Result ${params.resultId}`,
        parent: signals.router.breadcrumbParent({
          fallback: signals.router.breadcrumbEntry({
            id: "plain-fallback",
            label: "Plain Fallback",
            target: "/plain",
          }),
        }),
      }),
    }),
  });
  const story = signals.router.browserHistory.story();

  try {
    const searchReport = await routes.admitBrowserHistoryIngress(
      signals.router.browserHistory.load("/search", {
        routeIdentity: "search-root",
      }),
    );
    story.record(searchReport);
    const carriedBreadcrumbs = signals.router.carryBreadcrumbs(story.breadcrumbTrail());

    const carriedReport = await routes.applyBrowserHistoryWriteback(
      signals.router.browserHistory.writeback.push(
        "/search/results/r55",
        {
          routeIdentity: "result:r55",
          carriedBreadcrumbs,
        },
      ),
    );
    story.record(carriedReport);
    const carriedTrail = story.breadcrumbTrail();

    assert.deepEqual(
      carriedTrail.entries.map((entry) => ({
        crumbId: entry.crumbId,
        label: entry.label,
        status: entry.status,
        sourceKind: entry.sourceKind,
        targetHref: entry.targetHref,
      })),
      [
        {
          crumbId: "search-results",
          label: "Search Results",
          status: "carried",
          sourceKind: "carriedProvenance",
          targetHref: "/search",
        },
        {
          crumbId: "result",
          label: "Result r55",
          status: "resolved",
          sourceKind: "routeDeclaration",
          targetHref: "/search/results/r55",
        },
      ],
    );
    assert.equal(
      carriedReport.carriedBreadcrumbs()?.verification().carriedBreadcrumbsDigest,
      carriedBreadcrumbs.verification().carriedBreadcrumbsDigest,
    );
    assert.equal(carriedTrail.entries[0].provenance().sourceKind, "carriedProvenance");
    assert.equal(carriedTrail.entries[0].provenance().restoreAvailability, "unavailable");

    const noCarryReport = await routes.applyBrowserHistoryWriteback(
      signals.router.browserHistory.writeback.push(
        "/plain/results/r77",
        {
          routeIdentity: "plain:r77",
          carriedBreadcrumbs,
        },
      ),
    );
    story.record(noCarryReport);
    const noCarryTrail = story.current()?.breadcrumbTrail();

    assert.deepEqual(
      noCarryTrail?.entries.map((entry) => ({
        crumbId: entry.crumbId,
        sourceKind: entry.sourceKind,
        label: entry.label,
      })),
      [
        {
          crumbId: "plain-fallback",
          sourceKind: "fallback",
          label: "Plain Fallback",
        },
        {
          crumbId: "result",
          sourceKind: "routeDeclaration",
          label: "Result r77",
        },
      ],
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

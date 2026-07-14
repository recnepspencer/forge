import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";

test("phase-11 route sequence simulation replays step outcomes without app-authored ingress loops", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/", {
      breadcrumb: signals.router.breadcrumb({
        id: "home",
        label: "Home",
      }),
    }),
    users: signals.router.route("/users", {
      breadcrumb: signals.router.breadcrumb({
        id: "users",
        label: "Users",
      }),
    }),
    userDetail: signals.router.route("/users/:userId", {
      breadcrumb: signals.router.breadcrumb({
        id: "user-detail",
        label: ({ params }) => `User ${params.userId}`,
        parent: signals.router.breadcrumbParent({
          carry: true,
        }),
      }),
    }),
  });

  try {
    const scenario = routes.simulateSequence([
      routes.home.to(),
      routes.users.to(),
      routes.userDetail.to({ params: { userId: "u-7" } }),
    ]);

    const result = await scenario.run();

    assert.equal(result.steps.length, 3);
    assert.equal(result.story.current()?.routeId, "userDetail");
    assert.deepEqual(
      result.steps.map((step) => ({
        navigationKind: step.navigationKind,
        routeId: step.report.outcome().routeId,
        eventRouteId: step.event.routeId,
      })),
      [
        { navigationKind: "load", routeId: "home", eventRouteId: "home" },
        { navigationKind: "pushstate", routeId: "users", eventRouteId: "users" },
        { navigationKind: "pushstate", routeId: "userDetail", eventRouteId: "userDetail" },
      ],
    );
    assert.deepEqual(
      result.replay.breadcrumbTrail().map((trail) => trail.entries.map((entry) => entry.label)),
      [
        ["Home"],
        ["Users"],
        ["Users", "User u-7"],
      ],
    );
    assert.deepEqual(
      result.replay.currentEntries().map((entry) => entry?.routeId ?? null),
      ["home", "users", "userDetail"],
    );
    assert.deepEqual(result.diagnostics(), {
      hasFailures: false,
      denied: [],
      notAdmitted: [],
    });
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-11 route sequence simulation retains structured not-admitted outcomes", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
  });

  try {
    const result = await routes.simulateSequence([
      routes.home.to(),
      "/missing",
    ]).run();

    assert.equal(result.steps[1].report.outcome().kind, "notFound");
    assert.equal(result.steps[1].event.boundaryArtifact, "routeOutcomeNotAdmitted");
    assert.equal(result.story.current()?.routeId, "home");
    assert.deepEqual(
      result.replay.outcomes().map((outcome) => outcome.kind),
      ["admitted", "notFound"],
    );
    assert.deepEqual(result.diagnostics(), {
      hasFailures: true,
      denied: [],
      notAdmitted: [
        {
          index: 1,
          targetHref: "/missing",
          outcomeKind: "notFound",
          eventBoundaryArtifact: "routeOutcomeNotAdmitted",
        },
      ],
    });
  } finally {
    signals.free();
    await cleanup();
  }
});

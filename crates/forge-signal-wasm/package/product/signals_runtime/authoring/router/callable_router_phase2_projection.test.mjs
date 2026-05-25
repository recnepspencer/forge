import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-2 route projection builds explicit layout and outlet composition", async () => {
  await withPhaseTwoRouterFixture(async ({ routes }) => {
    const projectedCandidate = routes.project("/users/user%201?tab=activity");
    assert.ok(projectedCandidate);
    assert.equal(projectedCandidate.kind, "projectedCandidate");
    assert.equal(projectedCandidate.href, "/users/user%201?tab=activity");
    assert.equal(projectedCandidate.routeId, "app.users.detail");
    assert.equal(projectedCandidate.canonicalUrl().href, "/users/user%201?tab=activity");

    const projectedRoute = projectedCandidate.route();
    assert.equal(projectedRoute.kind, "projectedRouteCapability");
    assert.equal(projectedRoute.routeId, "app.users.detail");
    assert.deepEqual(projectedRoute.params, { userId: "user 1" });
    assert.deepEqual(projectedRoute.search, { tab: "activity" });
    assert.equal(projectedRoute.hash, undefined);
    assert.equal(projectedRoute.canonical().href, "/users/user%201?tab=activity");
    assert.deepEqual(projectedRoute.controllerNames(), ["detail"]);
    assert.deepEqual(projectedRoute.graphNames(), ["detailGraph"]);

    assert.deepEqual(
      projectedCandidate.layouts().map((layout) => ({
        routeId: layout.routeId,
        outletId: layout.outletId,
      })),
      [
        { routeId: "app", outletId: "shell" },
        { routeId: "app.users", outletId: "detail" },
      ],
    );
    assert.equal(projectedCandidate.layouts()[0].capability().href, "/");
    assert.equal(projectedCandidate.layouts()[1].descriptor().routeId, "app.users");
    assert.deepEqual(projectedCandidate.outlet().descriptor(), {
      outletId: "detail",
      parentLayoutRouteId: "app.users",
      occupantRouteId: "app.users.detail",
      occupantKind: "projectedRouteCapability",
    });
    assert.deepEqual(projectedRoute.controller("detail").outputNames(), ["routeLabel"]);
    assert.equal(projectedRoute.graph("detailGraph").summary().id, "routeDetailGraph");
    assert.deepEqual(projectedRoute.graph("detailGraph").outputNames(), ["routeLabel"]);
    assert.deepEqual(
      projectedCandidate.outlets().map((contract) => contract.descriptor()),
      [
        {
          outletId: "shell",
          parentLayoutRouteId: "app",
          occupantRouteId: "app.users",
          occupantKind: "projectedLayoutPlacement",
        },
        {
          outletId: "detail",
          parentLayoutRouteId: "app.users",
          occupantRouteId: "app.users.detail",
          occupantKind: "projectedRouteCapability",
        },
      ],
    );
    assert.equal(projectedCandidate.layouts()[0].outlet().occupant().kind, "projectedLayoutPlacement");
    assert.equal(projectedCandidate.layouts()[1].outlet().occupant().kind, "projectedRouteCapability");
    assert.equal(projectedCandidate.layouts()[1].outlet().occupant().routeId, "app.users.detail");
    assert.equal(
      projectedCandidate.outlet().verification().outletContractDigest,
      'forge-router:projected-outlet-contract:{"outletId":"detail","parentLayoutRouteId":"app.users","occupantRouteId":"app.users.detail","occupantKind":"projectedRouteCapability","occupantDigest":"forge-router:projected-outlet-route-occupant:{\\"routeId\\":\\"app.users.detail\\",\\"verification\\":\\"forge-router:url:\\\\\\"/users/user%201?tab=activity\\\\\\"\\"}"}',
    );
    assert.equal(
      projectedCandidate.verification().routeCompositionDigest,
      'forge-router:projected-route-composition:{"routeId":"app.users.detail","controllerNames":["detail"],"graphNames":["detailGraph"],"graphIds":["routeDetailGraph"],"resourceNames":[],"resourcePrefetchPostures":[]}',
    );
    assert.equal(
      projectedCandidate.verification().outletStackDigest,
      'forge-router:projected-outlet-stack:[{"outletId":"shell","parentLayoutRouteId":"app","occupantRouteId":"app.users","occupantKind":"projectedLayoutPlacement"},{"outletId":"detail","parentLayoutRouteId":"app.users","occupantRouteId":"app.users.detail","occupantKind":"projectedRouteCapability"}]',
    );
  });
});

test("phase-2 projected candidates stay distinct from admitted outcomes and local invalid matches", async () => {
  await withPhaseTwoRouterFixture(async ({ signals, routes, flatRoutes }) => {
    const fromRawAuthority = routes.project(
      signals.router.raw("/users/user%201?tab=activity", { navigationType: "manual" }),
    );
    const fromCanonicalAuthority = routes.project(
      signals.router.canonical("/users/user%201?tab=activity"),
    );
    assert.ok(fromRawAuthority);
    assert.ok(fromCanonicalAuthority);
    assert.equal(
      fromRawAuthority.verification().projectedCandidateDigest,
      fromCanonicalAuthority.verification().projectedCandidateDigest,
    );
    assert.equal(
      fromRawAuthority.verification().routeCompositionDigest,
      fromCanonicalAuthority.verification().routeCompositionDigest,
    );
    assert.equal(
      fromRawAuthority.verification().outletStackDigest,
      fromCanonicalAuthority.verification().outletStackDigest,
    );
    assert.equal("plan" in fromRawAuthority.route(), false);
    assert.equal("intent" in fromRawAuthority.route(), false);
    assert.equal("route" in fromRawAuthority.route(), false);
    assert.equal("formsAuthority" in fromRawAuthority.route(), false);
    assert.equal("outputs" in fromRawAuthority.route().controller("detail"), false);
    assert.equal("output" in fromRawAuthority.route().graph("detailGraph"), false);
    assert.throws(
      () => fromRawAuthority.route().controller("missing"),
      /does not expose controller "missing"/,
    );
    assert.throws(
      () => fromRawAuthority.route().graph("missing"),
      /does not expose graph "missing"/,
    );

    assert.equal(routes.project("/users/user%201?extra=1"), null);
    assert.equal(routes.project("/users/%2E%2E/user%201"), null);
    assert.equal(routes.project("https://evil.test/users/user%201"), null);
    const flatProjectedCandidate = flatRoutes.project("/plain");
    assert.ok(flatProjectedCandidate);
    assert.equal(flatProjectedCandidate.layouts().length, 0);
    assert.deepEqual(
      flatProjectedCandidate.outlets().map((contract) => contract.descriptor()),
      [
        {
          outletId: null,
          parentLayoutRouteId: null,
          occupantRouteId: "plain",
          occupantKind: "projectedRouteCapability",
        },
      ],
    );
    assert.equal(flatProjectedCandidate.outlet().occupant().kind, "projectedRouteCapability");
  });
});

test("phase-2 route declarations fail closed for invalid composition artifacts", async () => {
  await withPhaseTwoRouterFixture(async ({ signals }) => {
    assert.throws(
      () => signals.router.route("/broken", {
        controllers: {
          bad: {},
        },
      }),
      /controllers must be controller artifacts/,
    );
    assert.throws(
      () => signals.router.route("/broken", {
        graphs: {
          bad: {},
        },
      }),
      /graphs must be published graph artifacts/,
    );
  });
});

test("phase-2 route definition fails closed for ambiguous projection truth", async () => {
  await withPhaseTwoRouterFixture(async ({ signals }) => {
    assert.throws(
      () => signals.router.define({
        byId: signals.router.route("/users/:userId"),
        bySlug: signals.router.route("/users/:slug"),
      }),
      /projects ambiguous route truth/,
    );
    assert.throws(
      () => signals.router.define({
        stringTab: signals.router.route("/users/:userId", {
          search: {
            tab: signals.router.search.required.string(),
          },
        }),
        numericTab: signals.router.route("/users/:userId", {
          search: {
            tab: signals.router.search.required.number(),
          },
        }),
      }),
      /projects ambiguous route truth/,
    );
    assert.throws(
      () => signals.router.define({
        stringFlag: signals.router.route("/users/:userId", {
          search: {
            flag: signals.router.search.required.string(),
          },
        }),
        booleanFlag: signals.router.route("/users/:userId", {
          search: {
            flag: signals.router.search.required.boolean(),
          },
        }),
      }),
      /projects ambiguous route truth/,
    );
  });
});

test("phase-2 route projection admits disjoint same-path search contracts without ambiguity", async () => {
  await withPhaseTwoRouterFixture(async ({ signals }) => {
    const disjointRoutes = signals.router.define({
      tabDetail: signals.router.route("/users/:userId", {
        search: {
          tab: signals.router.search.required.string(),
        },
      }),
      viewDetail: signals.router.route("/users/:userId", {
        search: {
          view: signals.router.search.required.string(),
        },
      }),
    });

    assert.equal(disjointRoutes.project("/users/user-1") , null);
    assert.equal(disjointRoutes.project("/users/user-1?tab=activity")?.routeId, "tabDetail");
    assert.equal(disjointRoutes.project("/users/user-1?view=profile")?.routeId, "viewDetail");
  });
});

async function withPhaseTwoRouterFixture(run) {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const routeLabel = signals.output(() => "detail");
    const detailController = signals.controller({
      outputs: {
        routeLabel,
      },
    });
    const detailGraph = signals.graph("routeDetailGraph", {
      outputs: {
        routeLabel,
      },
    });
    const routes = signals.router.define({
      app: signals.router.layout("/", { outlet: "shell" }, {
        home: signals.router.route("/"),
        users: signals.router.layout("/users", { outlet: "detail" }, {
          index: signals.router.route("/users"),
          detail: signals.router.route("/users/:userId", {
            search: {
              tab: signals.router.search.optional.string(),
            },
            controllers: {
              detail: detailController,
            },
            graphs: {
              detailGraph,
            },
          }),
        }),
      }),
    });
    const flatRoutes = signals.router.define({
      plain: signals.router.route("/plain"),
    });
    try {
      await run({ signals, routes, flatRoutes });
    } finally {
      signals.free();
    }
  } finally {
    await cleanup();
  }
}

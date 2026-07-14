import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-1 raw and canonical url authority stay explicit and fail closed", async () => {
  await withPhaseOneRouterFixture(async ({ signals, routes, variantRoutes }) => {
    const rawLocation = signals.router.raw(
      "/users/user%201?page=2&tab=activity&active=false&page=2#panel%2Fmain",
      { navigationType: "manual" },
    );
    assert.equal(rawLocation.pathname, "/users/user%201");
    assert.equal(rawLocation.hashFragment, "panel%2Fmain");
    assert.deepEqual(rawLocation.searchParams, [
      { key: "page", value: "2" },
      { key: "tab", value: "activity" },
      { key: "active", value: "false" },
      { key: "page", value: "2" },
    ]);
    assert.equal(signals.router.isRawLocationAuthority(rawLocation), true);
    assert.equal(signals.router.isCanonicalUrlAuthority(rawLocation), false);

    const canonicalUrl = rawLocation.canonical();
    assert.equal(
      canonicalUrl.href,
      "/users/user%201?active=false&page=2&page=2&tab=activity#panel%2Fmain",
    );
    assert.equal(canonicalUrl.hashFragment, "panel/main");
    assert.deepEqual(canonicalUrl.searchParams, [
      { key: "active", value: "false" },
      { key: "page", value: "2" },
      { key: "page", value: "2" },
      { key: "tab", value: "activity" },
    ]);
    assert.equal(signals.router.isCanonicalUrlAuthority(canonicalUrl), true);
    assert.equal(
      canonicalUrl.verification().canonicalUrlDigest,
      signals.router
        .canonical("/users/user%201?active=false&page=2&page=2&tab=activity#panel%2Fmain")
        .verification()
        .canonicalUrlDigest,
    );
    assert.notEqual(
      rawLocation.verification().rawLocationDigest,
      signals.router
        .raw("/users/user%201?active=false&page=2&tab=activity#panel%2Fmain")
        .verification().rawLocationDigest,
    );

    assert.throws(
      () => signals.router.raw("/users/../users/user%201"),
      /without dot segments/,
    );
    assert.throws(
      () => signals.router.raw("https://evil.test/users/user%201"),
      /local origin/,
    );
    assert.throws(
      () => signals.router.raw("/users/user%201", { navigationType: "teleport" }),
      /navigationType must be one of/,
    );

    assert.equal(routes.users.detail.match(rawLocation), null);
    assert.equal(routes.users.detail.match(canonicalUrl), null);
    assert.equal(
      routes.users.detail.match(signals.router.raw(
        "/users/user%201?active=false&page=2&tab=activity#panel%2Fmain",
        { navigationType: "manual" },
      ))?.href,
      "/users/user%201?tab=activity&page=2&active=false#panel%2Fmain",
    );
    assert.equal(
      routes.users.detail.match(
        signals.router.canonical("/users/user%201?active=false&page=2&tab=activity#panel%2Fmain"),
      )?.href,
      "/users/user%201?tab=activity&page=2&active=false#panel%2Fmain",
    );
  });
});

test("phase-1 canonical route equivalence stays stable across declared forms", async () => {
  await withPhaseOneRouterFixture(async ({ signals, routes, variantRoutes, scopedRoutes }) => {
    const detailHref = routes.users.detail.href({
      params: { userId: "user 1" },
      search: { tab: "activity", page: 2, active: false },
      hash: "panel/main",
    });
    const canonicalDetail = routes.users.detail.canonical({
      params: { userId: "user 1" },
      search: { tab: "activity", page: 2, active: false },
      hash: "panel/main",
    });
    assert.equal(
      detailHref,
      "/users/user%201?tab=activity&page=2&active=false#panel%2Fmain",
    );
    assert.equal(canonicalDetail.href, detailHref);
    assert.equal(canonicalDetail.pathname, "/users/user%201");
    assert.equal(
      canonicalDetail.canonicalUrlDigest,
      'worth-router:url:"/users/user%201?tab=activity&page=2&active=false#panel%2Fmain"',
    );

    const detailReferenceVerification = routes.users.detail.verification();
    const detailCanonicalVerification = canonicalDetail.verification();
    assert.equal(
      detailCanonicalVerification.routeReferenceDigest,
      detailReferenceVerification.routeReferenceDigest,
    );
    assert.equal(
      detailCanonicalVerification.routeSchemaDigest,
      detailReferenceVerification.routeSchemaDigest,
    );
    assert.notEqual(
      detailReferenceVerification.routeSchemaDigest,
      variantRoutes.detailSchemaVariant.verification().routeSchemaDigest,
    );

    const detailLocation = routes.users.detail.to({
      params: { userId: "user 1" },
      search: { tab: "activity", page: 2, active: false },
      hash: "panel/main",
    });
    assert.equal(detailLocation.href, detailHref);
    assert.deepEqual(detailLocation.params, { userId: "user 1" });
    assert.deepEqual(detailLocation.search, {
      tab: "activity",
      page: 2,
      active: false,
    });
    assert.equal(detailLocation.hash, "panel/main");
    assert.equal(detailLocation.descriptor().routeId, "users.detail");
    assert.deepEqual(detailLocation.descriptor().declarationPath, ["users", "detail"]);
    assert.equal(detailLocation.canonical().equivalenceDigest, canonicalDetail.equivalenceDigest);
    assert.equal(
      detailLocation.canonical().verification().canonicalUrlDigest,
      canonicalDetail.verification().canonicalUrlDigest,
    );

    const matched = routes.users.detail.match(
      "/users/user%201?tab=activity&page=2&active=false#panel%2Fmain",
    );
    assert.ok(matched);
    assert.deepEqual(matched.params, { userId: "user 1" });
    assert.deepEqual(matched.search, {
      tab: "activity",
      page: 2,
      active: false,
    });
    assert.equal(matched.hash, "panel/main");
    assert.equal(matched.descriptor().routeId, "users.detail");
    assert.equal(routes.users.detail.match("/users/user%201?extra=1"), null);
    assert.equal(routes.users.detail.match("/users/user%201?page=oops"), null);
    assert.equal(routes.users.detail.match("/users/user%201?active=maybe"), null);
    assert.equal(routes.users.detail.match("/users/../users/user%201"), null);
    assert.equal(routes.users.detail.match("/users/%2E%2E/users/user%201"), null);
    assert.equal(routes.users.detail.match("/users/./user%201"), null);
    assert.equal(routes.users.detail.match("https://evil.test/users/user%201"), null);
    assert.equal(routes.users.detail.match("//evil.test/users/user%201"), null);

    const reorderedMatch = routes.users.detail.match(
      "/users/user%201?active=false&page=2&tab=activity#panel%2Fmain",
    );
    assert.ok(reorderedMatch);
    assert.equal(reorderedMatch.href, detailHref);
    assert.equal(
      reorderedMatch.canonical().equivalenceDigest,
      detailLocation.canonical().equivalenceDigest,
    );

    const plusEncodedMatch = routes.users.detail.match(
      "/users/user%201?page=2&tab=activity&active=false#panel%2Fmain",
    );
    assert.ok(plusEncodedMatch);
    assert.equal(
      plusEncodedMatch.canonical().canonicalUrlDigest,
      detailLocation.canonical().canonicalUrlDigest,
    );

    const scopedLocation = scopedRoutes.step.to({
      params: { stepId: 3 },
    });
    assert.equal(scopedLocation.href, "/wizard/3");
    assert.equal(scopedLocation.descriptor().routeId, "wizard:step");
    assert.equal(scopedLocation.descriptor().scopeId, "wizard");
    assert.equal(signals.router.isRouteLocation(scopedLocation), true);
    assert.equal(signals.router.isRouteLocation({ href: "/wizard/3" }), false);

    assert.notEqual(
      routes.users.detail.verification().routeReferenceDigest,
      scopedRoutes.step.verification().routeReferenceDigest,
    );
    assert.notEqual(
      routes.users.detail
        .canonical({
          params: { userId: "user 2" },
          search: { tab: "activity", page: 2, active: false },
          hash: "panel/main",
        })
        .verification().canonicalUrlDigest,
      detailLocation.canonical().verification().canonicalUrlDigest,
    );

    assert.throws(
      () => routes.users.detail.href({ params: { userId: "u1" }, search: { extra: "nope" } }),
      /undeclared search param/,
    );
  });
});

test("phase-1 typed search and hash normalization stay fail closed", async () => {
  await withPhaseOneRouterFixture(async ({ routes }) => {
    const detailLocation = routes.users.detail.to({
      params: { userId: "user 1" },
      search: { tab: "activity", page: 2, active: false },
      hash: "panel/main",
    });
    assert.deepEqual(detailLocation.search, {
      tab: "activity",
      page: 2,
      active: false,
    });
    assert.equal(detailLocation.hash, "panel/main");

    assert.equal(routes.users.detail.match("/users/user%201?extra=1"), null);
    assert.equal(routes.users.detail.match("/users/user%201?page=oops"), null);
    assert.equal(routes.users.detail.match("/users/user%201?active=maybe"), null);
    assert.throws(
      () => routes.users.detail.href({ params: { userId: "u1" }, search: { extra: "nope" } }),
      /undeclared search param/,
    );
    assert.throws(
      () => routes.users.detail.href({ params: {} }),
      /missing required path param "userId"/,
    );
  });
});

test("phase-1 shared route grammar and navigation artifacts expose typed evidence", async () => {
  await withPhaseOneRouterFixture(async ({ routes }) => {
    const detailLocation = routes.users.detail.to({
      params: { userId: "user 1" },
      search: { tab: "activity", page: 2, active: false },
      hash: "panel/main",
    });

    const replaceIntent = routes.users.detail.intent(
      {
        params: { userId: "user 1" },
        search: { tab: "activity", page: 2, active: false },
        hash: "panel/main",
      },
      { kind: "replace" },
    );
    assert.equal(replaceIntent.descriptor().kind, "replace");
    assert.equal(replaceIntent.descriptor().href, detailLocation.href);
    assert.equal(
      replaceIntent.descriptor().canonical().equivalenceDigest,
      detailLocation.canonical().equivalenceDigest,
    );
    assert.equal(
      replaceIntent.verification().navigationIntentDigest,
      routes.users.detail.intent(
        {
          params: { userId: "user 1" },
          search: { tab: "activity", page: 2, active: false },
          hash: "panel/main",
        },
        { kind: "replace" },
      ).verification().navigationIntentDigest,
    );

    const replacePlan = replaceIntent.policy({
      continuity: "preserve-visible-while-pending",
      projectionRefresh: "explicit",
      artifactPolicy: "diagnostics",
      deployment: "workerFirst",
    }).compile();
    assert.equal(replacePlan.kind, "replace");
    assert.equal(replacePlan.href, detailLocation.href);
    assert.equal(replacePlan.projectionPolicy().projectionRefresh, "explicit");
    assert.equal(
      replacePlan.projectionPolicy().continuity,
      "preserve-visible-while-pending",
    );
    assert.equal(replacePlan.canonical().href, detailLocation.href);
    assert.equal(replacePlan.explain().deployment, "workerFirst");
    assert.equal(replacePlan.explain().artifactPolicy, "diagnostics");
    assert.equal(
      replacePlan.explain().canonical.equivalenceDigest,
      detailLocation.canonical().equivalenceDigest,
    );
    assert.equal(
      replacePlan.verification().navigationPlanDigest,
      detailLocation
        .intent({
          kind: "replace",
          policy: {
            continuity: "preserve-visible-while-pending",
            projectionRefresh: "explicit",
            artifactPolicy: "diagnostics",
            deployment: "workerFirst",
          },
        })
        .compile()
        .verification().navigationPlanDigest,
    );
    assert.equal(replacePlan.cost().costClass, "deferred-visible-refresh");
    assert.equal(replacePlan.cost().looksExpensive, true);

    const directPlan = detailLocation.plan({
      continuity: "refresh-immediately",
      projectionRefresh: "immediate",
    });
    assert.equal(directPlan.kind, "push");
    assert.equal(directPlan.cost().costClass, "url-only-navigation");
    assert.equal(directPlan.explain().href, detailLocation.href);
    assert.equal(
      directPlan.canonical().canonicalUrlDigest,
      detailLocation.canonical().canonicalUrlDigest,
    );

    assert.equal(routes.users.detail.match("/users/%2E%2E/users/user%201"), null);
    assert.equal(routes.users.detail.match("/users/./user%201"), null);

    assert.throws(
      () => routes.users.detail.intent({ params: { userId: "u1" } }, { kind: "teleport" }),
      /intent kind must be one of/,
    );
    assert.throws(
      () => detailLocation.plan({ projectionRefresh: "later" }),
      /projectionRefresh policy must be one of/,
    );
  });
});

async function withPhaseOneRouterFixture(run) {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const routes = signals.router.define({
      home: signals.router.route("/"),
      users: {
        detail: signals.router.route("/users/:userId", {
          search: {
            tab: signals.router.search.optional.string(),
            page: signals.router.search.optional.number(),
            active: signals.router.search.optional.boolean(),
          },
          hash: signals.router.hash.string(),
        }),
      },
    });
    const variantRoutes = signals.router.define({
      detailSchemaVariant: signals.router.route("/users/:userId", {
        search: {
          tab: signals.router.search.required.string(),
          page: signals.router.search.optional.string(),
          active: signals.router.search.optional.boolean(),
        },
      }),
    });
    const scopedRoutes = signals.scope("wizard").router.define({
      step: signals.router.route("/wizard/:stepId"),
    });
    try {
      await run({ signals, routes, variantRoutes, scopedRoutes });
    } finally {
      signals.free();
    }
  } finally {
    await cleanup();
  }
}

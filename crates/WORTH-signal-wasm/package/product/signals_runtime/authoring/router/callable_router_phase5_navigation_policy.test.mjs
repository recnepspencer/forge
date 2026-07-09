import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-5 typed navigation policy exposes restore and breadcrumb intent semantics with explicit transition posture", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
    detail: signals.router.route("/users/:userId", {
      search: {
        tab: signals.router.search.optional.string(),
      },
    }),
  });

  try {
    const restorePlan = routes.detail.intent(
      {
        params: { userId: "u1" },
        search: { tab: "activity" },
      },
      {
        kind: "restoreBack",
        policy: {
          continuity: "preserve-visible-until-explicit-refresh",
          projectionRefresh: "explicit",
          artifactPolicy: "diagnostics",
          commit: "speculativeBranch",
          redirect: "surfaceRedirect",
          deployment: "workerFirst",
        },
      },
    ).compile();
    const breadcrumbPlan = routes.home.intent(undefined, {
      kind: "breadcrumbReturn",
      policy: {
        continuity: "preserve-visible-while-pending",
        projectionRefresh: "after-admission",
        commit: "directCommit",
        redirect: "followRedirect",
      },
    }).compile();

    assert.equal(restorePlan.kind, "restoreBack");
    assert.equal(restorePlan.policy().commit, "speculativeBranch");
    assert.equal(restorePlan.policy().redirect, "surfaceRedirect");
    assert.equal(
      restorePlan.freshness().visibleFreshness,
      "intentionally-stale",
    );
    assert.equal(
      restorePlan.freshness().admittedRouteTruth,
      "may-advance-before-visible-refresh",
    );
    assert.equal(restorePlan.projectionPolicy().visibleFreshness, "intentionally-stale");
    assert.equal(
      restorePlan.projectionPolicy().admittedRouteTruth,
      "may-advance-before-visible-refresh",
    );
    assert.equal(restorePlan.cost().costClass, "speculative-transition");
    assert.equal(restorePlan.explain().transitionPolicy.commit, "speculativeBranch");
    assert.equal(
      restorePlan.explain().freshness.visibleFreshness,
      "intentionally-stale",
    );
    assert.match(
      restorePlan.verification().navigationTransitionPolicyDigest,
      /navigation-transition-policy/,
    );
    assert.match(
      restorePlan.verification().navigationFreshnessPolicyDigest,
      /navigation-freshness-policy/,
    );
    assert.match(
      restorePlan.verification().navigationFreshnessDigest,
      /navigation-freshness/,
    );

    assert.equal(breadcrumbPlan.kind, "breadcrumbReturn");
    assert.equal(breadcrumbPlan.policy().commit, "directCommit");
    assert.equal(breadcrumbPlan.policy().redirect, "followRedirect");
    assert.equal(
      breadcrumbPlan.freshness().visibleFreshness,
      "continuity-preserved",
    );
    assert.equal(breadcrumbPlan.cost().costClass, "restore-navigation");
    assert.equal(
      breadcrumbPlan.explain().projectionPolicy.admittedRouteTruth,
      "may-advance-before-visible-refresh",
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 direct navigation intents stay on the cheap explicit plan lane", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId", {
      search: {
        tab: signals.router.search.optional.string(),
      },
    }),
  });

  try {
    for (const kind of ["push", "replace", "canonicalize"]) {
      const plan = routes.detail.intent(
        {
          params: { userId: "u1" },
          search: { tab: "activity" },
        },
        {
          kind,
          policy: {
            continuity:
              kind === "softRefresh" || kind === "sameRouteMutation"
                ? "preserve-visible-while-pending"
                : "refresh-immediately",
            projectionRefresh:
              kind === "softRefresh" || kind === "sameRouteMutation"
                ? "after-admission"
                : "immediate",
          },
        },
      ).compile();

      assert.equal(plan.kind, kind);
      assert.equal(plan.explain().kind, kind);
      assert.equal(plan.cost().intentKind, kind);
      assert.equal(plan.cost().costClass, "url-only-navigation");
      assert.match(plan.verification().navigationIntentDigest, /navigation-intent/);
      assert.match(plan.verification().navigationPlanDigest, /navigation-plan/);
    }
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 refresh and same-route mutation intents keep visible freshness explicit", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId", {
      search: {
        tab: signals.router.search.optional.string(),
      },
    }),
  });

  try {
    for (const kind of ["softRefresh", "sameRouteMutation"]) {
      const plan = routes.detail.intent(
        {
          params: { userId: "u1" },
          search: { tab: "activity" },
        },
        {
          kind,
          policy: {
            continuity: "preserve-visible-while-pending",
            projectionRefresh: "after-admission",
          },
        },
      ).compile();

      assert.equal(plan.kind, kind);
      assert.equal(plan.cost().costClass, "deferred-visible-refresh");
      assert.equal(plan.freshness().visibleFreshness, "continuity-preserved");
      assert.equal(
        plan.freshness().admittedRouteTruth,
        "may-advance-before-visible-refresh",
      );
    }
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 restore and breadcrumb intents stay distinct from direct and refresh navigation families", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    detail: signals.router.route("/users/:userId", {
      search: {
        tab: signals.router.search.optional.string(),
      },
    }),
  });

  try {
    for (const kind of ["breadcrumbReturn", "restoreBack"]) {
      const plan = routes.detail.intent(
        {
          params: { userId: "u1" },
          search: { tab: "activity" },
        },
        { kind },
      ).compile();

      assert.equal(plan.kind, kind);
      assert.equal(plan.cost().costClass, "restore-navigation");
      assert.equal(plan.cost().looksExpensive, true);
    }
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 freshness policy stays explicit across immediate, deferred, and explicit visible projection postures", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
  });

  try {
    const immediatePlan = routes.home.to().plan({
      continuity: "refresh-immediately",
      projectionRefresh: "immediate",
    });
    const deferredPlan = routes.home.to().plan({
      continuity: "preserve-visible-while-pending",
      projectionRefresh: "after-admission",
    });
    const explicitPlan = routes.home.to().plan({
      continuity: "preserve-visible-until-explicit-refresh",
      projectionRefresh: "explicit",
      artifactPolicy: "diagnostics",
    });

    assert.equal(immediatePlan.freshness().visibleFreshness, "freshly-refreshed");
    assert.equal(
      immediatePlan.freshness().admittedRouteTruth,
      "converges-with-visible-refresh",
    );
    assert.equal(immediatePlan.cost().costClass, "url-only-navigation");

    assert.equal(deferredPlan.freshness().visibleFreshness, "continuity-preserved");
    assert.equal(
      deferredPlan.freshness().admittedRouteTruth,
      "may-advance-before-visible-refresh",
    );
    assert.equal(deferredPlan.cost().costClass, "deferred-visible-refresh");

    assert.equal(explicitPlan.freshness().visibleFreshness, "intentionally-stale");
    assert.equal(
      explicitPlan.freshness().admittedRouteTruth,
      "may-advance-before-visible-refresh",
    );
    assert.equal(explicitPlan.cost().costClass, "explicit-visible-staleness");
    assert.equal(explicitPlan.cost().looksExpensive, true);
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 typed navigation policy fails closed for invalid commit, redirect, and freshness values", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
  });

  try {
    assert.throws(
      () => routes.home.to().plan({ commit: "branchish" }),
      /commit policy must be one of/,
    );
    assert.throws(
      () => routes.home.to().plan({ redirect: "guessRedirect" }),
      /redirect policy must be one of/,
    );
    assert.throws(
      () => routes.home.to().plan({ projectionRefresh: "later" }),
      /projectionRefresh policy must be one of/,
    );
    assert.throws(
      () => routes.home.intent(undefined, { kind: "teleportBack" }),
      /intent kind must be one of/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

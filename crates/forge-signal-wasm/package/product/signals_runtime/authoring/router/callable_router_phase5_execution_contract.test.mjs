import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-5 execution contract makes direct, canonicalize, refresh, mutation, and restore route-truth effects explicit", async () => {
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
    const pushPlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      { kind: "push" },
    ).compile();
    const canonicalizePlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      { kind: "canonicalize" },
    ).compile();
    const refreshPlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      {
        kind: "softRefresh",
        policy: {
          continuity: "preserve-visible-while-pending",
          projectionRefresh: "after-admission",
        },
      },
    ).compile();
    const mutationPlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      {
        kind: "sameRouteMutation",
        policy: {
          continuity: "preserve-visible-while-pending",
          projectionRefresh: "after-admission",
        },
      },
    ).compile();
    const restorePlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
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

    assert.equal(pushPlan.execution().routeTruthEffect, "advance-admitted-route-truth");
    assert.equal(
      canonicalizePlan.execution().routeTruthEffect,
      "canonicalize-admitted-route-truth",
    );
    assert.equal(refreshPlan.execution().routeTruthEffect, "re-admit-current-route-truth");
    assert.equal(
      mutationPlan.execution().routeTruthEffect,
      "re-admit-current-route-with-mutation",
    );
    assert.equal(restorePlan.execution().routeTruthEffect, "restore-admitted-route-truth");
    assert.match(
      restorePlan.verification().navigationExecutionContractDigest,
      /navigation-execution-contract/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 execution contract keeps visible projection and artifact posture explicit in one lower-level plan surface", async () => {
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
    const explicitPlan = routes.home.to().plan({
      continuity: "preserve-visible-until-explicit-refresh",
      projectionRefresh: "explicit",
      artifactPolicy: "diagnostics",
      commit: "speculativeBranch",
      redirect: "surfaceRedirect",
      deployment: "workerFirst",
    });

    assert.equal(
      immediatePlan.execution().visibleProjectionEffect,
      "refresh-visible-projection-immediately",
    );
    assert.equal(
      immediatePlan.execution().artifactEffect,
      "materialize-minimal-navigation-artifacts",
    );

    assert.equal(
      explicitPlan.execution().visibleProjectionEffect,
      "preserve-visible-projection-until-explicit-refresh",
    );
    assert.equal(
      explicitPlan.execution().artifactEffect,
      "materialize-diagnostic-navigation-artifacts",
    );
    assert.equal(explicitPlan.execution().commitBoundary, "speculativeBranch");
    assert.equal(explicitPlan.execution().redirectBoundary, "surfaceRedirect");
    assert.equal(explicitPlan.execution().deployment, "workerFirst");
    assert.deepEqual(
      explicitPlan.explain().executionContract,
      explicitPlan.execution(),
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

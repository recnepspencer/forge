import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("phase-5 low-level navigation plan policy exposes history effect and navigation family explicitly", async () => {
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
    const directPlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      { kind: "push" },
    ).compile();
    const canonicalizePlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      { kind: "canonicalize" },
    ).compile();
    const mutationPlan = routes.detail.intent(
      { params: { userId: "u1" }, search: { tab: "activity" } },
      { kind: "sameRouteMutation" },
    ).compile();

    assert.equal(directPlan.policy().navigationFamily, "direct-route");
    assert.equal(directPlan.policy().historyEffect, "pushstate");
    assert.equal(directPlan.policy().artifactPolicy, "minimal");

    assert.equal(canonicalizePlan.policy().navigationFamily, "canonicalization");
    assert.equal(canonicalizePlan.policy().historyEffect, "replacestate");

    assert.equal(mutationPlan.policy().navigationFamily, "same-route-mutation");
    assert.equal(mutationPlan.policy().historyEffect, "none");
    assert.match(
      mutationPlan.verification().navigationHistoryEffectDigest,
      /navigation-history-effect/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 freshness diagnostics attribute deferred and explicit stale visibility honestly", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
  });

  try {
    const deferredPlan = routes.home.to().plan({
      continuity: "preserve-visible-while-pending",
      projectionRefresh: "after-admission",
    });
    const explicitPlan = routes.home.to().plan({
      continuity: "preserve-visible-until-explicit-refresh",
      projectionRefresh: "explicit",
      artifactPolicy: "diagnostics",
    });

    assert.equal(
      deferredPlan.projectionPolicy().refreshAttribution,
      "refreshes-visible-projection-after-admission",
    );
    assert.equal(
      deferredPlan.projectionPolicy().continuityAttribution,
      "preserve-visible-while-pending",
    );
    assert.equal(
      deferredPlan.freshness().staleVisibilityReason,
      "waiting-for-admission-refresh",
    );

    assert.equal(
      explicitPlan.projectionPolicy().refreshAttribution,
      "requires-explicit-visible-refresh",
    );
    assert.equal(
      explicitPlan.projectionPolicy().continuityAttribution,
      "preserve-visible-until-explicit-refresh",
    );
    assert.equal(
      explicitPlan.freshness().staleVisibilityReason,
      "waiting-for-explicit-refresh",
    );
    assert.match(
      explicitPlan.verification().navigationContinuityAttributionDigest,
      /navigation-continuity-attribution/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

test("phase-5 freshness policy fails closed for contradictory continuity and projection refresh posture", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  const routes = signals.router.define({
    home: signals.router.route("/"),
  });

  try {
    assert.throws(
      () => routes.home.to().plan({
        continuity: "refresh-immediately",
        projectionRefresh: "after-admission",
      }),
      /refresh-immediately requires projectionRefresh immediate/,
    );
    assert.throws(
      () => routes.home.to().plan({
        continuity: "preserve-visible-while-pending",
        projectionRefresh: "explicit",
      }),
      /preserve-visible-while-pending requires projectionRefresh after-admission/,
    );
    assert.throws(
      () => routes.home.to().plan({
        continuity: "preserve-visible-until-explicit-refresh",
        projectionRefresh: "immediate",
      }),
      /preserve-visible-until-explicit-refresh requires projectionRefresh explicit/,
    );
  } finally {
    signals.free();
    await cleanup();
  }
});

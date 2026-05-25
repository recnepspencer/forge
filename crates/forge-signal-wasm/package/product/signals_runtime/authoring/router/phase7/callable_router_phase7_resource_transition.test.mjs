import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../../module_loading/load_signals_module.mjs";
import {
  flushTasks,
  withPhaseSevenRouterFixture,
} from "./callable_router_phase7_support.mjs";

test("phase-7 projected route resources prefetch through the native resource family without a second cache", async () => {
  await withPhaseSevenRouterFixture(async ({ routes }) => {
    const projectedCandidate = routes.project("/users/user-1");
    assert.ok(projectedCandidate);
    assert.deepEqual(projectedCandidate.route().resourceNames(), ["detail"]);

    const projectedResource = projectedCandidate.route().resource("detail");
    assert.equal(projectedResource.prefetchPosture(), "hover");

    const prefetch = projectedResource.prefetch();
    assert.equal(prefetch.name, "detail");
    assert.equal(prefetch.prefetchPosture, "hover");
    assert.equal(prefetch.line().descriptor().family.kind, "detail");
    assert.equal(prefetch.current().descriptor.canonicalParams.canonicalKey, "user-1");
    assert.match(prefetch.verification().routeResourcePrefetchDigest, /route-resource-prefetch/);

    const outcome = await routes.admit("/users/user-1");
    assert.equal(outcome.kind, "admitted");
    const admittedResource = outcome.route().resource("detail");
    assert.equal(admittedResource.prefetchPosture(), "hover");
    assert.equal(
      admittedResource.line().descriptor().runtimeLineId,
      prefetch.line().descriptor().runtimeLineId,
    );
    assert.equal(
      admittedResource.current().descriptor.canonicalParams.canonicalKey,
      "user-1",
    );
    assert.match(
      admittedResource.verification().routeResourceBindingDigest,
      /route-resource-binding/,
    );
    prefetch.free();
  });
});

test("phase-7 admitted route resources report native continuity through refresh-pending visible truth", async () => {
  await withPhaseSevenRouterFixture(async ({ routes, settleLoad }) => {
    const admittedOutcome = await routes.admit("/users/user-2");
    assert.equal(admittedOutcome.kind, "admitted");

    const admittedResource = admittedOutcome.route().resource("detail");
    const line = admittedResource.line();
    assert.equal(line.status().kind, "pending");
    assert.equal(line.status().continuity, "noVisibleValueYet");

    settleLoad("user-2", { id: "user-2", title: "User 2" });
    await flushTasks();

    assert.equal(line.status().kind, "fulfilled");
    const refreshStatus = line.refresh();
    assert.equal(refreshStatus.kind, "pending");
    assert.equal(refreshStatus.continuity, "preservedVisibleValue");

    const current = admittedResource.current();
    assert.equal(current.status.kind, "pending");
    assert.equal(current.freshness.kind, "stale");
    assert.equal(current.diagnosticsSummary.activity.continuity, "preserveVisibleValue");
    assert.equal(current.diagnosticsSummary.current.hasVisibleValue, true);
  });
});

test("phase-7 route resource declarations fail closed for invalid resource artifacts", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    assert.throws(
      () => signals.router.route("/users/:userId", {
        resources: {
          detail: {},
        },
      }),
      /must be declared with signals\.router\.resourceLine/,
    );
  } finally {
    await cleanup();
  }
});

test("phase-7 projected prefetch artifacts own resource lifecycle explicitly", async () => {
  await withPhaseSevenRouterFixture(async ({ routes }) => {
    const prefetch = routes.project("/users/user-3").route().resource("detail").prefetch();
    const runtimeLineId = prefetch.line().descriptor().runtimeLineId;
    prefetch.free();

    const nextPrefetch = routes.project("/users/user-3").route().resource("detail").prefetch();
    assert.notEqual(nextPrefetch.line().descriptor().runtimeLineId, runtimeLineId);
    nextPrefetch.free();
  });
});

test("phase-7 route warmup matches mixed declared trigger posture honestly", async () => {
  await withPhaseSevenRouterFixture(async ({ routes }) => {
    const hoverWarmup = routes.project("/warm/user-4").warmup("hover");
    assert.deepEqual(hoverWarmup.declaredResourceNames(), [
      "hoverCard",
      "focusPanel",
      "viewportStats",
    ]);
    assert.deepEqual(hoverWarmup.resourceNames(), ["hoverCard"]);
    assert.deepEqual(hoverWarmup.skippedResourceNames(), ["focusPanel", "viewportStats"]);

    const focusWarmup = routes.warmup("/warm/user-4", "focus");
    assert.ok(focusWarmup);
    assert.deepEqual(focusWarmup.resourceNames(), ["focusPanel"]);

    const intentWarmup = routes.project("/warm/user-4").warmup("intent");
    assert.deepEqual(intentWarmup.resourceNames(), [
      "hoverCard",
      "focusPanel",
      "viewportStats",
    ]);
    assert.deepEqual(intentWarmup.skippedResourceNames(), []);
    assert.match(intentWarmup.verification().routePrefetchDigest, /route-prefetch-admission/);

    hoverWarmup.free();
    focusWarmup.free();
    intentWarmup.free();
  });
});

test("phase-7 route warmup fails closed when no declared resource matches the trigger posture", async () => {
  await withPhaseSevenRouterFixture(async ({ routes }) => {
    assert.throws(
      () => routes.project("/viewport-only/user-5").warmup("hover"),
      /does not declare any resources for trigger "hover"/,
    );
  });
});

import assert from "node:assert/strict";
import test from "node:test";

import {
  flushTasks,
  withPhaseSevenRouterFixture,
} from "./callable_router_phase7_support.mjs";

test("phase-7 route transitions report direct, speculative, redirect, and prefetch admission sources honestly", async () => {
  await withPhaseSevenTransitionFixture(async ({ routes, settleLoad }) => {
    const home = await routes.admit("/");
    assert.equal(home.kind, "admitted");

    const directTransition = await routes.transition(home, "/about");
    assert.equal(directTransition.diagnostics().visibleChangeSource, "directNavigation");
    assert.equal(directTransition.diagnostics().visiblePolicy, "switch-to-target-route");

    const speculativeTransition = await routes.transition(home, "/about", {
      source: "speculativeCommit",
    });
    assert.equal(speculativeTransition.diagnostics().visibleChangeSource, "speculativeCommit");

    const redirectTransition = await routes.transition(home, "/private", {
      facts: { auth: "anonymous" },
    });
    assert.equal(redirectTransition.target().kind, "admitted");
    assert.equal(redirectTransition.target().routeId, "login");
    assert.equal(redirectTransition.diagnostics().visibleChangeSource, "redirect");

    const prefetched = routes.warmup("/users/user-1", "hover");
    assert.ok(prefetched);
    settleLoad("user-1", { id: "user-1", title: "User 1" });
    await flushTasks();
    const prefetchTransition = await routes.transition(home, prefetched);
    assert.equal(prefetchTransition.target().kind, "admitted");
    assert.equal(prefetchTransition.diagnostics().visibleChangeSource, "prefetchAdmission");
    prefetched.free();
  });
});

test("phase-7 route transitions preserve target visible truth through native resource continuity when pending", async () => {
  await withPhaseSevenTransitionFixture(async ({ routes, settleLoad }) => {
    const home = await routes.admit("/");
    assert.equal(home.kind, "admitted");

    const prefetched = routes.project("/users/user-2").prefetch("hover");
    settleLoad("user-2", { id: "user-2", title: "User 2" });
    await flushTasks();

    const prefetchedLine = prefetched.resource("detail").line();
    const refreshStatus = prefetchedLine.refresh();
    assert.equal(refreshStatus.kind, "pending");
    assert.equal(refreshStatus.continuity, "preservedVisibleValue");

    const transition = await routes.transition(home, prefetched, {
      continuity: "preserve-visible-while-pending",
    });
    assert.equal(
      transition.diagnostics().visibleChangeSource,
      "resourceContinuityPreservation",
    );
    assert.equal(
      transition.diagnostics().visiblePolicy,
      "show-target-resource-continuity-while-pending",
    );
    assert.deepEqual(transition.diagnostics().pendingResourceNames, ["detail"]);
    assert.match(transition.verification().routeTransitionDigest, /route-transition/);
    prefetched.free();
  });
});

test("phase-7 route prefetch fails closed when trigger posture does not match declaration", async () => {
  await withPhaseSevenTransitionFixture(async ({ routes }) => {
    const candidate = routes.project("/users/user-3");
    assert.throws(
      () => candidate.route().resource("detail").prefetch("intent"),
      /requires prefetch trigger "hover"/,
    );
  });
});

test("phase-7 warmup-backed transitions do not materialize skipped route resources just to explain visibility", async () => {
  await withPhaseSevenTransitionFixture(async ({ routes, hasPendingLoad, settleLoad }) => {
    const home = await routes.admit("/");
    assert.equal(home.kind, "admitted");

    const warmup = routes.warmup("/warm/user-4", "hover");
    assert.ok(warmup);
    assert.equal(hasPendingLoad("hoverCard", "user-4"), true);
    assert.equal(hasPendingLoad("focusPanel", "user-4"), false);
    assert.equal(hasPendingLoad("viewportStats", "user-4"), false);

    settleLoad("hoverCard", "user-4", { id: "user-4", title: "Hover 4" });
    await flushTasks();

    const transition = await routes.transition(home, warmup);
    assert.equal(transition.target().kind, "admitted");
    assert.deepEqual(transition.diagnostics().pendingResourceNames, []);
    assert.equal(hasPendingLoad("focusPanel", "user-4"), false);
    assert.equal(hasPendingLoad("viewportStats", "user-4"), false);
    warmup.free();
  });
});

const withPhaseSevenTransitionFixture = withPhaseSevenRouterFixture;

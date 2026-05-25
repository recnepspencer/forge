import assert from "node:assert/strict";
import test from "node:test";

import {
  withPhaseSevenRouterFixture,
} from "./callable_router_phase7_support.mjs";

test("phase-7 warmup ingress converges host hover events into typed route warmup artifacts", async () => {
  await withPhaseSevenRouterFixture(async ({ signals, routes }) => {
    const ingress = signals.router.warmup.hover("/warm/user-8", {
      sourceId: "sidebar-link",
      sourceValue: { lane: "hover" },
      routeIdentity: "warmRoute",
    });
    const report = routes.applyWarmupIngress(ingress);
    const artifact = report.artifact();

    assert.ok(artifact);
    assert.equal(report.envelopeFamily, "routeWarmupIngress");
    assert.equal(report.trigger, "hover");
    assert.equal(report.routeIdentity, "warmRoute");
    assert.equal(report.diagnostics().boundaryArtifact, "routeWarmupStarted");
    assert.deepEqual(report.diagnostics().warmedResourceNames, ["hoverCard"]);
    assert.deepEqual(report.diagnostics().skippedResourceNames, ["focusPanel", "viewportStats"]);
    assert.deepEqual(artifact.resourceNames(), ["hoverCard"]);
    assert.match(report.verification().routeWarmupReportDigest, /route-warmup-report/);
    artifact.free();
  });
});

test("phase-7 warmup ingress reports no matching resources without throwing host-event noise", async () => {
  await withPhaseSevenRouterFixture(async ({ signals, routes }) => {
    const ingress = signals.router.warmup.hover("/viewport-only/user-9");
    const report = routes.applyWarmupIngress(ingress);

    assert.equal(report.artifact(), null);
    assert.equal(report.diagnostics().boundaryArtifact, "noMatchingWarmupResources");
    assert.deepEqual(report.diagnostics().warmedResourceNames, []);
    assert.deepEqual(report.diagnostics().skippedResourceNames, []);
  });
});

test("phase-7 warmup ingress reports unroutable host targets explicitly", async () => {
  await withPhaseSevenRouterFixture(async ({ signals, routes }) => {
    const ingress = signals.router.warmup.focus("/missing/user-10");
    const report = routes.applyWarmupIngress(ingress);

    assert.equal(report.artifact(), null);
    assert.equal(report.diagnostics().boundaryArtifact, "noProjectedCandidate");
    assert.equal(report.rawLocationHref, "/missing/user-10");
    assert.deepEqual(report.diagnostics().warmedResourceNames, []);
  });
});

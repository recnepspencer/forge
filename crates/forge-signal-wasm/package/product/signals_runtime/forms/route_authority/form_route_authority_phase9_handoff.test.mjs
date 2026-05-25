import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  createAdmittedAuthorityArtifact,
  createRouteCoupledForm,
} from "./form_route_authority_support.mjs";

test("signals.form exposes explicit route authority handoff posture to route-coupled forms", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    for (const continuity of ["preserve", "freeze", "discard", "defer"]) {
      const form = createRouteCoupledForm(signals);
      const authority = await createAdmittedAuthorityArtifact(signals, `${continuity}-surface`, continuity);
      form.reportRouteAuthority(authority);

      const handoff = form.routeAuthority().summary.handoff;
      assert.notEqual(handoff, null);
      assert.equal(handoff.surfaceId, `${continuity}-surface`);
      assert.equal(handoff.posture, continuity);
      assert.equal(
        handoff.routeCoupledBehavior,
        continuity === "defer" ? "deferred" : "admitted",
      );
    }

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form resolves route-coupled steps from deferred to admitted through later route authority", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "defer"));
    assert.equal(form.routeAuthority().summary.handoff?.posture, "defer");
    assert.equal(form.routeAuthority().summary.handoff?.routeCoupledBehavior, "deferred");
    assert.equal(form.steps().artifacts[0].posture, "unavailable");
    assert.equal(form.actionPlan("reviewRoute").status, "denied");

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "preserve"));
    assert.equal(form.routeAuthority().summary.handoff?.posture, "preserve");
    assert.equal(form.routeAuthority().summary.handoff?.routeCoupledBehavior, "admitted");
    assert.equal(form.routeAuthority().summary.transitionKind, "authorityChanged");
    assert.equal(form.steps().artifacts[0].posture, "active");
    assert.equal(form.actionPlan("reviewRoute").status, "accepted");

    form.clearRouteAuthority({ reason: "route settled elsewhere" });
    assert.equal(form.routeAuthority().summary.handoff?.posture, "cleared");
    assert.equal(form.routeAuthority().summary.handoff?.routeCoupledBehavior, "cleared");
    assert.equal(form.steps().artifacts[0].readiness.blockers[0].reason, "route settled elsewhere");

    signals.free();
  } finally {
    await cleanup();
  }
});

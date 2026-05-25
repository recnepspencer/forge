import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  createAdmittedAuthorityArtifact,
  createRouteCoupledForm,
} from "./form_route_authority_support.mjs";

test("signals.form verification exposes freeze discard and clear route-authority continuity through the public audit surface", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "freeze"));
    let verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "freeze");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledBehavior, "admitted");
    assert.equal(verification.routeAuthorityContinuity.draftResolution, "preservedFrozenValue");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.denied, 1);
    assert.equal(verification.routeAuthorityContinuity.blockingReason, "router admitted route authority");

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "discard"));
    verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "discard");
    assert.equal(verification.routeAuthorityContinuity.draftResolution, "replacedFromSource");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.denied, 0);
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.accepted, 1);

    form.clearRouteAuthority({ reason: "route settled elsewhere" });
    verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "cleared");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledBehavior, "cleared");
    assert.equal(verification.routeAuthorityContinuity.draftResolution, "authorityCleared");
    assert.equal(verification.routeAuthorityContinuity.blockingReason, "route settled elsewhere");

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form verification package carries route authority continuity audit proof", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "defer"));
    let verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "defer");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledSteps.unavailable, 1);
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.denied, 1);
    assert.equal(
      verification.digests.routeAuthorityContinuityDigest,
      verification.routeAuthorityContinuity.digest,
    );

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "preserve"));
    verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "preserve");
    assert.equal(verification.routeAuthorityContinuity.routeCoupledSteps.active, 1);
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.accepted, 1);

    form.clearRouteAuthority({ reason: "route settled elsewhere" });
    verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.handoffPosture, "cleared");
    assert.equal(verification.routeAuthorityContinuity.blockingReason, "route settled elsewhere");

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form route authority continuity audit prefers route-authority blockers over unrelated denial blockers", async () => {
  const loaded = await loadSignalsModule({ rawSurface: "real" });
  const {
    cleanup,
    createSignals,
    hostCapabilityPlan,
    onlineCapability,
  } = loaded;
  try {
    const onlineState = { online: false };
    const signals = await createSignals({
      deployment: "mainThreadCompatibility",
      hostCapabilities: hostCapabilityPlan({
        online: onlineCapability({
          source: {
            current() {
              return onlineState.online;
            },
            subscribe() {
              return () => {};
            },
          },
        }),
      }),
    });
    const form = createRouteCoupledForm(signals, {
      host: {
        online: signals.host.online,
      },
      routeAction: {
        hostRequirements: ["online"],
      },
    });

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "defer"));
    const verification = form.verification();
    assert.equal(verification.routeAuthorityContinuity.routeCoupledActions.denied, 1);
    assert.equal(
      verification.routeAuthorityContinuity.blockingReason,
      "route authority deferred route-coupled form behavior until later admitted truth is present",
    );

    signals.free();
  } finally {
    await cleanup();
  }
});

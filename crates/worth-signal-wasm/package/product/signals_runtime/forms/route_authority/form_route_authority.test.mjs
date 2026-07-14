import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  createAdmittedAuthorityArtifact,
  createRouteCoupledForm,
} from "./form_route_authority_support.mjs";

test("signals.form admits route-coupled steps and step actions through router route authority", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals, {
      routeAction: {
        includeSubmit: true,
      },
    });
    const routes = signals.router.define({
      review: signals.router.route("/review", {
        forms: signals.router.forms("review-surface", {
          continuity: "preserve",
        }),
      }),
    });

    assert.equal(form.steps().artifacts[0].posture, "unavailable");
    assert.equal(form.actionPlan("reviewRoute").status, "denied");

    const outcome = await routes.admit("/review");
    assert.equal(outcome.kind, "admitted");
    form.bindRouteAuthority(outcome.route());

    assert.equal(form.routeAuthority().summary.authorityAvailable, true);
    assert.equal(form.routeAuthority().summary.surfaceId, "review-surface");
    assert.equal(form.routeAuthority().summary.continuityApplied, "preservedDraft");
    assert.equal(form.steps().artifacts[0].posture, "active");
    assert.equal(form.actionPlan("reviewRoute").status, "accepted");
    assert.equal(form.actionPlan("reviewRoute").effectPolicy, "deferred");

    const execution = form.executeAction("reviewRoute");
    assert.equal(execution.resultKind, "pending");
    assert.equal(form.handoff().summary.scopeKind, "route");
    assert.equal(form.handoff().summary.surfaceId, "review-surface");
    assert.equal(typeof form.verification().digests.routeAuthorityDigest, "string");

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form bindRouteAuthority rejects admitted routes without a forms surface", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);
    const routes = signals.router.define({
      plain: signals.router.route("/plain"),
    });

    const outcome = await routes.admit("/plain");
    assert.equal(outcome.kind, "admitted");
    assert.throws(
      () => form.bindRouteAuthority(outcome.route()),
      /does not declare a forms authority surface to bind/,
    );

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form keeps route-coupled behavior deferred when router authority defers continuity", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);
    const routes = signals.router.define({
      review: signals.router.route("/review", {
        forms: signals.router.forms("review-surface", {
          continuity: "defer",
        }),
      }),
    });

    const outcome = await routes.admit("/review");
    assert.equal(outcome.kind, "admitted");
    form.reportRouteAuthority(outcome.route().formsAuthority());

    assert.equal(form.routeAuthority().summary.authorityAvailable, false);
    assert.equal(form.routeAuthority().summary.continuity, "defer");
    assert.equal(form.routeAuthority().summary.continuityApplied, "deferredDraft");
    assert.equal(form.steps().artifacts[0].posture, "unavailable");
    assert.equal(
      form.steps().artifacts[0].readiness.blockers[0].reason,
      "route authority deferred route-coupled form behavior until later admitted truth is present",
    );
    assert.equal(form.actionPlan("reviewRoute").status, "denied");
    assert.equal(
      form.actionPlan("reviewRoute").readiness.blockers[0].reason,
      "route authority deferred route-coupled form behavior until later admitted truth is present",
    );

    form.clearRouteAuthority({ reason: "route settled elsewhere" });
    assert.equal(form.routeAuthority().summary.authorityAvailable, false);

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form applies preserve discard and freeze continuity semantics to draft truth", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const preserveForm = createRouteCoupledForm(signals, {
      routeAction: {
        includeSubmit: true,
      },
    });
    preserveForm.fields.title.set("Preserved draft");
    preserveForm.reportRouteAuthority(
      await createAdmittedAuthorityArtifact(signals, "preserve-surface", "preserve"),
    );
    assert.equal(preserveForm.effective().title, "Preserved draft");
    assert.equal(preserveForm.routeAuthority().summary.continuityApplied, "preservedDraft");
    assert.equal(preserveForm.routeAuthority().summary.transitionKind, "initialAuthority");

    const discardForm = createRouteCoupledForm(signals, {
      routeAction: {
        includeSubmit: true,
      },
    });
    discardForm.fields.title.set("Discarded draft");
    discardForm.reportRouteAuthority(
      await createAdmittedAuthorityArtifact(signals, "discard-surface", "discard"),
    );
    assert.equal(discardForm.effective().title, "Ship docs");
    assert.equal(discardForm.draft().title, undefined);
    assert.equal(discardForm.routeAuthority().summary.continuityApplied, "discardedDraft");
    assert.equal(discardForm.routeAuthority().summary.transitionKind, "initialAuthority");
    assert.notEqual(
      discardForm.routeAuthority().summary.previousDraftDigest,
      discardForm.routeAuthority().summary.nextDraftDigest,
    );
    discardForm.fields.title.set("Keep this after refresh");
    discardForm.reportRouteAuthority(
      await createAdmittedAuthorityArtifact(signals, "discard-surface", "discard"),
    );
    assert.equal(discardForm.effective().title, "Keep this after refresh");
    assert.equal(discardForm.routeAuthority().summary.continuityApplied, "maintainedAuthority");
    assert.equal(discardForm.routeAuthority().summary.transitionKind, "authorityRefreshed");
    discardForm.reportRouteAuthority(
      await createAdmittedAuthorityArtifact(signals, "discard-surface-next", "discard"),
    );
    assert.equal(discardForm.effective().title, "Ship docs");
    assert.equal(discardForm.routeAuthority().summary.continuityApplied, "discardedDraft");
    assert.equal(discardForm.routeAuthority().summary.transitionKind, "authorityChanged");
    assert.equal(discardForm.routeAuthority().counters.refreshedReports, 1);
    assert.equal(discardForm.routeAuthority().counters.changedReports, 1);

    const freezeForm = createRouteCoupledForm(signals, {
      routeAction: {
        includeSubmit: true,
      },
    });
    freezeForm.fields.title.set("Frozen draft");
    freezeForm.reportRouteAuthority(
      await createAdmittedAuthorityArtifact(signals, "freeze-surface", "freeze"),
    );
    assert.equal(freezeForm.routeAuthority().summary.continuityApplied, "frozeDraft");
    assert.throws(
      () => freezeForm.fields.title.set("Blocked by freeze"),
      /router admitted route authority/,
    );
    assert.equal(freezeForm.fieldWritePosture("title").blockers[0].kind, "routeAuthority:frozen");
    assert.equal(freezeForm.readiness().canSubmit, false);
    assert.equal(freezeForm.readiness().blockers[0].kind, "routeAuthority:frozen");
    assert.equal(freezeForm.actionPlan("submit").status, "denied");
    assert.equal(freezeForm.actionPlan("submit").readiness.blockers[0].kind, "routeAuthority:frozen");
    assert.equal(freezeForm.effective().title, "Frozen draft");
    freezeForm.clearRouteAuthority({ reason: "route moved elsewhere" });
    assert.equal(freezeForm.routeAuthority().summary.transitionKind, "authorityCleared");

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form rejects structurally forged route authority artifacts", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);

    assert.throws(
      () => form.reportRouteAuthority(createForgedAuthorityArtifact("forged-surface", "preserve")),
      /requires a route forms authority artifact from router admission/,
    );

    signals.free();
  } finally {
    await cleanup();
  }
});


function createForgedAuthorityArtifact(surfaceId, continuity) {
  return Object.freeze({
    kind: "routeFormsAuthority",
    routeId: "review",
    href: "/review",
    scopeKind: "route",
    surfaceId,
    continuity,
    reason: "forged route authority",
    verification() {
      return Object.freeze({
        formsAuthorityDigest: `forged-route-forms-authority:${surfaceId}:${continuity}`,
      });
    },
  });
}

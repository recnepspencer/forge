import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  createAdmittedAuthorityArtifact,
  createRouteCoupledForm,
} from "./form_route_authority_support.mjs";

test("signals.form exposes semantic draft continuity across authority changes", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });

    const preserveForm = createRouteCoupledForm(signals);
    preserveForm.fields.title.set("Preserved draft");
    preserveForm.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "preserve-a", "preserve"));
    assert.equal(preserveForm.routeAuthority().summary.draftContinuity?.draftResolution, "preservedValue");
    preserveForm.fields.title.set("Preserved change");
    preserveForm.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "preserve-b", "preserve"));
    assert.equal(preserveForm.routeAuthority().summary.draftContinuity?.draftResolution, "preservedValue");

    const discardForm = createRouteCoupledForm(signals);
    discardForm.fields.title.set("Discard me");
    discardForm.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "discard-a", "discard"));
    assert.equal(discardForm.routeAuthority().summary.draftContinuity?.draftResolution, "replacedFromSource");
    assert.equal(discardForm.effective().title, "Ship docs");

    const freezeForm = createRouteCoupledForm(signals);
    freezeForm.fields.title.set("Freeze me");
    freezeForm.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "freeze-a", "freeze"));
    assert.equal(freezeForm.routeAuthority().summary.draftContinuity?.draftResolution, "preservedFrozenValue");

    const deferForm = createRouteCoupledForm(signals);
    deferForm.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "defer-a", "defer"));
    assert.equal(deferForm.routeAuthority().summary.draftContinuity?.draftResolution, "awaitingAdmittedTruth");

    assert.equal(preserveForm.routeAuthority().counters.preservedDraftUpdates, 2);
    assert.equal(discardForm.routeAuthority().counters.discardedDraftUpdates, 1);
    assert.equal(freezeForm.routeAuthority().counters.frozenDraftUpdates, 1);
    assert.equal(deferForm.routeAuthority().counters.deferredDraftUpdates, 1);

    signals.free();
  } finally {
    await cleanup();
  }
});

test("signals.form reports deferred-to-admitted route-coupled transition through draft continuity artifacts", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);
    form.fields.title.set("Keep me");

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "defer"));
    assert.equal(form.routeAuthority().summary.draftContinuity?.draftResolution, "awaitingAdmittedTruth");
    assert.equal(form.steps().artifacts[0].posture, "unavailable");

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "preserve"));
    assert.equal(form.routeAuthority().summary.transitionKind, "authorityChanged");
    assert.equal(form.routeAuthority().summary.draftContinuity?.draftResolution, "preservedValue");
    assert.equal(form.routeAuthority().summary.draftContinuity?.authorityChange, "authorityChanged");
    assert.equal(form.steps().artifacts[0].posture, "active");
    assert.equal(form.effective().title, "Keep me");

    signals.free();
  } finally {
    await cleanup();
  }
});

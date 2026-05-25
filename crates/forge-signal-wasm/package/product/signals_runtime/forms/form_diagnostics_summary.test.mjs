import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";
import {
  createAdmittedAuthorityArtifact,
  createRouteCoupledForm,
} from "./route_authority/form_route_authority_support.mjs";

test("signals.form diagnostics summary agrees with full diagnostics current state", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", approved: false },
      fields: ({ field }) => ({
        title: field("title"),
        approved: field("approved"),
      }),
      actions: ({ action }) => ({
        approve: action("approve", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.approve",
        }),
      }),
    });

    form.fields.title.set("Ship docs now");

    const summary = form.diagnosticsSummary();
    const diagnostics = form.diagnostics();
    assert.equal(summary.digest, diagnostics.summary.digest);
    assert.equal(summary.fieldCount, diagnostics.fieldCount);
    assert.equal(summary.dirty.isDirty, diagnostics.dirty.isDirty);
    assert.equal(summary.patch.empty, diagnostics.patchPlan.empty);
    assert.equal(summary.patch.operationCount, diagnostics.patchPlan.operations.length);
    assert.equal(summary.readiness.canSubmit, diagnostics.readiness.canSubmit);
    assert.equal(summary.validation.summary.invalid, diagnostics.validation.summary.invalid);
    assert.equal(summary.actions.summary.total, diagnostics.actions.summary.total);
    assert.equal(summary.steps.summary.total, diagnostics.steps.summary.total);
    assert.equal(summary.routeAuthority.authorityAvailable, diagnostics.routeAuthority.summary.authorityAvailable);
    assert.equal(summary.routeAuthority.digest, diagnostics.routeAuthority.digest);
    assert.equal(
      summary.routeAuthority.continuityAudit.digest,
      diagnostics.summary.routeAuthority.continuityAudit.digest,
    );
    assert.equal(summary.sourceCompatibility.posture, diagnostics.sourceCompatibility.posture);
  } finally {
    await cleanup();
  }
});

test("signals.form retains diagnostics history only when diagnostics truth materially changes", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    const initialHistory = form.diagnosticsHistory();
    assert.equal(initialHistory.length, 1);
    assert.equal(initialHistory[0].summaryDigest, form.diagnosticsSummary().digest);
    assert.equal(form.diagnosticsHistory().length, 1);

    form.fields.title.set("Ship docs now");
    const afterDraftWrite = form.diagnosticsHistory();
    assert.equal(afterDraftWrite.length, 2);
    assert.notEqual(
      afterDraftWrite[1].diagnosticsStateDigest,
      afterDraftWrite[0].diagnosticsStateDigest,
    );

    form.diagnostics();
    assert.equal(form.diagnosticsHistory().length, 2);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package certifies diagnostics summary and retained diagnostics history", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", approved: false },
      fields: ({ field }) => ({
        title: field("title"),
        approved: field("approved"),
      }),
    });

    form.fields.title.set("Ship docs now");

    const summary = form.diagnosticsSummary();
    const diagnostics = form.diagnostics();
    const verification = form.verification();
    assert.equal(verification.digests.diagnosticsSummaryDigest, summary.digest);
    assert.equal(verification.digests.diagnosticsDigest, diagnostics.digest);
    assert.equal(verification.diagnosticsHistory.operations, form.diagnosticsHistory().length);
    assert.equal(
      verification.diagnosticsHistory.digest,
      verification.digests.diagnosticsHistoryDigest,
    );
    assert.equal(
      diagnostics.verification.digests.diagnosticsSummaryDigest,
      verification.digests.diagnosticsSummaryDigest,
    );
  } finally {
    await cleanup();
  }
});

test("signals.form diagnostics history digest does not depend on observation wall clock", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  const originalNow = Date.now;
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    Date.now = () => 1000;
    const firstForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    const firstDigest = firstForm.diagnosticsHistory()[0].diagnosticsDigest;

    Date.now = () => 9000;
    const secondForm = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });
    const secondArtifact = secondForm.diagnosticsHistory()[0];
    assert.equal(firstDigest, secondArtifact.diagnosticsDigest);
    assert.notEqual(firstForm.diagnosticsHistory()[0].observedAtMs, secondArtifact.observedAtMs);
  } finally {
    Date.now = originalNow;
    await cleanup();
  }
});

test("signals.form diagnostics summary and history expose route authority continuity transitions", async () => {
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  try {
    const signals = await createSignals({ deployment: "mainThreadCompatibility" });
    const form = createRouteCoupledForm(signals);

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "defer"));
    let summary = form.diagnosticsSummary();
    let history = form.diagnosticsHistory();
    assert.equal(summary.routeAuthority.handoff?.posture, "defer");
    assert.equal(summary.routeAuthority.handoff?.routeCoupledBehavior, "deferred");
    assert.equal(summary.routeAuthority.draftContinuity?.draftResolution, "awaitingAdmittedTruth");
    assert.equal(summary.routeAuthority.continuityAudit.routeCoupledSteps.unavailable, 1);
    assert.equal(summary.routeAuthority.continuityAudit.routeCoupledActions.denied, 1);
    assert.equal(history.at(-1)?.routeAuthorityTransitionKind, "initialAuthority");
    assert.equal(history.at(-1)?.routeAuthorityHandoffPosture, "defer");
    assert.equal(history.at(-1)?.routeAuthorityDraftResolution, "awaitingAdmittedTruth");
    assert.equal(history.at(-1)?.routeAuthorityContinuityAuditDigest, summary.routeAuthority.continuityAudit.digest);

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "freeze"));
    summary = form.diagnosticsSummary();
    history = form.diagnosticsHistory();
    assert.equal(summary.routeAuthority.handoff?.posture, "freeze");
    assert.equal(summary.routeAuthority.handoff?.routeCoupledBehavior, "admitted");
    assert.equal(summary.routeAuthority.draftContinuity?.draftResolution, "preservedFrozenValue");
    assert.equal(summary.routeAuthority.continuityAudit.routeCoupledActions.denied, 1);
    assert.equal(summary.routeAuthority.continuityAudit.blockingReason, "router admitted route authority");
    assert.equal(history.at(-1)?.routeAuthorityTransitionKind, "authorityChanged");
    assert.equal(history.at(-1)?.routeAuthorityHandoffPosture, "freeze");
    assert.equal(history.at(-1)?.routeAuthorityDraftResolution, "preservedFrozenValue");

    form.reportRouteAuthority(await createAdmittedAuthorityArtifact(signals, "review-surface", "discard"));
    summary = form.diagnosticsSummary();
    history = form.diagnosticsHistory();
    assert.equal(summary.routeAuthority.handoff?.posture, "discard");
    assert.equal(summary.routeAuthority.draftContinuity?.draftResolution, "replacedFromSource");
    assert.equal(summary.routeAuthority.continuityAudit.routeCoupledActions.denied, 0);
    assert.equal(history.at(-1)?.routeAuthorityTransitionKind, "authorityChanged");
    assert.equal(history.at(-1)?.routeAuthorityHandoffPosture, "discard");
    assert.equal(history.at(-1)?.routeAuthorityDraftResolution, "replacedFromSource");

    form.clearRouteAuthority({ reason: "route settled elsewhere" });
    summary = form.diagnosticsSummary();
    history = form.diagnosticsHistory();
    assert.equal(summary.routeAuthority.handoff?.posture, "cleared");
    assert.equal(summary.routeAuthority.handoff?.routeCoupledBehavior, "cleared");
    assert.equal(summary.routeAuthority.draftContinuity?.draftResolution, "authorityCleared");
    assert.equal(summary.routeAuthority.continuityAudit.blockingReason, "route settled elsewhere");
    assert.equal(history.at(-1)?.routeAuthorityTransitionKind, "authorityCleared");
    assert.equal(history.at(-1)?.routeAuthorityHandoffPosture, "cleared");
    assert.equal(history.at(-1)?.routeAuthorityDraftResolution, "authorityCleared");

    signals.free();
  } finally {
    await cleanup();
  }
});

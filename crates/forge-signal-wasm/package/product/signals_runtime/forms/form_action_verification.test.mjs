import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form action plan digests include effect and action schema proof", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const createFormWithEffect = (hostEffect) => signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        workflow: action("workflow", {
          patchPolicy: "allowEmpty",
          hostEffect,
          schema: { version: 1, effect: hostEffect },
        }),
      }),
    });

    const approvePlan = createFormWithEffect("workflow.approve").actionPlan("workflow");
    const rejectPlan = createFormWithEffect("workflow.reject").actionPlan("workflow");
    assert.notEqual(approvePlan.planDigest, rejectPlan.planDigest);
    assert.notEqual(approvePlan.proof.effectDigest, rejectPlan.proof.effectDigest);
    assert.notEqual(approvePlan.proof.actionSchemaDigest, rejectPlan.proof.actionSchemaDigest);
  } finally {
    await cleanup();
  }
});

test("signals.form verification package separates action planning from lifecycle history", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs", approved: false },
      fields: ({ field }) => ({
        title: field("title"),
        approved: field("approved"),
      }),
      admission: ({ action }) => ({
        approveAdmission: action("approve", "approval", ["approved"], () => ({
          posture: "requiresApproval",
          actorDigest: "actor:reviewer",
          policyDigest: "policy:approve",
        })),
      }),
      actions: ({ action }) => ({
        approve: action("approve", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.approve",
          idempotency: "deny",
        }),
      }),
    });

    const beforeAttempt = form.verification();
    assert.equal(beforeAttempt.kind, "formVerification");
    assert.equal(
      beforeAttempt.digests.actionCatalogDigest,
      form.actions().digests.catalogDigest,
    );
    assert.equal(
      beforeAttempt.digests.actionReadinessAdmissionDigest,
      form.actions().digests.readinessAdmissionDigest,
    );
    assert.equal(beforeAttempt.digests.submitPlanDigest, form.actionPlan("submit").planDigest);
    assert.equal(beforeAttempt.actionHistory.attempts, 0);
    assert.equal(beforeAttempt.performanceEnvelope.actions.plans, 2);

    const deniedAttempt = form.attemptAction("approve");
    const afterAttempt = form.verification();
    assert.equal(deniedAttempt.resultKind, "denied");
    assert.equal(afterAttempt.actionHistory.attempts, 1);
    assert.equal(
      afterAttempt.digests.actionCatalogDigest,
      beforeAttempt.digests.actionCatalogDigest,
    );
    assert.equal(
      afterAttempt.digests.actionReadinessAdmissionDigest,
      beforeAttempt.digests.actionReadinessAdmissionDigest,
    );
    assert.notEqual(
      afterAttempt.digests.actionLifecycleDigest,
      beforeAttempt.digests.actionLifecycleDigest,
    );
    assert.notEqual(afterAttempt.packageDigest, beforeAttempt.packageDigest);
    assert.equal(form.diagnostics().verification.packageDigest, afterAttempt.packageDigest);
  } finally {
    await cleanup();
  }
});

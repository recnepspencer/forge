import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form keeps semantic action fulfillment distinct from visible presentation settlement", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title", { row: "main" }),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          hostEffect: "workflow.save",
        }),
      }),
      presentation: {
        action: { delayedBusyRevealMs: 0, minimumBusyMs: 0 },
      },
    });

    form.fields.title.set("Ship docs now");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "pending");
    assert.equal(form.presentationLifecycle("action:submit").status, "busy");

    form.fulfillAction(execution.operationId, {
      canonicalValue: { title: "Ship docs now" },
    });
    const settlingLane = form.presentationLifecycle("action:submit");
    const canonicalizationLane = form.presentationLifecycle("canonicalization");
    assert.equal(settlingLane.status, "settling");
    assert.equal(canonicalizationLane.status, "settling");
    assert.equal(settlingLane.acknowledgement.required, true);
    assert.equal(canonicalizationLane.acknowledgement.required, true);
    assert.ok(form.presentation().summary.settling >= 2);
    assert.equal(form.presentation().acknowledgements.required >= 2, true);
    assert.equal(form.presentation().acknowledgements.pending >= 2, true);
    assert.equal(typeof form.presentation().acknowledgements.digest, "string");

    const acknowledgement = form.acknowledgePresentation("action:submit");
    assert.equal(acknowledgement.resultKind, "acknowledged");
    const canonicalizationAcknowledgement = form.acknowledgePresentation("canonicalization");
    assert.equal(canonicalizationAcknowledgement.resultKind, "acknowledged");
    const readyLane = form.presentationLifecycle("action:submit");
    const readyCanonicalizationLane = form.presentationLifecycle("canonicalization");
    assert.equal(readyLane.status, "ready");
    assert.equal(readyCanonicalizationLane.status, "ready");
    assert.equal(readyLane.acknowledgement.status, "acknowledged");
    assert.equal(readyCanonicalizationLane.acknowledgement.status, "acknowledged");
    assert.equal(form.presentation().acknowledgements.acknowledged >= 2, true);
    assert.equal(
      form.verification().digests.presentationSettlementAcknowledgementDigest,
      form.presentation().acknowledgements.digest,
    );

    form.fields.title.set("Ship docs final");
    const secondExecution = form.executeAction("submit");
    form.fulfillAction(secondExecution.operationId, {
      canonicalValue: { title: "Ship docs final" },
    });
    const timedOut = form.timeoutPresentation("action:submit", {
      reason: "save banner never settled",
    });
    assert.equal(timedOut.resultKind, "timedOut");
    assert.equal(form.presentationLifecycle("action:submit").status, "failed");
    assert.equal(form.presentation().acknowledgements.timedOut >= 1, true);
    assert.equal(form.presentationHistory().length, 3);
  } finally {
    await cleanup();
  }
});

test("signals.form layout presentation can lag semantic truth without rerunning semantic authority", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "" },
      fields: ({ field }) => ({
        title: field("title", { row: "main" }),
      }),
      validation: ({ field }) => ({
        titleRequired: field("title", (value) => (
          value.length > 0
            ? true
            : {
              kind: "invalid",
              message: {
                code: "task.title.required",
                message: "Title is required",
                target: "title",
                severity: "error",
                audience: "user",
                visibility: "visible",
              },
            }
        )),
      }),
    });

    assert.equal(form.presentationLifecycle("layout").status, "pending");
    form.recordLayoutMeasurement([
      {
        row: "main",
        labelHeight: 18,
        controlHeight: 32,
        messageHeight: 0,
      },
    ], {
      cause: "animationFrame",
      frameToken: "frame-1",
    });
    assert.equal(form.presentationLifecycle("layout").status, "ready");

    form.fields.title.set("Ship docs");
    assert.equal(form.presentationLifecycle("layout").status, "settling");

    form.recordLayoutMeasurement([
      {
        row: "main",
        labelHeight: 18,
        controlHeight: 32,
        messageHeight: 0,
      },
    ], {
      cause: "animationFrame",
      frameToken: "frame-2",
    });
    assert.equal(form.presentationLifecycle("layout").status, "ready");
  } finally {
    await cleanup();
  }
});

test("signals.form exposes route-coupled navigation presentation as typed unavailable posture", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      steps: ({ step }) => ({
        reviewRoute: step("reviewRoute", ["title"], {
          routeCoupled: true,
        }),
      }),
      actions: ({ step }) => ({
        gotoReview: step("gotoReview", "reviewRoute", "jump", {
          routeCoupled: true,
        }),
      }),
    });

    const stepLane = form.presentationLifecycle("navigation:step:reviewRoute");
    const actionLane = form.presentationLifecycle("navigation:action:gotoReview");
    assert.equal(stepLane.status, "unavailable");
    assert.equal(stepLane.scope, "step");
    assert.equal(actionLane.status, "unavailable");
    assert.equal(actionLane.scope, "route");
    assert.ok(form.presentation().summary.unavailable >= 2);
  } finally {
    await cleanup();
  }
});

test("signals.form only emits a dedicated acknowledgement digest when settlement acknowledgement participates", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ submit }) => ({
        submit: submit({
          hostEffect: "workflow.save",
        }),
      }),
      presentation: {
        action: {
          delayedBusyRevealMs: 0,
          minimumBusyMs: 0,
          settlementAcknowledgement: "none",
        },
        canonicalization: {
          settlementAcknowledgement: "none",
        },
      },
    });

    form.fields.title.set("Ship docs now");
    const execution = form.executeAction("submit");
    form.fulfillAction(execution.operationId, {
      canonicalValue: { title: "Ship docs now" },
    });

    const presentation = form.presentation();
    assert.equal(presentation.acknowledgements.required, 0);
    assert.equal(presentation.acknowledgements.digest, null);
    assert.equal(form.verification().digests.presentationSettlementAcknowledgementDigest, null);
    assert.equal(form.presentationLifecycle("action:submit").status, "ready");
  } finally {
    await cleanup();
  }
});

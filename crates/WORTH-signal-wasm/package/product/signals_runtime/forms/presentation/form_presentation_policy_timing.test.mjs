import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form action presentation honors delayed busy reveal and minimum busy duration", async () => {
  const originalNow = Date.now;
  let nowMs = 1_000;
  Date.now = () => nowMs;
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
          delayedBusyRevealMs: 50,
          minimumBusyMs: 120,
          settlementAcknowledgement: "required",
        },
      },
    });

    form.fields.title.set("Ship docs now");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "pending");
    assert.equal(form.presentationLifecycle("action:submit").status, "pending");

    nowMs = 1_060;
    assert.equal(form.presentationLifecycle("action:submit").status, "busy");

    nowMs = 1_070;
    form.fulfillAction(execution.operationId, {
      canonicalValue: { title: "Ship docs now" },
    });
    assert.equal(form.presentationLifecycle("action:submit").status, "busy");

    nowMs = 1_125;
    assert.equal(form.presentationLifecycle("action:submit").status, "settling");
    assert.equal(form.acknowledgePresentation("action:submit").resultKind, "acknowledged");
    assert.equal(form.presentationLifecycle("action:submit").status, "ready");
  } finally {
    Date.now = originalNow;
    await cleanup();
  }
});

test("signals.form presentation settlement timeout and handoff policy are explicit for external lanes", async () => {
  const originalNow = Date.now;
  let nowMs = 5_000;
  Date.now = () => nowMs;
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        handoff: {
          scope: "externalHandoff",
          settlementAcknowledgement: "required",
          settlementTimeoutMs: 40,
          supersessionHandoff: "handoff",
        },
      },
    });

    form.reportPresentationLane("handoff", {
      status: "settling",
      target: "share-modal",
      reason: "waiting for first handoff acknowledgement",
      token: "handoff-1",
      scopeKind: "modal",
      surfaceId: "share-modal",
    });
    nowMs = 5_010;
    form.reportPresentationLane("handoff", {
      status: "settling",
      target: "share-modal",
      reason: "waiting for replacement handoff acknowledgement",
      token: "handoff-2",
      scopeKind: "modal",
      surfaceId: "share-modal",
    });

    const handoffArtifact = form.presentationHistory().find((entry) => (
      entry.kind === "presentationLaneUpdate" &&
      entry.source === "handoff" &&
      entry.token === "handoff-1"
    ));
    assert.equal(handoffArtifact?.supersededByToken, "handoff-2");
    assert.equal(form.presentationLifecycle("handoff").status, "settling");

    nowMs = 5_060;
    const failedLane = form.presentationLifecycle("handoff");
    assert.equal(failedLane.status, "failed");
    assert.equal(failedLane.acknowledgement.status, "timedOut");
    assert.ok(form.presentationHistory().some((entry) => (
      entry.kind === "presentationSettlement" &&
      entry.resultKind === "timedOut" &&
      entry.token === "handoff-2"
    )));
  } finally {
    Date.now = originalNow;
    await cleanup();
  }
});

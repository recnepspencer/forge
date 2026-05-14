import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form exit visibility combines dirty exit guard truth with first-class route confirmation state", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
      presentation: {
        exit: { settlementAcknowledgement: "required" },
      },
    });

    form.fields.title.set("Ship docs now");
    const before = form.verification();
    assert.equal(form.exit().summary.guardKind, "dirty");
    assert.equal(form.exit().summary.requiresConfirmation, true);
    assert.equal(form.presentationLifecycle("exit").status, "busy");

    const artifact = form.reportExit({
      status: "settling",
      target: "leave-page",
      reason: "waiting for browser leave confirmation",
      token: "exit-1",
      scopeKind: "route",
      surfaceId: "browser-history",
      operation: "confirm",
    });
    assert.equal(artifact.scopeKind, "route");
    assert.equal(form.exit().summary.scopeKind, "route");
    assert.equal(form.exit().summary.surfaceId, "browser-history");
    assert.equal(form.presentationLifecycle("exit").status, "settling");
    assert.equal(form.verification().digests.semanticEqualityDigest, before.digests.semanticEqualityDigest);
    assert.notEqual(form.verification().digests.exitDigest, before.digests.exitDigest);
    assert.equal(form.diagnostics().exit.summary.guardKind, "dirty");

    const acknowledgement = form.acknowledgePresentation("exit");
    assert.equal(acknowledgement.resultKind, "acknowledged");
    assert.equal(form.presentationLifecycle("exit").status, "ready");

    const clear = form.clearExit({ reason: "exit guard dismissed" });
    assert.equal(clear.source, "clear");
    assert.equal(form.exit().summary.status, "busy");
    assert.equal(form.exit().summary.guardKind, "dirty");
    assert.equal(form.presentationLifecycle("exit").status, "busy");
    assert.ok(form.exit().history.some((entry) => (
      entry.source === "clear" &&
      entry.reason === "exit guard dismissed"
    )));
  } finally {
    await cleanup();
  }
});

test("signals.form exit visibility keeps pending action guard explicit and generic exit compatibility aligned", async () => {
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
    });

    form.fields.title.set("Ship docs now");
    const execution = form.executeAction("submit");
    assert.equal(execution.resultKind, "pending");
    assert.equal(form.exit().summary.guardKind, "pendingAction");
    assert.equal(form.exit().summary.pendingActions, 1);
    assert.equal(form.presentationLifecycle("exit").status, "busy");

    form.reportPresentationLane("exit", {
      status: "busy",
      target: "route-exit",
      reason: "generic route exit guard is active",
      token: "exit-2",
    });
    assert.equal(form.exit().summary.activeTarget, "route-exit");

    const genericClear = form.clearPresentationLane("exit", {
      reason: "generic exit lane cleared",
    });
    assert.equal(genericClear.lane, "exit");
    assert.equal(form.exit().summary.guardKind, "pendingAction");
    assert.equal(form.exit().summary.activeTarget, "pending-actions");
    assert.ok(form.exit().history.some((entry) => (
      entry.source === "clear" &&
      entry.reason === "generic exit lane cleared"
    )));
    assert.equal(form.verification().performanceEnvelope.exit.pendingActions, 1);
  } finally {
    await cleanup();
  }
});

test("signals.form exit visibility denies malformed scope and operation metadata", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const form = signals.form({
      source: { title: "Ship docs" },
      fields: ({ field }) => ({
        title: field("title"),
      }),
    });

    assert.throws(
      () =>
        form.reportExit({
          status: "busy",
          reason: "bad scope",
          scopeKind: "teleport",
        }),
      /scope kind is not supported/,
    );

    assert.throws(
      () =>
        form.reportExit({
          status: "busy",
          reason: "bad operation",
          operation: "warp",
        }),
      /operation is not supported/,
    );
  } finally {
    await cleanup();
  }
});

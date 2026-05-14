import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../../runtime_fixture/graph_operational_runtime.mjs";

test("signals.form resource drift presentation stays explicit across passive and conflicting authoritative drift", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    const source = signals.input({ title: "Ship docs" });
    const form = signals.form({
      source,
      fields: ({ field }) => ({
        title: field("title"),
      }),
      actions: ({ action }) => ({
        publish: action("publish", {
          patchPolicy: "allowEmpty",
          hostEffect: "workflow.publish",
        }),
      }),
    });

    assert.equal(form.presentationLifecycle("resourceDrift").status, "ready");

    const pending = form.executeAction("publish");
    form.fulfillAction(pending.operationId, {
      canonicalValue: { title: "Server title" },
    });
    const settledLane = form.presentationLifecycle("resourceDrift");
    assert.equal(settledLane.status, "ready");
    assert.equal(settledLane.target, "publish");

    source.set({ title: "Remote source drift" });
    const passiveDriftLane = form.presentationLifecycle("resourceDrift");
    assert.equal(passiveDriftLane.status, "busy");
    assert.match(passiveDriftLane.reason, /replaced the last canonicalized source projection/);

    form.fields.title.set("Local draft after drift");
    const conflictingDriftLane = form.presentationLifecycle("resourceDrift");
    assert.equal(conflictingDriftLane.status, "failed");
    assert.match(conflictingDriftLane.reason, /while local draft edits remain/);
    assert.equal(form.presentation().counters.resourceDriftLanes, 1);
  } finally {
    await cleanup();
  }
});

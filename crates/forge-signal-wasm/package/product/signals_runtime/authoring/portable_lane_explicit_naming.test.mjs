import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("The Portable Lane Explicit Naming Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial, options) {
        calls.push(["input", id, initial, options]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        calls.push(["computedSpec", id, spec]);
        return createRawReadableHandle(id, { spec });
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, callback()]);
        return createRawReadableHandle(id, id.length);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, { spec });
      },
      read(target) {
        return typeof target === "string" ? target : target.id;
      },
      watch() {
        throw new Error("watch not needed");
      },
      effect() {
        throw new Error("effect not needed");
      },
      transaction() {
        throw new Error("transaction not needed");
      },
      batch() {
        throw new Error("batch not needed");
      },
      nuke() {
        return true;
      },
      diagnostics() {
        throw new Error("diagnostics not needed");
      },
      history() {
        throw new Error("history not needed");
      },
      specialist() {
        throw new Error("specialist not needed");
      },
      adapters() {
        throw new Error("adapters not needed");
      },
      compatibilityApp() {
        throw new Error("compatibilityApp not needed");
      },
      compatibilityRuntime() {
        throw new Error("compatibilityRuntime not needed");
      },
      free() {},
    };

    const signals = wrapSignals(rawSignals);
    const doubledSpec = { expr: { kind: "value", value: 2 } };
    const labelSpec = { expr: { kind: "value", value: "label" } };

    const count = signals.spec.input("count", 1, { producesAspects: [1] });
    const doubled = signals.spec.computed("doubled", doubledSpec);
    const label = signals.spec.output("label", labelSpec);
    const callbackLabel = signals.spec.outputCallback(
      "callbackLabel",
      () => "callback-label",
    );
    const generated = signals.spec.computedCallback(
      "generated",
      () => count() + 1,
    );

    assert.equal(count.id, "count");
    assert.equal(doubled.id, "doubled");
    assert.equal(label.id, "label");
    assert.equal(callbackLabel.id, "callbackLabel");
    assert.equal(generated.id, "generated");
    assert.deepEqual(label(), { spec: labelSpec });
    assert.throws(
      () => signals.input("count", 1),
      /input app authoring does not accept an explicit id; use signals\.spec\.input/,
    );
    assert.throws(
      () => signals.computed("doubled", doubledSpec),
      /computed app authoring does not accept an explicit id; use signals\.spec\.computed/,
    );

    assert.deepEqual(calls[0], ["input", "count", 1, { producesAspects: [1] }]);
    assert.deepEqual(calls[1], ["computedSpec", "doubled", doubledSpec]);
    assert.deepEqual(calls[2], ["outputSpec", "label", labelSpec]);
    assert.equal(calls[3][0], "computedCallback");
    assert.equal(calls[3][1], "__forgeSignal.outputProjection.callbackLabel.1");
    assert.deepEqual(calls[4], [
      "outputSpec",
      "callbackLabel",
      {
        reads: ["__forgeSignal.outputProjection.callbackLabel.1"],
        expr: {
          kind: "read",
          id: "__forgeSignal.outputProjection.callbackLabel.1",
        },
      },
    ]);
    assert.equal(calls[5][0], "computedCallback");
    assert.equal(calls[5][1], "generated");
  } finally {
    await cleanup();
  }
});



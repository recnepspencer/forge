import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals keeps callback forms and rejects malformed metadata mixes", async () => {
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
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, typeof callback]);
        return createRawReadableHandle(id, id);
      },
      outputSpec(id, spec) {
        calls.push(["outputSpec", id, spec]);
        return createRawReadableHandle(id, spec);
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
    const deferred = signals.spec.outputCallback("panel", () => 1);
    const explicit = signals.spec.outputCallback("panelExplicit", () => 2);
    const namedComputed = signals.spec.computedCallback("named", () => 3);

    assert.equal(deferred.id, "panel");
    assert.equal(explicit.id, "panelExplicit");
    assert.equal(namedComputed.id, "named");

    assert.deepEqual(calls.slice(0, 5), [
      [
        "computedCallback",
        "__WORTHSignal.outputProjection.panel.1",
        "function",
      ],
      [
        "outputSpec",
        "panel",
        {
          reads: ["__WORTHSignal.outputProjection.panel.1"],
          expr: {
            kind: "read",
            id: "__WORTHSignal.outputProjection.panel.1",
          },
        },
      ],
      [
        "computedCallback",
        "__WORTHSignal.outputProjection.panelExplicit.2",
        "function",
      ],
      [
        "outputSpec",
        "panelExplicit",
        {
          reads: ["__WORTHSignal.outputProjection.panelExplicit.2"],
          expr: {
            kind: "read",
            id: "__WORTHSignal.outputProjection.panelExplicit.2",
          },
        },
      ],
      ["computedCallback", "named", "function"],
    ]);

    assert.throws(
      () => signals.input(1, "nope"),
      /input options must be an object when provided/,
    );
    assert.throws(
      () =>
        signals.computed("named", { expr: { kind: "value", value: 1 } }, {}),
      /computed app authoring does not accept an explicit id; use signals\.spec\.computed/,
    );
    assert.throws(
      () =>
        signals.output(
          "label",
          { expr: { kind: "value", value: 1 } },
          { id: "extra" },
        ),
      /output app authoring does not accept an explicit id; use signals\.spec\.output/,
    );
    assert.throws(
      () => signals.output(() => 1, "panel"),
      /output callback options must be an object when provided/,
    );
  } finally {
    await cleanup();
  }
});



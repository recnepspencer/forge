import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals supports no-arg host capability plans and named spec shorthand callbacks", async () => {
  const { hostCapabilityPlan, wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const calls = [];
    const rawSignals = {
      input(id, initial) {
        calls.push(["input", id, initial]);
        return createRawReadableHandle(id, initial);
      },
      computedSpec(id, spec) {
        calls.push(["computedSpec", id, spec]);
        return createRawReadableHandle(id, spec);
      },
      computedCallback(id, callback) {
        calls.push(["computedCallback", id, typeof callback]);
        return createRawReadableHandle(id, callback());
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
      transaction(callback) {
        callback({
          set() {},
        });
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

    const plan = hostCapabilityPlan();
    assert.equal(plan.viewport, undefined);
    assert.equal(plan.visibility, undefined);
    assert.equal(plan.online, undefined);
    assert.equal(plan.clock, undefined);
    assert.equal(plan.persistence, undefined);
    assert.equal(Object.isFrozen(plan), true);

    const signals = wrapSignals(rawSignals);
    const source = signals.input(3, { debugName: "count" });
    const computed = signals.scope("audit").computedSpec("label", {
      compute: () => source.value() + 1,
    });
    const output = signals.scope("audit").outputSpec("panel", {
      compute: () => computed.value() + 1,
    });

    assert.equal(source.value(), 3);
    assert.deepEqual(computed.value(), {
      __WorthSignalCallbackCapture: true,
      value: 4,
      reads: ["__WorthSignal.input.1"],
      hostCapabilityReads: [],
      runtimeReadBreadth: 0,
    });
    assert.deepEqual(output.get(), {
      reads: ["__WorthSignal.outputProjection.audit.panel.1"],
      expr: {
        kind: "read",
        id: "__WorthSignal.outputProjection.audit.panel.1",
      },
    });
    assert.deepEqual(calls, [
      ["input", "__WorthSignal.input.1", 3],
      ["computedCallback", "audit.label", "function"],
      ["computedCallback", "__WorthSignal.outputProjection.audit.panel.1", "function"],
      [
        "outputSpec",
        "audit.panel",
        {
          reads: ["__WorthSignal.outputProjection.audit.panel.1"],
          expr: {
            kind: "read",
            id: "__WorthSignal.outputProjection.audit.panel.1",
          },
        },
      ],
    ]);
  } finally {
    await cleanup();
  }
});

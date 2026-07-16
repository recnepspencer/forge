import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("wrapSignals rejects raw handles, foreign-runtime handles, and non-input mutations", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const firstCalls = [];
    const secondCalls = [];

    function buildRawSignals(callLog) {
      return {
        input(id, initial, options) {
          callLog.push(["input", id, initial, options]);
          return createRawReadableHandle(id, initial);
        },
        computedSpec(id, spec) {
          callLog.push(["computedSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        computedCallback(id, callback) {
          callLog.push(["computedCallback", id, typeof callback]);
          return createRawReadableHandle(id, id);
        },
        outputSpec(id, spec) {
          callLog.push(["outputSpec", id, spec]);
          return createRawReadableHandle(id, spec);
        },
        read(target) {
          callLog.push(["read", target.id ?? target]);
          return typeof target === "string" ? target : target.id;
        },
        watch(target) {
          callLog.push(["watch", target.id ?? target]);
          return { free() {} };
        },
        effect(target) {
          callLog.push(["effect", target.id ?? target]);
          return { free() {} };
        },
        transaction(callback) {
          const ops = [];
          callback({
            set(target, value) {
              ops.push(["set", target.id, value]);
            },
            setWithAspects(target, value, aspects) {
              ops.push(["setWithAspects", target.id, value, aspects]);
            },
            setWithRegions(target, value, changedRegions) {
              ops.push(["setWithRegions", target.id, value, changedRegions]);
            },
            setWithRegionsAndAspects(target, value, changedRegions, aspects) {
              ops.push([
                "setWithRegionsAndAspects",
                target.id,
                value,
                changedRegions,
                aspects,
              ]);
            },
            free() {},
          });
          callLog.push(["transaction", ops]);
          return ops;
        },
        batch(callback) {
          return this.transaction(callback);
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
    }

    const firstSignals = wrapSignals(buildRawSignals(firstCalls));
    const secondSignals = wrapSignals(buildRawSignals(secondCalls));

    const firstInput = firstSignals.input(1, { debugName: "count" });
    const secondInput = secondSignals.input(2, { debugName: "other" });
    const computed = firstSignals.computed(
      { expr: { kind: "value", value: 4 } },
      { debugName: "double" },
    );
    const rawHandle = createRawReadableHandle("raw", 9);

    assert.throws(
      () => firstSignals.read(rawHandle),
      /signals\.read expects a string id, a product signal handle created by this package/,
    );
    assert.throws(
      () => firstSignals.watch(secondInput, () => {}),
      /signals\.watch cannot use signal `other` from a different Signals runtime/,
    );
    assert.throws(
      () => firstSignals.effect(secondInput, () => {}),
      /signals\.effect cannot use signal `other` from a different Signals runtime/,
    );

    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(computed, 4)),
      /transaction\.set expects an input handle, but received a computed handle for `double`/,
    );
    assert.throws(
      () => firstSignals.transaction((tx) => tx.set(secondInput, 4)),
      /transaction\.set cannot use signal `other` from a different Signals runtime/,
    );

    const commit = firstSignals.transaction((tx) => tx.set(firstInput, 7));
    assert.deepEqual(commit, [["set", firstInput.id, 7]]);
  } finally {
    await cleanup();
  }
});



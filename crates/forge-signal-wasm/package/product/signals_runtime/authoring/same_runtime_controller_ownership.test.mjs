import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createRawReadableHandle } from "../runtime_fixture/raw_readable_handle.mjs";

test("The Same-Runtime Controller Ownership Test", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    function buildRawSignals() {
      return {
        input(id, initial) {
          return createRawReadableHandle(id, initial);
        },
        computedSpec(id, spec) {
          return createRawReadableHandle(id, spec);
        },
        computedCallback(id, callback) {
          return createRawReadableHandle(id, callback());
        },
        outputSpec(id, spec) {
          return createRawReadableHandle(id, spec);
        },
        read(target) {
          return typeof target === "string" ? target : target.id;
        },
        watch() {
          return { free() {} };
        },
        effect() {
          return { free() {} };
        },
        transaction(callback) {
          callback({ set() {}, free() {} });
          return {};
        },
        batch(callback) {
          callback({ set() {}, free() {} });
          return {};
        },
        nuke() {
          return true;
        },
        diagnostics() {
          return {};
        },
        history() {
          return {};
        },
        specialist() {
          return {};
        },
        adapters() {
          return {};
        },
        compatibilityApp() {
          return {};
        },
        compatibilityRuntime() {
          return {};
        },
        free() {},
      };
    }

    const firstSignals = wrapSignals(buildRawSignals());
    const secondSignals = wrapSignals(buildRawSignals());
    const count = firstSignals.input(1, { debugName: "count" });
    const other = secondSignals.input(2, { debugName: "other" });

    assert.throws(
      () => firstSignals.graph("", { outputs: { count } }),
      /signals\.graph requires a non-empty string graph id/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail"),
      /signals\.graph requires a graph definition object/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: {} }),
      /signals\.graph requires at least one published output/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: { count: "count" } }),
      /signals\.graph output `itemDetail\.count` expects a product signal handle created by this package/,
    );
    assert.throws(
      () => firstSignals.graph("itemDetail", { outputs: { other } }),
      /signals\.graph output `itemDetail\.other` cannot use signal `other` from a different Signals runtime/,
    );

    const graph = firstSignals.graph("itemDetail", { outputs: { count } });
    assert.throws(
      () => graph.output("missing"),
      /signals\.graph output `itemDetail\.missing` is not part of the published graph/,
    );
    assert.throws(
      () => graph.why("missing"),
      /signals\.graph output `itemDetail\.missing` is not part of the published graph/,
    );
  } finally {
    await cleanup();
  }
});



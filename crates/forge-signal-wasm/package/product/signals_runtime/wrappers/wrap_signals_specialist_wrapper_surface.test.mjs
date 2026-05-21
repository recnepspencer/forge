import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

test("wrapSignals exposes a typed specialist wrapper without dropping legacy expert methods", async () => {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const rawSignals = {
      input() {
        throw new Error("input not needed");
      },
      computedSpec() {
        throw new Error("computedSpec not needed");
      },
      computedCallback() {
        throw new Error("computedCallback not needed");
      },
      outputSpec() {
        throw new Error("outputSpec not needed");
      },
      read() {
        throw new Error("read not needed");
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
        return {
          evaluate_dirty() {
            return { touchedNodes: 3, nodesEvaluated: 2 };
          },
          graph_summary() {
            return { profile: "Development", activeNodeCount: 4 };
          },
          read_versions(ids) {
            return ids.map((id, index) => ({ id, version: index + 1 }));
          },
          free() {},
        };
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
    const specialist = signals.specialist();

    assert.equal(specialist.graphSummary().profile, "Development");
    assert.equal(specialist.graph_summary().activeNodeCount, 4);
    assert.equal(specialist.evaluateDirty().touchedNodes, 3);
    assert.equal(specialist.evaluate_dirty().nodesEvaluated, 2);
    assert.deepEqual(specialist.readVersions(["a", "b"]), [
      { id: "a", version: 1 },
      { id: "b", version: 2 },
    ]);
    assert.deepEqual(specialist.read_versions(["c"]), [
      { id: "c", version: 1 },
    ]);
  } finally {
    await cleanup();
  }
});

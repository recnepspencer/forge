import assert from "node:assert/strict";
import test from "node:test";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";

async function projectionModules() {
  const loaded = await loadSignalsModule();
  const schema = await loaded.importProductModule("local_truth/schema/schema_declaration.js");
  const projection = await loaded.importProductModule("local_truth/projection/signal_projection.js");
  return { ...schema, ...projection };
}

test("projection plans contain exact committed aspects and no truth authority", async () => {
  const { declareLocalTruthSchema, buildPlan } = await projectionModules();
  const schema = declareLocalTruthSchema({
    id: "projection.gear",
    aspects: ["label", "material", "rotation", "teeth"].map((field) => ({
      id: field,
      field,
      valueType: "any",
      equivalence: { kind: "exact" },
      costClass: "constant",
    })),
  });
  const snapshot = {
    values: { gear: { label: "Drive", material: "steel", rotation: 30, teeth: 20 } },
  };
  const plan = buildPlan(
    schema,
    new Map([["gear", {
      signalId: "gear-input",
      aspectMap: { label: 0, material: 1, rotation: 2, teeth: 3 },
    }]]),
    "branch:target",
    "truth-commit:1",
    snapshot,
    [
      { entityId: "gear", aspectId: "rotation" },
      { entityId: "gear", aspectId: "teeth" },
    ],
  );
  assert.deepEqual(plan.updates, [{
    entityId: "gear",
    signalId: "gear-input",
    value: snapshot.values.gear,
    truthAspects: ["rotation", "teeth"],
    aspects: [2, 3],
  }]);
  assert.equal(plan.counters.invalidatedAspects, 2);
  assert.equal("basis" in plan, false);
  assert.equal("decisions" in plan, false);
});

test("post-commit driver failure yields rebuild-required without changing truth inputs", async () => {
  const { declareLocalTruthSchema, createLocalTruthSignalProjection } = await projectionModules();
  const schema = declareLocalTruthSchema({
    id: "projection.failure",
    aspects: [{
      id: "teeth",
      field: "teeth",
      valueType: "number",
      equivalence: { kind: "exact" },
      costClass: "constant",
    }],
  });
  const projection = createLocalTruthSignalProjection({
    schema,
    bindings: [{ entityId: "gear", signalId: "gear-input", aspectMap: { teeth: 0 } }],
    driver: {
      initialize: async () => ({ branchId: 0 }),
      fork: async () => ({ branchId: 1 }),
      apply: async () => { throw new Error("injected evaluator interruption"); },
      rebuild: async () => ({ branchId: 2 }),
      destroy: async () => {},
    },
  });
  const snapshot = Object.freeze({ values: Object.freeze({ gear: Object.freeze({ teeth: 22 }) }) });
  const commit = Object.freeze({
    id: "truth-commit:committed",
    branchId: "branch:main",
    operations: Object.freeze([{ entityId: "gear", aspectId: "teeth" }]),
  });
  const receipt = await projection.project(commit, snapshot);
  assert.equal(receipt.posture, "RebuildRequired");
  assert.equal(receipt.commitId, commit.id);
  assert.match(receipt.reason, /evaluator interruption/);
  const rebuilt = await projection.rebuild({ id: "branch:main" }, snapshot);
  assert.equal(rebuilt.posture, "Current");
  assert.equal(rebuilt.invalidatedAspects, 1);
});

import assert from "node:assert/strict";

import { loadSignalsModule } from "../module_loading/load_signals_module.mjs";
import { createGraphOperationalRuntime } from "../runtime_fixture/graph_operational_runtime.mjs";

export async function withSignals(assertion) {
  const { wrapSignals, cleanup } = await loadSignalsModule();
  try {
    const signals = wrapSignals(createGraphOperationalRuntime());
    await assertion(signals);
  } finally {
    await cleanup();
  }
}

export function createTitleActionForm(signals, actionId, options) {
  return signals.form({
    source: { title: "Ship docs" },
    fields: ({ field }) => ({
      title: field("title"),
    }),
    actions: ({ action }) => ({
      [actionId]: action(actionId, options),
    }),
  });
}

export function assertActionExecution(artifact, expected) {
  for (const [key, value] of Object.entries(expected)) {
    assert.deepEqual(artifact[key], value, `action execution ${key}`);
  }
  assert.equal(typeof artifact.executionDigest, "string");
  assert.ok(artifact.executionDigest.length > 0);
}

export function assertHistoryKinds(form, expectedKinds) {
  assert.deepEqual(
    form.actionExecutionHistory().map((entry) => entry.resultKind),
    expectedKinds,
  );
}

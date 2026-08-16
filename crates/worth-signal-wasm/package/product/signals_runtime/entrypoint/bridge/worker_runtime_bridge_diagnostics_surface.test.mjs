import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function comparablePerformanceSummary(summary) {
  if (Array.isArray(summary)) {
    return summary.map(comparablePerformanceSummary);
  }
  if (!summary || typeof summary !== "object") {
    return summary;
  }
  const comparable = {};
  for (const [key, value] of Object.entries(summary)) {
    if (
      key.endsWith("_nanos")
      || key.startsWith("hostCapability")
      || key === "activeCallbackCount"
      || key === "activeComputeCallbackCount"
      || key.startsWith("computeCallback")
    ) {
      continue;
    }
    comparable[key] = comparablePerformanceSummary(value);
  }
  return comparable;
}

test("createWorkerRuntimeBridge exposes the remaining diagnostics read surface from worker-owned truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, createWorkerRuntimeBridge, cleanup } = mod;

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("bridgeDiagnostics", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("bridge:diagnostics:doubled", {
        reads: [count.id],
        expr: {
          kind: "sum",
          args: [
            { kind: "read", id: count.id },
            { kind: "read", id: count.id },
          ],
        },
        identity: { kind: "exact" },
      }),
    },
  });
  const bridge = createWorkerRuntimeBridge();

  try {
    await bridge.bootstrapRecord();
    await bridge.workerRuntimeShellLock();
    await bridge.publishPortableGraph({
      ...compatibilitySignals.adapters().exportDefinitions(),
      outputIds: [graph.output("doubled").id],
    });

    assert.deepEqual(
      await bridge.health(),
      compatibilitySignals.diagnostics().health(),
    );
    assert.deepEqual(
      comparablePerformanceSummary(await bridge.performanceSummary()),
      comparablePerformanceSummary(
        compatibilitySignals.diagnostics().performanceSummary(),
      ),
    );
    assert.deepEqual(
      await bridge.latestFailure(),
      compatibilitySignals.diagnostics().latestFailure(),
    );
    assert.deepEqual(
      await bridge.latestRollback(),
      compatibilitySignals.diagnostics().latestRollback(),
    );
    assert.deepEqual(
      await bridge.latestInvalidationPlanningEstimate(),
      compatibilitySignals.diagnostics().latestInvalidationPlanningEstimate(),
    );
    assert.deepEqual(
      await bridge.latestInvalidationTraceRecords(),
      compatibilitySignals.diagnostics().latestInvalidationTraceRecords(),
    );
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

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
      key.endsWith("_nanos") ||
      key.startsWith("hostCapability") ||
      key === "activeCallbackCount" ||
      key === "activeComputeCallbackCount" ||
      key.startsWith("computeCallback")
    ) {
      continue;
    }
    comparable[key] = comparablePerformanceSummary(value);
  }
  return comparable;
}

function comparableDiagnosticsSummary(summary) {
  return comparablePerformanceSummary(summary);
}

test("worker-first diagnostics facade preserves supported diagnostics parity and keeps host capability replay unavailability explicit", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, importProductModule, cleanup } = mod;
  const { createWorkerRuntimeBridge } = await importProductModule(
    "entrypoint/bridge/worker_runtime_bridge.js",
  );
  const { createWorkerFirstDiagnosticsFacade } = await importProductModule(
    "entrypoint/worker_first_diagnostics.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstDiagnostics", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:diagnostics:doubled", {
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
    const diagnostics = createWorkerFirstDiagnosticsFacade({ bridge });

    assert.deepEqual(
      await diagnostics.why(graph.output("doubled").id),
      compatibilitySignals.diagnostics().why(graph.output("doubled").id),
    );
    assert.deepEqual(
      await diagnostics.health(),
      compatibilitySignals.diagnostics().health(),
    );
    assert.deepEqual(
      comparableDiagnosticsSummary(await diagnostics.summaryNow()),
      comparableDiagnosticsSummary(compatibilitySignals.diagnostics().summaryNow()),
    );
    assert.deepEqual(
      await diagnostics.historyNow(),
      compatibilitySignals.diagnostics().historyNow(),
    );
    assert.deepEqual(
      await diagnostics.latestFlow(),
      compatibilitySignals.diagnostics().latestFlow(),
    );
    assert.deepEqual(
      await diagnostics.latestObservation(),
      compatibilitySignals.diagnostics().latestObservation(),
    );
    assert.deepEqual(
      comparablePerformanceSummary(await diagnostics.performanceSummary()),
      comparablePerformanceSummary(
        compatibilitySignals.diagnostics().performanceSummary(),
      ),
    );
    assert.deepEqual(
      await diagnostics.latestFailure(),
      compatibilitySignals.diagnostics().latestFailure(),
    );
    assert.deepEqual(
      await diagnostics.latestRollback(),
      compatibilitySignals.diagnostics().latestRollback(),
    );
    assert.deepEqual(
      await diagnostics.latestFrontierExecution(),
      compatibilitySignals.diagnostics().latestFrontierExecution(),
    );
    assert.deepEqual(
      await diagnostics.latestInvalidationTraceRecords(),
      compatibilitySignals.diagnostics().latestInvalidationTraceRecords(),
    );
    assert.deepEqual(
      await diagnostics.recentHistory(),
      compatibilitySignals.diagnostics().recentHistory(),
    );

    assert.deepEqual(diagnostics.latestHostCapabilityEvent(), {
      kind: "unavailable",
      reason: "workerFirstHostCapabilityEventReplayNotImplemented",
      message:
        "worker-first diagnostics host capability event replay is not implemented yet",
    });
    assert.deepEqual(diagnostics.recentHostCapabilityEvents(), [
      {
        kind: "unavailable",
        reason: "workerFirstHostCapabilityEventReplayNotImplemented",
        message:
          "worker-first diagnostics host capability event replay is not implemented yet",
      },
    ]);
    assert.deepEqual(await diagnostics.hostCapabilityReport(), {
      posture: "workerFirstUnavailable",
      reason: "workerFirstHostCapabilityEventReplayNotImplemented",
      message:
        "worker-first diagnostics host capability event replay is not implemented yet",
    });
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

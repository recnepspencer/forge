import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge bootstraps a dedicated worker-owned runtime shell", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    const bootstrap = await bridge.bootstrapRecord();
    const shellLock = await bridge.workerRuntimeShellLock();

    assert.equal(bootstrap.boundarySurface, "workerFirstConstruction");
    assert.equal(
      bootstrap.shellLock.identity.runtimeAuthority,
      "workerOwnedRuntime",
    );
    assert.equal(shellLock.identity.deploymentPosture, "workerFirst");
    assert.equal(shellLock.graphPublicationAdmission, "portableDefinitionsOnly");
  } finally {
    await bridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge publishes portable graphs, commits transactions, and reads diagnostics from the worker-owned runtime", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    const summary = await bridge.publishPortableGraph({
      policy: { preset: "development" },
      sources: [
        { id: "counter", initial: 1 },
      ],
      recipes: [
        {
          id: "doubleCounter",
          reads: ["counter"],
          expr: {
            kind: "sum",
            args: [
              { kind: "read", id: "counter" },
              { kind: "read", id: "counter" },
            ],
          },
          identity: { kind: "exact" },
        },
      ],
    });
    const envelope = await bridge.applyTransaction([
      { kind: "set", id: "counter", value: 7 },
    ]);
    const diagnostics = await bridge.readDiagnosticsSummary();

    assert.equal(summary.publishedSourceCount, 1);
    assert.equal(summary.publishedRecipeCount, 1);
    assert.equal(envelope.deploymentPosture, "workerFirst");
    assert.equal(envelope.runtimeAuthority, "workerOwnedRuntime");
    assert.equal(diagnostics.runtimeAuthority, "workerOwnedRuntime");
    assert.equal(typeof diagnostics.diagnosticsSummaryDigest, "string");
  } finally {
    await bridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

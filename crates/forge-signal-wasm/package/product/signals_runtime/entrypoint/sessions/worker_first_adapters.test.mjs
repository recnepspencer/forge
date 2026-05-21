import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("worker-first adapters facade preserves definitions and restores exact runtime truth without inventing a main-thread runtime", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, importProductModule, cleanup } = mod;
  const { createWorkerRuntimeBridge } = await importProductModule(
    "entrypoint/bridge/worker_runtime_bridge.js",
  );
  const { createWorkerFirstAdaptersFacade } = await importProductModule(
    "entrypoint/worker_first_adapters.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityGraph = compatibilitySignals.graph("workerFirstAdapters", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(3, { id: "count" });
    const doubled = scope.computedSpec("doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [{ kind: "read", id: count.id }, { kind: "read", id: count.id }],
      },
      identity: { kind: "exact" },
    });
    return graph.expose({ inputs: { count }, outputs: { doubled } });
  });
  const bridge = createWorkerRuntimeBridge();

  try {
    await bridge.publishPortableGraph({
      ...compatibilitySignals.adapters().exportDefinitions(),
      outputIds: [compatibilityGraph.output("doubled").id],
    });
    const adapters = createWorkerFirstAdaptersFacade({ bridge });

    assert.deepEqual(
      await adapters.exportDefinitions(),
      compatibilitySignals.adapters().exportDefinitions(),
    );

    await bridge.applyTransaction([{ kind: "set", id: compatibilityGraph.input("count").id, value: 11 }]);
    const artifact = await adapters.exportRuntimeEnvelope();
    assert.ok(artifact.snapshot);
    assert.equal(typeof artifact.runtimeEnvelopeRestoreToken, "string");
    assert.equal(typeof artifact.runtimeEnvelopePortableWire, "string");
    assert.equal(artifact.runtimeEnvelopeRestoreMode, "SameRuntimeExact");

    await bridge.applyTransaction([{ kind: "set", id: compatibilityGraph.input("count").id, value: 4 }]);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: [compatibilityGraph.output("doubled").id] })).outputs[0].value,
      8,
    );

    const exactImport = await adapters.restoreExactRuntimeEnvelope(artifact);
    assert.equal(exactImport.importOutcome, "AdmittedExact");
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: [compatibilityGraph.output("doubled").id] })).outputs[0].value,
      22,
    );

    await bridge.applyTransaction([{ kind: "set", id: compatibilityGraph.input("count").id, value: 2 }]);
    const portableImport = await adapters.replaceRuntimeEnvelope(artifact);
    assert.equal(portableImport.importOutcome, "Admitted");
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: [compatibilityGraph.output("doubled").id] })).outputs[0].value,
      22,
    );

    assert.deepEqual(
      await adapters.hostCapabilityTransportReport(artifact),
      compatibilitySignals.adapters().hostCapabilityTransportReport(
        compatibilitySignals.adapters().exportRuntimeEnvelope(),
      ),
    );
    assert.deepEqual(
      await adapters.runtimeProofReport(),
      compatibilitySignals.adapters().runtimeProofReport(),
    );
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first adapters facade rejects malformed runtime-envelope artifacts before worker import begins", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { importProductModule, cleanup } = mod;
  const { createWorkerFirstAdaptersFacade } = await importProductModule(
    "entrypoint/worker_first_adapters.js",
  );
  const adapters = createWorkerFirstAdaptersFacade({
    bridge: {
      exportDefinitions() {
        throw new Error("should not be called");
      },
      runtimeProofReport() {
        throw new Error("should not be called");
      },
    },
  });

  try {
    assert.throws(
      () => adapters.restoreExactRuntimeEnvelope({ runtimeEnvelopePortableWire: "wire-only" }),
      /exportRuntimeEnvelope/,
    );
    assert.throws(
      () => adapters.replaceRuntimeEnvelope({ runtimeEnvelopeRestoreToken: "token-only" }),
      /exportRuntimeEnvelope/,
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first adapters facade returns a structured denial report for callback-backed portable import artifacts", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, importProductModule, cleanup } = mod;
  const { createWorkerRuntimeBridge } = await importProductModule(
    "entrypoint/bridge/worker_runtime_bridge.js",
  );
  const { createWorkerFirstAdaptersFacade } = await importProductModule(
    "entrypoint/worker_first_adapters.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const bridge = createWorkerRuntimeBridge();

  try {
    await bridge.publishPortableGraph(counterPublicationWithOutput());
    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 6 }]);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      12,
    );

    const count = compatibilitySignals.input(5);
    compatibilitySignals.scope("workerFirstPortableDenial").computedSpec("label", {
      compute: () => count.value() + 1,
    });

    const adapters = createWorkerFirstAdaptersFacade({ bridge });
    const artifact = compatibilitySignals.adapters().exportRuntimeEnvelope();
    const report = await adapters.replaceRuntimeEnvelope(artifact);

    assert.equal(report.importOutcome, "Denied");
    assert.equal(report.rejectedCallbackCount, 1);
    assert.deepEqual(report.rejectedCallbackIds, ["workerFirstPortableDenial.label"]);
    assert.equal(report.hostCapabilityTransportCount, 0);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      12,
    );
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function counterPublicationWithOutput() {
  return {
    policy: { preset: "development" },
    sources: [{ id: "counter", initial: 1 }],
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
    outputIds: ["doubleCounter"],
  };
}

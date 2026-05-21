import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge exports exact and portable runtime-envelope wires that restore worker-owned truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    await bridge.publishPortableGraph(counterPublicationWithOutput());
    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 9 }]);

    const [runtimeEnvelopeRestoreToken, runtimeEnvelopePortableWire] = await Promise.all([
      bridge.exportWorkerRuntimeEnvelopeWire(),
      bridge.exportWorkerRuntimeEnvelopePortableWire(),
    ]);
    assert.equal(typeof runtimeEnvelopePortableWire, "string");

    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 2 }]);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      4,
    );

    const exactImport = await bridge.admitWorkerRuntimeEnvelopeImportWire(
      runtimeEnvelopeRestoreToken,
    );
    assert.equal(exactImport.importOutcome, "AdmittedExact");
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      18,
    );

    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 4 }]);
    const portableImport = await bridge.admitWorkerRuntimeEnvelopeImportPortableWire(
      runtimeEnvelopePortableWire,
    );
    assert.equal(portableImport.importOutcome, "Admitted");
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      18,
    );
  } finally {
    await bridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge returns a structured denial report for callback-backed portable runtime-envelope import", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({
    rawSurface: "real",
  });
  const bridge = createWorkerRuntimeBridge();
  const signals = await createSignals({ deployment: "mainThreadCompatibility" });
  try {
    await bridge.publishPortableGraph(counterPublicationWithOutput());
    await bridge.applyTransaction([{ kind: "set", id: "counter", value: 8 }]);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      16,
    );

    const count = signals.input(3);
    signals.scope("callbackPortableImport").computedSpec("label", {
      compute: () => count.value() + 1,
    });

    const artifact = signals.adapters().exportRuntimeEnvelope();
    const report = await bridge.admitWorkerRuntimeEnvelopeImportPortableWire(
      artifact.runtimeEnvelopePortableWire,
    );

    assert.equal(report.importOutcome, "Denied");
    assert.equal(report.rejectedCallbackCount, 1);
    assert.deepEqual(report.rejectedCallbackIds, ["callbackPortableImport.label"]);
    assert.equal(report.hostCapabilityTransportCount, 0);
    assert.equal(
      (await bridge.deliverOutputs({ outputIds: ["doubleCounter"] })).outputs[0].value,
      16,
    );
  } finally {
    signals.free();
    await bridge.terminate();
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

import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("default worker-first root exposes static resource helpers and keeps family construction explicit", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  try {
    const workerSignals = await createSignals();
    const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });

    assert.deepEqual(
      workerSignals.resource.effects.branchNative(),
      compatibilitySignals.resource.effects.branchNative(),
    );
    assert.deepEqual(
      workerSignals.resource.mutationResponses,
      compatibilitySignals.resource.mutationResponses,
    );

    const detailResponse = workerSignals.resource.response.detail()({
      valuePath: "task",
    });
    assert.equal(detailResponse.kind, "detail");
    assert.equal(detailResponse.source, "resource.response.detail<T>()");

    const deliveryPacket = workerSignals.resource.compatibility.delivery.basisRefresh({
      packetId: "packet-1",
      basisId: "basis-1",
      nextBasisId: "basis-2",
    });
    assert.equal(deliveryPacket.kind, "basisRefresh");

    const branchPlan = workerSignals.resource.branch.planMerge({
      source_branch_id: 0,
      target_branch_id: 0,
    });
    assert.equal(branchPlan.kind, "denied");
    assert.equal(branchPlan.reason, "mergePlanUnavailable");

    assert.throws(
      () => workerSignals.resource.detail({
        params: {},
        load: () => ({ title: "x" }),
      }),
      /worker-first resource surface/i,
    );
    assert.throws(
      () => workerSignals.scope("wizard").resource.collection({
        params: {},
        load: () => ({ items: [] }),
      }),
      /worker-first resource surface/i,
    );
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function comparableWhy(summary) {
  return {
    id: summary.id,
    apiFamily: summary.apiFamily,
    recipeFamily: summary.recipeFamily,
    state: summary.state,
    changedRegions: summary.changedRegions,
    propagationSuppressed: summary.propagationSuppressed,
    outputChange: summary.outputChange,
    outputIdentity: summary.outputIdentity,
    callback: summary.callback,
    upstreamCount: Array.isArray(summary.upstream) ? summary.upstream.length : 0,
  };
}

test("createWorkerRuntimeBridge preserves graph inspection parity for published graph truth", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, createWorkerRuntimeBridge, cleanup } = mod;
  const bridge = createWorkerRuntimeBridge();

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const compatibilityGraph = compatibilitySignals.graph("bridgeInspection", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(2, { id: "count" });
    const item = scope.input({ count: 2, label: "alpha" }, { id: "item" });
    const doubled = scope.computedSpec("doubled", {
      reads: [count.id],
      expr: {
        kind: "sum",
        args: [
          { kind: "read", id: count.id },
          { kind: "read", id: count.id },
        ],
      },
      identity: { kind: "exact" },
    });
    return graph.expose({
      inputs: { count, item },
      outputs: { doubled, item },
    });
  });
  const exportedDefinition = compatibilityGraph.exportDefinition();

  try {
    await bridge.publishPortableGraph({
      ...exportedDefinition.compatibility.definitions,
      outputIds: exportedDefinition.descriptors.map((descriptor) => descriptor.publishedId),
    });

    assert.deepEqual(
      await bridge.exportDefinitions(),
      {
        ...exportedDefinition.compatibility.definitions,
        workerPublicOutputIds: exportedDefinition.descriptors.map(
          (descriptor) => descriptor.publishedId,
        ),
      },
    );
    assert.deepEqual(
      comparableWhy(await bridge.why(exportedDefinition.inputDescriptors[0].sourceId)),
      {
        id: exportedDefinition.inputDescriptors[0].sourceId,
        apiFamily: null,
        recipeFamily: null,
        state: "Clean",
        changedRegions: [],
        propagationSuppressed: false,
        outputChange: "Replaced",
        outputIdentity: null,
        callback: null,
        upstreamCount: 0,
      },
    );
    assert.deepEqual(
      comparableWhy(await bridge.why(exportedDefinition.descriptors[0].publishedId)),
      comparableWhy(compatibilityGraph.why("doubled")),
    );
    assert.deepEqual(
      await bridge.replayFor(exportedDefinition.descriptors[0].publishedId),
      compatibilityGraph.replayFor("doubled"),
    );
    assert.deepEqual(
      await bridge.lineageFor(exportedDefinition.inputDescriptors[1].sourceId),
      compatibilityGraph.inspectHistory().input("item").lineage,
    );
    assert.deepEqual(
      await bridge.readVersions(
        exportedDefinition.descriptors.map((descriptor) => descriptor.publishedId),
      ),
      compatibilityGraph.readVersions(),
    );
    assert.deepEqual(
      await bridge.runtimeProofReport(),
      compatibilityGraph.adapters().runtimeProofReport(),
    );

    compatibilityGraph.writeInput("count", 6);
    await bridge.applyTransaction([
      {
        kind: "set",
        id: exportedDefinition.inputDescriptors[0].sourceId,
        value: 6,
      },
    ]);

    assert.deepEqual(
      await bridge.readVersions(
        exportedDefinition.descriptors.map((descriptor) => descriptor.publishedId),
      ),
      compatibilityGraph.readVersions(),
    );
    assert.deepEqual(await bridge.latestObservation(), {
      observation: {
        classified_event_count: 0,
        trigger_matched_event_count: 0,
        delivered_event_count: 0,
        rollback_suppressed_event_count: 0,
        boundary_events: [],
      },
      callbackNodes: [],
    });
    await assert.rejects(
      () => bridge.deliverLatestObservation(),
      /requires an active lifecycle subscription/,
    );
    assert.ok((await bridge.latestFlow()).flow);
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge attaches and detaches committed observation delivery subscriptions", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, createWorkerRuntimeBridge, cleanup } = mod;
  const bridge = createWorkerRuntimeBridge();

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(3, { debugName: "count" });
  const graph = compatibilitySignals.graph("bridgeObservationDelivery", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("bridge:observation:doubled", {
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
  const definition = graph.exportDefinition();
  const outputId = graph.output("doubled").id;

  try {
    await bridge.publishPortableGraph({
      ...definition.compatibility.definitions,
      outputIds: definition.descriptors.map((descriptor) => descriptor.publishedId),
    });
    const lifecycle = await bridge.attachObservationDelivery({ signalId: outputId });

    await bridge.applyTransaction([{ kind: "set", id: definition.inputDescriptors[0].sourceId, value: 8 }]);
    const packet = await bridge.deliverLatestObservation();
    const detachPacket = await bridge.detachObservationDelivery({
      lifecycleSubscriptionId: lifecycle.lifecycleSubscriptionId,
    });

    assert.equal(packet.envelopeFamily, "observationDelivery");
    assert.equal(packet.runtimeAuthority, "workerOwnedRuntime");
    assert.equal(packet.observation.observation.delivered_event_count, 1);
    assert.equal(packet.observation.observation.boundary_events[0].outcome, "Delivered");
    assert.equal(detachPacket.lifecycleEvent, "ObserverDetached");
    await assert.rejects(
      () => bridge.deliverLatestObservation(),
      /requires an active lifecycle subscription/,
    );
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

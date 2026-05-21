import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

test("createWorkerRuntimeBridge can return one combined committed projection packet that matches the authoritative component reads", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const projectionBridge = createWorkerRuntimeBridge();
  const parityBridge = createWorkerRuntimeBridge();
  try {
    const publication = counterPublicationWithOutput();
    await projectionBridge.publishPortableGraph(publication);
    await parityBridge.publishPortableGraph(publication);

    const projection = await projectionBridge.applyTransactionProjection({
      transactionOps: [{ kind: "set", id: "counter", value: 7 }],
      outputIds: ["doubleCounter"],
    });

    const transaction = await parityBridge.applyTransaction([
      { kind: "set", id: "counter", value: 7 },
    ]);
    const outputs = await parityBridge.deliverOutputs({ outputIds: ["doubleCounter"] });
    const diagnosticsSummary = await parityBridge.readDiagnosticsSummary();
    const diagnosticsHistory = await parityBridge.readDiagnosticsHistory();

    assert.equal(projection.envelopeFamily, "workerCommittedProjection");
    assert.equal(projection.runtimeAuthority, "workerOwnedRuntime");
    assert.equal(
      projection.workerFirstTruthDigest,
      projection.transaction.committedTruthDigest,
    );
    assert.equal(
      projection.transaction.committedTruthDigest,
      transaction.committedTruthDigest,
    );
    assert.deepEqual(
      summarizeRunCounters(projection.transaction.runSummary),
      summarizeRunCounters(transaction.runSummary),
    );
    assert.equal(projection.outputs.outputDigest, outputs.outputDigest);
    assert.deepEqual(projection.outputs.outputs, outputs.outputs);
    assert.equal(
      projection.diagnosticsSummary.diagnosticsSummaryDigest,
      diagnosticsSummary.diagnosticsSummaryDigest,
    );
    assert.deepEqual(
      summarizeGraphSummary(projection.diagnosticsSummary.summary),
      summarizeGraphSummary(diagnosticsSummary.summary),
    );
    assert.equal(
      projection.diagnosticsHistory.diagnosticsHistoryDigest,
      diagnosticsHistory.diagnosticsHistoryDigest,
    );
    assert.deepEqual(
      summarizeDiagnosticsHistory(projection.diagnosticsHistory.history),
      summarizeDiagnosticsHistory(diagnosticsHistory.history),
    );
    assert.equal(typeof projection.projectionDigest, "string");
    assert.equal(typeof projection.packetDigest, "string");
  } finally {
    await projectionBridge.terminate();
    await parityBridge.terminate();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("createWorkerRuntimeBridge committed projection rejects invalid output delivery requests before claiming a projection packet", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createWorkerRuntimeBridge, cleanup } = await loadSignalsModule({ rawSurface: "real" });
  const bridge = createWorkerRuntimeBridge();
  try {
    await bridge.publishPortableGraph(counterPublicationWithOutput());

    await assert.rejects(
      () =>
        bridge.applyTransactionProjection({
          transactionOps: [{ kind: "set", id: "counter", value: 11 }],
          outputIds: [],
        }),
      /at least one output id/,
    );
  } finally {
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

function summarizeRunCounters(runSummary) {
  return {
    touchedNodes: runSummary.touchedNodes,
    nodesEvaluated: runSummary.nodesEvaluated,
    nodesRecomputed: runSummary.nodesRecomputed,
    nodesSuppressed: runSummary.nodesSuppressed,
    plansBuilt: runSummary.plansBuilt,
    stagesExecuted: runSummary.stagesExecuted,
  };
}

function summarizeGraphSummary(summary) {
  return {
    profile: summary.profile,
    active_node_count: summary.active_node_count,
    clean_node_count: summary.clean_node_count,
    maybe_stale_node_count: summary.maybe_stale_node_count,
    dirty_node_count: summary.dirty_node_count,
    dependency_edge_count: summary.dependency_edge_count,
    subscriber_edge_count: summary.subscriber_edge_count,
    nodes_with_execution_record: summary.nodes_with_execution_record,
    nodes_with_causality: summary.nodes_with_causality,
    sample_dirty_nodes: summary.sample_dirty_nodes,
    sample_nodes_with_execution_record: summary.sample_nodes_with_execution_record,
  };
}

function summarizeDiagnosticsHistory(history) {
  const callbackNodes =
    Array.isArray(history.callbackNodes)
      ? history.callbackNodes
      : Array.isArray(history.callback_nodes)
        ? history.callback_nodes
        : [];
  return {
    profile: history.history.profile,
    traced_node_count: history.history.traced_node_count,
    execution_record_count: history.history.execution_record_count,
    latest_execution_record_id: history.history.latest_execution_record_id,
    callback_node_count: callbackNodes.length,
  };
}

import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

function comparableRuntimeSummary(summary) {
  return {
    profile: summary.profile,
    active_node_count: summary.active_node_count,
    clean_node_count: summary.clean_node_count,
    maybe_stale_node_count: summary.maybe_stale_node_count,
    dirty_node_count: summary.dirty_node_count,
    dependency_edge_count: summary.dependency_edge_count,
    subscriber_edge_count: summary.subscriber_edge_count,
    nodes_with_trace_summary: summary.nodes_with_trace_summary,
    nodes_with_execution_record: summary.nodes_with_execution_record,
  };
}

function comparableRuntimeHistory(history) {
  return {
    profile: history.profile,
    traced_node_count: history.traced_node_count,
    execution_record_count: history.execution_record_count,
    latest_execution_record_id: history.latest_execution_record_id,
    node_ids: Array.isArray(history.nodes) ? history.nodes.map((node) => node.id) : [],
  };
}

test("worker-first published graph session exposes declared published-graph facades and exports reusable worker snapshots", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, cleanup, importProductModule } = mod;
  const { createWorkerFirstPublishedGraphSession } = await importProductModule(
    "entrypoint/worker_first_published_graph.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstPublishedGraphSurface", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:published:surface:doubled", {
        reads: [count.id],
        expr: {
          kind: "sum",
          args: [{ kind: "read", id: count.id }, { kind: "read", id: count.id }],
        },
        identity: { kind: "exact" },
      }),
    },
  });
  const workerRootSignals = await createSignals();
  const workerGraph = await createWorkerFirstPublishedGraphSession({
    definition: graph.exportDefinition(),
  });

  try {
    await workerGraph.writeInput("count", 7);

    const diagnostics = workerGraph.diagnostics();
    const specialist = workerGraph.specialist();
    const adapters = workerGraph.adapters();
    const contractDelta = workerGraph.contractDelta(workerGraph.contract());

    assert.equal(contractDelta.changed, false);
    assert.equal(contractDelta.graphId, workerGraph.id);
    assert.deepEqual(
      comparableRuntimeSummary(await diagnostics.summaryNow()),
      comparableRuntimeSummary(workerGraph.diagnosticsSummary()),
    );
    assert.deepEqual(
      comparableRuntimeHistory(await diagnostics.historyNow()),
      comparableRuntimeHistory(workerGraph.diagnosticsHistory()),
    );
    assert.deepEqual(
      comparableRuntimeSummary(specialist.graphSummary()),
      comparableRuntimeSummary(workerGraph.diagnosticsSummary()),
    );
    assert.deepEqual(
      comparableRuntimeSummary(specialist.graph_summary()),
      comparableRuntimeSummary(workerGraph.diagnosticsSummary()),
    );
    assert.deepEqual(
      await specialist.readVersions([workerGraph.output("doubled").id]),
      await workerGraph.readVersions(),
    );
    assert.deepEqual(
      await specialist.read_versions([workerGraph.output("doubled").id]),
      await workerGraph.readVersions(),
    );
    assert.deepEqual(await adapters.runtimeProofReport(), await workerGraph.runtimeProofReport());

    await workerGraph.transaction((tx) => {
      tx.set("count", 8);
    });
    assert.equal(workerGraph.read().doubled, 16);

    await workerGraph.transactionAsync((tx) => {
      tx.set(workerGraph.input("count"), 9);
    });
    assert.equal(workerGraph.read().doubled, 18);

    await workerGraph.batchAsync((tx) => {
      tx.set("count", 10);
    });
    assert.equal(workerGraph.read().doubled, 20);

    await assert.rejects(
      () => workerGraph.transactionAsync(() => {}),
      /requires at least one staged mutation/,
    );

    const snapshot = await workerGraph.exportSnapshot();
    const importedGraph = workerRootSignals.importGraph(snapshot.definition, snapshot);
    await importedGraph.ready();

    assert.equal(snapshot.runtimeEnvelope.runtimeEnvelopeRestoreMode, "SameRuntimeExact");
    assert.equal(typeof snapshot.runtimeEnvelope.runtimeEnvelopeRestoreToken, "string");
    assert.equal(typeof snapshot.runtimeEnvelope.runtimeEnvelopePortableWire, "string");
    assert.deepEqual(snapshot.definition, workerGraph.exportDefinition());
    assert.deepEqual(importedGraph.readInputs(), workerGraph.readInputs());
    assert.deepEqual(importedGraph.read(), workerGraph.read());

    await importedGraph.terminate();
  } finally {
    await workerGraph.terminate();
    workerRootSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first published graph session denies compatibility-sidecar app and runtime doors", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, cleanup, importProductModule } = mod;
  const { createWorkerFirstPublishedGraphSession } = await importProductModule(
    "entrypoint/worker_first_published_graph.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(1, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstPublishedGraphUnavailable", {
    inputs: { count },
    outputs: { count },
  });
  const workerGraph = await createWorkerFirstPublishedGraphSession({
    definition: graph.exportDefinition(),
  });

  try {
    for (const operation of ["compatibilityApp", "compatibilityRuntime"]) {
      assert.throws(() => workerGraph[operation](), (error) => {
        assert.equal(error?.name, "WorkerFirstPublishedGraphUnavailable");
        assert.equal(error?.code, "workerFirstPublishedGraphUnavailable");
        assert.equal(error?.compatibilityRecovery?.deployment, "mainThreadCompatibility");
        return /compatibility-sidecar runtime/.test(error?.message ?? "");
      });
    }
  } finally {
    await workerGraph.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";
import {
  materializeGraphDiagnosticsSurface,
  materializeGraphHistorySurface,
} from "../../runtime_fixture/surface_materialization.mjs";

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

function comparableInputWhy(summary) {
  return {
    id: summary.id,
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

function comparableDiagnosticsSummary(summary) {
  return {
    profile: summary.profile,
    active_node_count: summary.active_node_count,
    arena_capacity: summary.arena_capacity,
    tombstone_count: summary.tombstone_count,
    clean_node_count: summary.clean_node_count,
    maybe_stale_node_count: summary.maybe_stale_node_count,
    dirty_node_count: summary.dirty_node_count,
    dependency_edge_count: summary.dependency_edge_count,
    subscriber_edge_count: summary.subscriber_edge_count,
    nodes_with_trace_summary: summary.nodes_with_trace_summary,
    nodes_with_execution_record: summary.nodes_with_execution_record,
    sample_dirty_nodes: summary.sample_dirty_nodes,
    sample_nodes_with_execution_record: summary.sample_nodes_with_execution_record,
  };
}

function comparableDiagnosticsHistory(history) {
  return {
    profile: history.profile,
    traced_node_count: history.traced_node_count,
    execution_record_count: history.execution_record_count,
    latest_execution_record_id: history.latest_execution_record_id,
    reuse_origin_counts: history.reuse_origin_counts,
    node_ids: Array.isArray(history.nodes) ? history.nodes.map((node) => node.id) : [],
  };
}

function comparableGraphDiagnosticsSurface(surface) {
  const materialized = materializeGraphDiagnosticsSurface(surface);
  return {
    graph: materialized.graph,
    contract: materialized.contract,
    dependencies: materialized.dependencies,
    inputDescriptors: materialized.inputDescriptors,
    descriptors: materialized.descriptors,
    inputVersions: materialized.inputVersions,
    outputVersions: materialized.outputVersions,
    inputs: Object.fromEntries(
      Object.entries(materialized.inputs).map(([name, entry]) => [
        name,
        {
          descriptor: entry.descriptor,
          version: entry.version,
          why: comparableInputWhy(entry.why),
        },
      ]),
    ),
    outputs: Object.fromEntries(
      Object.entries(materialized.outputs).map(([name, entry]) => [
        name,
        {
          descriptor: entry.descriptor,
          version: entry.version,
          why: comparableWhy(entry.why),
        },
      ]),
    ),
    runtimeGraph: comparableDiagnosticsSummary(materialized.runtimeGraph),
    executionHistory: comparableDiagnosticsHistory(materialized.executionHistory),
    latestObservationPhase:
      materialized.latestObservation?.observation?.phase ?? null,
    hasLatestFlow: materialized.latestFlow !== null,
  };
}

function comparableGraphHistorySurface(surface) {
  const materialized = materializeGraphHistorySurface(surface);
  return {
    graph: materialized.graph,
    contract: materialized.contract,
    dependencies: materialized.dependencies,
    inputDescriptors: materialized.inputDescriptors,
    descriptors: materialized.descriptors,
    inputs: Object.fromEntries(
      Object.entries(materialized.inputs).map(([name, entry]) => [
        name,
        {
          descriptor: entry.descriptor,
          replay: stripRuntimeNodeFields(entry.replay),
          lineage: stripRuntimeNodeFields(entry.lineage),
        },
      ]),
    ),
    outputs: Object.fromEntries(
      Object.entries(materialized.outputs).map(([name, entry]) => [
        name,
        {
          descriptor: entry.descriptor,
          replay: stripRuntimeNodeFields(entry.replay),
          lineage: stripRuntimeNodeFields(entry.lineage),
        },
      ]),
    ),
    executionHistory: comparableDiagnosticsHistory(materialized.executionHistory),
    recentHistoryIds: Array.isArray(materialized.recentHistory)
      ? materialized.recentHistory.map((entry) => entry.latest_execution_record_id)
      : [],
  };
}

function comparableReplayOrLineage(summary) {
  return stripRuntimeNodeFields(summary);
}

function stripRuntimeNodeFields(value) {
  if (Array.isArray(value)) {
    return value.map(stripRuntimeNodeFields);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const copy = {};
  for (const [key, entry] of Object.entries(value)) {
    if (key === "node") {
      continue;
    }
    copy[key] = stripRuntimeNodeFields(entry);
  }
  return copy;
}

test("worker-first published graph session preserves committed graph truth and diagnostics parity with explicit compatibility construction", async () => {
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
  const compatibilityGraph = compatibilitySignals.graph("entryCounter", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(2, { id: "count" });
    const item = scope.input({ count: 2, label: "alpha" }, { id: "item" });
    const locked = scope.input("sealed", { id: "locked" });
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
      inputs: {
        count,
        item,
        locked: graph.publicInput(locked, { authority: "readOnly" }),
      },
      outputs: {
        doubled,
        item,
        locked,
      },
    });
  });
  const workerGraph = await createWorkerFirstPublishedGraphSession({
    definition: compatibilityGraph.exportDefinition(),
  });

  try {
    const workerItemInput = workerGraph.readInput("item");
    assert.equal(Object.isFrozen(workerItemInput), true);
    assert.throws(() => {
      workerItemInput.label = "mutated";
    }, /Cannot assign/);

    assert.deepEqual(workerGraph.readInputs(), compatibilityGraph.readInputs());
    assert.deepEqual(workerGraph.read(), compatibilityGraph.read());
    assert.deepEqual(
      comparableWhy(await workerGraph.why("doubled")),
      comparableWhy(compatibilityGraph.why("doubled")),
    );
    assert.deepEqual(
      comparableReplayOrLineage(await workerGraph.replayFor("doubled")),
      comparableReplayOrLineage(compatibilityGraph.replayFor("doubled")),
    );
    assert.deepEqual(
      comparableReplayOrLineage(await workerGraph.lineageFor("locked")),
      comparableReplayOrLineage(compatibilityGraph.lineageFor("locked")),
    );
    assert.deepEqual(
      await workerGraph.readVersions(),
      compatibilityGraph.readVersions(),
    );
    assert.deepEqual(
      comparableDiagnosticsSummary(workerGraph.diagnosticsSummary()),
      comparableDiagnosticsSummary(compatibilityGraph.diagnostics().summaryNow()),
    );
    assert.deepEqual(
      comparableDiagnosticsHistory(workerGraph.diagnosticsHistory()),
      comparableDiagnosticsHistory(compatibilityGraph.diagnostics().historyNow()),
    );
    assert.deepEqual(
      comparableGraphDiagnosticsSurface(await workerGraph.inspectDiagnostics()),
      comparableGraphDiagnosticsSurface(compatibilityGraph.inspectDiagnostics()),
    );
    assert.deepEqual(
      comparableGraphHistorySurface(await workerGraph.inspectHistory()),
      comparableGraphHistorySurface(compatibilityGraph.inspectHistory()),
    );
    assert.deepEqual(
      workerGraph.exportCompatibilityDefinition(),
      compatibilityGraph.exportCompatibilityDefinition(),
    );
    assert.deepEqual(
      workerGraph.exportDefinition(),
      compatibilityGraph.exportDefinition(),
    );
    assert.deepEqual(
      await workerGraph.runtimeProofReport(),
      compatibilityGraph.adapters().runtimeProofReport(),
    );

    compatibilityGraph.writeInput("count", 5);
    await workerGraph.writeInput("count", 5);
    assert.deepEqual(workerGraph.readInputs(), compatibilityGraph.readInputs());
    assert.deepEqual(workerGraph.read(), compatibilityGraph.read());

    compatibilityGraph.patchInput("item", { label: "beta" });
    await workerGraph.patchInput("item", { label: "beta" });
    assert.deepEqual(workerGraph.readInputs(), compatibilityGraph.readInputs());
    assert.deepEqual(workerGraph.read(), compatibilityGraph.read());

    compatibilityGraph.resetInput("count");
    await workerGraph.resetInput("count");
    assert.deepEqual(workerGraph.readInputs(), compatibilityGraph.readInputs());
    assert.deepEqual(workerGraph.read(), compatibilityGraph.read());
    assert.deepEqual(
      comparableDiagnosticsSummary(workerGraph.diagnosticsSummary()),
      comparableDiagnosticsSummary(compatibilityGraph.diagnostics().summaryNow()),
    );
    assert.deepEqual(
      comparableDiagnosticsHistory(workerGraph.diagnosticsHistory()),
      comparableDiagnosticsHistory(compatibilityGraph.diagnostics().historyNow()),
    );
    assert.deepEqual(
      await workerGraph.readVersions(),
      compatibilityGraph.readVersions(),
    );
    assert.deepEqual(
      comparableGraphDiagnosticsSurface(await workerGraph.inspectDiagnostics()),
      comparableGraphDiagnosticsSurface(compatibilityGraph.inspectDiagnostics()),
    );
    assert.deepEqual(
      comparableGraphHistorySurface(await workerGraph.inspectHistory()),
      comparableGraphHistorySurface(compatibilityGraph.inspectHistory()),
    );
  } finally {
    await workerGraph.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("worker-first published graph session denies writes that violate the exported operational contract", async () => {
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
  const compatibilityGraph = compatibilitySignals.graph("entryCounter", (graph) => {
    const scope = graph.scope("worker");
    const count = scope.input(1, { id: "count" });
    const locked = scope.input("sealed", { id: "locked" });
    return graph.expose({
      inputs: {
        count,
        locked: graph.publicInput(locked, { authority: "readOnly" }),
      },
      outputs: {
        count,
        locked,
      },
    });
  });
  const workerGraph = await createWorkerFirstPublishedGraphSession({
    definition: compatibilityGraph.exportDefinition(),
  });

  try {
    await assert.rejects(
      () => workerGraph.writeInput("locked", "mutated"),
      /operational contract denies supportsWrite/,
    );
    await assert.rejects(
      () => workerGraph.patchInput("count", 4),
      /operational contract denies supportsPatch/,
    );
    assert.throws(
      () => workerGraph.readOutput("missing"),
      /does not expose published output `missing`/,
    );
  } finally {
    await workerGraph.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

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

function comparableGraphSummary(summary) {
  if (Array.isArray(summary)) {
    return summary.map(comparableGraphSummary);
  }
  if (!summary || typeof summary !== "object") {
    return summary;
  }
  const comparable = {};
  for (const [key, value] of Object.entries(summary)) {
    if (
      key.endsWith("_nanos")
      || key === "nodes_with_execution_record"
      || key === "sample_nodes_with_execution_record"
      || key === "patch_application_breadth"
      || key === "shared_snapshot_replacement_count"
      || key === "snapshot_batch_size"
      || key === "structural_replace_batch_commit_count"
    ) {
      continue;
    }
    comparable[key] = comparableGraphSummary(value);
  }
  return comparable;
}

function comparableHistorySurfaceSummary(summary) {
  if (Array.isArray(summary)) {
    return summary.map(comparableHistorySurfaceSummary);
  }
  if (!summary || typeof summary !== "object") {
    return summary;
  }
  const comparable = {};
  for (const [key, value] of Object.entries(summary)) {
    if (key === "execution_record_count" || key === "latest_execution_record_id") {
      continue;
    }
    comparable[key] = comparableHistorySurfaceSummary(value);
  }
  return comparable;
}

function comparableRecentHistory(history) {
  return history.filter(
    (entry) => !(entry.execution_record_count === 0 && entry.latest_execution_record_id == null),
  );
}

function comparableSnapshotEnvelope(envelope) {
  return {
    state: envelope.state,
  };
}

function comparableSnapshotArtifact(snapshot) {
  return {
    state: snapshot.state,
  };
}

function comparableRuntimeBranch(branch) {
  return {
    id: branch.id,
    name: branch.name,
    parent_branch_id: branch.parent_branch_id,
  };
}

function comparableBranchStateProof(proof) {
  return {
    proofSchemaVersion: proof.proofSchemaVersion,
    branchId: proof.branchId,
    branchName: proof.branchName,
    stateDigest: proof.stateDigest,
  };
}

function comparableReplayParityProof(proof) {
  return {
    proofSchemaVersion: proof.proofSchemaVersion,
    expectedBranchId: proof.expectedBranchId,
    expectedBranchName: proof.expectedBranchName,
    expectedStateDigest: proof.expectedStateDigest,
    replayedBranchId: proof.replayedBranchId,
    replayedBranchName: proof.replayedBranchName,
    replayedStateDigest: proof.replayedStateDigest,
    parity: proof.parity,
    mismatchClasses: proof.mismatchClasses,
  };
}

test("default worker-first root exposes synchronous cached diagnostics, history, and adapters for the active imported graph", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.input(4, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstRootSurfaces", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:root:surfaces:doubled", {
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
  graph.writeInput("count", 9);
  const definition = graph.exportDefinition();
  const snapshot = graph.exportSnapshot();
  const outputId = graph.output("doubled").id;
  const compatibilityImportedSignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  await compatibilityImportedSignals.importGraph(definition, snapshot).ready();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, snapshot);
    await importedGraph.ready();

    assert.deepEqual(
      workerSignals.diagnostics().why(outputId),
      compatibilityImportedSignals.diagnostics().why(outputId),
    );
    assert.deepEqual(
      workerSignals.diagnostics().health(),
      compatibilityImportedSignals.diagnostics().health(),
    );
    assert.deepEqual(
      comparableGraphSummary(workerSignals.diagnostics().summaryNow()),
      comparableGraphSummary(compatibilityImportedSignals.diagnostics().summaryNow()),
    );
    assert.deepEqual(
      comparableHistorySurfaceSummary(workerSignals.diagnostics().historyNow()),
      comparableHistorySurfaceSummary(compatibilityImportedSignals.diagnostics().historyNow()),
    );
    assert.equal(workerSignals.diagnostics().latestHostCapabilityEvent(), null);
    assert.deepEqual(workerSignals.diagnostics().recentHostCapabilityEvents(), []);
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.registrationCount,
      0,
    );
    assert.equal(
      workerSignals.diagnostics().hostCapabilityReport().totals.retainedEventCount,
      0,
    );
    assert.deepEqual(
      comparablePerformanceSummary(workerSignals.diagnostics().performanceSummary()),
      comparablePerformanceSummary(compatibilityImportedSignals.diagnostics().performanceSummary()),
    );
    assert.equal(
      workerSignals.read(outputId),
      compatibilityImportedSignals.read(outputId),
    );
    assert.equal(
      workerSignals.read(importedGraph.output("doubled")),
      compatibilityImportedSignals.read(outputId),
    );

    assert.deepEqual(
      workerSignals.history().replay_for(outputId),
      compatibilityImportedSignals.history().replay_for(outputId),
    );
    assert.deepEqual(
      workerSignals.history().lineage_for(outputId),
      compatibilityImportedSignals.history().lineage_for(outputId),
    );
    assert.deepEqual(
      comparableRecentHistory(workerSignals.history().recentHistory()),
      comparableRecentHistory(compatibilityImportedSignals.diagnostics().recentHistory()),
    );
    assert.deepEqual(
      comparableSnapshotEnvelope(workerSignals.history().snapshot()),
      comparableSnapshotEnvelope(compatibilityImportedSignals.history().snapshot()),
    );
    assert.deepEqual(
      comparableRuntimeBranch(workerSignals.history().current_branch()),
      comparableRuntimeBranch(compatibilityImportedSignals.history().current_branch()),
    );
    assert.deepEqual(
      workerSignals.history().branches().map(comparableRuntimeBranch),
      compatibilityImportedSignals.history().branches().map(comparableRuntimeBranch),
    );
    const workerBranchReplay = workerSignals.history().replay_for_branch(0);
    assert.ok(workerBranchReplay.frames.length >= 1);
    assert.equal(
      workerBranchReplay.frames[workerBranchReplay.frames.length - 1].branchId,
      0,
    );
    const workerBranchSnapshot = workerSignals.history().branch_snapshot(0);
    assert.equal(
      workerSignals.history().branch_snapshot_id(0),
      workerSignals.history().branch_snapshot_envelope(0).snapshot.meta.snapshot_id,
    );
    assert.equal(typeof workerBranchSnapshot.snapshotRestoreToken, "string");
    assert.deepEqual(
      comparableSnapshotEnvelope(workerSignals.history().branch_snapshot_envelope(0)),
      comparableSnapshotEnvelope(compatibilityImportedSignals.history().branch_snapshot_envelope(0)),
    );
    assert.deepEqual(
      comparableSnapshotArtifact(workerBranchSnapshot),
      comparableSnapshotArtifact(compatibilityImportedSignals.history().branch_snapshot(0)),
    );
    assert.deepEqual(
      comparableBranchStateProof(workerSignals.history().branch_state_proof(0)),
      comparableBranchStateProof(compatibilityImportedSignals.history().branch_state_proof(0)),
    );
    assert.deepEqual(
      comparableReplayParityProof(workerSignals.history().replay_parity_proof(0, 0)),
      comparableReplayParityProof(compatibilityImportedSignals.history().replay_parity_proof(0, 0)),
    );
    const replayArtifactInput = {
      proofSchemaVersion: compatibilityImportedSignals.adapters().runtimeProofReport().proofSchemaVersion,
      registryBundleDigest: compatibilityImportedSignals.adapters().runtimeProofReport().registryBundleDigest,
      loweredStrategyBundleDigest: null,
      mergePlanDigest: null,
      mergeResultDigest: null,
      lineageDigest: null,
      branchStateDigest: compatibilityImportedSignals.history().branch_state_proof(0).stateDigest,
    };
    assert.deepEqual(
      workerSignals.history().replay_artifact_proof(replayArtifactInput, 0),
      compatibilityImportedSignals.history().replay_artifact_proof(replayArtifactInput, 0),
    );
    assert.throws(
      () => workerSignals.history().create_branch("feature"),
      /WorkerFirstHistoryUnavailable/,
    );

    assert.deepEqual(
      workerSignals.adapters().exportDefinitions(),
      compatibilityImportedSignals.adapters().exportDefinitions(),
    );
    assert.deepEqual(
      workerSignals.adapters().exportRuntimeEnvelope().definitions,
      compatibilityImportedSignals.adapters().exportDefinitions(),
    );
    assert.ok(workerSignals.adapters().exportRuntimeEnvelope().snapshot);
    assert.equal(
      typeof workerSignals.adapters().exportRuntimeEnvelope().runtimeEnvelopeRestoreToken,
      "string",
    );
    assert.equal(
      typeof workerSignals.adapters().exportRuntimeEnvelope().runtimeEnvelopePortableWire,
      "string",
    );
    assert.equal(
      workerSignals.adapters().exportRuntimeEnvelope().runtimeEnvelopeRestoreMode,
      "SameRuntimeExact",
    );
    assert.deepEqual(
      workerSignals.adapters().runtimeProofReport(),
      compatibilityImportedSignals.adapters().runtimeProofReport(),
    );
    assert.deepEqual(
      workerSignals.adapters().hostCapabilityTransportReport(),
      compatibilityImportedSignals.adapters().hostCapabilityTransportReport(
        compatibilityImportedSignals.adapters().exportRuntimeEnvelope(),
      ),
    );
    assert.deepEqual(
      comparableGraphSummary(workerSignals.specialist().graphSummary()),
      comparableGraphSummary(compatibilityImportedSignals.specialist().graphSummary()),
    );
    assert.deepEqual(
      comparableGraphSummary(workerSignals.specialist().graph_summary()),
      comparableGraphSummary(compatibilityImportedSignals.specialist().graph_summary()),
    );
    assert.deepEqual(
      workerSignals.specialist().readVersions([outputId]),
      compatibilityImportedSignals.specialist().readVersions([outputId]),
    );
    assert.deepEqual(
      workerSignals.specialist().read_versions([outputId]),
      compatibilityImportedSignals.specialist().read_versions([outputId]),
    );
    assert.throws(
      () => workerSignals.specialist().evaluateDirty(),
      /WorkerFirstSpecialistUnavailable/,
    );
    assert.throws(
      () => workerSignals.specialist().evaluate_dirty(),
      /WorkerFirstSpecialistUnavailable/,
    );
    assert.throws(
      () => workerSignals.diagnostics().why("not-an-active-id"),
      /active imported graph/,
    );
    assert.throws(
      () => workerSignals.read("not-an-active-id"),
      /currently available worker-first signal/,
    );
    assert.throws(
      () => workerSignals.specialist().readVersions(["not-an-active-id"]),
      /active imported graph/,
    );

    const notices = [];
    let effectCount = 0;
    const watchHandle = workerSignals.watch(outputId, (notice) => {
      notices.push(notice);
    });
    const effectHandle = workerSignals.effect(importedGraph.output("doubled"), () => {
      effectCount += 1;
    });

    graph.writeInput("count", 11);
    const changedSnapshot = graph.exportSnapshot();
    const reimportedGraph = workerSignals.importGraph(definition, changedSnapshot);
    await reimportedGraph.ready();
    assert.equal(workerSignals.read(outputId), 22);
    assert.equal(notices.length, 1);
    assert.equal(notices[0].signalId, outputId);
    assert.equal(notices[0].meaningfulChange, true);
    assert.equal(effectCount, 1);
    assert.equal(workerSignals.nuke(watchHandle), true);
    assert.equal(workerSignals.nuke(watchHandle), false);
    assert.equal(workerSignals.nuke(effectHandle), true);

    const unchangedGraph = workerSignals.importGraph(definition, changedSnapshot);
    await unchangedGraph.ready();
    assert.equal(notices.length, 1);
    assert.equal(effectCount, 1);

    await unchangedGraph.terminate();
    await reimportedGraph.terminate();
    await importedGraph.terminate();
    workerSignals.free();
  } finally {
    compatibilityImportedSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

test("default worker-first root adapters restore runtime envelopes, preserve truth on denied import, and invalidate the active imported graph on admitted import", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const { createSignals, cleanup } = await loadSignalsModule({ rawSurface: "real" });

  const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstRootAdapterMutation", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:root:adapter:doubled", {
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
  const baselineSnapshot = graph.exportSnapshot();
  graph.writeInput("count", 9);
  const changedSnapshot = graph.exportSnapshot();
  const outputId = graph.output("doubled").id;

  const callbackSignals = await createSignals({ deployment: "mainThreadCompatibility" });
  const callbackCount = callbackSignals.input(5);
  callbackSignals.scope("workerFirstRootAdapterDenied").computedSpec("callbackBacked", {
    compute: () => callbackCount.value() + 1,
  });
  const deniedArtifact = callbackSignals.adapters().exportRuntimeEnvelope();

  try {
    const workerSignals = await createSignals();
    const importedGraph = workerSignals.importGraph(definition, baselineSnapshot);
    await importedGraph.ready();

    assert.equal(workerSignals.read(outputId), 4);

    const deniedPortableImport = await workerSignals.adapters().replaceRuntimeEnvelope(deniedArtifact);
    assert.equal(deniedPortableImport.importOutcome, "Denied");
    assert.equal(workerSignals.read(outputId), 4);
    assert.equal(importedGraph.read().doubled, 4);

    const portableImport = await workerSignals.adapters().replaceRuntimeEnvelope(changedSnapshot.runtimeEnvelope);
    assert.equal(portableImport.importOutcome, "Admitted");
    assert.throws(
      () => importedGraph.read(),
      /replaced the active imported graph runtime/,
    );
    assert.throws(
      () => workerSignals.read(outputId),
      /currently available worker-first signal/,
    );

    const reimportedAfterPortable = workerSignals.importGraph(definition, changedSnapshot);
    await reimportedAfterPortable.ready();
    assert.equal(workerSignals.read(outputId), 18);
    const workerChangedArtifact = workerSignals.adapters().exportRuntimeEnvelope();

    const exactImport = await workerSignals.adapters().restoreExactRuntimeEnvelope(workerChangedArtifact);
    assert.equal(exactImport.importOutcome, "AdmittedExact");
    assert.throws(
      () => reimportedAfterPortable.read(),
      /replaced the active imported graph runtime/,
    );
    assert.throws(
      () => workerSignals.diagnostics().summaryNow(),
      /active imported graph/,
    );

    const reimportedAfterExact = workerSignals.importGraph(definition, changedSnapshot);
    await reimportedAfterExact.ready();
    assert.equal(workerSignals.read(outputId), 18);

    const portableBaselineImport = await workerSignals.adapters().replaceRuntimeEnvelope(baselineSnapshot.runtimeEnvelope);
    assert.equal(portableBaselineImport.importOutcome, "Admitted");
    assert.throws(
      () => reimportedAfterExact.read(),
      /replaced the active imported graph runtime/,
    );

    const reimportedAfterBaseline = workerSignals.importGraph(definition, baselineSnapshot);
    await reimportedAfterBaseline.ready();
    assert.equal(workerSignals.read(outputId), 4);

    await reimportedAfterBaseline.terminate();
    workerSignals.free();
  } finally {
    callbackSignals.free();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

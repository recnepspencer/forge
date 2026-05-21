import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

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

function comparableBranchReplay(summary) {
  return {
    frames: summary.frames.filter(
      (frame) => frame.kind !== "SnapshotCaptured" || frame.snapshotId === 0,
    ),
  };
}

test("worker-first history facade preserves replay, snapshot, and branch read truth while keeping branch mutation operations explicit", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const mod = await loadSignalsModule({ rawSurface: "real" });
  const { createSignals, importProductModule, cleanup } = mod;
  const { createWorkerRuntimeBridge } = await importProductModule(
    "entrypoint/bridge/worker_runtime_bridge.js",
  );
  const { createWorkerFirstHistoryFacade } = await importProductModule(
    "entrypoint/worker_first_history.js",
  );

  const compatibilitySignals = await createSignals({
    deployment: "mainThreadCompatibility",
  });
  const count = compatibilitySignals.input(2, { debugName: "count" });
  const graph = compatibilitySignals.graph("workerFirstHistory", {
    inputs: { count },
    outputs: {
      doubled: compatibilitySignals.computedSpec("worker:first:history:doubled", {
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
  graph.writeInput("count", 5);
  const outputId = graph.output("doubled").id;
  const bridge = createWorkerRuntimeBridge();

  try {
    await bridge.bootstrapRecord();
    await bridge.workerRuntimeShellLock();
    await bridge.publishPortableGraph({
      ...compatibilitySignals.adapters().exportDefinitions(),
      outputIds: [outputId],
    });
    await bridge.applyTransaction([
      {
        kind: "set",
        id: count.id,
        value: 5,
      },
    ]);
    const history = createWorkerFirstHistoryFacade({ bridge });

    assert.deepEqual(
      await history.replay_for(outputId),
      compatibilitySignals.history().replay_for(outputId),
    );
    assert.deepEqual(
      await history.lineage_for(outputId),
      compatibilitySignals.history().lineage_for(outputId),
    );
    assert.deepEqual(
      await history.recentHistory(),
      compatibilitySignals.diagnostics().recentHistory(),
    );
    assert.deepEqual(
      comparableSnapshotEnvelope(await history.snapshot()),
      comparableSnapshotEnvelope(compatibilitySignals.history().snapshot()),
    );
    assert.deepEqual(
      comparableRuntimeBranch(await history.current_branch()),
      comparableRuntimeBranch(compatibilitySignals.history().current_branch()),
    );
    assert.deepEqual(
      (await history.branches()).map(comparableRuntimeBranch),
      compatibilitySignals.history().branches().map(comparableRuntimeBranch),
    );
    assert.deepEqual(
      comparableBranchReplay(await history.replay_for_branch(0)),
      comparableBranchReplay(compatibilitySignals.history().replay_for_branch(0)),
    );
    const workerBranchEnvelope = await history.branch_snapshot_envelope(0);
    const workerBranchSnapshot = await history.branch_snapshot(0);
    assert.equal(typeof await history.branch_snapshot_id(0), "number");
    assert.equal(
      workerBranchSnapshot.snapshotRestoreToken.length > 0,
      true,
    );
    assert.deepEqual(
      comparableSnapshotEnvelope(workerBranchEnvelope),
      comparableSnapshotEnvelope(compatibilitySignals.history().branch_snapshot_envelope(0)),
    );
    assert.deepEqual(
      comparableSnapshotArtifact(workerBranchSnapshot),
      comparableSnapshotArtifact(compatibilitySignals.history().branch_snapshot(0)),
    );
    assert.deepEqual(
      comparableBranchStateProof(await history.branch_state_proof(0)),
      comparableBranchStateProof(compatibilitySignals.history().branch_state_proof(0)),
    );
    assert.deepEqual(
      comparableReplayParityProof(await history.replay_parity_proof(0, 0)),
      comparableReplayParityProof(compatibilitySignals.history().replay_parity_proof(0, 0)),
    );
    const replayArtifactInput = {
      proofSchemaVersion: compatibilitySignals.adapters().runtimeProofReport().proofSchemaVersion,
      registryBundleDigest: compatibilitySignals.adapters().runtimeProofReport().registryBundleDigest,
      loweredStrategyBundleDigest: null,
      mergePlanDigest: null,
      mergeResultDigest: null,
      lineageDigest: null,
      branchStateDigest: compatibilitySignals.history().branch_state_proof(0).stateDigest,
    };
    assert.deepEqual(
      await history.replay_artifact_proof(replayArtifactInput, 0),
      compatibilitySignals.history().replay_artifact_proof(replayArtifactInput, 0),
    );
    assert.throws(
      () => history.restore_exact_snapshot({}),
      /mainThreadCompatibility/,
    );
    assert.throws(() => history.create_branch("feature"), /worker-first history facade/);
  } finally {
    await bridge.terminate();
    compatibilitySignals.free();
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

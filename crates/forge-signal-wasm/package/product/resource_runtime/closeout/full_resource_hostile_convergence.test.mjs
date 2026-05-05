import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";
import {
  createHistoryOverrides,
  createHostileApp,
  runHostileScript,
} from "./full_resource_hostile_scenario.mjs";
import {
  normalizeForProof,
  projectAuthoringConvergenceDigest,
  projectConvergenceDigest,
  projectHostileAppPackage,
  projectReplayReconstructionDigest,
} from "./resource_verification_package_helpers.mjs";

test("canonical verification packages keep the mixed hostile resource app convergent across native, external, restore, and replay availability modes", async () => {
  const mod = await loadResourceModule();
  try {
    const fullRestoreState = { active: false };
    const fullLines = createHostileApp(
      mod,
      createFakeSignalNamespace("root", createHistoryOverrides(fullRestoreState, "full")),
      fullRestoreState,
    );
    const deliveryResults = await runHostileScript(fullLines, mod);
    const fullForward = projectHostileAppPackage(fullLines);
    const restoreResults = [
      fullLines.detail.history().restoreExact(),
      fullLines.retryDetail.history().restoreExact(),
      fullLines.transferDetail.history().restoreExact(),
      fullLines.nativeCollection.history().restoreExact(),
      fullLines.externalCollection.history().restoreExact(),
      fullLines.paged.history().restoreExact(),
    ];
    const fullRestored = projectHostileAppPackage(fullLines);

    const retainedRestoreState = { active: false };
    const retainedLines = createHostileApp(
      mod,
      createFakeSignalNamespace("retained", createHistoryOverrides(retainedRestoreState, "retained")),
      retainedRestoreState,
    );
    await runHostileScript(retainedLines, mod);
    const retainedForward = projectHostileAppPackage(retainedLines);

    const replayState = { active: false, replaySignalIds: [] };
    const replayLines = createHostileApp(
      mod,
      createFakeSignalNamespace("replay", createHistoryOverrides(replayState, "full")),
      replayState,
    );
    await runHostileScript(replayLines, mod);
    const replayForward = projectHostileAppPackage(replayLines);
    const replayResults = [
      replayLines.detail.history().replayExact(),
      replayLines.retryDetail.history().replayExact(),
      replayLines.transferDetail.history().replayExact(),
      replayLines.nativeCollection.history().replayExact(),
      replayLines.externalCollection.history().replayExact(),
      replayLines.paged.history().replayExact(),
    ];
    const replayed = projectHostileAppPackage(replayLines);

    assert.deepEqual(deliveryResults, {
      duplicateNative: {
        kind: "duplicateIgnored",
        packetId: "pkt-native-b3",
        deliveryKind: "patch",
      },
      staleExternal: {
        kind: "basisRejected",
        packetId: "pkt-stale",
        expectedBasisId: "basis-1",
        actualBasisId: "basis-2",
      },
    });
    assert.deepEqual(
      normalizeForProof(fullForward.externalCollection.externalCompatibility),
      {
        kind: "externalDefinition",
        version: "forge-resource-external-v1",
        definitionId: "suite0-external-collection",
        requestContract: "native-v1",
        reconciliationContract: "collection-v1",
      },
    );
    assert.equal(fullForward.nativeCollection.externalCompatibility.kind, "native");
    assert.deepEqual(
      projectAuthoringConvergenceDigest(fullForward.nativeCollection),
      projectAuthoringConvergenceDigest(fullForward.externalCollection),
    );
    assert.deepEqual(restoreResults, [
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: "basis-3",
        basisAdvanceCount: 2,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: "basis-3",
        basisAdvanceCount: 2,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: 201,
        snapshotId: 301,
        basisCurrentId: "basis-1",
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
    ]);
    assert.deepEqual(
      projectAuthoringConvergenceDigest(fullRestored.nativeCollection),
      projectAuthoringConvergenceDigest(fullRestored.externalCollection),
    );
    assert.deepEqual(
      replayResults.map((result) => ({
        kind: result.kind,
        mode: result.kind === "replayed" ? result.mode : null,
        reloadKind: result.kind === "replayed" ? result.reloadStatus.kind : null,
        reloadOperation:
          result.kind === "replayed" ? result.reloadStatus.operation : null,
      })),
      [
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
        {
          kind: "replayed",
          mode: "SameRuntimeSignalExact",
          reloadKind: "fulfilled",
          reloadOperation: "replay",
        },
      ],
    );
    assert.equal(replayState.replaySignalIds.length, 6);
    assert.deepEqual(
      projectReplayReconstructionDigest(replayForward),
      projectReplayReconstructionDigest(replayed),
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).detail,
      projectConvergenceDigest(retainedForward).detail,
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).retryDetail,
      projectConvergenceDigest(retainedForward).retryDetail,
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).transferDetail,
      projectConvergenceDigest(retainedForward).transferDetail,
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).nativeCollection,
      projectConvergenceDigest(retainedForward).nativeCollection,
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).externalCollection,
      projectConvergenceDigest(retainedForward).externalCollection,
    );
    assert.deepEqual(
      projectConvergenceDigest(fullForward).paged,
      projectConvergenceDigest(retainedForward).paged,
    );
    assert.deepEqual(
      fullForward.retryDetail.processing,
      {
        kind: "ready",
        completionKind: "none",
        jobId: null,
        message: null,
      },
    );
    assert.deepEqual(fullForward.transferDetail.processing, {
      kind: "ready",
      completionKind: "poll",
      jobId: null,
      message: null,
    });
    assert.deepEqual(fullForward.transferDetail.upload, {
      kind: "ready",
      transportKind: "signed",
      uploadId: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
      hasDescriptor: false,
    });
    assert.equal(fullForward.transferDetail.lifecycle.timeoutCount, 1);
    assert.equal(fullForward.transferDetail.lifecycle.supersessionCount, 1);
    assert.equal(fullForward.retryDetail.lifecycle.retryAttemptCount, 1);
    assert.equal(fullForward.retryDetail.lifecycle.rejectionCount, 0);
    for (const replaySensitivePackage of Object.values(retainedForward)) {
      assert.equal(
        replaySensitivePackage.historyReplayRestore.availability.replay.kind,
        "unavailable",
      );
      assert.match(
        replaySensitivePackage.typedDenials.replay.detail,
        /retained replay history/,
      );
      assert.equal(
        replaySensitivePackage.historyReplayRestore.availability.replayExact.kind,
        "unavailable",
      );
      assert.match(
        replaySensitivePackage.typedDenials.replayExact.detail,
        /retained replay execution/,
      );
    }
    for (const fullReplayPackage of Object.values(fullForward)) {
      assert.equal(
        fullReplayPackage.historyReplayRestore.availability.replay.kind,
        "available",
      );
      assert.equal(
        fullReplayPackage.historyReplayRestore.availability.replayExact.kind,
        "available",
      );
      assert.equal(fullReplayPackage.typedDenials.replay, null);
      assert.equal(fullReplayPackage.typedDenials.replayExact, null);
    }
    assert.deepEqual(fullRestored.transferDetail.committedValue, {
      id: "receipt-1",
      status: "restored-ready",
    });
  } finally {
    await mod.cleanup();
  }
});

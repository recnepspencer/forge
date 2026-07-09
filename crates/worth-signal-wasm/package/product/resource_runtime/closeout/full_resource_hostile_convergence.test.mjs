import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceNamespace,
  createRealResourceRuntime,
  installHistoryOverrides,
} from "../runtime_fixture/real_resource_signals.mjs";
import {
  normalizeForProof,
  projectAuthoringConvergenceDigest,
  projectHostileAppPackage,
} from "./resource_verification_package_helpers.mjs";
import { projectBehavioralConvergenceDigest } from "./resource_behavioral_convergence_helpers.mjs";
import {
  createHostileApp,
  freeHostileApp,
  runHostileScript,
} from "./full_resource_hostile_scenario.mjs";

function projectCloseoutSnapshot(appPackage) {
  return {
    behavioral: projectBehavioralConvergenceDigest(appPackage),
    externalCompatibility: normalizeForProof(
      appPackage.externalCollection.externalCompatibility,
    ),
    nativeAuthoring: projectAuthoringConvergenceDigest(appPackage.nativeCollection),
    externalAuthoring: projectAuthoringConvergenceDigest(
      appPackage.externalCollection,
    ),
    transferProcessing: normalizeForProof(appPackage.transferDetail.processing),
    transferUpload: normalizeForProof(appPackage.transferDetail.upload),
    transferLifecycle: {
      timeoutCount: appPackage.transferDetail.lifecycle.timeoutCount,
      supersessionCount: appPackage.transferDetail.lifecycle.supersessionCount,
    },
    retryLifecycle: {
      retryAttemptCount: appPackage.retryDetail.lifecycle.retryAttemptCount,
      rejectionCount: appPackage.retryDetail.lifecycle.rejectionCount,
    },
    availability: Object.fromEntries(
      Object.entries(appPackage).map(([key, pkg]) => [
        key,
        {
          replay: normalizeForProof(pkg.historyReplayRestore.availability.replay),
          replayExact: normalizeForProof(
            pkg.historyReplayRestore.availability.replayExact,
          ),
          branch: normalizeForProof(pkg.historyReplayRestore.availability.branch),
          restoreExact: normalizeForProof(
            pkg.historyReplayRestore.availability.restoreExact,
          ),
          typedDenials: {
            replay: normalizeForProof(pkg.typedDenials.replay),
            replayExact: normalizeForProof(pkg.typedDenials.replayExact),
            branch: normalizeForProof(pkg.typedDenials.branch),
            restoreExact: normalizeForProof(pkg.typedDenials.restoreExact),
          },
        },
      ]),
    ),
    transferCommittedValue: normalizeForProof(appPackage.transferDetail.committedValue),
  };
}

async function settleRuntime() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test(
  "canonical verification packages keep the hostile resource app convergent across native, external, restore, unsupported replay, and retained branch modes",
  { concurrency: false },
  async () => {
  const runtime = await createRealResourceRuntime();
  let fullLines = null;
  let phase = "setup";
  try {
    const restoreState = { active: false };
    const branch = createBranchHead(runtime.signals, "suite-0");
    await settleRuntime();
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(BigInt(branch.id)),
    );
    const uninstallRestoreHook = installHistoryOverrides(runtime.signals, {
      restore_branch_snapshot_by_id(history, branchId, targetSnapshotId) {
        restoreState.active = true;
        return history.restore_branch_snapshot_by_id(branchId, targetSnapshotId);
      },
    });
    const resource = createRealResourceNamespace(
      runtime.resourceMod,
      runtime.signals,
    );
    fullLines = createHostileApp(runtime.resourceMod, resource, restoreState);
    phase = "run hostile script";
    const deliveryResults = await runHostileScript(fullLines, runtime.resourceMod);
    phase = "forward snapshot";
    await settleRuntime();
    const fullForward = projectCloseoutSnapshot(projectHostileAppPackage(fullLines));
    phase = "restore results";
    const restoreResults = [
      fullLines.detail.history().restoreExact(),
      fullLines.retryDetail.history().restoreExact(),
      fullLines.transferDetail.history().restoreExact(),
      fullLines.nativeCollection.history().restoreExact(),
      fullLines.externalCollection.history().restoreExact(),
      fullLines.paged.history().restoreExact(),
    ];
    phase = "restored snapshot";
    await settleRuntime();
    const fullRestored = projectCloseoutSnapshot(projectHostileAppPackage(fullLines));
    phase = "replay results";
    const replayResults = [
      fullLines.detail.history().replayExact(),
      fullLines.retryDetail.history().replayExact(),
      fullLines.transferDetail.history().replayExact(),
      fullLines.nativeCollection.history().replayExact(),
      fullLines.externalCollection.history().replayExact(),
      fullLines.paged.history().replayExact(),
    ];
    phase = "replay stable snapshot";
    await settleRuntime();
    const replayStable = projectCloseoutSnapshot(projectHostileAppPackage(fullLines));
    uninstallRestoreHook();

    const retainedRuntime = await createRealResourceRuntime();
    let retainedForward;
    try {
      const retainedRestoreState = { active: false };
      createBranchHead(retainedRuntime.signals, "retained-suite-0");
      await settleRuntime();
      const uninstallRetained = installHistoryOverrides(retainedRuntime.signals, {
        current_branch() {
          throw new Error("retained branch snapshots are unavailable");
        },
      });
      const retainedResource = createRealResourceNamespace(
        retainedRuntime.resourceMod,
        retainedRuntime.signals,
      );
      const retainedLines = createHostileApp(
        retainedRuntime.resourceMod,
        retainedResource,
        retainedRestoreState,
      );
      phase = "retained script";
      await runHostileScript(retainedLines, retainedRuntime.resourceMod);
      phase = "retained snapshot";
      await settleRuntime();
      retainedForward = projectCloseoutSnapshot(
        projectHostileAppPackage(retainedLines),
      );
      freeHostileApp(retainedLines);
      uninstallRetained();
      await settleRuntime();
    } finally {
      await retainedRuntime.cleanup();
    }

    phase = "assertions";
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
      fullForward.externalCompatibility,
      {
        kind: "externalDefinition",
        version: "worth-resource-external-v1",
        definitionId: "suite0-external-collection",
        requestContract: "native-v1",
        reconciliationContract: "collection-v1",
      },
    );
    assert.deepEqual(fullForward.nativeAuthoring, fullForward.externalAuthoring);
    assert.deepEqual(restoreResults, [
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: null,
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: "basis-3",
        basisAdvanceCount: 2,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: "basis-3",
        basisAdvanceCount: 2,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
      {
        kind: "restored",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        basisCurrentId: "basis-1",
        basisAdvanceCount: 0,
        reloadStatus: { kind: "fulfilled", operation: "restore" },
      },
    ]);
    assert.deepEqual(fullRestored.nativeAuthoring, fullRestored.externalAuthoring);
    assert.deepEqual(
      replayResults,
      [
        fullForward.behavioral.detail,
        fullForward.behavioral.retryDetail,
        fullForward.behavioral.transferDetail,
        fullForward.behavioral.nativeCollection,
        fullForward.behavioral.externalCollection,
        fullForward.behavioral.paged,
      ].map((pkg, index) => ({
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
        basisCurrentId: [
          null,
          null,
          null,
          "basis-3",
          "basis-3",
          "basis-1",
        ][index],
        basisAdvanceCount: [0, 0, 0, 2, 2, 0][index],
      })),
    );
    assert.deepEqual(replayStable.behavioral, fullRestored.behavioral);
    assert.deepEqual(fullForward.behavioral, retainedForward.behavioral);
    assert.deepEqual(fullForward.transferProcessing, {
      kind: "ready",
      completionKind: "poll",
      jobId: null,
      message: null,
    });
    assert.deepEqual(fullForward.transferUpload, {
      kind: "ready",
      transportKind: "signed",
      uploadId: null,
      finalizeRequired: false,
      awaitingProcessing: false,
      message: null,
      hasDescriptor: false,
    });
    assert.equal(fullForward.transferLifecycle.timeoutCount, 1);
    assert.equal(fullForward.transferLifecycle.supersessionCount, 1);
    assert.equal(fullForward.retryLifecycle.retryAttemptCount, 1);
    assert.equal(fullForward.retryLifecycle.rejectionCount, 0);
    for (const retainedPackage of Object.values(retainedForward.availability)) {
      assert.equal(retainedPackage.replay.kind, "available");
      assert.equal(retainedPackage.replayExact.kind, "unavailable");
      assert.equal(retainedPackage.typedDenials.replay, null);
      assert.deepEqual(retainedPackage.typedDenials.replayExact, {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      });
      assert.equal(retainedPackage.branch.kind, "unavailable");
      assert.match(retainedPackage.typedDenials.branch.detail, /retained branch snapshots are unavailable/);
      assert.equal(retainedPackage.restoreExact.kind, "unavailable");
      assert.match(retainedPackage.typedDenials.restoreExact.detail, /retained branch snapshots are unavailable/);
    }
    for (const fullPackage of Object.values(fullForward.availability)) {
      assert.equal(fullPackage.replay.kind, "available");
      assert.equal(fullPackage.replayExact.kind, "unavailable");
      assert.equal(fullPackage.typedDenials.replay, null);
      assert.deepEqual(fullPackage.typedDenials.replayExact, {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      });
      assert.equal(fullPackage.branch.kind, "available");
      assert.equal(fullPackage.restoreExact.kind, "available");
    }
    assert.deepEqual(fullRestored.transferCommittedValue, {
      id: "receipt-1",
      status: "restored-ready",
    });
  } catch (error) {
    throw new Error(`closeout phase failed: ${phase}`, { cause: error });
  } finally {
    if (fullLines !== null) {
      freeHostileApp(fullLines);
      await settleRuntime();
    }
    await runtime.cleanup();
  }
  },
);

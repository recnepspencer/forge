import assert from "node:assert/strict";
import test from "node:test";

import { projectBasisProof } from "../delivery/delivery_basis_history_proof_helpers.mjs";
import {
  createBranchHead,
  createRealResourceCollection,
  createRealResourceDetail,
  createRealResourceSignals,
  installHistoryOverrides,
} from "../runtime_fixture/real_resource_signals.mjs";

function createRestoreCollectionLine(mod, signals) {
  return createRealResourceCollection(mod, signals, {
    load: (_params, request) => ({
      items: [{ id: "demo:1", title: `Load:${request.context.basisId}` }],
    }),
  }).line({ workspaceId: "demo" });
}

test("line history restoreExact uses the real runtime branch snapshot target and preserves basis evidence", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "restore-branch");
    const line = createRestoreCollectionLine(runtime.mod, runtime.signals);

    line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );

    const before = projectBasisProof(line);
    const result = line.history().restoreExact();
    const after = projectBasisProof(line);

    assert.deepEqual(result, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(after.requestBasisId, before.requestBasisId);
    assert.deepEqual(after.diagnosticsBasis, before.diagnosticsBasis);
    assert.deepEqual(after.summaryBasis, before.summaryBasis);
    assert.deepEqual(after.historyBasis, before.historyBasis);
    assert.deepEqual(line.value(), {
      items: [{ id: "demo:1", title: "Load:basis-3" }],
    });
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.deepEqual(line.history().availability.restoreExact, {
      kind: "available",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("line history restoreExact can use exact restore with a branch snapshot artifact when by-id restore is absent", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "exact-only");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const calls = [];
    const uninstall = installHistoryOverrides(runtime.signals, {
      restore_branch_snapshot_by_id: undefined,
      branch_snapshot(history, branchId) {
        calls.push(["snapshot", branchId]);
        return history.branch_snapshot(branchId);
      },
      restore_exact_branch_snapshot(history, branchId, snapshotValue) {
        calls.push(["restore", branchId, snapshotValue.snapshotRestoreToken]);
        return history.restore_exact_branch_snapshot(branchId, snapshotValue);
      },
    });
    const line = createRealResourceDetail(runtime.mod, runtime.signals, {
      load: ({ id }) => ({ id }),
    }).line({ id: "plain" });

    const result = line.history().restoreExact();

    assert.deepEqual(result, {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
      basisCurrentId: null,
      basisAdvanceCount: 0,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.deepEqual(calls, [
      ["snapshot", BigInt(branch.id)],
      ["restore", BigInt(branch.id), "snapshot:1"],
    ]);
    uninstall();
  } finally {
    await runtime.cleanup();
  }
});

test("line history restoreExact returns explicit unsupportedByRuntime and runtimeRejected artifacts without rewriting basis proof", async () => {
  const unsupportedRuntime = await createRealResourceSignals();
  try {
    createBranchHead(unsupportedRuntime.signals, "missing-restore-support");
    const uninstall = installHistoryOverrides(unsupportedRuntime.signals, {
      restore_branch_snapshot_by_id: undefined,
      restore_exact_branch_snapshot: undefined,
      branch_snapshot: undefined,
    });
    const unsupported = createRealResourceDetail(
      unsupportedRuntime.mod,
      unsupportedRuntime.signals,
      {
        load: ({ id }) => ({ id }),
      },
    ).line({ id: "plain" });

    assert.deepEqual(unsupported.history().availability.restoreExact, {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
    });
    assert.deepEqual(unsupported.history().restoreExact(), {
      kind: "unavailable",
      reason: "unsupportedByRuntime",
      detail:
        "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
      basisCurrentId: null,
      basisAdvanceCount: 0,
    });
    uninstall();
  } finally {
    await unsupportedRuntime.cleanup();
  }

  const rejectedRuntime = await createRealResourceSignals();
  try {
    createBranchHead(rejectedRuntime.signals, "rejecting-restore");
    const uninstall = installHistoryOverrides(rejectedRuntime.signals, {
      restore_branch_snapshot_by_id() {
        throw new Error("snapshot 44 is no longer retained");
      },
    });
    const rejected = createRestoreCollectionLine(
      rejectedRuntime.mod,
      rejectedRuntime.signals,
    );
    rejected.deliver(
      rejectedRuntime.mod.resourceDelivery.replace({
        packetId: "pkt-basis-2",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: {
          items: [{ id: "demo:1", title: "Delivered Basis 2" }],
        },
      }),
    );
    rejected.deliver(
      rejectedRuntime.mod.resourceDelivery.patch({
        packetId: "pkt-basis-3",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        patch: rejectedRuntime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Basis 3",
        }),
      }),
    );

    const before = projectBasisProof(rejected);
    const result = rejected.history().restoreExact();
    const after = projectBasisProof(rejected);

    assert.deepEqual(result, {
      kind: "unavailable",
      reason: "runtimeRejected",
      detail:
        "resource line exact branch restore is unavailable because restore execution failed: snapshot 44 is no longer retained",
      basisCurrentId: "basis-3",
      basisAdvanceCount: 2,
    });
    assert.deepEqual(after.requestBasisId, before.requestBasisId);
    assert.deepEqual(after.diagnosticsBasis, before.diagnosticsBasis);
    assert.deepEqual(after.summaryBasis, before.summaryBasis);
    assert.deepEqual(after.historyBasis, before.historyBasis);
    assert.deepEqual(after.lifecycleBasis, before.lifecycleBasis);
    uninstall();
  } finally {
    await rejectedRuntime.cleanup();
  }
});

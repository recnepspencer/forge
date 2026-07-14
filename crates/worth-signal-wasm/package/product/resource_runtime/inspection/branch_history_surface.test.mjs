import assert from "node:assert/strict";
import test from "node:test";

import {
  createBranchHead,
  createRealResourceDetail,
  createRealResourceSignals,
} from "../runtime_fixture/real_resource_signals.mjs";

test("resource line history exposes the real current branch and derives exact restore availability from the runtime snapshot target", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "feature/resources");
    const detail = createRealResourceDetail(runtime.mod, runtime.signals, {
      load: ({ id }) => ({ id }),
    });
    const line = detail.line({ id: "branchy" });
    line.value();
    const history = line.history();
    assert.deepEqual(
      {
        id: history.branch?.id,
        name: history.branch?.name,
        parentBranchId: history.branch?.parentBranchId,
      },
      {
        id: branch.id,
        name: "feature/resources",
        parentBranchId: 0,
      },
    );
    assert.deepEqual(history.availability, {
      replay: { kind: "available" },
      replayExact: {
        kind: "unavailable",
        reason: "unsupportedByRuntime",
        detail:
          "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
      },
      lineage: { kind: "available" },
      branch: { kind: "available" },
      restoreExact: history.availability.restoreExact,
    });
    assert.deepEqual(history.availability.restoreExact.kind, "available");
    assert.deepEqual(history.availability.restoreExact.mode, "SameRuntimeBranchExact");
    assert.deepEqual(history.availability.restoreExact.branchId, branch.id);
    assert.equal(
      Number.isInteger(history.availability.restoreExact.snapshotId),
      true,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource line history keeps exact restore available on a branch whose current_branch() artifact omits head_snapshot_id", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = runtime.signals.history().create_branch("empty-branch");
    runtime.signals.history().switch_branch(branch.id);
    const detail = createRealResourceDetail(runtime.mod, runtime.signals, {
      load: ({ id }) => ({ id }),
    });
    const line = detail.line({ id: "empty" });
    line.value();
    const history = line.history();

    assert.deepEqual(history.branch, {
      id: branch.id,
      name: "empty-branch",
      parentBranchId: 0,
      headSnapshotId: 0,
    });
    assert.equal(history.availability.restoreExact.kind, "available");
    assert.equal(history.availability.restoreExact.mode, "SameRuntimeBranchExact");
    assert.equal(history.availability.restoreExact.branchId, branch.id);
    assert.equal(
      Number.isInteger(history.availability.restoreExact.snapshotId),
      true,
    );
    assert.deepEqual(history.restoreExact(), {
      kind: "restored",
      mode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId: history.availability.restoreExact.snapshotId,
      basisCurrentId: null,
      basisAdvanceCount: 0,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
  } finally {
    await runtime.cleanup();
  }
});

import assert from "node:assert/strict";
import test from "node:test";

import { loadResourceModule } from "../module_loading/load_resource_module.mjs";
import { createFakeSignalNamespace } from "../runtime_fixture/fake_signal_namespace.mjs";

test("resource line history exposes current branch and exact restore availability", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 7n,
          name: "feature/resources",
          parent_branch_id: 3n,
          head_snapshot_id: 42n,
        };
      },
      branch_snapshot() {
        return Object.freeze({ snapshotRestoreToken: "branch-7-snapshot" });
      },
      restore_exact_branch_snapshot() {},
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const history = detail.line({ id: "branchy" }).history();

    assert.deepEqual(history.branch, {
      id: 7,
      name: "feature/resources",
      parentBranchId: 3,
      headSnapshotId: 42,
    });
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
      restoreExact: {
        kind: "available",
        mode: "SameRuntimeBranchExact",
        branchId: 7,
        snapshotId: 42,
      },
    });
  } finally {
    await mod.cleanup();
  }
});

test("resource line history reports explicit restore unavailability when branch snapshots do not exist", async () => {
  const mod = await loadResourceModule();
  try {
    const signalNamespace = createFakeSignalNamespace("root", {
      current_branch() {
        return {
          id: 9n,
          name: "empty-branch",
          parent_branch_id: null,
          head_snapshot_id: null,
        };
      },
      restore_exact_branch_snapshot() {},
    });
    const resource = mod.createResourceNamespace(signalNamespace, {});
    const detail = resource.detail({
      params: mod.resourceParams(),
      normalizeParams: ({ id }) => mod.resourceParamIdentity({ id }, id),
      load: ({ id }) => ({ id }),
    });

    const history = detail.line({ id: "empty" }).history();

    assert.deepEqual(history.branch, {
      id: 9,
      name: "empty-branch",
      parentBranchId: null,
      headSnapshotId: null,
    });
    assert.deepEqual(history.availability.restoreExact, {
      kind: "unavailable",
      reason: "branchHeadUnavailable",
      detail:
        "resource line exact branch restore is unavailable because branch 9 has no head snapshot",
    });
  } finally {
    await mod.cleanup();
  }
});

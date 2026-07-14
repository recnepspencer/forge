import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("branch lifecycle records selected current branch without resource-owned disposal", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "effect-branch-lifecycle");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.sensitive(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Speculative Branch Lifecycle",
      }),
    );

    const effect = line.diagnostics().lastEffect;
    assert.deepEqual(effect.branchLifecycle, {
      kind: "selectedExistingBranch",
      acquisition: "currentRuntimeBranch",
      creation: "notCreatedByResourceRuntime",
      reuse: "currentBranchReuse",
      ownership: "signalsRuntimeOwned",
      branchId: branch.id,
      snapshotId,
      restoreMode: "SameRuntimeBranchExact",
      disposal: {
        kind: "notOwnedByResourceRuntime",
        detail:
          "resource effect selected an existing Signals branch and must not dispose branch state it did not create",
      },
      leakDenial: {
        kind: "noResourceOwnedBranch",
        detail:
          "resource effect did not create package-local speculative branch state that could survive disposal",
      },
    });
    assert.equal(effect.counters.branchLifecycleBreadth, 1);
    assert.deepEqual(line.history().lifecycle.at(-1).lastEffect, effect);
  } finally {
    await runtime.cleanup();
  }
});

test("branch lifecycle denies leaks when speculation is unavailable before branch ownership", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch: undefined,
  });
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "No Speculation",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.branchLifecycle, {
      kind: "unavailable",
      creation: "deniedBeforeBranchCreation",
      reason: "unsupportedByRuntime",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime does not expose current_branch(...)",
      branchId: null,
      snapshotId: null,
      disposal: {
        kind: "notApplicable",
        detail:
          "resource effect did not acquire a speculative branch, so there is no branch disposal action",
      },
      leakDenial: {
        kind: "optimismDeniedBeforeResourceOwnedBranch",
        detail:
          "resource effect denied branch speculation before creating resource-owned speculative branch state",
      },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("branch lifecycle preserves selected branch identity for compact inverse rollback", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
    restore_exact_branch_snapshot: undefined,
    branch_snapshot: undefined,
  });
  try {
    const branch = createBranchHead(runtime.signals, "effect-branch-no-restore");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Restore Unavailable",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.branchLifecycle, {
      kind: "selectedExistingBranch",
      acquisition: "currentRuntimeBranch",
      creation: "notCreatedByResourceRuntime",
      reuse: "currentBranchReuse",
      ownership: "signalsRuntimeOwned",
      branchId: branch.id,
      snapshotId,
      restoreMode: null,
      disposal: {
        kind: "notOwnedByResourceRuntime",
        detail:
          "resource effect selected an existing Signals branch and must not dispose branch state it did not create",
      },
      leakDenial: {
        kind: "noResourceOwnedBranch",
        detail:
          "resource effect did not create package-local speculative branch state that could survive disposal",
      },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("committed-only lifecycle avoids branch proof reads and disposal claims", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch() {
      throw new Error("committed effects must not read branch lifecycle proof");
    },
  });
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.pessimistic(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Committed Lifecycle",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.branchLifecycle, {
      kind: "notApplicable",
      creation: "notApplicable",
      reason: "profileDisablesOptimism",
      detail:
        'resource effect profile "pessimistic" disables optimistic branch application',
      disposal: {
        kind: "notApplicable",
        detail:
          "committed-only resource effects do not acquire speculative branch state",
      },
      leakDenial: {
        kind: "notApplicable",
        detail:
          "committed-only resource effects do not create speculative branch state",
      },
    });
  } finally {
    await runtime.cleanup();
  }
});

function createEffectCollectionLine(runtime, options) {
  const { mod, resource } = runtime;
  const family = resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    effects: options.effects,
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (item) => item.title,
          write: (item, title) => ({ ...item, title: String(title) }),
        },
      }),
    }),
    load: () => ({
      items: [{ id: "demo:1", title: "Loaded" }],
    }),
  });
  return family.line({ workspaceId: "demo" });
}

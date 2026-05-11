import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("branch-speculative patches preserve rejected restore-target lookup detail", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch(history) {
      return {
        ...history.current_branch(),
        head_snapshot_id: null,
      };
    },
    branch_snapshot_id() {
      throw new Error("snapshot index was compacted");
    },
  });
  try {
    const branch = createBranchHead(runtime.signals, "effect-snapshot-rejected");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Committed After Snapshot Rejection",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.plan.branch, {
      kind: "optimisticUnavailable",
      profileName: "branchNative",
      optimism: "branchSpeculative",
      rollback: "branchRestoreOrInverse",
      reason: "runtimeRejected",
      detail:
        "resource effect branch speculation is unavailable because branch_snapshot_id(...) rejected restore-target lookup: snapshot index was compacted",
      branchId: branch.id,
      snapshotId: null,
      inverseAvailable: false,
      proofBreadth: 2,
    });
    assert.deepEqual(line.history().lifecycle.at(-1).lastEffect.optimistic, {
      kind: "unavailable",
      admissionKind: "localPatch",
      branchPosture: "optimisticUnavailable",
      reason: "runtimeRejected",
      detail:
        "resource effect branch speculation is unavailable because branch_snapshot_id(...) rejected restore-target lookup: snapshot index was compacted",
      branchId: branch.id,
      snapshotId: null,
      inverseAvailable: false,
      rollback: {
        kind: "unavailable",
        reason: "runtimeRejected",
        detail:
          "resource effect branch speculation is unavailable because branch_snapshot_id(...) rejected restore-target lookup: snapshot index was compacted",
        branchId: branch.id,
        snapshotId: null,
        inverseAvailable: false,
      },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("branch-speculative patches record compact inverse posture when exact restore is unsafe", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const branch = createBranchHead(runtime.signals, "effect-denial");
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
        value: "Committed Without Speculation",
      }),
    );

    const effect = line.diagnostics().lastEffect;
    assert.deepEqual(effect.optimistic.rollback, {
      kind: "compactInverseAvailable",
      mode: "CompactInversePatch",
      branchId: branch.id,
      snapshotId,
      inverse: {
        kind: "compactPatchInverse",
        mode: "CompactInversePatch",
        preimage: "aspectValue",
        scope: "aspect",
        itemId: "demo:1",
        aspect: "title",
        summary: null,
        patch: {
          kind: "itemAspect",
          itemId: "demo:1",
          aspect: "title",
          value: "Loaded",
        },
        cost: {
          retainedValueCount: 1,
          retainedResponsePreimage: false,
        },
      },
      detail:
        "resource effect rollback can apply the compact inverse captured before speculative mutation",
    });
    assert.equal(effect.counters.rollbackReadinessBreadth, 1);
    assert.equal(line.value().items[0].title, "Committed Without Speculation");
  } finally {
    await runtime.cleanup();
  }
});

test("branch-speculative patches deny optimism when exact restore and compact inverse are both unavailable", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const branch = createBranchHead(runtime.signals, "effect-inverse-denial");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.replace({
        items: [{ id: "demo:1", title: "Broad Replacement" }],
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.optimistic.rollback, {
      kind: "unavailable",
      reason: "restoreUnavailable",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime cannot restore a captured exact branch snapshot by id and the local patch does not carry an admissible safe compact inverse",
      branchId: branch.id,
      snapshotId,
      inverseAvailable: false,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("branch-speculative patches explain missing branch proof without route-local fallback", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch: undefined,
  });
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.sensitive(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Committed Without Branch Proof",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.plan.branch, {
      kind: "optimisticUnavailable",
      profileName: "sensitive",
      optimism: "branchSpeculative",
      rollback: "branchRestore",
      reason: "unsupportedByRuntime",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime does not expose current_branch(...)",
      branchId: null,
      snapshotId: null,
      inverseAvailable: false,
      proofBreadth: 0,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("non-optimistic local patches stay committed-only without branch proof reads", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch() {
      throw new Error("branch proof must not be read for pessimistic effects");
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
        value: "Pessimistic Commit",
      }),
    );

    assert.deepEqual(line.diagnostics().lastEffect.plan.branch, {
      kind: "committedOnly",
      profileName: "pessimistic",
      optimism: "none",
      rollback: "unavailable",
      reason: "profileDisablesOptimism",
      detail:
        'resource effect profile "pessimistic" disables optimistic branch application',
      proofBreadth: 0,
    });
    assert.deepEqual(line.diagnostics().lastEffect.optimistic, {
      kind: "committed",
      admissionKind: "localPatch",
      branchPosture: "committedOnly",
      reason: "profileDisablesOptimism",
      detail:
        'resource effect profile "pessimistic" disables optimistic branch application',
      rollback: {
        kind: "notApplicable",
        reason: "profileDisablesOptimism",
        detail:
          "committed-only resource effects do not carry speculative rollback state",
      },
      confirmation: {
        kind: "notApplicable",
        detail: "local resource effects await server confirmation",
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

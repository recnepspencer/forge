import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("branch-native effects isolate rollback in an effect-owned branch", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const canonical = createBranchHead(runtime.signals, "effect-owned-posture");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });
    const baselineBranchCount = runtime.signals.history().branches().length;

    await line.patch(titlePatch(runtime, "Optimistic"));

    const effect = line.diagnostics().lastEffect;
    assert.equal(effect.plan.branch.kind, "effectOwnedBranch");
    assert.notEqual(effect.plan.branch.branchId, Number(canonical.id));
    assert.equal(
      effect.plan.branch.nativeAncestryProof.parentBranchId,
      Number(canonical.id),
    );
    assert.deepEqual(effect.plan.branch.semanticDependencyProof.effectIds, []);
    assert.deepEqual(effect.optimistic.rollback, {
      kind: "effectBranchRetirementAvailable",
      branchId: effect.plan.branch.branchId,
      dependencyBasisBranchId: null,
      mode: "EffectBranchRetirement",
    });
    assert.equal(line.effects().open().length, 1);
    assert.equal(
      runtime.signals.history().branches().length,
      baselineBranchCount + 2,
    );

    await line.effects().reject(effect.effectId);
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
    assert.equal(line.value().items[0].title, "Loaded");
  } finally {
    await runtime.cleanup();
  }
});

test("branch-native admission fails typed and leak-free without command authority", async () => {
  const runtime = await createRealResourceTestRuntime({ fork_branch: undefined });
  try {
    createBranchHead(runtime.signals, "effect-command-denial");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });
    const baselineBranchCount = runtime.signals.history().branches().length;

    assert.throws(
      () => line.patch(titlePatch(runtime, "Denied")),
      (error) => error.name === "ResourceEffectBranchUnavailable"
        && error.code === "unsupportedByRuntime",
    );

    assert.equal(line.value().items[0].title, "Loaded");
    assert.equal(line.effects().open().length, 0);
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
  } finally {
    await runtime.cleanup();
  }
});

test("pessimistic effects remain committed-only without branch reads", async () => {
  const runtime = await createRealResourceTestRuntime({
    current_branch() {
      throw new Error("committed effects must not read branch proof");
    },
  });
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.pessimistic(),
    });

    await line.patch(titlePatch(runtime, "Committed"));

    const effect = line.diagnostics().lastEffect;
    assert.equal(effect.plan.branch.kind, "committedOnly");
    assert.equal(effect.optimistic.kind, "committed");
    assert.equal(effect.branchLifecycle.kind, "notApplicable");
    assert.equal(line.value().items[0].title, "Committed");
  } finally {
    await runtime.cleanup();
  }
});

function titlePatch(runtime, value) {
  return runtime.mod.resourcePatch.itemAspect({
    itemId: "demo:1",
    aspect: "title",
    value,
  });
}

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
    load: () => ({ items: [{ id: "demo:1", title: "Loaded" }] }),
  });
  return family.line({ workspaceId: "demo" });
}

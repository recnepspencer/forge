import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("effect-owned branch lifecycle requires retirement proof", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const canonical = createBranchHead(runtime.signals, "effect-lifecycle");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.sensitive(),
    });

    await line.patch(titlePatch(runtime, "Pending"));

    const effect = line.diagnostics().lastEffect;
    assert.equal(effect.branchLifecycle.kind, "effectOwnedBranch");
    assert.equal(effect.branchLifecycle.creation, "createdByResourceRuntime");
    assert.equal(effect.branchLifecycle.ownership, "resourceEffectOwned");
    assert.equal(effect.branchLifecycle.reuse, "forbidden");
    assert.equal(
      effect.branchLifecycle.nativeAncestryProof.parentBranchId,
      Number(canonical.id),
    );
    assert.equal(effect.branchLifecycle.disposal.kind, "retireOwnedBranch");
    assert.equal(
      effect.branchLifecycle.leakDenial.kind,
      "retirementReceiptRequired",
    );
    assert.deepEqual(line.history().lifecycle.at(-1).lastEffect, effect);

    const settlement = await line.effects().reject(effect.effectId);
    assert.equal(
      Number(settlement.retired[0].retirement.retiredEffect.retiredBranchId),
      effect.branchLifecycle.branchId,
    );
    assert.equal(
      typeof settlement.retired[0].retirement.retiredEffect.closeoutDigest,
      "string",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("missing branch commands deny admission before ownership", async () => {
  const runtime = await createRealResourceTestRuntime({ current_branch: undefined });
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    await assert.rejects(
      line.patch(titlePatch(runtime, "Denied")),
      (error) => error.name === "ResourceEffectBranchUnavailable"
        && error.code === "workerBranchCommandUnavailable",
    );
    assert.equal(line.effects().open().length, 0);
  } finally {
    await runtime.cleanup();
  }
});

test("committed-only lifecycle creates no disposal claim", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.pessimistic(),
    });

    await line.patch(titlePatch(runtime, "Committed"));

    const lifecycle = line.diagnostics().lastEffect.branchLifecycle;
    assert.equal(lifecycle.kind, "notApplicable");
    assert.equal(lifecycle.disposal.kind, "notApplicable");
    assert.equal(lifecycle.leakDenial.kind, "notApplicable");
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

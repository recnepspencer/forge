import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("visible selection identifies derived projection without granting authority", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "visible-projection");
    const line = createEffectCollectionLine(
      runtime,
      runtime.mod.resourceEffects.branchNative(),
    );
    assert.equal(line.diagnostics().visibleSelection.kind, "committed");

    await line.patch(titlePatch(runtime, "Projected"));
    const effect = line.effects().open()[0];
    const selection = line.diagnostics().visibleSelection;

    assert.equal(selection.kind, "derivedEffectProjectionBranch");
    assert.equal(selection.source, "openResourceEffects");
    assert.deepEqual(selection.affectedEffectIds, [effect.effectId]);
    assert.equal(selection.branchId, line.effects().projection().branch.id);
    assert.equal(line.effects().projection().canonicalAuthority, false);
    assert.deepEqual(
      line.history().verificationPackage().continuity.visibleSelection,
      selection,
    );

    await line.effects().reject(effect.effectId);
    assert.equal(line.diagnostics().visibleSelection.kind, "committed");
    assert.equal(line.diagnostics().visibleSelection.source, "effectSettlement");
    assert.equal(line.value().items[0].title, "Loaded");
  } finally {
    await runtime.cleanup();
  }
});

test("settlement keeps projection visible while sibling work remains open", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "visible-open-sibling");
    const line = createEffectCollectionLine(
      runtime,
      runtime.mod.resourceEffects.branchNative(),
    );
    await line.patch(titlePatch(runtime, "First"));
    await line.patch(titlePatch(runtime, "Second"));
    const [first, second] = line.effects().open();

    await line.effects().reject(first.effectId);

    const selection = line.diagnostics().visibleSelection;
    assert.equal(selection.kind, "derivedEffectProjectionBranch");
    assert.equal(selection.source, "effectSettlement");
    assert.deepEqual(selection.affectedEffectIds, [first.effectId]);
    assert.equal(line.value().items[0].title, "Second");
    await line.effects().confirm(second.effectId);
    assert.equal(line.diagnostics().visibleSelection.kind, "committed");
  } finally {
    await runtime.cleanup();
  }
});

test("committed-only patch selection remains canonical", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const line = createEffectCollectionLine(
      runtime,
      runtime.mod.resourceEffects.pessimistic(),
    );

    await line.patch(titlePatch(runtime, "Committed"));

    assert.equal(line.diagnostics().visibleSelection.kind, "committed");
    assert.equal(line.diagnostics().visibleSelection.source, "localPatch");
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

function createEffectCollectionLine(runtime, effects) {
  const { mod, resource } = runtime;
  const family = resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    effects,
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

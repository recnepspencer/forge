import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("same-locus server delivery records preserved speculative truth when visible value does not change", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-confirm-preserved");
    const line = createEffectConfirmationLine(runtime);

    await line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Speculative",
      }),
    );
    const speculativeEffect = line.diagnostics().lastEffect;

    line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-confirm",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Speculative",
        }),
      }),
    );

    assert.deepEqual(
      line.diagnostics().lastEffect.optimistic.confirmation,
      {
        kind: "preservedSpeculativeTruth",
        previousEffectId: speculativeEffect.effectId,
        previousPlanId: speculativeEffect.plan.planId,
        previousBranchId: speculativeEffect.optimistic.branchId,
        previousSnapshotId: speculativeEffect.optimistic.snapshotId,
        locusMatches: true,
        detail:
          "server delivery confirmed the visible truth already produced by the pending speculative resource effect",
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

test("broad server delivery with unchanged visible value still records canonical server truth when locus differs", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-confirm-broad-canonical");
    const line = createEffectConfirmationLine(runtime);

    await line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Speculative",
      }),
    );
    const speculativeEffect = line.diagnostics().lastEffect;

    line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-broad-confirm",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: { items: [{ id: "demo:1", title: "Speculative" }] },
      }),
    );

    assert.deepEqual(
      line.diagnostics().lastEffect.optimistic.confirmation,
      {
        kind: "consumedCanonicalServerTruth",
        previousEffectId: speculativeEffect.effectId,
        previousPlanId: speculativeEffect.plan.planId,
        previousBranchId: speculativeEffect.optimistic.branchId,
        previousSnapshotId: speculativeEffect.optimistic.snapshotId,
        locusMatches: false,
        valueChanged: false,
        detail:
          "server delivery consumed canonical server truth after a pending speculative resource effect",
      },
    );
  } finally {
    await runtime.cleanup();
  }
});

test("server delivery records canonical server truth when it changes speculative value", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-confirm-canonical");
    const line = createEffectConfirmationLine(runtime);

    await line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Speculative",
      }),
    );
    const speculativeEffect = line.diagnostics().lastEffect;

    line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-canonical",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        nextValue: { items: [{ id: "demo:1", title: "Canonical" }] },
      }),
    );

    assert.deepEqual(
      line.history().lifecycle.at(-1).lastEffect.optimistic.confirmation,
      {
        kind: "consumedCanonicalServerTruth",
        previousEffectId: speculativeEffect.effectId,
        previousPlanId: speculativeEffect.plan.planId,
        previousBranchId: speculativeEffect.optimistic.branchId,
        previousSnapshotId: speculativeEffect.optimistic.snapshotId,
        locusMatches: false,
        valueChanged: true,
        detail:
          "server delivery consumed canonical server truth after a pending speculative resource effect",
      },
    );
    assert.equal(line.value().items[0].title, "Canonical");
  } finally {
    await runtime.cleanup();
  }
});

function createEffectConfirmationLine(runtime) {
  const { mod, resource } = runtime;
  const family = resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({ basisId: "basis-1" }),
    effects: mod.resourceEffects.branchNative(),
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

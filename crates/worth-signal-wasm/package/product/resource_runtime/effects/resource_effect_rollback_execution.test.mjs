import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("reject retires effect-owned state and restores canonical truth", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-reject");
    const line = createEffectCollectionLine(runtime);
    const baselineBranchCount = runtime.signals.history().branches().length;
    await line.patch(titlePatch(runtime, "demo:1", "Optimistic"));
    const effect = line.effects().open()[0];

    const result = await line.effects().reject(effect.effectId);

    assert.equal(result.kind, "rejectedAndRetired");
    assert.equal(result.retired[0].effectId, effect.effectId);
    assert.equal(
      Number(result.retired[0].retirement.retiredEffect.retiredBranchId),
      effect.branchId,
    );
    assert.equal(
      typeof result.retired[0].retirement.retiredEffect.closeoutDigest,
      "string",
    );
    assert.equal(line.value().items[0].title, "Loaded 1");
    assert.equal(line.effects().open().length, 0);
    assert.equal(runtime.signals.history().branches().length, baselineBranchCount);
  } finally {
    await runtime.cleanup();
  }
});

test("rejecting one sibling preserves independent optimistic work", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-sibling-reject");
    const line = createEffectCollectionLine(runtime);
    await Promise.all([
      line.patch(titlePatch(runtime, "demo:1", "First")),
      line.patch(titlePatch(runtime, "demo:2", "Second")),
    ]);
    const [first, second] = line.effects().open();

    await line.effects().reject(first.effectId);

    assert.deepEqual(
      line.value().items.map((item) => item.title),
      ["Loaded 1", "Second"],
    );
    assert.deepEqual(
      line.effects().open().map((effect) => effect.effectId),
      [second.effectId],
    );
    await line.effects().confirm(second.effectId);
    assert.equal(line.value().items[1].title, "Second");
  } finally {
    await runtime.cleanup();
  }
});

test("history rollback targets the last open effect through settlement", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-history-boundary");
    const line = createEffectCollectionLine(runtime);
    await line.patch(titlePatch(runtime, "demo:1", "Pending"));
    const effect = line.effects().open()[0];

    const result = await line.history().rollbackLastEffect();

    assert.equal(result.kind, "rejectedAndRetired");
    assert.equal(result.effectId, effect.effectId);
    assert.equal(line.effects().open().length, 0);
    assert.equal(line.value().items[0].title, "Loaded 1");
    assert.equal(line.effects().get(effect.effectId).lifecycle, "Retired");
    assert.equal(
      line.effects().get(effect.effectId).terminal.kind,
      "rejectedAndRetired",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("targeted and last-effect rollback are deterministic with many effects", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-targeted-history");
    const line = createEffectCollectionLine(runtime);
    const none = await line.history().rollbackLastEffect();
    assert.equal(none.kind, "unavailable");
    assert.equal(none.reason, "noOpenEffect");

    await line.patch(titlePatch(runtime, "demo:1", "First"));
    await line.patch(titlePatch(runtime, "demo:2", "Second"));
    const [first, second] = line.effects().open();

    const targeted = await line.history().rollbackEffect(first.effectId);
    assert.equal(targeted.effectId, first.effectId);
    assert.equal(line.value().items[1].title, "Second");
    const last = await line.history().rollbackLastEffect();
    assert.equal(last.effectId, second.effectId);
    assert.equal(line.effects().open().length, 0);

    const settled = await line.history().rollbackEffect(second.effectId);
    assert.equal(settled.kind, "unavailable");
    assert.equal(settled.reason, "effectAlreadySettled");
  } finally {
    await runtime.cleanup();
  }
});

test("line release denies open effect branch leaks and succeeds after settlement", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    createBranchHead(runtime.signals, "effect-release-boundary");
    const line = createEffectCollectionLine(runtime);
    await line.patch(titlePatch(runtime, "demo:1", "Pending"));
    const effectId = line.effects().open()[0].effectId;

    assert.throws(
      () => line.free(),
      (error) => error?.code === "openEffects"
        && error.effectIds?.[0] === effectId,
    );
    await line.history().rollbackEffect(effectId);
    assert.doesNotThrow(() => line.free());
  } finally {
    await runtime.cleanup();
  }
});

function titlePatch(runtime, itemId, value) {
  return runtime.mod.resourcePatch.itemAspect({
    itemId,
    aspect: "title",
    value,
  });
}

function createEffectCollectionLine(runtime) {
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
      items: [
        { id: "demo:1", title: "Loaded 1" },
        { id: "demo:2", title: "Loaded 2" },
      ],
    }),
  });
  return family.line({ workspaceId: "demo" });
}

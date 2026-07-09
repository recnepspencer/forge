import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import {
  createBranchHead,
  installHistoryOverrides,
} from "../runtime_fixture/real_resource_signals.mjs";

test("rollbackLastEffect restores the exact branch snapshot carried by the effect", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "effect-rollback");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const calls = [];
    const uninstall = installHistoryOverrides(runtime.signals, {
      restore_branch_snapshot_by_id(history, branchId, targetSnapshotId) {
        calls.push([branchId, targetSnapshotId]);
        return history.restore_branch_snapshot_by_id(
          branchId,
          targetSnapshotId,
        );
      },
    });
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });
    const beforePatchValue = line.value();

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Optimistic Title",
      }),
    );
    const effect = line.diagnostics().lastEffect;

    const result = line.history().rollbackLastEffect();

    assert.deepEqual(calls, [[BigInt(branch.id), BigInt(snapshotId)]]);
    assert.deepEqual(result, {
      kind: "rolledBack",
      mode: "SameRuntimeBranchExact",
      effectId: effect.effectId,
      branchId: branch.id,
      snapshotId,
      basisCurrentId: "basis-1",
      basisAdvanceCount: 0,
      rollback: effect.optimistic.rollback,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.equal(line.value().items[0].title, beforePatchValue.items[0].title);
    assert.deepEqual(line.status(), {
      kind: "fulfilled",
      operation: "restore",
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.deepEqual(line.history().lifecycle.at(-1)?.lastEffect, effect);
    uninstall();
  } finally {
    await runtime.cleanup();
  }
});

test("rollbackLastEffect returns a typed unavailable artifact when no effect exists", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    assert.deepEqual(line.history().rollbackLastEffect(), {
      kind: "unavailable",
      reason: "noEffect",
      detail:
        "resource effect rollback is unavailable because the line has no recorded resource effect",
      effectId: null,
      basisCurrentId: "basis-1",
      basisAdvanceCount: 0,
      rollback: null,
    });
    assert.equal(line.history().lifecycle.at(-1)?.event, "materialized");
  } finally {
    await runtime.cleanup();
  }
});

test("rollbackLastEffect applies compact inverse when exact branch restore is unavailable", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    createBranchHead(runtime.signals, "effect-rollback-unavailable");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Cannot Restore",
      }),
    );
    const effect = line.diagnostics().lastEffect;
    const result = line.history().rollbackLastEffect();

    assert.deepEqual(result, {
      kind: "rolledBack",
      mode: "CompactInversePatch",
      effectId: effect.effectId,
      branchId: effect.optimistic.branchId,
      snapshotId: effect.optimistic.snapshotId,
      basisCurrentId: "basis-1",
      basisAdvanceCount: 0,
      rollback: effect.optimistic.rollback,
      reloadStatus: {
        kind: "fulfilled",
        operation: "restore",
      },
    });
    assert.equal(line.history().availability.restoreExact.kind, "available");
    assert.equal(line.value().items[0].title, "Loaded");
    assert.equal(line.history().lifecycle.at(-1)?.event, "restored");
    assert.deepEqual(line.history().lifecycle.at(-1)?.lastEffect, effect);
  } finally {
    await runtime.cleanup();
  }
});

test("compact inverse rollback consumes a snapshotted preimage instead of a retained object reference", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    createBranchHead(runtime.signals, "effect-rollback-snapshot-inverse");
    const line = createMetadataCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });
    const previousMetadataReference = line.value().items[0].metadata;

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "metadata",
        value: { label: "Optimistic", nested: { count: 2 } },
      }),
    );
    const inverseValue =
      line.diagnostics().lastEffect.optimistic.rollback.inverse.patch.value;
    previousMetadataReference.label = "Externally Mutated";
    previousMetadataReference.nested.count = 99;
    try {
      inverseValue.label = "Envelope Mutated";
      inverseValue.nested.count = 100;
    } catch {
      // Host read behavior may reject mutation; rollback truth is asserted below.
    }

    const result = line.history().rollbackLastEffect();

    assert.equal(result.kind, "rolledBack");
    assert.deepEqual(line.value().items[0].metadata, {
      label: "Loaded",
      nested: { count: 1 },
    });
  } finally {
    await runtime.cleanup();
  }
});

test("rollbackLastEffect preserves visible value when neither exact restore nor compact inverse is available", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    createBranchHead(runtime.signals, "effect-rollback-no-inverse");
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.replace({
        items: [{ id: "demo:1", title: "Broad Replacement" }],
      }),
    );
    const effect = line.diagnostics().lastEffect;

    assert.deepEqual(line.history().rollbackLastEffect(), {
      kind: "unavailable",
      reason: "restoreUnavailable",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime cannot restore a captured exact branch snapshot by id and the local patch does not carry an admissible safe compact inverse",
      effectId: effect.effectId,
      basisCurrentId: "basis-1",
      basisAdvanceCount: 0,
      rollback: effect.optimistic.rollback,
    });
    assert.equal(line.value().items[0].title, "Broad Replacement");
    assert.equal(line.history().lifecycle.at(-1)?.event, "patched");
  } finally {
    await runtime.cleanup();
  }
});

test("rollbackLastEffect denies compact inverse when retained preimage cannot be snapshotted", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    createBranchHead(runtime.signals, "effect-rollback-uncloneable-inverse");
    const line = createMetadataCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "computed",
        value: { label: "Optimistic" },
      }),
    );
    const effect = line.diagnostics().lastEffect;

    assert.deepEqual(effect.optimistic.rollback, {
      kind: "unavailable",
      reason: "restoreUnavailable",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime cannot restore a captured exact branch snapshot by id and the local patch does not carry an admissible safe compact inverse",
      branchId: effect.optimistic.branchId,
      snapshotId: effect.optimistic.snapshotId,
      inverseAvailable: false,
    });
    assert.deepEqual(line.history().rollbackLastEffect(), {
      kind: "unavailable",
      reason: "restoreUnavailable",
      detail:
        "resource effect branch speculation is unavailable because the Signals runtime cannot restore a captured exact branch snapshot by id and the local patch does not carry an admissible safe compact inverse",
      effectId: effect.effectId,
      basisCurrentId: "basis-1",
      basisAdvanceCount: 0,
      rollback: effect.optimistic.rollback,
    });
    assert.deepEqual(line.value().items[0].metadata, { label: "Optimistic" });
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
    requestContext: mod.resourceRequestContext({
      correlationId: "trace-demo",
      branchId: "branch-demo",
      basisId: "basis-1",
    }),
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

function createMetadataCollectionLine(runtime, options) {
  const { mod, resource } = runtime;
  const family = resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    requestContext: mod.resourceRequestContext({
      correlationId: "trace-demo",
      branchId: "branch-demo",
      basisId: "basis-1",
    }),
    effects: options.effects,
    itemIdentity: (item) => item.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: [...nextItems] }),
      aspects: mod.resourceItemAspects({
        metadata: {
          read: (item) => item.metadata,
          write: (item, metadata) => ({ ...item, metadata }),
        },
        computed: {
          read: () => function uncloneablePreimage() {},
          write: (item, metadata) => ({ ...item, metadata }),
        },
      }),
    }),
    load: () => ({
      items: [
        {
          id: "demo:1",
          metadata:
            options.metadata ?? { label: "Loaded", nested: { count: 1 } },
        },
      ],
    }),
  });
  return family.line({ workspaceId: "demo" });
}

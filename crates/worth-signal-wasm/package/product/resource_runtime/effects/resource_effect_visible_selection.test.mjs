import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("visible selection names speculative confirmed and restored resource truth", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "visible-selection");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });
    assert.equal(line.diagnostics().visibleSelection.kind, "committed");

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Speculative",
      }),
    );
    const speculativeEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.diagnostics().visibleSelection, {
      kind: "speculative",
      source: "localPatch",
      effectId: speculativeEffect.effectId,
      branchId: branch.id,
      snapshotId,
      basisId: "basis-1",
      rollbackKind: "exactBranchRestoreAvailable",
      detail:
        "resource line visible truth is the selected speculative branch effect",
    });
    assert.deepEqual(
      line.history().lifecycle.at(-1).visibleSelection,
      line.diagnostics().visibleSelection,
    );

    line.history().rollbackLastEffect();
    assert.deepEqual(line.diagnostics().visibleSelection, {
      kind: "restored",
      source: "exactBranchRestore",
      effectId: speculativeEffect.effectId,
      branchId: branch.id,
      snapshotId,
      basisId: "basis-1",
      rollbackKind: "exactBranchRestoreAvailable",
      detail:
        "resource effect rollback can restore the exact branch snapshot captured before speculative application",
    });

    line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Confirmed",
      }),
    );
    const confirmedLocalEffect = line.diagnostics().lastEffect;
    line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-confirm",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Confirmed",
        }),
      }),
    );
    assert.equal(line.diagnostics().visibleSelection.kind, "confirmed");
    assert.equal(
      line.diagnostics().visibleSelection.previousEffectId,
      confirmedLocalEffect.effectId,
    );
    assert.deepEqual(
      line.history().verificationPackage().continuity.visibleSelection,
      line.diagnostics().visibleSelection,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("visible selection distinguishes committed fallback and compact inverse restore", async () => {
  const runtime = await createRealResourceTestRuntime({
    restore_branch_snapshot_by_id: undefined,
  });
  try {
    const branch = createBranchHead(runtime.signals, "visible-selection-inverse");
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
        value: "Compact Speculative",
      }),
    );
    const compactEffect = line.diagnostics().lastEffect;
    assert.deepEqual(line.diagnosticsSummary().current.visibleSelection, {
      kind: "speculative",
      source: "localPatch",
      effectId: compactEffect.effectId,
      branchId: branch.id,
      snapshotId,
      basisId: "basis-1",
      rollbackKind: "compactInverseAvailable",
      detail:
        "resource line visible truth is the selected speculative branch effect",
    });

    line.history().rollbackLastEffect();
    assert.equal(line.diagnostics().visibleSelection.kind, "restored");
    assert.equal(line.diagnostics().visibleSelection.source, "compactInverse");
    assert.equal(
      line.diagnostics().visibleSelection.rollbackKind,
      "compactInverseAvailable",
    );

    line.patch(
      runtime.mod.resourcePatch.replace({
        items: [{ id: "demo:1", title: "Broad Committed" }],
      }),
    );
    assert.deepEqual(line.diagnostics().visibleSelection, {
      kind: "committed",
      source: "optimismUnavailable",
      effectId: line.diagnostics().lastEffect.effectId,
      branchId: "branch-demo",
      snapshotId: null,
      basisId: "basis-1",
      unavailableReason: "restoreUnavailable",
      detail:
        "resource line visible truth is committed directly because speculative branch visibility was unavailable",
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

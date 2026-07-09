import assert from "node:assert/strict";
import test from "node:test";

import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";
import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";

test("local patches create a sealed branch-native effect envelope across diagnostics and history", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const branch = createBranchHead(runtime.signals, "effect-envelope");
    const snapshotId = Number(
      runtime.signals.history().branch_snapshot_id(branch.id),
    );
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.branchNative(),
    });

    const result = line.patch(
      runtime.mod.resourcePatch.itemAspect({
        itemId: "demo:1",
        aspect: "title",
        value: "Locally Patched",
      }),
    );

    assert.deepEqual(result, {
      kind: "narrowed",
      scope: "aspect",
      itemId: "demo:1",
      aspect: "title",
      field: null,
    });
    const effect = line.diagnostics().lastEffect;
    assert.equal(effect.version, "resource-effect-envelope-v1");
    assert.equal(effect.provenance, "localPatch");
    assert.equal(effect.idempotencyKey, null);
    assert.equal(effect.serverCorrelationId, null);
    assert.equal(effect.plan.admissionKind, "localPatch");
    assert.equal(effect.plan.planId, effect.effectId);
    assert.match(effect.plan.causalSequence, /localPatch#1$/);
    assert.equal(effect.plan.retryLineageId, null);
    assert.deepEqual(effect.plan.branch, {
      kind: "speculativeBranch",
      profileName: "branchNative",
      optimism: "branchSpeculative",
      rollback: "branchRestoreOrInverse",
      rollbackMode: "SameRuntimeBranchExact",
      branchId: branch.id,
      snapshotId,
      restoreMode: "SameRuntimeBranchExact",
      inverse: null,
      proofBreadth: 2,
    });
    assert.equal(effect.profile.name, "branchNative");
    assert.equal(effect.profile.rollback, "branchRestoreOrInverse");
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
    assert.deepEqual(effect.optimistic, {
      kind: "applied",
      admissionKind: "localPatch",
      branchPosture: "speculativeBranch",
      branchId: branch.id,
      snapshotId,
      restoreMode: "SameRuntimeBranchExact",
      rollback: {
        kind: "exactBranchRestoreAvailable",
        mode: "SameRuntimeBranchExact",
        branchId: branch.id,
        snapshotId,
        detail:
          "resource effect rollback can restore the exact branch snapshot captured before speculative application",
      },
      confirmation: "pendingServer",
      detail:
        "resource effect was applied under a branch-native speculative posture and awaits server confirmation",
    });
    assert.deepEqual(effect.request, {
      correlationId: "trace-demo",
      branchId: "branch-demo",
      basisId: "basis-1",
    });
    assert.deepEqual(effect.locus, {
      kind: "itemAspect",
      itemId: "demo:1",
      aspect: "title",
    });
    assert.equal(effect.locusProof, null);
    assert.deepEqual(effect.patch, {
      kind: "itemAspect",
      scope: "aspect",
      itemId: "demo:1",
      field: null,
      fieldProof: null,
      regionName: null,
      path: null,
      aspect: "title",
      summary: null,
      valueChanged: true,
      region: null,
      jsonPath: null,
    });
    assert.deepEqual(effect.counters, {
      patchCountBefore: 0,
      deliveryCountBefore: 0,
      basisAdvanceCountBefore: 0,
      planningBreadth: 1,
      executionBreadth: 1,
      branchProofBreadth: 2,
      branchLifecycleBreadth: 1,
      optimisticLifecycleBreadth: 1,
      serverConfirmationBreadth: 0,
      rollbackReadinessBreadth: 1,
      responseLensBreadth: 0,
      effectLocusBreadth: 1,
      detailFieldTraversalBreadth: 0,
      detailFieldReconstructionBreadth: 0,
      detailRegionTraversalBreadth: 0,
      detailRegionReconstructionBreadth: 0,
      jsonPathTraversalBreadth: 0,
      jsonPathReconstructionBreadth: 0,
    });
    assert.deepEqual(line.diagnosticsSummary().latest.effect, effect);
    assert.deepEqual(line.history().lifecycle.at(-1).lastEffect, effect);
    assert.deepEqual(
      line.history().verificationPackage().lifecycle.lastEffect,
      effect,
    );
    assert.deepEqual(
      line.history().verificationPackage().deliveryProvenance.lastEffect,
      effect,
    );

    assert.throws(
      () =>
        line.patch(
          runtime.mod.resourcePatch.itemAspect({
            itemId: "demo:1",
            aspect: "unknown",
            value: "Denied",
          }),
        ),
      /undeclared aspect "unknown"/,
    );
    assert.deepEqual(line.diagnostics().lastEffect, effect);
  } finally {
    await runtime.cleanup();
  }
});

test("server deliveries create effect envelopes for patch replace invalidate and basis refresh", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const line = createEffectCollectionLine(runtime, {
      effects: runtime.mod.resourceEffects.deliveryAuthoritative(),
    });

    line.deliver(
      runtime.mod.resourceDelivery.patch({
        packetId: "pkt-patch",
        basisId: "basis-1",
        nextBasisId: "basis-2",
        patch: runtime.mod.resourcePatch.itemAspect({
          itemId: "demo:1",
          aspect: "title",
          value: "Delivered Patch",
        }),
      }),
    );
    const patchEffect = line.diagnostics().lastEffect;
    assert.equal(patchEffect.provenance, "deliveredPatch");
    assert.equal(patchEffect.plan.admissionKind, "delivery");
    assert.match(patchEffect.plan.causalSequence, /deliveredPatch#1$/);
    assert.equal(
      patchEffect.plan.retryLineageId.endsWith(":demo:pkt-patch"),
      true,
    );
    assert.equal(patchEffect.idempotencyKey, "pkt-patch");
    assert.equal(patchEffect.delivery.packetId, "pkt-patch");
    assert.equal(patchEffect.delivery.nextBasisId, "basis-2");
    assert.equal(patchEffect.profile.name, "deliveryAuthoritative");
    assert.deepEqual(patchEffect.branchLifecycle, {
      kind: "notApplicable",
      creation: "notApplicable",
      reason: "deliveryAuthority",
      detail:
        "server deliveries are already authoritative and are not admitted as speculative branch patches",
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
    assert.deepEqual(patchEffect.plan.branch, {
      kind: "committedOnly",
      profileName: "deliveryAuthoritative",
      optimism: "none",
      rollback: "unavailable",
      reason: "deliveryAuthority",
      detail:
        "server deliveries are already authoritative and are not admitted as speculative branch patches",
      proofBreadth: 0,
    });
    assert.deepEqual(patchEffect.optimistic, {
      kind: "committed",
      admissionKind: "delivery",
      branchPosture: "committedOnly",
      reason: "deliveryAuthority",
      detail:
        "server deliveries are already authoritative and are not admitted as speculative branch patches",
      rollback: {
        kind: "notApplicable",
        reason: "deliveryAuthority",
        detail:
          "committed-only resource effects do not carry speculative rollback state",
      },
      confirmation: {
        kind: "independentServerTruth",
        previousEffectId: null,
        detail:
          "server delivery committed without consuming a pending speculative resource effect",
      },
    });
    assert.equal(patchEffect.counters.branchProofBreadth, 0);
    assert.equal(patchEffect.counters.branchLifecycleBreadth, 1);
    assert.equal(patchEffect.counters.optimisticLifecycleBreadth, 1);
    assert.equal(patchEffect.counters.serverConfirmationBreadth, 1);
    assert.equal(patchEffect.counters.rollbackReadinessBreadth, 1);
    assert.deepEqual(patchEffect.locus, {
      kind: "itemAspect",
      itemId: "demo:1",
      aspect: "title",
    });

    assert.deepEqual(
      line.deliver(
        runtime.mod.resourceDelivery.patch({
          packetId: "pkt-patch",
          basisId: "basis-2",
          nextBasisId: "basis-3",
          patch: runtime.mod.resourcePatch.itemAspect({
            itemId: "demo:1",
            aspect: "title",
            value: "Duplicate",
          }),
        }),
      ),
      {
        kind: "duplicateIgnored",
        packetId: "pkt-patch",
        deliveryKind: "patch",
      },
    );
    assert.deepEqual(line.diagnostics().lastEffect, patchEffect);

    line.deliver(
      runtime.mod.resourceDelivery.replace({
        packetId: "pkt-replace",
        basisId: "basis-2",
        nextBasisId: "basis-3",
        nextValue: { items: [{ id: "demo:1", title: "Replaced" }] },
      }),
    );
    assert.equal(line.diagnostics().lastEffect.provenance, "deliveredReplace");
    assert.deepEqual(line.diagnostics().lastEffect.locus, { kind: "line" });

    line.deliver(
      runtime.mod.resourceDelivery.invalidate({
        packetId: "pkt-invalidate",
        basisId: "basis-3",
        nextBasisId: "basis-3",
      }),
    );
    assert.equal(line.diagnostics().lastEffect.provenance, "deliveryInvalidate");
    assert.deepEqual(line.diagnostics().lastEffect.locus, {
      kind: "invalidation",
    });

    line.deliver(
      runtime.resource.compatibility.delivery.basisRefresh({
        packetId: "pkt-basis",
        basisId: "basis-3",
        nextBasisId: "basis-4",
      }),
    );
    const basisEffect = line.diagnostics().lastEffect;
    assert.equal(basisEffect.provenance, "deliveryBasisRefresh");
    assert.equal(basisEffect.plan.admissionKind, "delivery");
    assert.equal(basisEffect.request.basisId, "basis-4");
    assert.deepEqual(basisEffect.locus, { kind: "basis" });

    assert.deepEqual(
      line.deliver(
        runtime.mod.resourceDelivery.replace({
          packetId: "pkt-stale",
          basisId: "basis-1",
          nextBasisId: "basis-5",
          nextValue: { items: [] },
        }),
      ),
      {
        kind: "basisRejected",
        packetId: "pkt-stale",
        expectedBasisId: "basis-4",
        actualBasisId: "basis-1",
      },
    );
    assert.deepEqual(line.diagnostics().lastEffect, basisEffect);
  } finally {
    await runtime.cleanup();
  }
});

function createEffectCollectionLine(runtime, options) {
  const { mod, resource } = runtime;
  let loadCount = 0;
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
    load: () => {
      loadCount += 1;
      return {
        items: [{ id: "demo:1", title: `Loaded ${loadCount}` }],
      };
    },
  });
  return family.line({ workspaceId: "demo" });
}

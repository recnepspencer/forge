import assert from "node:assert/strict";
import test from "node:test";
import { Worker as NodeWorker } from "node:worker_threads";

import { loadSignalsModule } from "../../module_loading/load_signals_module.mjs";

async function settleWorkerResourceLine() {
  await Promise.resolve();
  await new Promise((resolve) => setTimeout(resolve, 0));
}

test("default worker-first root materializes native and compatibility resource families, including scoped collection families", async () => {
  const previousWorker = globalThis.Worker;
  globalThis.Worker = NodeWorker;
  const {
    createSignals,
    cleanup,
    resourceParams,
    resourceParamIdentity,
  } = await loadSignalsModule({ rawSurface: "real" });

  try {
    const workerSignals = await createSignals();
    const compatibilitySignals = await createSignals({ deployment: "mainThreadCompatibility" });

    assert.deepEqual(
      workerSignals.resource.effects.branchNative(),
      compatibilitySignals.resource.effects.branchNative(),
    );
    assert.deepEqual(
      workerSignals.resource.mutationResponses,
      compatibilitySignals.resource.mutationResponses,
    );

    const nativeDetail = workerSignals.resource.detail({
      params: resourceParams(),
      normalizeParams: ({ taskId }) => resourceParamIdentity({ taskId }, taskId),
      load: ({ taskId }) => ({ id: taskId, title: `Task:${taskId}` }),
    });
    const nativeDetailLine = nativeDetail.line({ taskId: "task-1" });
    await settleWorkerResourceLine();
    assert.deepEqual(nativeDetailLine.value(), {
      id: "task-1",
      title: "Task:task-1",
    });

    const scopedCollection = workerSignals.scope("wizard").resource.collection({
      params: resourceParams(),
      normalizeParams: ({ workspaceId }) =>
        resourceParamIdentity({ workspaceId }, workspaceId),
      itemIdentity: (item) => item.id,
      load: ({ workspaceId }) => [{
        id: `${workspaceId}:1`,
        title: `Workspace:${workspaceId}`,
      }],
    });
    const scopedCollectionLine = scopedCollection.line({ workspaceId: "demo" });
    await settleWorkerResourceLine();
    assert.deepEqual(scopedCollectionLine.value(), [{
      id: "demo:1",
      title: "Workspace:demo",
    }]);

    const nativePaged = workerSignals.resource.paged({
      params: resourceParams(),
      normalizeParams: ({ feedId }) => resourceParamIdentity({ feedId }, feedId),
      itemIdentity: (item) => item.id,
      accumulatePage: (existing, next) => [...existing, ...next],
      load: ({ feedId }) => [{ id: `${feedId}:1`, title: `Feed:${feedId}` }],
    });
    const nativePagedLine = nativePaged.line({ feedId: "feed" });
    await settleWorkerResourceLine();
    assert.deepEqual(nativePagedLine.value(), [{
      id: "feed:1",
      title: "Feed:feed",
    }]);

    const compatibilityDetail = workerSignals.resource.compatibility.detail({
      version: "worth-resource-external-v1",
      family: "detail",
      definitionId: "external-task-detail",
      requestContract: "native-v1",
      reconciliationContract: "none",
      declaration: {
        params: resourceParams(),
        normalizeParams: ({ taskId }) => resourceParamIdentity({ taskId }, taskId),
        load: ({ taskId }) => ({ id: taskId, title: `External:${taskId}` }),
      },
    });
    const compatibilityDetailLine = compatibilityDetail.line({ taskId: "task-2" });
    await settleWorkerResourceLine();
    assert.deepEqual(compatibilityDetailLine.value(), {
      id: "task-2",
      title: "External:task-2",
    });
    assert.deepEqual(compatibilityDetailLine.descriptor().compatibility, {
      kind: "externalDefinition",
      version: "worth-resource-external-v1",
      definitionId: "external-task-detail",
      requestContract: "native-v1",
      reconciliationContract: "none",
    });

    const compatibilityPaged = workerSignals.scope("wizard").resource.compatibility.paged({
      version: "worth-resource-external-v1",
      family: "paged",
      definitionId: "external-feed",
      requestContract: "native-v1",
      reconciliationContract: "none",
      declaration: {
        params: resourceParams(),
        normalizeParams: ({ feedId }) => resourceParamIdentity({ feedId }, feedId),
        itemIdentity: (item) => item.id,
        accumulatePage: (existing, next) => [...existing, ...next],
        load: ({ feedId }) => [{ id: `${feedId}:compat`, title: `Compat:${feedId}` }],
      },
    });
    const compatibilityPagedLine = compatibilityPaged.line({ feedId: "paged" });
    await settleWorkerResourceLine();
    assert.deepEqual(compatibilityPagedLine.value(), [{
      id: "paged:compat",
      title: "Compat:paged",
    }]);

    const deliveryPacket = workerSignals.resource.compatibility.delivery.basisRefresh({
      packetId: "packet-1",
      basisId: "basis-1",
      nextBasisId: "basis-2",
    });
    assert.equal(deliveryPacket.kind, "basisRefresh");

    const workerBranch = await workerSignals.history().create_branch("feature/resource-branch");
    const compatibilityBranch = compatibilitySignals.history().create_branch("feature/resource-branch");
    await workerSignals.history().switch_branch(workerBranch.id);
    compatibilitySignals.history().switch_branch(compatibilityBranch.id);
    const workerBranchPlan = await workerSignals.resource.branch.planMerge({
      source_branch_id: workerBranch.id,
      target_branch_id: 0,
    });
    const compatibilityBranchPlan = compatibilitySignals.resource.branch.planMerge({
      source_branch_id: compatibilityBranch.id,
      target_branch_id: 0,
    });
    assert.deepEqual(
      comparableResourceBranchPlan(workerBranchPlan),
      comparableResourceBranchPlan(compatibilityBranchPlan),
    );

    const workerTasks = createBranchMergeTasks(workerSignals);
    const compatibilityTasks = createBranchMergeTasks(compatibilitySignals);
    const workerLine = workerTasks.line({});
    const compatibilityLine = compatibilityTasks.line({});
    await settleWorkerResourceLine();
    await workerLine.patch(workerTasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Merged Through Worker-First Resource Effect",
    }));
    await compatibilityLine.patch(compatibilityTasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Merged Through Worker-First Resource Effect",
    }));
    const workerEffect = workerLine.diagnostics().lastEffect;
    const compatibilityEffect = compatibilityLine.diagnostics().lastEffect;
    const workerEffectPlan = await workerSignals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: workerEffect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect: workerEffect,
    });
    const compatibilityEffectPlan = compatibilitySignals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: compatibilityEffect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect: compatibilityEffect,
    });
    assert.deepEqual(
      comparableResourceEffectPlan(workerEffectPlan),
      comparableResourceEffectPlan(compatibilityEffectPlan),
    );
    await workerLine.effects().reject(workerEffect.effectId, {
      responseId: "worker-resource-plan-closeout",
    });
    await compatibilityLine.effects().reject(compatibilityEffect.effectId, {
      responseId: "compatibility-resource-plan-closeout",
    });

    workerLine.free();
    compatibilityLine.free();
    nativeDetailLine.free();
    scopedCollectionLine.free();
    nativePagedLine.free();
    compatibilityDetailLine.free();
    compatibilityPagedLine.free();
    await workerSignals.terminate();
    compatibilitySignals.free();
  } finally {
    await cleanup();
    globalThis.Worker = previousWorker;
  }
});

function createBranchMergeTasks(signals) {
  const response = signals.resource.response.array({
    itemId: (task) => task.id,
    aspects: signals.resource.response.objectAspects()({
      title: "title",
    }),
  });
  return signals.api({ effects: signals.resource.effects.branchNative() })
    .url("/branch-execution-tasks")
    .response(response)
    .list({
      load: () => [{ id: "task:1", title: "Loaded" }],
    });
}

function comparableResourceBranchPlan(result) {
  if (result.kind === "denied") {
    return {
      kind: result.kind,
      reason: result.reason,
      detail: result.detail ?? null,
    };
  }
  return {
    kind: result.kind,
    sourceBranchId: result.sourceBranchId,
    targetBranchId: result.targetBranchId,
    mergeKind: result.mergeKind,
    selectedSemantics: result.selectedSemantics,
    conflicts: result.conflicts,
    proof: {
      proofSchemaVersion: result.proof.proofSchemaVersion,
      semanticsDigest: result.proof.semanticsDigest,
      selectedStrategyDigest: result.proof.selectedStrategyDigest,
      selectedMergeBaseDigest: result.proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest: result.proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest: result.proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest: result.proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest: result.proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest: result.proof.selectedDeletionPolicyDigest,
    },
  };
}

function comparableResourceEffectPlan(result) {
  if (result.kind === "denied") {
    return {
      kind: result.kind,
      reason: result.reason,
      detail: result.detail ?? null,
    };
  }
  return {
    kind: result.kind,
    sourceBranchId: result.sourceBranchId,
    targetBranchId: result.targetBranchId,
    mergeKind: result.mergeKind,
    selectedSemantics: result.selectedSemantics,
    conflicts: result.conflicts,
    proof: {
      proofSchemaVersion: result.proof.proofSchemaVersion,
      semanticsDigest: result.proof.semanticsDigest,
      selectedStrategyDigest: result.proof.selectedStrategyDigest,
      selectedMergeBaseDigest: result.proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest: result.proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest: result.proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest: result.proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest: result.proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest: result.proof.selectedDeletionPolicyDigest,
    },
    resourceEffect: {
      topology: result.resourceEffect.topology,
      effectLocus: result.resourceEffect.effectLocus,
      locus: result.resourceEffect.locus,
      conflictIsolation: result.resourceEffect.conflictIsolation,
      rebase: result.resourceEffect.rebase,
      policyBinding: result.resourceEffect.policyBinding,
      proof: {
        semanticsDigest: result.resourceEffect.proof.semanticsDigest,
        effectLocusDigest: result.resourceEffect.proof.effectLocusDigest,
        compiledLensDigest: result.resourceEffect.proof.compiledLensDigest,
      },
      rebaseArtifact: comparableResourceEffectArtifact(result.resourceEffect.rebaseArtifact),
    },
  };
}

function comparableResourceEffectArtifact(artifact) {
  return {
    kind: artifact.kind,
    reason: artifact.reason ?? null,
    conflictCount: artifact.conflictCount ?? null,
    conflicts: artifact.conflicts ?? null,
  };
}

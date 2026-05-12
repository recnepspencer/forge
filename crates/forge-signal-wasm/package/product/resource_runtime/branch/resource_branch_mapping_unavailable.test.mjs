import assert from "node:assert/strict";
import test from "node:test";

import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealResourceTestRuntime } from "../runtime_fixture/real_resource_runtime.mjs";

test("resource effect merge planning emits mapping-unavailable artifacts without stable locus proof", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-unavailable-plan");
    const line = createEffectCollectionLine(runtime);

    line.patch(runtime.mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Unmapped Planning Conflict",
    }));
    const effect = line.diagnostics().lastEffect;
    assert.equal(effect.locusProof, null);

    const merge = {
      source_branch_id: effect.optimistic.branchId,
      target_branch_id: 0,
    };
    const baseEnvelope = signals.history()
      .plan_merge_policy_preview_with_proof(merge);
    const resourceWithConflictHistory = runtime.mod.createResourceNamespace(
      null,
      {
        history() {
          return {
            plan_merge_policy_preview_with_proof(request) {
              return createConflictPreviewEnvelope(baseEnvelope, request);
            },
          };
        },
      },
    );

    const plan = resourceWithConflictHistory.branch.planEffectMerge({
      merge,
      effect,
    });

    assert.equal(plan.kind, "planned");
    assert.equal(plan.conflicts.kind, "nativeConflicts");
    assert.equal(plan.resourceEffect.rebaseArtifact.kind, "mappingUnavailable");
    assert.equal(
      plan.resourceEffect.rebaseArtifact.reason,
      "resourceTopologyMappingUnavailable",
    );
    assert.equal(plan.resourceEffect.rebaseArtifact.conflictCount, 1);
    assert.deepEqual(plan.resourceEffect.rebaseArtifact.conflicts, []);
    assert.equal(
      plan.resourceEffect.rebaseArtifact.native.sourceBranchId,
      effect.optimistic.branchId,
    );
    assert.equal(plan.resourceEffect.rebaseArtifact.native.targetBranchId, 0);
    assert.equal(plan.resourceEffect.rebaseArtifact.native.records.length, 1);
    assert.deepEqual(plan.resourceEffect.rebaseArtifact.resource.locus,
      effect.locus);
    assert.equal(plan.resourceEffect.rebaseArtifact.resource.topology, null);
    assert.equal(
      plan.resourceEffect.rebaseArtifact.proof.nativeMergePlanDigest,
      plan.proof.planDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge execution emits mapping-unavailable artifacts without stable locus proof", async () => {
  const runtime = await createRealResourceTestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-unavailable-execution");
    const line = createEffectCollectionLine(runtime);

    line.patch(runtime.mod.resourcePatch.itemAspect({
      itemId: "demo:1",
      aspect: "title",
      value: "Unmapped Execution Conflict",
    }));
    const effect = line.diagnostics().lastEffect;
    const merge = {
      source_branch_id: effect.optimistic.branchId,
      target_branch_id: 0,
    };
    const baseEnvelope = signals.history()
      .plan_merge_policy_preview_with_proof(merge);
    const resourceWithConflictHistory = runtime.mod.createResourceNamespace(
      null,
      {
        history() {
          return {
            merge_branches_policy_preview_with_proof(request) {
              return createConflictExecutionEnvelope(baseEnvelope, request);
            },
          };
        },
      },
    );

    const result = resourceWithConflictHistory.branch.mergeEffect({
      merge,
      effect,
    });

    assert.equal(result.kind, "merged");
    assert.equal(result.conflicts.kind, "nativeConflicts");
    assert.equal(result.resourceEffect.mergeArtifact.kind, "mappingUnavailable");
    assert.equal(
      result.resourceEffect.mergeArtifact.reason,
      "resourceTopologyMappingUnavailable",
    );
    assert.equal(result.resourceEffect.mergeArtifact.conflictCount, 1);
    assert.deepEqual(result.resourceEffect.mergeArtifact.conflicts, []);
    assert.equal(
      result.resourceEffect.mergeArtifact.native.sourceBranchId,
      effect.optimistic.branchId,
    );
    assert.equal(result.resourceEffect.mergeArtifact.native.records.length, 1);
    assert.deepEqual(result.resourceEffect.mergeArtifact.resource.locus,
      effect.locus);
    assert.equal(result.resourceEffect.mergeArtifact.resource.topology, null);
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.nativeMergeResultDigest,
      result.proof.resultDigest,
    );
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.nativeMergeLineageDigest,
      result.proof.lineageDigest,
    );
  } finally {
    await runtime.cleanup();
  }
});

function createEffectCollectionLine(runtime) {
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

function createConflictPreviewEnvelope(baseEnvelope, request) {
  return {
    ...baseEnvelope,
    plan: {
      ...baseEnvelope.plan,
      source_branch_id: request.source_branch_id,
      target_branch_id: request.target_branch_id,
      resolution_plan: createConflictResolutionPlan(request),
    },
  };
}

function createConflictExecutionEnvelope(baseEnvelope, request) {
  return {
    result: {
      source_branch: request.source_branch_id,
      target_branch: request.target_branch_id,
      schema_registry_digest: baseEnvelope.plan.schema_registry_digest,
      registry_bundle_digest: baseEnvelope.plan.registry_bundle_digest,
      lowered_strategy_bundle_digest:
        baseEnvelope.plan.lowered_strategy_bundle_digest,
      merge_kind: baseEnvelope.plan.merge_kind,
      selected_semantics: baseEnvelope.plan.selected_semantics,
      merged_snapshot_id: 1,
      source_snapshot_id: baseEnvelope.plan.source_snapshot_id,
      target_snapshot_id_before: baseEnvelope.plan.target_snapshot_id_before,
      target_snapshot_id_after: 2,
      lowered_merge_base: baseEnvelope.plan.lowered_merge_base,
      resolution_plan: createConflictResolutionPlan(request),
      records: [],
      counters: {
        source_only_count: 0,
        target_only_count: 0,
      },
    },
    proof: {
      proofSchemaVersion: baseEnvelope.proof.proofSchemaVersion,
      registryBundleDigest: baseEnvelope.proof.registryBundleDigest,
      resultDigest: "merge-result|unmapped-resource-execution",
      semanticsDigest: baseEnvelope.proof.semanticsDigest,
      loweredStrategyBundleDigest: baseEnvelope.proof.loweredStrategyBundleDigest,
      lineageDigest: "merge-lineage|unmapped-resource-execution",
      selectedStrategyDigest: baseEnvelope.proof.selectedStrategyDigest,
      selectedMergeBaseDigest: baseEnvelope.proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest:
        baseEnvelope.proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest:
        baseEnvelope.proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest:
        baseEnvelope.proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest:
        baseEnvelope.proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest:
        baseEnvelope.proof.selectedDeletionPolicyDigest,
    },
  };
}

function createConflictResolutionPlan(request) {
  return {
    source_branch_id: request.source_branch_id,
    target_branch_id: request.target_branch_id,
    divergence: "ConflictingOutputs",
    records: [{
      source_node: "resource.effect.source.unmapped",
      target_node: "resource.effect.target.unmapped",
      required_resolution: ["Manual"],
      supported_strategies: ["Manual"],
    }],
  };
}

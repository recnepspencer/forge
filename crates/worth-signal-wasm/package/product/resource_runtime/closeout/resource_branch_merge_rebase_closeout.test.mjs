import assert from "node:assert/strict";
import test from "node:test";

import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";
import { normalizeForProof } from "./resource_verification_package_helpers.mjs";

test("branch-native resource merge and rebase artifacts form one closeout verification package", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "closeout-merge-rebase");
    const groupedTasks = createGroupedTasks(signals);
    const groupedLine = groupedTasks.line({});

    await groupedLine.patch(groupedTasks.patch.item({
      itemId: "task:1",
      nextItem: { id: "task:1", group: "todo", title: "Grouped" },
    }));
    const groupedEffect = groupedLine.diagnostics().lastEffect;
    const groupedMerge = createMergeRequest(groupedEffect);
    const groupedPlan = signals.resource.branch.planEffectMerge({
      merge: groupedMerge,
      effect: groupedEffect,
    });
    const conflictResource = createResourceWithConflictPreview(
      runtime,
      signals.history().plan_merge_policy_preview_with_proof(groupedMerge),
    );
    const groupedConflictPlan = conflictResource.branch.planEffectMerge({
      merge: groupedMerge,
      effect: groupedEffect,
    });

    const rawLine = createRawTasks(runtime).line({ workspaceId: "demo" });
    createBranchHead(signals, "closeout-unmapped-rebase");
    await rawLine.patch(runtime.mod.resourcePatch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Raw",
    }));
    const rawEffect = rawLine.diagnostics().lastEffect;
    const rawMerge = createMergeRequest(rawEffect);
    const rawConflictResource = createResourceWithConflictPreview(
      runtime,
      signals.history().plan_merge_policy_preview_with_proof(rawMerge),
    );
    const rawConflictPlan = rawConflictResource.branch.planEffectMerge({
      merge: rawMerge,
      effect: rawEffect,
    });

    const groupedExecution = signals.resource.branch.mergeEffect({
      merge: groupedMerge,
      effect: groupedEffect,
    });
    const closeoutPackage = projectMergeRebaseCloseoutPackage({
      groupedPlan,
      groupedConflictPlan,
      groupedExecution,
      rawConflictPlan,
    });

    assert.deepEqual(closeoutPackage.hostRegion, {
      planKind: "rebaseAvailable",
      executionKind: "merged",
      resourceGranularity: "hostRegion",
      nativeMapping: "hostRegionMappedToNativeNode",
      hostRegion: {
        topology: "groupedCollection",
        lookup: "group-key-item-id",
        traversal: "single-group",
        reconstruction: "replaceGroupItem",
      },
      planDigestPresent: true,
      resultDigestPresent: true,
    });
    assert.deepEqual(closeoutPackage.stableConflict, {
      artifactKind: "conflict",
      conflictKind: "resourceMergeConflict",
      topology: "groupedCollection",
      effectLocus: "groupedCollection",
      nativeSourceNode: "resource.closeout.source",
      nativeTargetNode: "resource.closeout.target",
      nativeRequiredResolution: ["Manual"],
      proofDigestPresent: true,
    });
    assert.deepEqual(closeoutPackage.mappingUnavailable, {
      artifactKind: "mappingUnavailable",
      reason: "resourceTopologyMappingUnavailable",
      topology: null,
      sourceBranchId: rawEffect.optimistic.branchId,
      targetBranchId: 0,
      nativeRecordCount: 1,
      nativeSourceNode: "resource.closeout.source",
      nativeTargetNode: "resource.closeout.target",
      proofDigestPresent: true,
    });
  } finally {
    await runtime.cleanup();
  }
});

function createGroupedTasks(signals) {
  const response = signals.resource.response.grouped()({
    itemId: (task) => task.id,
    groupId: (task) => task.group,
    groupForItem: () => "todo",
    groups: (value) => value.groups,
    replaceGroups: (value, groups) => ({ ...value, groups }),
    replaceGroupItem: (value, groupId, itemId, nextItem) => ({
      ...value,
      groups: replaceGroupedItem(value.groups, groupId, itemId, nextItem),
    }),
    aspects: signals.resource.response.objectAspects()({ title: "title" }),
  });
  return signals.api({ effects: signals.resource.effects.branchNative() })
    .url("/closeout-merge-rebase-grouped")
    .response(response)
    .list({
      load: () => ({
        groups: {
          todo: [{ id: "task:1", group: "todo", title: "Loaded" }],
          done: [],
        },
      }),
    });
}

function createRawTasks(runtime) {
  const { mod, resource } = runtime;
  return resource.collection({
    params: mod.resourceParams(),
    normalizeParams: ({ workspaceId }) =>
      mod.resourceParamIdentity({ workspaceId }, workspaceId),
    effects: mod.resourceEffects.branchNative(),
    itemIdentity: (task) => task.id,
    reconcile: mod.resourceCollectionShape({
      items: (value) => value.items,
      replaceItems: (value, nextItems) => ({ ...value, items: nextItems }),
      aspects: mod.resourceItemAspects({
        title: {
          read: (task) => task.title,
          write: (task, title) => ({ ...task, title }),
        },
      }),
    }),
    load: () => ({ items: [{ id: "task:1", title: "Loaded" }] }),
  });
}

function createResourceWithConflictPreview(runtime, baseEnvelope) {
  return runtime.mod.createResourceNamespace(null, {
    history() {
      return {
        plan_merge_policy_preview_with_proof(request) {
          return createConflictPreviewEnvelope(baseEnvelope, request);
        },
      };
    },
  });
}

function createConflictPreviewEnvelope(baseEnvelope, request) {
  return {
    ...baseEnvelope,
    plan: {
      ...baseEnvelope.plan,
      source_branch_id: request.source_branch_id,
      target_branch_id: request.target_branch_id,
      resolution_plan: {
        source_branch_id: request.source_branch_id,
        target_branch_id: request.target_branch_id,
        divergence: "ConflictingOutputs",
        records: [{
          source_node: "resource.closeout.source",
          target_node: "resource.closeout.target",
          required_resolution: ["Manual"],
          supported_strategies: ["Manual"],
        }],
      },
    },
  };
}

function projectMergeRebaseCloseoutPackage(results) {
  const hostBinding = results.groupedPlan.resourceEffect.policyBinding;
  const stableConflict =
    results.groupedConflictPlan.resourceEffect.rebaseArtifact.conflicts[0];
  const unavailable = results.rawConflictPlan.resourceEffect.rebaseArtifact;
  return normalizeForProof({
    hostRegion: {
      planKind: results.groupedPlan.resourceEffect.rebaseArtifact.kind,
      executionKind: results.groupedExecution.resourceEffect.mergeArtifact.kind,
      resourceGranularity: hostBinding.resourceGranularity,
      nativeMapping: hostBinding.nativeMapping,
      hostRegion: {
        topology: hostBinding.hostRegion.topology,
        lookup: hostBinding.hostRegion.lookup,
        traversal: hostBinding.hostRegion.traversal,
        reconstruction: hostBinding.hostRegion.reconstruction,
      },
      planDigestPresent:
        typeof results.groupedPlan.proof.planDigest === "string",
      resultDigestPresent:
        typeof results.groupedExecution.proof.resultDigest === "string",
    },
    stableConflict: {
      artifactKind: results.groupedConflictPlan.resourceEffect.rebaseArtifact.kind,
      conflictKind: stableConflict.kind,
      topology: stableConflict.resource.topology,
      effectLocus: stableConflict.resource.effectLocus,
      nativeSourceNode: stableConflict.native.sourceNode,
      nativeTargetNode: stableConflict.native.targetNode,
      nativeRequiredResolution: stableConflict.native.requiredResolution,
      proofDigestPresent:
        typeof stableConflict.proof.nativeMergePlanDigest === "string",
    },
    mappingUnavailable: {
      artifactKind: unavailable.kind,
      reason: unavailable.reason,
      topology: unavailable.resource.topology,
      sourceBranchId: unavailable.native.sourceBranchId,
      targetBranchId: unavailable.native.targetBranchId,
      nativeRecordCount: unavailable.native.records.length,
      nativeSourceNode: unavailable.native.records[0].sourceNode,
      nativeTargetNode: unavailable.native.records[0].targetNode,
      proofDigestPresent:
        typeof unavailable.proof.nativeMergePlanDigest === "string",
    },
  });
}

function createMergeRequest(effect) {
  return {
    source_branch_id: effect.optimistic.branchId,
    target_branch_id: 0,
  };
}

function replaceGroupedItem(groups, groupId, itemId, nextItem) {
  return Object.fromEntries(
    Object.entries(groups).map(([key, items]) => [
      key,
      key === groupId
        ? items.map((task) => task.id === itemId ? nextItem : task)
        : items,
    ]),
  );
}

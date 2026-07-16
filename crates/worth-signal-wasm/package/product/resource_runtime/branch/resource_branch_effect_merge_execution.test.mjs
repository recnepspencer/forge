import assert from "node:assert/strict";
import test from "node:test";

import { createBranchHead } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("resource effect merge execution binds native result proof to resource loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-execution");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Executed Through Resource Effect",
    }));
    const effect = line.diagnostics().lastEffect;
    const result = signals.resource.branch.mergeEffect({
      merge: {
        source_branch_id: effect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect,
    });

    assert.equal(result.kind, "merged");
    assert.equal(result.sourceBranchId, effect.optimistic.branchId);
    assert.equal(result.targetBranchId, 0);
    assert.equal(typeof result.proof.resultDigest, "string");
    assert.equal(typeof result.proof.lineageDigest, "string");
    assert.equal(result.resourceEffect.effectId, effect.effectId);
    assert.deepEqual(result.resourceEffect.locus, effect.locus);
    assert.equal(result.resourceEffect.mergeArtifact.kind, "merged");
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.nativeMergeResultDigest,
      result.proof.resultDigest,
    );
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.nativeMergeLineageDigest,
      result.proof.lineageDigest,
    );
    assert.equal(
      result.resourceEffect.mergeArtifact.proof.resourceLocusDigest,
      effect.locusProof.effectLocusDigest,
    );
    assert.equal(
      result.resourceEffect.policyBinding.nativeMapping,
      "resourceAspectMappedToNativeNode",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge execution binds effect policy into native execution", async () => {
  const runtime = await createRealRequestRuntime();
  const capturedRequests = [];
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-execution-policy");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Execution Policy Bound",
    }));
    const effect = line.diagnostics().lastEffect;
    const merge = {
      source_branch_id: effect.optimistic.branchId,
      target_branch_id: 0,
    };
    const baseEnvelope = signals.history()
      .plan_merge_policy_preview_with_proof(merge);
    const resourceWithCapturedHistory = runtime.mod.createResourceNamespace(
      null,
      {
        history() {
          return {
            merge_branches_policy_preview_with_proof(request) {
              capturedRequests.push(request);
              return createExecutionEnvelopeFromPlan(baseEnvelope, request);
            },
          };
        },
      },
    );

    const result = resourceWithCapturedHistory.branch.mergeEffect({
      merge,
      effect,
    });

    assert.equal(result.kind, "merged");
    assert.equal(capturedRequests[0].conflict_policy_name,
      "signal.conflict.resolve-source-when-structure-matches");
    assert.equal(
      capturedRequests[0].conflict_isolation_policy_name,
      "signal.conflict-isolation.per-node",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge execution denies branch mismatch before native execution", async () => {
  const runtime = await createRealRequestRuntime();
  let executionCount = 0;
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-execution-denial");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Execution Denial",
    }));
    const effect = line.diagnostics().lastEffect;
    const unrelatedBranch = createBranchHead(
      signals,
      "feature/effect-merge-execution-unrelated",
    );
    const resourceWithExplodingHistory = runtime.mod.createResourceNamespace(
      null,
      {
        history() {
          return {
            merge_branches_policy_preview_with_proof() {
              executionCount += 1;
              throw new Error("native merge execution should not be reached");
            },
          };
        },
      },
    );

    const denied = resourceWithExplodingHistory.branch.mergeEffect({
      merge: {
        source_branch_id: unrelatedBranch.id,
        target_branch_id: 0,
      },
      effect,
    });

    assert.equal(executionCount, 0);
    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        `resource branch effect merge execution requires merge source branch "${unrelatedBranch.id}" to match effect optimistic branch "${effect.optimistic.branchId}" before native merge execution`,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge execution denials name the execution facade", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const denied = runtime.signals.resource.branch.mergeEffect({
      merge: {
        source_branch_id: 0,
        target_branch_id: 0,
      },
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        "resource.branch.mergeEffect(...) requires a resource effect envelope",
    });
  } finally {
    await runtime.cleanup();
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

function createExecutionEnvelopeFromPlan(baseEnvelope, request) {
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
      resolution_plan: null,
      records: [],
      counters: createEmptyMergeCounters(),
    },
    proof: {
      proofSchemaVersion: baseEnvelope.proof.proofSchemaVersion,
      registryBundleDigest: baseEnvelope.proof.registryBundleDigest,
      resultDigest: "merge-result|captured-resource-execution",
      semanticsDigest: baseEnvelope.proof.semanticsDigest,
      loweredStrategyBundleDigest: baseEnvelope.proof.loweredStrategyBundleDigest,
      lineageDigest: "merge-lineage|captured-resource-execution",
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

function createEmptyMergeCounters() {
  return {
    source_only_count: 0,
    target_only_count: 0,
  };
}

import assert from "node:assert/strict";
import test from "node:test";
import { createBranchHead, createRealResourceSignals } from "../runtime_fixture/real_resource_signals.mjs";
import { createRealRequestRuntime } from "../runtime_fixture/real_request_runtime.mjs";

test("resource branch namespace exposes product merge-plan summaries with proof digests", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "feature/resource-branch");
    const summary = runtime.signals.resource.branch.planMerge({
      source_branch_id: branch.id,
      target_branch_id: 0,
    });

    assert.equal(summary.kind, "planned");
    assert.equal(summary.sourceBranchId, branch.id);
    assert.equal(summary.targetBranchId, 0);
    assert.equal(typeof summary.selectedSemantics.strategy, "string");
    assert.equal(typeof summary.selectedSemantics.conflictPolicy, "string");
    assert.equal(Number.isInteger(summary.breadth.nodePlanCount), true);
    assert.equal(typeof summary.proof.planDigest, "string");
    assert.equal(typeof summary.proof.semanticsDigest, "string");
    assert.equal(typeof summary.proof.selectedConflictPolicyDigest, "string");
  } finally {
    await runtime.cleanup();
  }
});

test("resource branch merge-plan summaries deny unsupported or malformed branch requests", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = runtime.signals.history().current_branch();
    const denied = runtime.signals.resource.branch.planMerge({
      source_branch_id: -1,
      target_branch_id: branch.id,
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "mergePlanUnavailable",
      detail:
        "history.plan_merge_policy_preview_with_proof.source_branch_id expects a non-negative safe integer branch id",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource branch namespace binds merge plans to resource effect loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/resource-effect-merge");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Merged Through Resource Effect",
    }));
    const effect = line.diagnostics().lastEffect;
    const plan = signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: effect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect,
    });

    assert.equal(plan.kind, "planned");
    assert.equal(plan.sourceBranchId, effect.optimistic.branchId);
    assert.equal(plan.resourceEffect.effectId, effect.effectId);
    assert.deepEqual(plan.resourceEffect.locus, effect.locus);
    assert.equal(plan.resourceEffect.topology, "directArray");
    assert.equal(plan.resourceEffect.effectLocus, "itemAspect");
    assert.equal(plan.resourceEffect.rebase, "nativeMergePlan");
    assert.deepEqual(plan.resourceEffect.policyBinding, {
      source: "resourceEffectLocus",
      locusKind: "itemAspect",
      aspect: "title",
      hostRegion: null,
      resourceGranularity: "resourceAspect",
      nativeIsolationGranularity: "nativeNode",
      nativeMapping: "resourceAspectMappedToNativeNode",
      conflictPolicyName:
        "signal.conflict.resolve-source-when-structure-matches",
      conflictIsolationPolicyName: "signal.conflict-isolation.per-node",
    });
    assert.equal(
      plan.resourceEffect.conflictIsolation,
      plan.selectedSemantics.conflictIsolation,
    );
    assert.equal(
      plan.resourceEffect.proof.planDigest,
      plan.proof.planDigest,
    );
    assert.equal(
      plan.resourceEffect.proof.effectLocusDigest,
      effect.locusProof.effectLocusDigest,
    );
    assert.equal(plan.conflicts.kind, "none");
    assert.equal(plan.resourceEffect.rebaseArtifact.kind, "rebaseAvailable");
    assert.equal(plan.resourceEffect.rebaseArtifact.conflictCount, 0);
    assert.equal(
      plan.resourceEffect.rebaseArtifact.proof.resourceLocusDigest,
      effect.locusProof.effectLocusDigest,
    );
    assert.equal(
      plan.resourceEffect.rebaseArtifact.proof.policyBindingDigest,
      "resource-policy-binding|resourceEffectLocus|itemAspect|title|host-region:none|resourceAspect|nativeNode|resourceAspectMappedToNativeNode|signal.conflict.resolve-source-when-structure-matches|signal.conflict-isolation.per-node",
    );
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge planning binds effect policy into native previews", async () => {
  const runtime = await createRealRequestRuntime();
  const capturedRequests = [];
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-policy");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Policy Bound",
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
            plan_merge_policy_preview_with_proof(request) {
              capturedRequests.push(request);
              return createConflictPreviewEnvelope(baseEnvelope, request);
            },
          };
        },
      },
    );

    const plan = resourceWithCapturedHistory.branch.planEffectMerge({
      merge,
      effect,
    });

    assert.equal(plan.kind, "planned");
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

test("resource effect merge planning projects native conflicts onto resource loci", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-conflict");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Conflict Target",
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
    assert.equal(plan.conflicts.records.length, 1);
    assert.equal(plan.resourceEffect.rebaseArtifact.kind, "conflict");
    assert.equal(plan.resourceEffect.rebaseArtifact.conflictCount, 1);
    assert.equal(plan.resourceEffect.rebaseArtifact.proof.nativeMergePlanDigest,
      plan.proof.planDigest);
    assert.equal(plan.resourceEffect.rebaseArtifact.proof.resourceLocusDigest,
      effect.locusProof.effectLocusDigest);
    const conflict = plan.resourceEffect.rebaseArtifact.conflicts[0];
    assert.equal(conflict.kind, "resourceMergeConflict");
    assert.deepEqual(conflict.resource.locus, effect.locus);
    assert.equal(conflict.resource.topology, "directArray");
    assert.equal(conflict.resource.effectLocus, "itemAspect");
    assert.deepEqual(conflict.native.requiredResolution, ["Manual"]);
    assert.deepEqual(conflict.native.supportedStrategies, ["Manual"]);
    assert.equal(conflict.proof.conflictIsolationDigest,
      plan.proof.selectedConflictIsolationDigest);
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge planning denies forged effect envelopes", async () => {
  const runtime = await createRealResourceSignals();
  try {
    const branch = createBranchHead(runtime.signals, "feature/effect-merge-denial");
    const denied = runtime.signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: branch.id,
        target_branch_id: 0,
      },
      effect: {
        version: "resource-effect-envelope-v1",
        effectId: "effect-without-locus",
        profile: { rebase: "nativeMergePlan" },
        line: { runtimeLineId: 1 },
        locus: { kind: "itemAspect" },
        optimistic: { branchId: branch.id },
      },
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        "resource.branch.planEffectMerge(...) requires a runtime-issued resource effect envelope",
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge planning denies branch-mismatched effects", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-source");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Branch Bound",
    }));
    const effect = line.diagnostics().lastEffect;
    const unrelatedBranch = createBranchHead(signals, "feature/effect-merge-unrelated");
    const denied = signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: unrelatedBranch.id,
        target_branch_id: 0,
      },
      effect,
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        `resource branch effect merge planning requires merge source branch "${unrelatedBranch.id}" to match effect optimistic branch "${effect.optimistic.branchId}"`,
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge planning denies contradictory caller policies", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-policy-denial");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Policy Denial",
    }));
    const effect = line.diagnostics().lastEffect;
    const denied = signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: effect.optimistic.branchId,
        target_branch_id: 0,
        conflict_isolation_policy_name: "caller.conflicting-policy",
      },
      effect,
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        'resource.branch.planEffectMerge(...) requires conflict_isolation_policy_name "signal.conflict-isolation.per-node" for the resource effect locus',
    });
  } finally {
    await runtime.cleanup();
  }
});

test("resource effect merge planning denies tampered issued effect envelopes", async () => {
  const runtime = await createRealRequestRuntime();
  try {
    const { signals } = runtime;
    createBranchHead(signals, "feature/effect-merge-tampered");
    const tasks = createBranchMergeTasks(signals);
    const line = tasks.line({});

    await line.patch(tasks.patch.itemAspect({
      itemId: "task:1",
      aspect: "title",
      value: "Tamper Target",
    }));
    const effect = line.diagnostics().lastEffect;
    const denied = signals.resource.branch.planEffectMerge({
      merge: {
        source_branch_id: effect.optimistic.branchId,
        target_branch_id: 0,
      },
      effect: {
        ...effect,
        locus: { kind: "itemAspect", itemId: "task:2", aspect: "title" },
      },
    });

    assert.deepEqual(denied, {
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail:
        "resource.branch.planEffectMerge(...) requires a runtime-issued resource effect envelope",
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
    .url("/branch-merge-tasks")
    .response(response)
    .list({
      load: () => [{ id: "task:1", title: "Loaded" }],
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
          source_node: "resource.effect.source.title",
          target_node: "resource.effect.target.title",
          required_resolution: ["Manual"],
          supported_strategies: ["Manual"],
        }],
      },
    },
  };
}

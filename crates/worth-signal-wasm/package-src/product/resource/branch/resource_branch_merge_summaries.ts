import {
  createNativeMergeConflictSummary,
  createResourceEffectMergeExecutionArtifact,
  createResourceEffectRebaseArtifact,
} from "./resource_effect_merge_rebase_artifact.js";
import {
  createResourceEffectMergePolicyBinding,
} from "./resource_effect_merge_policy_binding.js";

function createMergePlanSummary(envelope) {
  const plan = envelope.plan;
  const proof = envelope.proof;
  return Object.freeze({
    kind: "planned",
    sourceBranchId: plan.source_branch_id,
    targetBranchId: plan.target_branch_id,
    mergeKind: plan.merge_kind,
    selectedSemantics: createSelectedSemanticsSummary(plan.selected_semantics),
    breadth: Object.freeze({
      nodeMapCount: plan.node_map.length,
      nodePlanCount: plan.node_plan.length,
      adoptionPlanCount: plan.adoption_core.length,
      conflictRecordCount: plan.resolution_plan?.records.length ?? 0,
    }),
    conflicts: createNativeMergeConflictSummary(plan.resolution_plan),
    proof: Object.freeze({
      proofSchemaVersion: proof.proofSchemaVersion,
      planDigest: proof.planDigest,
      semanticsDigest: proof.semanticsDigest,
      selectedStrategyDigest: proof.selectedStrategyDigest,
      selectedMergeBaseDigest: proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest: proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest: proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest: proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest: proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest: proof.selectedDeletionPolicyDigest,
    }),
  });
}

function createMergeExecutionSummary(envelope) {
  const result = envelope.result;
  const proof = envelope.proof;
  return Object.freeze({
    kind: "merged",
    sourceBranchId: result.source_branch,
    targetBranchId: result.target_branch,
    mergeKind: result.merge_kind,
    selectedSemantics: createSelectedSemanticsSummary(result.selected_semantics),
    breadth: Object.freeze({
      recordCount: result.records.length,
      sourceOnlyCount: result.counters.source_only_count,
      targetOnlyCount: result.counters.target_only_count,
      conflictRecordCount: result.resolution_plan?.records.length ?? 0,
    }),
    conflicts: createNativeMergeConflictSummary(result.resolution_plan),
    proof: Object.freeze({
      proofSchemaVersion: proof.proofSchemaVersion,
      resultDigest: proof.resultDigest,
      semanticsDigest: proof.semanticsDigest,
      lineageDigest: proof.lineageDigest,
      selectedStrategyDigest: proof.selectedStrategyDigest,
      selectedMergeBaseDigest: proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest: proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest: proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest: proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest: proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest: proof.selectedDeletionPolicyDigest,
    }),
  });
}

function createSelectedSemanticsSummary(selectedSemantics) {
  return Object.freeze({
    strategy: selectedSemantics.strategy_name,
    mergeBase: selectedSemantics.merge_base_name,
    conflictPolicy: selectedSemantics.conflict_policy_name,
    conflictIsolation: selectedSemantics.conflict_isolation_name,
    identityMatcher: selectedSemantics.identity_matcher_name,
    sourceOnlyPolicy: selectedSemantics.source_only_policy_name,
    deletionPolicy: selectedSemantics.deletion_policy_name,
  });
}

function createEffectMergePlanSummary(mergePlan, effect) {
  const policyBinding = createResourceEffectMergePolicyBinding(effect);
  return Object.freeze({
    ...mergePlan,
    resourceEffect: Object.freeze({
      effectId: effect.effectId,
      provenance: effect.provenance,
      family: effect.family,
      line: effect.line,
      locus: effect.locus,
      topology: effect.locusProof?.topology ?? null,
      effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
      rebase: effect.profile.rebase,
      conflictIsolation: mergePlan.selectedSemantics.conflictIsolation,
      policyBinding,
      rebaseArtifact: createResourceEffectRebaseArtifact(
        mergePlan,
        effect,
        policyBinding,
      ),
      proof: Object.freeze({
        planDigest: mergePlan.proof.planDigest,
        semanticsDigest: mergePlan.proof.semanticsDigest,
        effectLocusDigest: effect.locusProof?.effectLocusDigest ?? null,
        compiledLensDigest: effect.locusProof?.compiledLensDigest ?? null,
      }),
    }),
  });
}

function createEffectMergeExecutionSummary(mergeResult, effect) {
  const policyBinding = createResourceEffectMergePolicyBinding(effect);
  return Object.freeze({
    ...mergeResult,
    resourceEffect: Object.freeze({
      effectId: effect.effectId,
      provenance: effect.provenance,
      family: effect.family,
      line: effect.line,
      locus: effect.locus,
      topology: effect.locusProof?.topology ?? null,
      effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
      rebase: effect.profile.rebase,
      conflictIsolation: mergeResult.selectedSemantics.conflictIsolation,
      policyBinding,
      mergeArtifact: createResourceEffectMergeExecutionArtifact(
        mergeResult,
        effect,
        policyBinding,
      ),
      proof: Object.freeze({
        resultDigest: mergeResult.proof.resultDigest,
        semanticsDigest: mergeResult.proof.semanticsDigest,
        lineageDigest: mergeResult.proof.lineageDigest,
        effectLocusDigest: effect.locusProof?.effectLocusDigest ?? null,
        compiledLensDigest: effect.locusProof?.compiledLensDigest ?? null,
      }),
    }),
  });
}

export {
  createEffectMergeExecutionSummary,
  createEffectMergePlanSummary,
  createMergeExecutionSummary,
  createMergePlanSummary,
};

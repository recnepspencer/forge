import {
  createResourceHostRegionDigest,
  createResourcePolicyBindingDigest,
} from "./resource_effect_merge_policy_binding.js";

function createNativeMergeConflictSummary(resolutionPlan) {
  if (resolutionPlan === null || resolutionPlan === undefined) {
    return Object.freeze({
      kind: "none",
      divergence: null,
      records: Object.freeze([]),
    });
  }
  return Object.freeze({
    kind: "nativeConflicts",
    divergence: resolutionPlan.divergence,
    records: Object.freeze(
      resolutionPlan.records.map((record) => Object.freeze({
        sourceNode: record.source_node,
        targetNode: record.target_node,
        requiredResolution: Object.freeze([...record.required_resolution]),
        supportedStrategies: Object.freeze([...record.supported_strategies]),
      })),
    ),
  });
}

function createResourceEffectRebaseArtifact(mergePlan, effect, policyBinding) {
  const proof = createResourceEffectRebaseProof(
    mergePlan,
    effect,
    policyBinding,
  );
  if (mergePlan.conflicts.kind === "none") {
    return Object.freeze({
      kind: "rebaseAvailable",
      conflictCount: 0,
      conflicts: Object.freeze([]),
      proof,
    });
  }
  if (!hasStableResourceConflictMapping(effect)) {
    return createResourceEffectMappingUnavailableArtifact(
      mergePlan,
      effect,
      proof,
    );
  }
  return Object.freeze({
    kind: "conflict",
    conflictCount: mergePlan.conflicts.records.length,
    conflicts: Object.freeze(
      mergePlan.conflicts.records.map((record) =>
        createResourceEffectConflictArtifact(record, effect, proof),
      ),
    ),
    proof,
  });
}

function createResourceEffectMergeExecutionArtifact(
  mergeResult,
  effect,
  policyBinding,
) {
  const proof = createResourceEffectMergeExecutionProof(
    mergeResult,
    effect,
    policyBinding,
  );
  if (mergeResult.conflicts.kind === "none") {
    return Object.freeze({
      kind: "merged",
      conflictCount: 0,
      conflicts: Object.freeze([]),
      proof,
    });
  }
  if (!hasStableResourceConflictMapping(effect)) {
    return createResourceEffectMappingUnavailableArtifact(
      mergeResult,
      effect,
      proof,
    );
  }
  return Object.freeze({
    kind: "mergedWithConflictRecords",
    conflictCount: mergeResult.conflicts.records.length,
    conflicts: Object.freeze(
      mergeResult.conflicts.records.map((record) =>
        createResourceEffectConflictArtifact(record, effect, proof),
      ),
    ),
    proof,
  });
}

function createResourceEffectRebaseProof(mergePlan, effect, policyBinding) {
  return Object.freeze({
    nativeMergePlanDigest: mergePlan.proof.planDigest,
    nativeMergeSemanticsDigest: mergePlan.proof.semanticsDigest,
    resourceLocusDigest: createResourceLocusDigest(effect),
    aspectPolicyDigest: createResourceAspectPolicyDigest(policyBinding, mergePlan),
    policyBindingDigest: createResourcePolicyBindingDigest(policyBinding),
    conflictIsolationDigest: mergePlan.proof.selectedConflictIsolationDigest,
  });
}

function createResourceEffectMergeExecutionProof(mergeResult, effect, policyBinding) {
  return Object.freeze({
    nativeMergeResultDigest: mergeResult.proof.resultDigest,
    nativeMergeSemanticsDigest: mergeResult.proof.semanticsDigest,
    nativeMergeLineageDigest: mergeResult.proof.lineageDigest,
    resourceLocusDigest: createResourceLocusDigest(effect),
    aspectPolicyDigest: createResourceAspectPolicyDigest(policyBinding, mergeResult),
    policyBindingDigest: createResourcePolicyBindingDigest(policyBinding),
    conflictIsolationDigest: mergeResult.proof.selectedConflictIsolationDigest,
  });
}

function createResourceEffectConflictArtifact(record, effect, proof) {
  return Object.freeze({
    kind: "resourceMergeConflict",
    native: Object.freeze({
      sourceNode: record.sourceNode,
      targetNode: record.targetNode,
      requiredResolution: record.requiredResolution,
      supportedStrategies: record.supportedStrategies,
    }),
    resource: Object.freeze({
      effectId: effect.effectId,
      family: effect.family,
      line: effect.line,
      locus: effect.locus,
      topology: effect.locusProof?.topology ?? null,
      effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
    }),
    proof,
  });
}

function createResourceEffectMappingUnavailableArtifact(merge, effect, proof) {
  return Object.freeze({
    kind: "mappingUnavailable",
    reason: "resourceTopologyMappingUnavailable",
    conflictCount: merge.conflicts.records.length,
    conflicts: Object.freeze([]),
    native: Object.freeze({
      sourceBranchId: merge.sourceBranchId,
      targetBranchId: merge.targetBranchId,
      divergence: merge.conflicts.divergence,
      records: merge.conflicts.records,
    }),
    resource: createResourceEffectConflictResourceSummary(effect),
    detail:
      "native merge conflict evidence cannot be mapped to a stable response topology locus for this resource effect",
    proof,
  });
}

function hasStableResourceConflictMapping(effect) {
  return typeof effect.locusProof?.effectLocusDigest === "string" &&
    effect.locusProof.effectLocusDigest.length > 0 &&
    typeof effect.locusProof?.compiledLensDigest === "string" &&
    effect.locusProof.compiledLensDigest.length > 0;
}

function createResourceEffectConflictResourceSummary(effect) {
  return Object.freeze({
    effectId: effect.effectId,
    family: effect.family,
    line: effect.line,
    locus: effect.locus,
    topology: effect.locusProof?.topology ?? null,
    effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
  });
}

function createResourceLocusDigest(effect) {
  return effect.locusProof?.effectLocusDigest ??
    `resource-locus|${effect.effectId}|${JSON.stringify(effect.line)}|${JSON.stringify(effect.locus)}`;
}

function createResourceAspectPolicyDigest(policyBinding, mergePlan) {
  return [
    "resource-aspect-policy",
    policyBinding.locusKind,
    policyBinding.aspect === null ? "none" : policyBinding.aspect,
    createResourceHostRegionDigest(policyBinding.hostRegion),
    policyBinding.resourceGranularity,
    policyBinding.nativeIsolationGranularity,
    policyBinding.nativeMapping,
    mergePlan.selectedSemantics.conflictPolicy,
    mergePlan.selectedSemantics.conflictIsolation,
  ].join("|");
}

export {
  createNativeMergeConflictSummary,
  createResourceEffectMergeExecutionArtifact,
  createResourceEffectRebaseArtifact,
};

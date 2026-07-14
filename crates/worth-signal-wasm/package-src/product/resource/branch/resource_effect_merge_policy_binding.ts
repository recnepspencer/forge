function bindResourceEffectMergePolicy(merge, effect, operation) {
  const binding = createResourceEffectMergePolicyBinding(effect);
  denyContradictoryPolicy(
    merge.conflict_policy_name,
    binding.conflictPolicyName,
    "conflict_policy_name",
    operation,
  );
  denyContradictoryPolicy(
    merge.conflict_isolation_policy_name,
    binding.conflictIsolationPolicyName,
    "conflict_isolation_policy_name",
    operation,
  );
  return Object.freeze({
    ...merge,
    conflict_policy_name: binding.conflictPolicyName,
    conflict_isolation_policy_name: binding.conflictIsolationPolicyName,
  });
}

function createResourceEffectMergePolicyBinding(effect) {
  const hostRegion = createResourceEffectHostRegion(effect);
  return Object.freeze({
    source: "resourceEffectLocus",
    locusKind: effect.locus.kind,
    aspect: effect.locus?.aspect ?? null,
    hostRegion,
    resourceGranularity: classifyResourceEffectGranularity(effect),
    nativeIsolationGranularity: "nativeNode",
    nativeMapping: classifyNativeIsolationMapping(effect, hostRegion),
    conflictPolicyName:
      "signal.conflict.resolve-source-when-structure-matches",
    conflictIsolationPolicyName: "signal.conflict-isolation.per-node",
  });
}

function denyContradictoryPolicy(actual, expected, field, operation) {
  if (actual === undefined || actual === null || actual === expected) {
    return;
  }
  throw new TypeError(
    `${operation} requires ${field} "${expected}" for the resource effect locus`,
  );
}

function classifyResourceEffectGranularity(effect) {
  if (isHostDeclaredRegionEffect(effect)) {
    return "hostRegion";
  }
  switch (effect.locus.kind) {
    case "itemAspect":
    case "jsonItemAspect":
      return "resourceAspect";
    case "item":
    case "membership":
    case "entityStore":
    case "connection":
    case "discriminatedTuple":
    case "groupedCollection":
    case "mapCollection":
    case "namedCollection":
    case "recursiveTree":
    case "sparsePage":
      return "resourceItem";
    default:
      return "resourceLine";
  }
}

function classifyNativeIsolationMapping(effect, hostRegion) {
  if (effect.locus.kind === "itemAspect" || effect.locus.kind === "jsonItemAspect") {
    return "resourceAspectMappedToNativeNode";
  }
  if (hostRegion !== null) {
    return "hostRegionMappedToNativeNode";
  }
  return "resourceLocusMappedToNativeNode";
}

function isHostDeclaredRegionEffect(effect) {
  return createResourceEffectHostRegion(effect) !== null;
}

function createResourceEffectHostRegion(effect) {
  const cost = effect.locusProof?.cost;
  if (cost === undefined || cost === null || effect.locusProof.patchScope === "line") {
    return null;
  }
  if (cost.traversal === "item-scope" || cost.traversal === "aspect-scope") {
    return null;
  }
  return Object.freeze({
    source: "responseLocusProofCost",
    topology: effect.locusProof.topology,
    lookup: cost.lookup,
    traversal: cost.traversal,
    reconstruction: cost.reconstruction,
  });
}

function createResourcePolicyBindingDigest(policyBinding) {
  return [
    "resource-policy-binding",
    policyBinding.source,
    policyBinding.locusKind,
    policyBinding.aspect === null ? "none" : policyBinding.aspect,
    createResourceHostRegionDigest(policyBinding.hostRegion),
    policyBinding.resourceGranularity,
    policyBinding.nativeIsolationGranularity,
    policyBinding.nativeMapping,
    policyBinding.conflictPolicyName,
    policyBinding.conflictIsolationPolicyName,
  ].join("|");
}

function createResourceHostRegionDigest(hostRegion) {
  if (hostRegion === null) {
    return "host-region:none";
  }
  return [
    "host-region",
    hostRegion.topology,
    hostRegion.lookup,
    hostRegion.traversal,
    hostRegion.reconstruction,
  ].join("|");
}

export {
  bindResourceEffectMergePolicy,
  createResourceHostRegionDigest,
  createResourceEffectMergePolicyBinding,
  createResourcePolicyBindingDigest,
};

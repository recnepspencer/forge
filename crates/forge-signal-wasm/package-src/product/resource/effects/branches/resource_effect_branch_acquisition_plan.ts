import { requireResourceEffectDependencySet } from "../dependencies/resource_effect_dependency_set.js";

const RESOURCE_EFFECT_BRANCH_ACQUISITION_PLAN = Symbol(
  "forgeSignal.resourceEffectBranchAcquisitionPlan",
);

function createResourceEffectBranchAcquisitionPlan(options) {
  const dependencySet = requireResourceEffectDependencySet(
    options.dependencySet,
  );
  const derivedBasisRequired = dependencySet.cardinality > 0;
  return Object.freeze({
    [RESOURCE_EFFECT_BRANCH_ACQUISITION_PLAN]:
      "resourceEffectBranchAcquisitionPlan",
    effectId: options.effectId,
    effectBranchName: `resource-effect:${options.effectId}`,
    canonicalBasis: options.canonicalBasis,
    dependencySet,
    dependencyBasis: Object.freeze({
      kind: derivedBasisRequired
        ? "derivedDependencyBasis"
        : "canonicalBranchHead",
      nativeParentBranchId: options.canonicalBasis.branchId,
      semanticDependencyIds: dependencySet.dependencyIds,
      value: derivedBasisRequired ? options.dependencyBasisValue : null,
      proofDigest: JSON.stringify([
        options.canonicalBasis.authoredStateDigest,
        dependencySet.proofDigest,
      ]),
    }),
    lifecycle: "Planned",
    counters: Object.freeze({
      effectLookupCount: dependencySet.cardinality,
      dependencyTraversalCount: options.dependencyTraversalCount,
      branchForkCount: derivedBasisRequired ? 2 : 1,
      basisMaterializationCount: derivedBasisRequired ? 1 : 0,
    }),
  });
}

function requireResourceEffectBranchAcquisitionPlan(value) {
  if (
    !value
    || value[RESOURCE_EFFECT_BRANCH_ACQUISITION_PLAN]
      !== "resourceEffectBranchAcquisitionPlan"
  ) {
    throw new TypeError(
      "resource effect branch execution requires a planned acquisition",
    );
  }
  return value;
}

export {
  createResourceEffectBranchAcquisitionPlan,
  requireResourceEffectBranchAcquisitionPlan,
};

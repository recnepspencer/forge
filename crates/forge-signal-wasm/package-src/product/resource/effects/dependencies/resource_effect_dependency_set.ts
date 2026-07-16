const RESOURCE_EFFECT_DEPENDENCY_SET_BRAND = Symbol(
  "forgeSignal.resourceEffectDependencySet",
);

function createResourceEffectDependencySet(options) {
  const effectId = requireEffectId(options.effectId, "effect dependency owner");
  const dependencyIds = canonicalDependencyIds(options.dependencies ?? []);
  const closeoutPolicy = requireCloseoutPolicy(
    options.closeoutPolicy,
    dependencyIds.length,
  );
  if (dependencyIds.includes(effectId)) {
    throw dependencyDenial("selfDependency", effectId, effectId);
  }
  const dependencies = dependencyIds.map((dependencyId) => {
    const dependency = options.lookupEffect(dependencyId);
    if (dependency === null) {
      throw dependencyDenial("unknownDependency", effectId, dependencyId);
    }
    if (dependency.lifecycle === "Retired") {
      throw dependencyDenial("retiredDependency", effectId, dependencyId);
    }
    if (
      dependency.canonicalGeneration
      !== options.canonicalGeneration
    ) {
      throw dependencyDenial(
        "generationIncompatible",
        effectId,
        dependencyId,
      );
    }
    if (options.dependsTransitivelyOn(dependencyId, effectId)) {
      throw dependencyDenial("dependencyCycle", effectId, dependencyId);
    }
    return Object.freeze({
      effectId: dependencyId,
      branchId: dependency.branch.branch.createdBasis.branchId,
      authoredStateDigest:
        dependency.branch.branch.createdBasis.authoredStateDigest,
      canonicalGeneration: dependency.canonicalGeneration,
    });
  });
  return Object.freeze({
    [RESOURCE_EFFECT_DEPENDENCY_SET_BRAND]: "resourceEffectDependencySet",
    effectId,
    dependencies: Object.freeze(dependencies),
    dependencyIds: Object.freeze(dependencyIds),
    cardinality: dependencyIds.length,
    closeoutPolicy,
    proofDigest: JSON.stringify([
      effectId,
      options.canonicalGeneration,
      closeoutPolicy,
      ...dependencies.map((dependency) => [
        dependency.effectId,
        dependency.branchId,
        dependency.authoredStateDigest,
      ]),
    ]),
  });
}

function requireCloseoutPolicy(value, dependencyCount) {
  const expected = dependencyCount === 0
    ? "independent"
    : "cancelOnDependencyRejection";
  if (value !== expected) {
    throw new TypeError(
      `resource effect dependency closeout policy must be ${expected}`,
    );
  }
  return value;
}

function requireResourceEffectDependencySet(value) {
  if (
    !value
    || value[RESOURCE_EFFECT_DEPENDENCY_SET_BRAND]
      !== "resourceEffectDependencySet"
  ) {
    throw new TypeError(
      "resource effect branch acquisition requires a planned dependency set",
    );
  }
  return value;
}

function canonicalDependencyIds(values) {
  if (!Array.isArray(values)) {
    throw new TypeError("resource effect dependencies must be an array");
  }
  return [...new Set(values.map((value) => requireEffectId(
    typeof value === "string" ? value : value?.effectId,
    "resource effect dependency",
  )))].sort();
}

function requireEffectId(value, label) {
  if (typeof value !== "string" || value.length === 0) {
    throw new TypeError(`${label} requires a non-empty effect id`);
  }
  return value;
}

function dependencyDenial(reason, effectId, dependencyId) {
  const error = new TypeError(
    `resource effect ${effectId} dependency ${dependencyId} denied: ${reason}`,
  );
  error.name = "ResourceEffectDependencyDenial";
  error.code = reason;
  error.effectId = effectId;
  error.dependencyId = dependencyId;
  return error;
}

export {
  createResourceEffectDependencySet,
  requireResourceEffectDependencySet,
};

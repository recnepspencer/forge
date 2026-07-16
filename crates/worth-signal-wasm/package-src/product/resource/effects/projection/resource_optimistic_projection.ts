const RESOURCE_OPTIMISTIC_PROJECTION_PLAN = Symbol(
  "WORTHSignal.resourceOptimisticProjectionPlan",
);

function planResourceOptimisticProjection(options) {
  const refresh = options.materializeProjection();
  const projectedValue = refresh.projectedValue;
  const affectedEffectIds = options.affectedEffectIds ?? [];
  const affectedLocusKeys = new Set(options.affectedLocusKeys ?? []);
  return Object.freeze({
    [RESOURCE_OPTIMISTIC_PROJECTION_PLAN]:
      "resourceOptimisticProjectionPlan",
    canonicalBasis: options.canonicalBasis,
    canonicalValue: options.canonicalValue,
    projectedValue,
    openEffectCount: options.openEffectCount,
    affectedEffectIds: Object.freeze([...affectedEffectIds]),
    affectedLocusKeys: Object.freeze([...affectedLocusKeys].sort()),
    projectionDigest: canonicalDigest([
      options.canonicalBasis.authoredStateDigest,
      projectedValue,
      options.openEffectIdentity,
    ]),
    strategy: refresh.strategy,
    counters: Object.freeze({
      openEffectLookupCount: refresh.openEffectLookupCount,
      dependencyTraversalCount: refresh.dependencyTraversalCount,
      affectedEffectCount: affectedEffectIds.length,
      affectedLocusCount: affectedLocusKeys.size,
      reconstructionCount: refresh.reconstructionCount,
      fallbackBreadth: refresh.fallbackBreadth,
    }),
  });
}

async function executeResourceOptimisticProjection(
  history,
  rawPlan,
  authoredSignalIds,
  previousProjection,
) {
  const plan = requireProjectionPlan(rawPlan);
  let retiredProjection = null;
  if (previousProjection !== null) {
    const current = await history.current_branch();
    if (Number(current.id) === Number(previousProjection.branch.id)) {
      await history.switch_branch(plan.canonicalBasis.branchId);
    }
    const liveProjectionBasis = await history.worker_branch_basis(
      previousProjection.branch.id,
    );
    retiredProjection = await history.retire_branch({
      branchId: previousProjection.branch.id,
      expectedBasis: liveProjectionBasis,
      reason: "projectionRebuild",
    });
  }
  if (plan.openEffectCount === 0) {
    const current = await history.current_branch();
    if (Number(current.id) !== Number(plan.canonicalBasis.branchId)) {
      await history.switch_branch(plan.canonicalBasis.branchId);
    }
    return Object.freeze({
      kind: "canonical",
      branch: null,
      basis: plan.canonicalBasis,
      projectedValue: plan.canonicalValue,
      projectionDigest: plan.projectionDigest,
      retiredProjection,
      plan,
      canonicalAuthority: false,
    });
  }
  const fork = await history.fork_branch({
    name: "resource-effect-projection",
    parentBranchId: plan.canonicalBasis.branchId,
    expectedParentBasis: plan.canonicalBasis,
  });
  const applied = await history.apply_transaction_to_branch({
    branchId: fork.branch.id,
    expectedBasis: fork.createdBasis,
    transactionOps: authoredSignalIds.map((id) => ({
      kind: "set",
      id,
      value: plan.projectedValue,
    })),
  });
  await history.switch_branch(fork.branch.id);
  return Object.freeze({
    kind: "derivedEffectProjectionBranch",
    branch: fork.branch,
    basis: applied.afterBasis,
    projectedValue: plan.projectedValue,
    projectionDigest: plan.projectionDigest,
    affectedEffectIds: plan.affectedEffectIds,
    retiredProjection,
    plan,
    canonicalAuthority: false,
    detail:
      "visible resource truth is a disposable projection rebuilt from canonical truth and open effects",
  });
}

function assertProjectionCannotAuthorizeCanonicalMerge(projection) {
  if (projection?.kind === "derivedEffectProjectionBranch") {
    const error = new TypeError(
      "derived resource effect projection branches cannot authorize canonical merge",
    );
    error.name = "ResourceProjectionAuthorityDenial";
    error.code = "projectionIsNotCanonicalAuthority";
    throw error;
  }
}

function requireProjectionPlan(value) {
  if (
    !value
    || value[RESOURCE_OPTIMISTIC_PROJECTION_PLAN]
      !== "resourceOptimisticProjectionPlan"
  ) {
    throw new TypeError("optimistic projection execution requires a sealed plan");
  }
  return value;
}

function canonicalDigest(value) {
  return JSON.stringify(canonicalize(value));
}

function canonicalize(value) {
  if (Array.isArray(value)) {
    return value.map(canonicalize);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  const canonical = {};
  for (const key of Object.keys(value).sort()) {
    canonical[key] = canonicalize(value[key]);
  }
  return canonical;
}

export {
  assertProjectionCannotAuthorizeCanonicalMerge,
  executeResourceOptimisticProjection,
  planResourceOptimisticProjection,
};

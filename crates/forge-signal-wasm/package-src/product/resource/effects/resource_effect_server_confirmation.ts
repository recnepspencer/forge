function createResourceEffectServerConfirmation(effectPlan, currentLocus, patch) {
  if (effectPlan.admissionKind !== "delivery") {
    return Object.freeze({
      kind: "notApplicable",
      detail: "local resource effects await server confirmation",
    });
  }
  const previousEffect = effectPlan.previousEffect;
  if (previousEffect?.optimistic?.kind !== "applied") {
    return Object.freeze({
      kind: "independentServerTruth",
      previousEffectId: previousEffect?.effectId ?? null,
      detail:
        "server delivery committed without consuming a pending speculative resource effect",
    });
  }
  const locusMatches = areEffectLociEquivalent(previousEffect.locus, currentLocus);
  if (locusMatches && patch.valueChanged === false) {
    return Object.freeze({
      kind: "preservedSpeculativeTruth",
      previousEffectId: previousEffect.effectId,
      previousPlanId: previousEffect.plan.planId,
      previousBranchId: previousEffect.optimistic.branchId,
      previousSnapshotId: previousEffect.optimistic.snapshotId,
      locusMatches,
      detail:
        "server delivery confirmed the visible truth already produced by the pending speculative resource effect",
    });
  }
  return Object.freeze({
    kind: "consumedCanonicalServerTruth",
    previousEffectId: previousEffect.effectId,
    previousPlanId: previousEffect.plan.planId,
    previousBranchId: previousEffect.optimistic.branchId,
    previousSnapshotId: previousEffect.optimistic.snapshotId,
    locusMatches,
    valueChanged: patch.valueChanged,
    detail:
      "server delivery consumed canonical server truth after a pending speculative resource effect",
  });
}

function areEffectLociEquivalent(previousLocus, currentLocus) {
  if (previousLocus.kind !== currentLocus.kind) {
    return false;
  }
  switch (currentLocus.kind) {
    case "line":
    case "broadResponse":
    case "detailResponse":
    case "summaryResponse":
    case "basis":
    case "invalidation":
      return true;
    case "item":
    case "membership":
    case "connection":
    case "discriminatedTuple":
    case "entityStore":
    case "groupedCollection":
    case "mapCollection":
    case "namedCollection":
    case "recursiveTree":
    case "sparsePage":
      return previousLocus.itemId === currentLocus.itemId;
    case "itemAspect":
    case "jsonItemAspect":
      return (
        previousLocus.itemId === currentLocus.itemId
        && previousLocus.aspect === currentLocus.aspect
      );
    case "summary":
      return previousLocus.summary === currentLocus.summary;
    default:
      throw new TypeError(
        `resource effect server confirmation cannot compare locus "${currentLocus.kind}"`,
      );
  }
}

export { createResourceEffectServerConfirmation };

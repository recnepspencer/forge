import { createLinePatchInverseDescriptor } from "../../lines/actions/line_patch_inverse.js";

function materializeResourceProjection(options) {
  if (options.previousProjection === null || options.forceBroadRebuild) {
    return materializeBroadProjection(options, 0);
  }
  let projectedValue = options.previousProjection.projectedValue;
  let visitedEffectCount = 0;
  for (const refresh of options.locusRefreshes) {
    const reset = tryCreateCanonicalLocusReset(options, refresh);
    if (reset === null || reset.patch.kind === "replace") {
      return materializeBroadProjection(options, 1);
    }
    projectedValue = options.applyPatch(
      reset.patch,
      projectedValue,
    ).nextValue;
    for (const effect of refresh.openEffects) {
      projectedValue = options.applyPatch(
        effect.patchIntent,
        projectedValue,
      ).nextValue;
      visitedEffectCount += 1;
    }
  }
  return Object.freeze({
    projectedValue,
    strategy: "affectedLocusRebuild",
    openEffectLookupCount: visitedEffectCount,
    dependencyTraversalCount: options.affectedEffectCount,
    reconstructionCount: options.locusRefreshes.length,
    fallbackBreadth: 0,
  });
}

function tryCreateCanonicalLocusReset(options, refresh) {
  try {
    return createLinePatchInverseDescriptor(
      options.materialization,
      refresh.templatePatch,
      options.canonicalValue,
    );
  } catch {
    return null;
  }
}

function materializeBroadProjection(options, fallbackBreadth) {
  const orderedEffects = options.loadAllOpenEffects();
  if (
    options.previousProjection === null
    && orderedEffects.length === 1
    && orderedEffects[0].dependencySet.cardinality === 0
  ) {
    return Object.freeze({
      projectedValue: orderedEffects[0].effectValue,
      strategy: "measuredBroadRebuild",
      openEffectLookupCount: 1,
      dependencyTraversalCount: 0,
      reconstructionCount: 0,
      fallbackBreadth,
    });
  }
  let projectedValue = options.canonicalValue;
  for (const effect of orderedEffects) {
    projectedValue = options.applyPatch(
      effect.patchIntent,
      projectedValue,
    ).nextValue;
  }
  return Object.freeze({
    projectedValue,
    strategy: "measuredBroadRebuild",
    openEffectLookupCount: orderedEffects.length,
    dependencyTraversalCount: orderedEffects.length,
    reconstructionCount: 1,
    fallbackBreadth,
  });
}

export { materializeResourceProjection };

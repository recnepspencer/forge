async function drainReadyRecordedConfirmations(options) {
  const settled = [];
  const pending = options.confirmedEffectIds.flatMap(
    (effectId) => options.index.reverseDependents(effectId),
  );
  const visited = new Set();
  let position = 0;
  while (position < pending.length) {
    const effectId = pending[position++];
    if (visited.has(effectId)) continue;
    visited.add(effectId);
    const effect = options.index.get(effectId);
    if (effect === null || !recordedConfirmationIsReady(
      effect,
      options.confirmedEffects,
    )) {
      continue;
    }
    const result = await options.confirmEffect(
      effect,
      effect.recordedSettlement,
      effect.settlementToken,
    );
    settled.push(options.settlements.terminal(effect.settlementToken, result));
    pending.push(...options.index.reverseDependents(effect.effectId));
  }
  return settled;
}

function recordedConfirmationIsReady(effect, confirmedEffects) {
  return effect.lifecycle === "ResponseRecorded"
    && effect.dependencySet.dependencyIds.every(
      (dependencyId) => confirmedEffects.has(dependencyId),
    );
}

export { drainReadyRecordedConfirmations };

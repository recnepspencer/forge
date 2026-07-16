function createRejectedEffectCheckpoint(effectId, retired, canonicalValue) {
  return Object.freeze({
    kind: "rejectedNativeRetirement",
    effectId,
    retired: Object.freeze(retired),
    canonicalValue,
  });
}

function createRejectedEffectResult(checkpoint, projection) {
  return Object.freeze({
    kind: "rejectedAndRetired",
    effectId: checkpoint.effectId,
    retired: checkpoint.retired,
    canonicalValue: checkpoint.canonicalValue,
    projection,
  });
}

async function finalizeRejectedEffectCheckpoint(options) {
  const checkpoint = options.checkpoint;
  const projection = await options.rebuildProjection(
    checkpoint.canonicalValue,
    checkpoint.retired.map((entry) => entry.effectId),
  );
  for (const retiredEffect of checkpoint.retired) {
    if (retiredEffect.effectId === checkpoint.effectId) continue;
    options.settlements.terminalCancellation(
      retiredEffect.effectId,
      Object.freeze({
        kind: "dependencyCancelled",
        effectId: retiredEffect.effectId,
        causedByEffectId: checkpoint.effectId,
        retirement: retiredEffect.retirement,
      }),
    );
  }
  return options.settlements.terminal(
    options.settlementToken,
    createRejectedEffectResult(checkpoint, projection),
  );
}

export {
  createRejectedEffectCheckpoint,
  finalizeRejectedEffectCheckpoint,
};

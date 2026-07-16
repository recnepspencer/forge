function createConfirmedEffectCheckpoint(effect, settlement, confirmation) {
  return Object.freeze({
    kind: "confirmedNativeCloseout",
    effect,
    settlement,
    confirmation,
  });
}

function createConfirmedEffectResult(effect, confirmation) {
  return Object.freeze({
    kind: confirmation.reconciliation.conflict.kind === "superseded"
      ? "supersededAndRetired"
      : "merged",
    effectId: effect.effectId,
    canonicalValue: confirmation.canonicalValue,
    reconciliation: confirmation.reconciliation,
    retirement: Object.freeze({
      retiredEffect: confirmation.closeout.effectRetirement,
      retiredDependencyBasis:
        confirmation.closeout.dependencyBasisRetirement,
    }),
    closeout: confirmation.closeout,
    closeoutPlan: confirmation.closeoutPlan,
  });
}

export {
  createConfirmedEffectCheckpoint,
  createConfirmedEffectResult,
};

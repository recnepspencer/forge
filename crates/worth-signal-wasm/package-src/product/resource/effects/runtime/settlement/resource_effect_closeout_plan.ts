const RESOURCE_EFFECT_CLOSEOUT_PLAN = Symbol(
  "WORTHSignal.resourceEffectCloseoutPlan",
);

function planResourceEffectCloseout(options) {
  requireCloseoutInputs(options);
  const superseded = options.reconciliation.conflict.kind === "superseded";
  return Object.freeze({
    [RESOURCE_EFFECT_CLOSEOUT_PLAN]: "resourceEffectCloseoutPlan",
    effectId: options.effect.effectId,
    lifecycle: "CloseoutEligible",
    terminalKind: superseded ? "SupersededAndRetired" : "Merged",
    canonicalTransaction: Object.freeze({
      branchId: options.canonicalBasis.branchId,
      expectedBasis: options.canonicalBasis,
      transactionOps: Object.freeze(options.authoredSignalIds.map((id) =>
        Object.freeze({
          kind: "set",
          id,
          value: options.reconciliation.canonicalValue,
        }))),
    }),
    effectRetirement: Object.freeze({
      branchId: options.effect.branch.branch.branch.id,
      expectedBasis: options.effect.branch.branch.appliedBasis,
      reason: superseded ? "superseded" : "merged",
    }),
    dependencyBasisRetirement:
      options.effect.branch.dependencyBasisBranch === null
        ? null
        : Object.freeze({
            branchId:
              options.effect.branch.dependencyBasisBranch.branch.id,
            expectedBasis:
              options.effect.branch.dependencyBasisBranch.appliedBasis,
            reason: "dependencyCancellation",
          }),
    counters: Object.freeze({
      canonicalTransactionCount: 1,
      effectRetirementCount: 1,
      dependencyBasisRetirementCount:
        options.effect.branch.dependencyBasisBranch === null ? 0 : 1,
    }),
  });
}

function executeResourceEffectCloseout(history, plan) {
  requireCloseoutPlan(plan);
  return history.closeout_effect_branch({
    canonicalTransaction: plan.canonicalTransaction,
    effectRetirement: plan.effectRetirement,
    dependencyBasisRetirement: plan.dependencyBasisRetirement,
  });
}

function requireCloseoutInputs(options) {
  if (!options?.effect || typeof options.effect.effectId !== "string") {
    throw new TypeError("resource effect closeout requires an admitted effect");
  }
  if (!options.canonicalBasis || options.canonicalBasis.branchId == null) {
    throw new TypeError("resource effect closeout requires canonical branch basis");
  }
  if (!options.reconciliation || !("canonicalValue" in options.reconciliation)) {
    throw new TypeError("resource effect closeout requires reconciliation truth");
  }
}

function requireCloseoutPlan(plan) {
  if (
    !plan
    || plan[RESOURCE_EFFECT_CLOSEOUT_PLAN] !== "resourceEffectCloseoutPlan"
  ) {
    throw new TypeError("resource effect closeout execution requires a sealed plan");
  }
}

export {
  executeResourceEffectCloseout,
  planResourceEffectCloseout,
};

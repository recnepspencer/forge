import { executeResourceEffectSettlement } from "../../effects/runtime/settlement/resource_effect_settlement_execution.js";

function executeLineEffectRollback(materialization, historyRead, effectId) {
  const effect = materialization.effectBranchDag.effect(effectId);
  if (effect === null || effect.lifecycle === "Retired") {
    return Promise.resolve(createUnavailableRollbackResult(
      historyRead.basis,
      effectId,
      effect === null ? "unknownEffect" : "effectAlreadySettled",
    ));
  }
  return executeResourceEffectSettlement(
    materialization,
    effectId,
    Object.freeze({ kind: "rejected", responseId: null }),
  );
}

function executeLineLastEffectRollback(materialization, historyRead) {
  const effect = materialization.effectBranchDag.lastOpenEffect();
  if (effect === null) {
    return Promise.resolve(createUnavailableRollbackResult(
      historyRead.basis,
      null,
      "noOpenEffect",
    ));
  }
  return executeLineEffectRollback(
    materialization,
    historyRead,
    effect.effectId,
  );
}

function createUnavailableRollbackResult(basis, effectId, reason) {
  const detail = reason === "noOpenEffect"
    ? "resource effect rollback is unavailable because the line has no open resource effect"
    : reason === "effectAlreadySettled"
      ? `resource effect ${effectId} is already settled`
      : `resource effect ${effectId} is unknown to this line`;
  return Object.freeze({
    kind: "unavailable",
    reason,
    detail,
    effectId,
    basisCurrentId: basis.currentBasisId,
    basisAdvanceCount: basis.advanceCount,
  });
}

export {
  executeLineEffectRollback,
  executeLineLastEffectRollback,
};

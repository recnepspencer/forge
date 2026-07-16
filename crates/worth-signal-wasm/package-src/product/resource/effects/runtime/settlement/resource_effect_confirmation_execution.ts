import { applyPatchValue } from "../../../lines/actions/line_patch_execution.js";
import {
  executePlannedResourceEffectReconciliation,
  planResourceEffectReconciliation,
} from "../../../reconciliation/resource_effect_reconciliation_plan.js";
import {
  executeResourceEffectCloseout,
  planResourceEffectCloseout,
} from "./resource_effect_closeout_plan.js";

async function executeResourceEffectConfirmation(options) {
  const nativeMergeProof = await options.history.plan_merge_branches_with_proof(
    options.effect.branch.branch.branch.id,
    options.canonicalBranchId,
  );
  const settlingEffect = Object.freeze({
    ...options.effect,
    serverRevision: options.settlement.serverRevision ?? null,
  });
  const materialize = (patch, value) =>
    applyPatchValue(options.materialization, patch, value);
  const plan = planResourceEffectReconciliation({
    effect: settlingEffect,
    canonicalValue: options.canonicalValue,
    serverPatch: options.settlement.serverPatch ?? null,
    serverRevision: options.settlement.serverRevision ?? null,
    sameLocusOpenEffects: options.sameLocusConfirmedEffects,
    nativeMergeProof,
    applyPatch: materialize,
  });
  const reconciliation = executePlannedResourceEffectReconciliation(
    plan,
    materialize,
  );
  const canonicalBasis = await options.history.worker_branch_basis(
    options.canonicalBranchId,
  );
  const closeoutPlan = planResourceEffectCloseout({
    effect: options.effect,
    canonicalBasis,
    reconciliation,
    authoredSignalIds: options.authoredSignalIds,
  });
  const closeout = await executeResourceEffectCloseout(
    options.history,
    closeoutPlan,
  );
  return Object.freeze({
    canonicalValue: reconciliation.canonicalValue,
    reconciliation,
    settlingEffect,
    closeout,
    closeoutPlan,
  });
}

export { executeResourceEffectConfirmation };

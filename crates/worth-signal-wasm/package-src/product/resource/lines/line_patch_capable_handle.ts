import { deliverLine } from "./actions/line_delivery.js";
import { requireActiveLine } from "./actions/line_activity_guard.js";
import { patchLine } from "./actions/line_patch.js";
import { readLineReconciliation } from "./reconciliation/line_reconciliation_read.js";
import { requireCurrentMaterialization } from "./state/line_handle_helpers.js";
import { readLineBindingState } from "./state/line_binding_state.js";
import { executeResourceEffectSettlement } from "../effects/runtime/settlement/resource_effect_settlement_execution.js";

function createPatchCapableLineHandle(handle, lineBacking) {
  return Object.freeze({
    ...handle,
    patch(patch, options = {}) {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "patch");
      return patchLine(materialization, patch, options);
    },
    deliver(packet) {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "deliver");
      return deliverLine(materialization, packet);
    },
    reconciliation() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "reconciliation");
      return readLineReconciliation(materialization);
    },
    effects() {
      const materialization = requireCurrentMaterialization(lineBacking);
      requireActiveLine(materialization, "effects");
      return createLineEffectFacade(materialization);
    },
  });
}

function createLineEffectFacade(materialization) {
  return Object.freeze({
    get(effectId) {
      return materialization.effectBranchDag.effect(effectId);
    },
    open() {
      return materialization.effectBranchDag.openEffects();
    },
    reject(effectId, options = {}) {
      return executeResourceEffectSettlement(materialization, effectId, Object.freeze({
        kind: "rejected",
        responseId: options.responseId ?? null,
      }));
    },
    confirm(effectId, options = {}) {
      return executeResourceEffectSettlement(materialization, effectId, Object.freeze({
        kind: "confirmed",
        serverPatch: options.serverPatch ?? null,
        serverRevision: options.serverRevision ?? null,
        responseId: options.responseId ?? null,
      }));
    },
    projection() {
      return materialization.effectBranchDag.projection();
    },
    rebuildProjection() {
      return materialization.effectBranchDag.rebuildProjection(
        readLineBindingState(materialization.binding),
      );
    },
    counters() {
      return materialization.effectBranchDag.counters();
    },
  });
}

export { createPatchCapableLineHandle };

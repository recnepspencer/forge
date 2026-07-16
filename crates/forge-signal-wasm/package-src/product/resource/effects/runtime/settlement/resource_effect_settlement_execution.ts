import { createEffectSettlementDiagnostics } from "../../../lines/state/line_effect_settlement_diagnostics.js";
import {
  patchLineBindingState,
  readLineBindingState,
} from "../../../lines/state/line_binding_state.js";
import { recordLineHistoryEntry } from "../../../lines/history/record_line_history_entry.js";

async function executeResourceEffectSettlement(
  materialization,
  effectId,
  settlement,
) {
  const previous = readLineBindingState(materialization.binding);
  const result = await materialization.effectBranchDag.settle(
    effectId,
    settlement,
    previous,
  );
  if (result.kind === "duplicateSettlement") {
    return result;
  }
  patchLineBindingState(materialization.binding, {
    value: result.projection.projectedValue,
    canonicalValue: result.canonicalValue,
    diagnostics: createEffectSettlementDiagnostics(
      previous.diagnostics,
      result,
    ),
  });
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "effectSettled",
    Object.freeze({ effectSettlement: result }),
  );
  return result;
}

export { executeResourceEffectSettlement };

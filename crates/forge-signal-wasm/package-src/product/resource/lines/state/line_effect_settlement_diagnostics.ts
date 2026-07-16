import { createEffectSettlementVisibleSelection } from "./line_visible_selection.js";

function createEffectSettlementDiagnostics(previous, settlement) {
  const diagnostics = {
    ...previous,
    lastOperation: "effectSettlement",
    lastOutcome: settlement.kind,
    pendingOperation: null,
    preservedVisibleValueOnLastRejection: false,
    lastErrorMessage: null,
    visibleValueVersion: previous.visibleValueVersion + 1,
  };
  Object.defineProperty(diagnostics, "visibleSelection", {
    value: createEffectSettlementVisibleSelection(
      previous.visibleSelection,
      settlement,
    ),
    enumerable: true,
  });
  return Object.freeze(diagnostics);
}

export { createEffectSettlementDiagnostics };

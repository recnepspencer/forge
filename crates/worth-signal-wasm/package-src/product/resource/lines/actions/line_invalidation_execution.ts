import { createInvalidatedDiagnostics } from "../state/line_diagnostics_value.js";
import { createInvalidatedFreshness } from "../state/line_freshness_value.js";
import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";
import {
  patchLineBindingState,
  readLineBindingState,
} from "../state/line_binding_state.js";

function invalidateLine(materialization, cause, scope) {
  const previousState = readLineBindingState(materialization.binding);
  const freshness = createInvalidatedFreshness(cause);
  const diagnostics = createInvalidatedDiagnostics(
    previousState.diagnostics,
    cause,
    scope,
  );
  patchLineBindingState(materialization.binding, {
    freshness,
    diagnostics,
  });
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "invalidated",
  );
  return freshness;
}

export { invalidateLine };

import { createInvalidatedDiagnostics } from "../state/line_diagnostics_value.js";
import { createInvalidatedFreshness } from "../state/line_freshness_value.js";
import { recordLineHistoryEntry } from "../history/record_line_history_entry.js";

function invalidateLine(materialization, cause, scope) {
  const freshness = createInvalidatedFreshness(cause);
  const diagnostics = createInvalidatedDiagnostics(
    materialization.binding.diagnosticsSignal(),
    cause,
    scope,
  );
  materialization.binding.freshnessSignal.set(freshness);
  materialization.binding.diagnosticsSignal.set(diagnostics);
  recordLineHistoryEntry(
    materialization.lifecycleHistory,
    materialization.binding,
    "invalidated",
  );
  return freshness;
}

export { invalidateLine };

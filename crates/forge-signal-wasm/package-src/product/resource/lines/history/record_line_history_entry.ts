import { createLineHistoryEntry } from "./line_history_entry.js";
import { readLineBindingState } from "../state/line_binding_state.js";

function recordLineHistoryEntry(
  lifecycleHistory,
  binding,
  event,
  overrides,
) {
  const state = readLineBindingState(binding);
  lifecycleHistory.append(
    createLineHistoryEntry(
      event,
      state.status,
      state.freshness,
      state.diagnostics,
      overrides,
    ),
  );
}

export { recordLineHistoryEntry };

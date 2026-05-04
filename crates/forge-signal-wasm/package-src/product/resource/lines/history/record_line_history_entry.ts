import { createLineHistoryEntry } from "./line_history_entry.js";

function recordLineHistoryEntry(
  lifecycleHistory,
  binding,
  event,
  overrides,
) {
  lifecycleHistory.append(
    createLineHistoryEntry(
      event,
      binding.statusSignal(),
      binding.freshnessSignal(),
      binding.diagnosticsSignal(),
      overrides,
    ),
  );
}

export { recordLineHistoryEntry };

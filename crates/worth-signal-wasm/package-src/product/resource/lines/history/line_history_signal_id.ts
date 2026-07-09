import { readHistoryRuntimeErrorDetail } from "./line_history_availability.js";

function readLineHistorySignalId(materialization, rejectedPrefix) {
  try {
    return Object.freeze({
      signalId: materialization.binding.readableValueSignal.id,
      errorDetail: null,
    });
  } catch (error) {
    return Object.freeze({
      signalId: null,
      errorDetail: readHistoryRuntimeErrorDetail(rejectedPrefix, error),
    });
  }
}

export { readLineHistorySignalId };

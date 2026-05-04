import { readLineHistoryAvailability } from "./line_history_availability_read.js";

function readLineHistory(materialization) {
  const signalId = materialization.binding.readableValueSignal.id;
  const history = materialization.history;
  const historyAvailability = readLineHistoryAvailability(materialization);
  return Object.freeze({
    replay:
      historyAvailability.availability.replay.kind === "available"
        ? history.replay_for(signalId)
        : null,
    lineage:
      historyAvailability.availability.lineage.kind === "available"
        ? history.lineage_for(signalId)
        : null,
    branch: historyAvailability.branch,
    availability: historyAvailability.availability,
    lifecycle: materialization.lifecycleHistory.entries(),
  });
}

export { readLineHistory };

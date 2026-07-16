import { readLineHistoryAvailability } from "./line_history_availability_read.js";
import {
  createUnavailableHistoryArtifact,
  readHistoryRuntimeErrorDetail,
} from "../history/line_history_availability.js";
import { readLineBasisHistory } from "../history/line_basis_history_read.js";
import {
  executeLineEffectRollback,
  executeLineLastEffectRollback,
} from "../history/line_effect_rollback.js";
import { executeLineHistoryExactReplay } from "../history/line_history_replay.js";
import { executeLineHistoryExactRestore } from "../history/line_history_restore.js";
import { readLineHistorySignalId } from "../history/line_history_signal_id.js";
import { readLineVerificationPackage } from "../history/line_verification_package.js";

function readLineHistory(materialization) {
  const history = materialization.history;
  const signalIdRead = readLineHistorySignalId(
    materialization,
    "resource line history is unavailable because readableValueSignal.id rejected explainability",
  );
  const historyAvailability = readLineHistoryAvailability(materialization);
  const lifecycle = materialization.lifecycleHistory
    .entries()
    .map(createPublicLifecycleEntry);
  const replayRead = readHistoryArtifact(
    history,
    "replay_for",
    signalIdRead,
    historyAvailability.availability.replay,
    "resource line replay history is unavailable because replay_for(...) rejected explainability",
  );
  const lineageRead = readHistoryArtifact(
    history,
    "lineage_for",
    signalIdRead,
    historyAvailability.availability.lineage,
    "resource line lineage history is unavailable because lineage_for(...) rejected explainability",
  );
  const historyRead = {
    replay: replayRead.value,
    lineage: lineageRead.value,
    branch: historyAvailability.branch,
    basis: readLineBasisHistory(lifecycle),
    availability: Object.freeze({
      ...historyAvailability.availability,
      replay: replayRead.availability,
      lineage: lineageRead.availability,
    }),
    lifecycle,
  };
  Object.defineProperty(historyRead, "restoreExact", {
    value() {
      return executeLineHistoryExactRestore(materialization, historyRead);
    },
    enumerable: false,
    configurable: false,
    writable: false,
  });
  Object.defineProperty(historyRead, "replayExact", {
    value() {
      return executeLineHistoryExactReplay(materialization, historyRead);
    },
    enumerable: false,
    configurable: false,
    writable: false,
  });
  Object.defineProperty(historyRead, "rollbackLastEffect", {
    value() {
      return executeLineLastEffectRollback(materialization, historyRead);
    },
    enumerable: false,
    configurable: false,
    writable: false,
  });
  Object.defineProperty(historyRead, "rollbackEffect", {
    value(effectId) {
      return executeLineEffectRollback(materialization, historyRead, effectId);
    },
    enumerable: false,
    configurable: false,
    writable: false,
  });
  Object.defineProperty(historyRead, "verificationPackage", {
    value() {
      return readLineVerificationPackage(materialization, historyRead);
    },
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze(historyRead);
}

function createPublicLifecycleEntry(entry) {
  const publicEntry = {
    ...entry,
  };
  Object.defineProperty(publicEntry, "visibleSelection", {
    value: entry.visibleSelection,
    enumerable: false,
    configurable: false,
    writable: false,
  });
  return Object.freeze(publicEntry);
}

function readHistoryArtifact(
  history,
  methodName,
  signalIdRead,
  baseAvailability,
  rejectedPrefix,
) {
  if (baseAvailability.kind !== "available") {
    return Object.freeze({
      value: null,
      availability: baseAvailability,
    });
  }
  if (signalIdRead.errorDetail !== null) {
    return Object.freeze({
      value: null,
      availability: createUnavailableHistoryArtifact(
        "runtimeRejected",
        `${rejectedPrefix}: ${signalIdRead.errorDetail}`,
      ),
    });
  }
  try {
    return Object.freeze({
      value: history[methodName](signalIdRead.signalId),
      availability: baseAvailability,
    });
  } catch (error) {
    return Object.freeze({
      value: null,
      availability: createUnavailableHistoryArtifact(
        "runtimeRejected",
        readHistoryRuntimeErrorDetail(rejectedPrefix, error),
      ),
    });
  }
}

export { readLineHistory };

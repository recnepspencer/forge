import {
  createAvailableHistoryArtifact,
  createAvailableReplayAvailability,
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableReplayAvailability,
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
} from "../history/line_history_availability.js";
import { readCurrentHistoryBranch } from "../history/line_history_branch.js";

function readLineHistoryAvailability(materialization) {
  const history = materialization.history;
  const replayAvailable = readArtifactAvailability(
    history,
    "replay_for",
    "replay_availability_for",
    materialization.binding.readableValueSignal.id,
    "resource line replay history is unavailable because the Signals runtime does not expose replay_for(...)",
    "resource line replay history is unavailable because replay_availability_for(...) rejected explainability",
  );
  const replayExactAvailable = readReplayExecutionAvailability(
    history,
    materialization.binding.readableValueSignal.id,
  );
  const lineageAvailable = readArtifactAvailability(
    history,
    "lineage_for",
    "lineage_availability_for",
    materialization.binding.readableValueSignal.id,
    "resource line lineage history is unavailable because the Signals runtime does not expose lineage_for(...)",
    "resource line lineage history is unavailable because lineage_availability_for(...) rejected explainability",
  );
  const branchRead = readCurrentHistoryBranch(history);
  const branch = branchRead.branch;
  return Object.freeze({
    branch,
    availability: Object.freeze({
      replay: replayAvailable,
      replayExact: replayExactAvailable,
      lineage: lineageAvailable,
      branch: branch !== null
        ? createAvailableHistoryArtifact()
        : branchRead.errorDetail !== null
          ? createUnavailableHistoryArtifact(
              "runtimeRejected",
              branchRead.errorDetail,
            )
        : createUnavailableHistoryArtifact(
            "unsupportedByRuntime",
            "resource line branch history is unavailable because the Signals runtime does not expose current_branch(...)",
          ),
      restoreExact:
        branch === null
          ? branchRead.errorDetail !== null
            ? createUnavailableRestoreAvailability(
                "runtimeRejected",
                `resource line exact branch restore is unavailable because branch explainability could not be read: ${branchRead.errorDetail}`,
              )
            : createUnavailableRestoreAvailability(
                "unsupportedByRuntime",
                "resource line exact branch restore is unavailable because the Signals runtime does not expose current_branch(...)",
              )
          : branch.headSnapshotId === null
            ? createUnavailableRestoreAvailability(
                "branchHeadUnavailable",
                `resource line exact branch restore is unavailable because branch ${branch.id} has no head snapshot`,
              )
            : typeof history.restore_branch_snapshot_by_id !== "function"
              && !(
                typeof history.restore_exact_branch_snapshot === "function"
                && typeof history.branch_snapshot === "function"
              )
              ? createUnavailableRestoreAvailability(
                  "unsupportedByRuntime",
                  "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_branch_snapshot_by_id(...) or a restore_exact_branch_snapshot(...) + branch_snapshot(...) pair",
                )
              : createAvailableRestoreAvailability(
                  "SameRuntimeBranchExact",
                  branch.id,
                  branch.headSnapshotId,
                ),
    }),
  });
}

function readReplayExecutionAvailability(history, signalId) {
  if (typeof history.replay_signal_by_id !== "function") {
    return createUnavailableReplayAvailability(
      "unsupportedByRuntime",
      "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
    );
  }
  if (typeof history.replay_execution_availability_for !== "function") {
    return createAvailableReplayAvailability("SameRuntimeSignalExact", signalId);
  }
  try {
    const availability = history.replay_execution_availability_for(signalId);
    return availability?.kind === "unavailable"
      ? createUnavailableReplayAvailability(
          availability.reason ?? "runtimeRejected",
          availability.detail
            ?? "resource line exact replay is unavailable because replay_execution_availability_for(...) rejected replay execution with no detail",
        )
      : createAvailableReplayAvailability("SameRuntimeSignalExact", signalId);
  } catch (error) {
    return createUnavailableReplayAvailability(
      "runtimeRejected",
      readHistoryRuntimeErrorDetail(
        "resource line exact replay is unavailable because replay_execution_availability_for(...) rejected replay execution",
        error,
      ),
    );
  }
}

function readArtifactAvailability(
  history,
  artifactMethodName,
  availabilityMethodName,
  signalId,
  missingDetail,
  rejectedPrefix,
) {
  if (typeof history[artifactMethodName] !== "function") {
    return createUnavailableHistoryArtifact("unsupportedByRuntime", missingDetail);
  }
  if (typeof history[availabilityMethodName] !== "function") {
    return createAvailableHistoryArtifact();
  }
  try {
    const availability = history[availabilityMethodName](signalId);
    return availability?.kind === "unavailable"
      ? createUnavailableHistoryArtifact(
          availability.reason ?? "runtimeRejected",
          availability.detail ?? `${rejectedPrefix} with no detail`,
        )
      : createAvailableHistoryArtifact();
  } catch (error) {
    return createUnavailableHistoryArtifact(
      "runtimeRejected",
      readHistoryRuntimeErrorDetail(rejectedPrefix, error),
    );
  }
}

export { readLineHistoryAvailability };

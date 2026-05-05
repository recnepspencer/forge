import {
  createAvailableHistoryArtifact,
  createAvailableReplayAvailability,
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableReplayAvailability,
  createUnavailableRestoreAvailability,
  readHistoryRuntimeErrorDetail,
} from "../history/line_history_availability.js";
import { readLineHistorySignalId } from "../history/line_history_signal_id.js";
import { readCurrentHistoryBranch } from "../history/line_history_branch.js";

function readLineHistoryAvailability(materialization) {
  const history = materialization.history;
  const signalIdRead = readLineHistorySignalId(
    materialization,
    "resource line history is unavailable because readableValueSignal.id rejected explainability",
  );
  const replayAvailable = readArtifactAvailability(
    history,
    "replay_for",
    "replay_availability_for",
    signalIdRead,
    "resource line replay history is unavailable because the Signals runtime does not expose replay_for(...)",
    "resource line replay history is unavailable because replay_availability_for(...) rejected explainability",
  );
  const replayExactAvailable = readReplayExecutionAvailability(
    history,
    signalIdRead,
  );
  const lineageAvailable = readArtifactAvailability(
    history,
    "lineage_for",
    "lineage_availability_for",
    signalIdRead,
    "resource line lineage history is unavailable because the Signals runtime does not expose lineage_for(...)",
    "resource line lineage history is unavailable because lineage_availability_for(...) rejected explainability",
  );
  const branchRead = readCurrentHistoryBranch(history);
  const branch = branchRead.branch;
  const restoreSnapshotRead =
    branch === null
      ? null
      : readRestoreSnapshotAvailability(history, branch);
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
          : restoreSnapshotRead?.errorDetail != null
            ? createUnavailableRestoreAvailability(
                "runtimeRejected",
                restoreSnapshotRead.errorDetail,
              )
          : restoreSnapshotRead?.snapshotId == null
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
                  restoreSnapshotRead.snapshotId,
                ),
    }),
  });
}

function readRestoreSnapshotAvailability(history, branch) {
  if (branch.headSnapshotId !== null) {
    return Object.freeze({
      snapshotId: branch.headSnapshotId,
      errorDetail: null,
    });
  }
  if (typeof history.branch_snapshot_id !== "function") {
    return Object.freeze({
      snapshotId: null,
      errorDetail: null,
    });
  }
  try {
    const snapshotId = history.branch_snapshot_id(branch.id);
    return Object.freeze({
      snapshotId: snapshotId === null ? null : Number(snapshotId),
      errorDetail: null,
    });
  } catch (error) {
    return Object.freeze({
      snapshotId: null,
      errorDetail: readHistoryRuntimeErrorDetail(
        "resource line exact branch restore is unavailable because branch_snapshot_id(...) rejected restore-target lookup",
        error,
      ),
    });
  }
}

function readReplayExecutionAvailability(history, signalIdRead) {
  if (signalIdRead.errorDetail !== null) {
    return createUnavailableReplayAvailability(
      "runtimeRejected",
      `resource line exact replay is unavailable because readableValueSignal.id rejected explainability: ${signalIdRead.errorDetail}`,
    );
  }
  if (typeof history.replay_signal_by_id !== "function") {
    return createUnavailableReplayAvailability(
      "unsupportedByRuntime",
      "resource line exact replay is unavailable because the Signals runtime does not expose replay_signal_by_id(...)",
    );
  }
  if (typeof history.replay_execution_availability_for !== "function") {
    return createAvailableReplayAvailability(
      "SameRuntimeSignalExact",
      signalIdRead.signalId,
    );
  }
  try {
    const availability = history.replay_execution_availability_for(
      signalIdRead.signalId,
    );
    return availability?.kind === "unavailable"
      ? createUnavailableReplayAvailability(
          availability.reason ?? "runtimeRejected",
          availability.detail
            ?? "resource line exact replay is unavailable because replay_execution_availability_for(...) rejected replay execution with no detail",
        )
      : createAvailableReplayAvailability(
          "SameRuntimeSignalExact",
          signalIdRead.signalId,
        );
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
  signalIdRead,
  missingDetail,
  rejectedPrefix,
) {
  if (signalIdRead.errorDetail !== null) {
    return createUnavailableHistoryArtifact(
      "runtimeRejected",
      `${rejectedPrefix.replace(" rejected explainability", "")}: ${signalIdRead.errorDetail}`,
    );
  }
  if (typeof history[artifactMethodName] !== "function") {
    return createUnavailableHistoryArtifact("unsupportedByRuntime", missingDetail);
  }
  if (typeof history[availabilityMethodName] !== "function") {
    return createAvailableHistoryArtifact();
  }
  try {
    const availability = history[availabilityMethodName](signalIdRead.signalId);
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

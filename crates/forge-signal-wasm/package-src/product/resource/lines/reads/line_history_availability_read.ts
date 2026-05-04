import {
  createAvailableHistoryArtifact,
  createAvailableRestoreAvailability,
  createUnavailableHistoryArtifact,
  createUnavailableRestoreAvailability,
} from "../history/line_history_availability.js";
import { readCurrentHistoryBranch } from "../history/line_history_branch.js";

function readLineHistoryAvailability(materialization) {
  const history = materialization.history;
  const replayAvailable = typeof history.replay_for === "function";
  const lineageAvailable = typeof history.lineage_for === "function";
  const branch = readCurrentHistoryBranch(history);
  return Object.freeze({
    branch,
    availability: Object.freeze({
      replay: replayAvailable
        ? createAvailableHistoryArtifact()
        : createUnavailableHistoryArtifact(
            "unsupportedByRuntime",
            "resource line replay history is unavailable because the Signals runtime does not expose replay_for(...)",
          ),
      lineage: lineageAvailable
        ? createAvailableHistoryArtifact()
        : createUnavailableHistoryArtifact(
            "unsupportedByRuntime",
            "resource line lineage history is unavailable because the Signals runtime does not expose lineage_for(...)",
          ),
      branch: branch !== null
        ? createAvailableHistoryArtifact()
        : createUnavailableHistoryArtifact(
            "unsupportedByRuntime",
            "resource line branch history is unavailable because the Signals runtime does not expose current_branch(...)",
          ),
      restoreExact:
        branch === null
          ? createUnavailableRestoreAvailability(
              "unsupportedByRuntime",
              "resource line exact branch restore is unavailable because the Signals runtime does not expose current_branch(...)",
            )
          : branch.headSnapshotId === null
            ? createUnavailableRestoreAvailability(
                "branchHeadUnavailable",
                `resource line exact branch restore is unavailable because branch ${branch.id} has no head snapshot`,
              )
            : typeof history.restore_exact_branch_snapshot !== "function"
              ? createUnavailableRestoreAvailability(
                  "unsupportedByRuntime",
                  "resource line exact branch restore is unavailable because the Signals runtime does not expose restore_exact_branch_snapshot(...)",
                )
              : createAvailableRestoreAvailability(
                  "SameRuntimeBranchExact",
                  branch.id,
                  branch.headSnapshotId,
                ),
    }),
  });
}

export { readLineHistoryAvailability };

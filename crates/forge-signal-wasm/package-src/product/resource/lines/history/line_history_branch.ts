import { readHistoryRuntimeErrorDetail } from "./line_history_availability.js";

function readCurrentHistoryBranch(history) {
  if (typeof history.current_branch !== "function") {
    return Object.freeze({
      branch: null,
      errorDetail: null,
    });
  }
  try {
    const branch = history.current_branch();
    return Object.freeze({
      branch: Object.freeze({
        id: Number(branch.id),
        name: branch.name,
        parentBranchId:
          branch.parent_branch_id === null
            ? null
            : Number(branch.parent_branch_id),
        headSnapshotId: readCurrentBranchHeadSnapshotId(history, branch),
      }),
      errorDetail: null,
    });
  } catch (error) {
    return Object.freeze({
      branch: null,
      errorDetail: readHistoryRuntimeErrorDetail(
        "resource line branch history is unavailable because current_branch(...) failed",
        error,
      ),
    });
  }
}

function readCurrentBranchHeadSnapshotId(history, branch) {
  if (branch.head_snapshot_id !== null) {
    return Number(branch.head_snapshot_id);
  }
  if (typeof history.branch_snapshot_id !== "function") {
    return null;
  }
  try {
    const snapshotId = history.branch_snapshot_id(branch.id);
    return snapshotId === null ? null : Number(snapshotId);
  } catch {
    return null;
  }
}

export { readCurrentHistoryBranch };

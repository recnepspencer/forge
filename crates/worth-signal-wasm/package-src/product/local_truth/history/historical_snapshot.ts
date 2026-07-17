import { canonicalDigest, deepFreeze } from "../support/canonical.js";
import { denied, success } from "../support/outcomes.js";

export function readHistoricalSnapshot(state, request) {
  const branchId = request?.branchId;
  const commitId = request?.commitId;
  if (typeof branchId !== "string" || typeof commitId !== "string") {
    return denied(
      "invalidHistoricalSnapshotRequest",
      "historical snapshot inspection requires branchId and commitId strings",
    );
  }
  const branch = state.branches.get(branchId);
  if (!branch || branch.retired) {
    return denied("unknownLocalTruthBranch", `branch ${branchId} is unavailable`);
  }

  const checkpoint = state.checkpoints.get(branchId) ?? null;
  let visitedCommits = 0;
  let cursor = branch.headCommitId;
  while (cursor) {
    visitedCommits += 1;
    if (cursor === commitId) {
      return historicalSnapshotAt(state, branch, checkpoint, commitId, visitedCommits);
    }
    if (cursor === checkpoint?.headCommitId) break;
    const commit = state.commits.get(cursor);
    if (!commit) {
      return denied(
        "unavailableHistoricalCommit",
        `retained history for branch ${branchId} cannot reach commit ${commitId}`,
      );
    }
    cursor = commit.parentCommitId;
  }
  return denied(
    "commitOutsideBranchHistory",
    `commit ${commitId} is not retained ancestry of branch ${branchId}`,
  );
}

function historicalSnapshotAt(state, branch, checkpoint, commitId, visitedCommits) {
  const checkpointMatch = checkpoint?.headCommitId === commitId;
  const commit = state.commits.get(commitId) ?? null;
  const snapshotId = checkpointMatch ? checkpoint.snapshotId : commit?.afterSnapshotId;
  const values = checkpointMatch
    ? checkpoint.values
    : snapshotId
      ? state.snapshots.get(snapshotId)?.values
      : null;
  if (!snapshotId || !values) {
    return denied(
      "unavailableHistoricalSnapshot",
      `snapshot for retained commit ${commitId} is unavailable`,
    );
  }
  const counters = deepFreeze({ visitedCommits });
  return success(deepFreeze({
    artifactFamily: "LocalTruthHistoricalSnapshot",
    authorityId: state.authorityId,
    schemaIdentity: state.schemaIdentity,
    branchId: branch.id,
    commitId,
    snapshotId,
    values,
    counters,
    digest: canonicalDigest({
      authorityId: state.authorityId,
      schemaIdentity: state.schemaIdentity,
      branchId: branch.id,
      commitId,
      snapshotId,
      values,
      counters,
    }),
  }));
}

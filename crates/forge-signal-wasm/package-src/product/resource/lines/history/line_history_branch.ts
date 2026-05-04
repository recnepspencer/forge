function readCurrentHistoryBranch(history) {
  if (typeof history.current_branch !== "function") {
    return null;
  }
  const branch = history.current_branch();
  return Object.freeze({
    id: Number(branch.id),
    name: branch.name,
    parentBranchId:
      branch.parent_branch_id === null ? null : Number(branch.parent_branch_id),
    headSnapshotId:
      branch.head_snapshot_id === null ? null : Number(branch.head_snapshot_id),
  });
}

export { readCurrentHistoryBranch };

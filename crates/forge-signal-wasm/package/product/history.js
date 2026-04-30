function attachSerializedField(value, field, serialized) {
  if (!value || typeof value !== "object" || typeof serialized !== "string") {
    return value;
  }
  Object.defineProperty(value, field, {
    value: serialized,
    enumerable: true,
    configurable: false,
    writable: false,
  });
  return value;
}

function snapshotEnvelopeRestoreToken(snapshot) {
  return snapshot?.snapshotEnvelopeRestoreToken;
}

function snapshotRestoreToken(snapshot) {
  return snapshot?.snapshotRestoreToken;
}

function normalizeBranchId(value, operation) {
  if (typeof value === "bigint") {
    if (value < 0n) {
      throw new RangeError(`${operation} expects a non-negative branch id`);
    }
    return value;
  }
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(`${operation} expects a non-negative safe integer branch id`);
  }
  return BigInt(value);
}

function normalizeSnapshotId(value, operation) {
  if (typeof value === "bigint") {
    if (value < 0n) {
      throw new RangeError(`${operation} expects a non-negative snapshot id`);
    }
    return value;
  }
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(`${operation} expects a non-negative safe integer snapshot id`);
  }
  return BigInt(value);
}

function normalizePreviewBranchId(value, operation) {
  if (typeof value === "bigint") {
    if (value < 0n) {
      throw new RangeError(`${operation} expects a non-negative branch id`);
    }
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
      throw new RangeError(`${operation} exceeds the safe integer range supported by merge preview requests`);
    }
    return Number(value);
  }
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(`${operation} expects a non-negative safe integer branch id`);
  }
  return value;
}

function normalizeMergePreviewRequest(request, operation) {
  if (!request || typeof request !== "object" || Array.isArray(request)) {
    throw new TypeError(`${operation} expects a merge preview request object`);
  }
  return {
    ...request,
    source_branch_id: normalizePreviewBranchId(
      request.source_branch_id,
      `${operation}.source_branch_id`,
    ),
    target_branch_id: normalizePreviewBranchId(
      request.target_branch_id,
      `${operation}.target_branch_id`,
    ),
  };
}

export function wrapHistory(rawHistory) {
  return Object.freeze({
    replay_for(id) {
      return rawHistory.replay_for(id);
    },
    lineage_for(id) {
      return rawHistory.lineage_for(id);
    },
    snapshot() {
      return attachSerializedField(
        rawHistory.snapshot(),
        "snapshotEnvelopeRestoreToken",
        rawHistory.snapshot_wire(),
      );
    },
    restore_snapshot(snapshot) {
      const restoreToken = snapshotEnvelopeRestoreToken(snapshot);
      if (typeof restoreToken === "string") {
        return rawHistory.restore_snapshot_wire(restoreToken);
      }
      return rawHistory.restore_snapshot(snapshot);
    },
    current_branch() {
      return rawHistory.current_branch();
    },
    branches() {
      return rawHistory.branches();
    },
    create_branch(name) {
      return rawHistory.create_branch(name);
    },
    switch_branch(branchId) {
      return rawHistory.switch_branch(normalizeBranchId(branchId, "history.switch_branch"));
    },
    replay_for_branch(branchId) {
      return rawHistory.replay_for_branch(normalizeBranchId(branchId, "history.replay_for_branch"));
    },
    branch_snapshot(branchId) {
      const normalizedBranchId = normalizeBranchId(branchId, "history.branch_snapshot");
      return attachSerializedField(
        rawHistory.branch_snapshot(normalizedBranchId),
        "snapshotRestoreToken",
        rawHistory.branch_snapshot_wire(normalizedBranchId),
      );
    },
    branch_snapshot_id(branchId) {
      return rawHistory.branch_snapshot_id(normalizeBranchId(branchId, "history.branch_snapshot_id"));
    },
    branch_snapshot_envelope(branchId) {
      const normalizedBranchId = normalizeBranchId(branchId, "history.branch_snapshot_envelope");
      return attachSerializedField(
        rawHistory.branch_snapshot_envelope(normalizedBranchId),
        "snapshotEnvelopeRestoreToken",
        rawHistory.branch_snapshot_envelope_wire(normalizedBranchId),
      );
    },
    restore_branch_snapshot(branchId, snapshot) {
      const normalizedBranchId = normalizeBranchId(branchId, "history.restore_branch_snapshot");
      const restoreToken = snapshotRestoreToken(snapshot);
      if (typeof restoreToken === "string") {
        return rawHistory.restore_branch_snapshot_wire(normalizedBranchId, restoreToken);
      }
      return rawHistory.restore_branch_snapshot(
        normalizedBranchId,
        snapshot,
      );
    },
    restore_branch_snapshot_by_id(branchId, snapshotId) {
      return rawHistory.restore_branch_snapshot_by_id(
        normalizeBranchId(branchId, "history.restore_branch_snapshot_by_id"),
        normalizeSnapshotId(snapshotId, "history.restore_branch_snapshot_by_id"),
      );
    },
    merge_branches(sourceBranchId, targetBranchId) {
      return rawHistory.merge_branches(
        normalizeBranchId(sourceBranchId, "history.merge_branches"),
        normalizeBranchId(targetBranchId, "history.merge_branches"),
      );
    },
    merge_branches_with_proof(sourceBranchId, targetBranchId) {
      return rawHistory.merge_branches_with_proof(
        normalizeBranchId(sourceBranchId, "history.merge_branches_with_proof"),
        normalizeBranchId(targetBranchId, "history.merge_branches_with_proof"),
      );
    },
    plan_merge_branches(sourceBranchId, targetBranchId) {
      return rawHistory.plan_merge_branches(
        normalizeBranchId(sourceBranchId, "history.plan_merge_branches"),
        normalizeBranchId(targetBranchId, "history.plan_merge_branches"),
      );
    },
    plan_merge_branches_with_proof(sourceBranchId, targetBranchId) {
      return rawHistory.plan_merge_branches_with_proof(
        normalizeBranchId(sourceBranchId, "history.plan_merge_branches_with_proof"),
        normalizeBranchId(targetBranchId, "history.plan_merge_branches_with_proof"),
      );
    },
    plan_merge_policy_preview(request) {
      return rawHistory.plan_merge_policy_preview(
        normalizeMergePreviewRequest(request, "history.plan_merge_policy_preview"),
      );
    },
    plan_merge_policy_preview_with_proof(request) {
      return rawHistory.plan_merge_policy_preview_with_proof(
        normalizeMergePreviewRequest(request, "history.plan_merge_policy_preview_with_proof"),
      );
    },
    merge_branches_policy_preview(request) {
      return rawHistory.merge_branches_policy_preview(
        normalizeMergePreviewRequest(request, "history.merge_branches_policy_preview"),
      );
    },
    merge_branches_policy_preview_with_proof(request) {
      return rawHistory.merge_branches_policy_preview_with_proof(
        normalizeMergePreviewRequest(request, "history.merge_branches_policy_preview_with_proof"),
      );
    },
    branch_state_proof(branchId) {
      return rawHistory.branch_state_proof(normalizeBranchId(branchId, "history.branch_state_proof"));
    },
    replay_parity_proof(expectedBranchId, replayedBranchId) {
      return rawHistory.replay_parity_proof(
        normalizeBranchId(expectedBranchId, "history.replay_parity_proof"),
        normalizeBranchId(replayedBranchId, "history.replay_parity_proof"),
      );
    },
    replay_artifact_proof(expected, replayedBranchId) {
      return rawHistory.replay_artifact_proof(
        expected,
        normalizeBranchId(replayedBranchId, "history.replay_artifact_proof"),
      );
    },
    free() {
      rawHistory.free();
    },
    [Symbol.dispose]() {
      if (typeof rawHistory[Symbol.dispose] === "function") {
        rawHistory[Symbol.dispose]();
        return;
      }
      rawHistory.free();
    },
  });
}

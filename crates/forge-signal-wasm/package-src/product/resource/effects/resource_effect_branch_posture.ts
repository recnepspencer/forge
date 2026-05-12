import { readHistoryRuntimeErrorDetail } from "../lines/history/line_history_availability.js";

function resolveResourceEffectBranchPosture(options) {
  const profile = options.requestDescriptor.effects;
  if (options.admissionKind === "delivery") {
    return createCommittedOnlyBranchPosture(
      profile,
      "deliveryAuthority",
      "server deliveries are already authoritative and are not admitted as speculative branch patches",
    );
  }
  if (profile === null) {
    return createCommittedOnlyBranchPosture(
      null,
      "unconfigured",
      "resource effects are not configured for this response",
    );
  }
  if (profile.optimism !== "branchSpeculative") {
    return createCommittedOnlyBranchPosture(
      profile,
      "profileDisablesOptimism",
      `resource effect profile "${profile.name}" disables optimistic branch application`,
    );
  }
  return resolveSpeculativeBranchPosture(
    options.materialization.history,
    profile,
    options.inverseDescriptor,
  );
}

function resolveSpeculativeBranchPosture(history, profile, inverseDescriptor) {
  const branchRead = readCurrentBranchSpeculationTarget(history);
  if (branchRead.errorDetail !== null) {
    return createOptimisticUnavailableBranchPosture(
      profile,
      "runtimeRejected",
      branchRead.errorDetail,
      branchRead.branch?.id ?? null,
      null,
      branchRead.proofBreadth,
    );
  }
  if (branchRead.branch === null) {
    return createOptimisticUnavailableBranchPosture(
      profile,
      "unsupportedByRuntime",
      "resource effect branch speculation is unavailable because the Signals runtime does not expose current_branch(...)",
      null,
      null,
      0,
    );
  }
  const branch = branchRead.branch;
  if (branch.headSnapshotId === null) {
    return createOptimisticUnavailableBranchPosture(
      profile,
      "branchHeadUnavailable",
      `resource effect branch speculation is unavailable because branch ${branch.id} has no head snapshot`,
      branch.id,
      null,
      branchRead.proofBreadth,
    );
  }
  if (!canRestoreExactBranch(history)) {
    if (canUseCompactInverse(profile, inverseDescriptor)) {
      return Object.freeze({
        kind: "speculativeBranch",
        profileName: profile.name,
        optimism: profile.optimism,
        rollback: profile.rollback,
        rollbackMode: "CompactInversePatch",
        branchId: branch.id,
        snapshotId: branch.headSnapshotId,
        restoreMode: null,
        inverse: inverseDescriptor,
        proofBreadth: branchRead.proofBreadth + 1,
      });
    }
    return createOptimisticUnavailableBranchPosture(
      profile,
      "restoreUnavailable",
      "resource effect branch speculation is unavailable because the Signals runtime cannot restore a captured exact branch snapshot by id and the local patch does not carry an admissible safe compact inverse",
      branch.id,
      branch.headSnapshotId,
      branchRead.proofBreadth + 1,
    );
  }
  return Object.freeze({
    kind: "speculativeBranch",
    profileName: profile.name,
    optimism: profile.optimism,
    rollback: profile.rollback,
    rollbackMode: "SameRuntimeBranchExact",
    branchId: branch.id,
    snapshotId: branch.headSnapshotId,
    restoreMode: "SameRuntimeBranchExact",
    inverse: null,
    proofBreadth: branchRead.proofBreadth + 1,
  });
}

function readCurrentBranchSpeculationTarget(history) {
  if (typeof history.current_branch !== "function") {
    return Object.freeze({
      branch: null,
      errorDetail: null,
      proofBreadth: 0,
    });
  }
  try {
    const branch = history.current_branch();
    const snapshotRead = readBranchSpeculationSnapshotId(history, branch);
    if (snapshotRead.errorDetail !== null) {
      return Object.freeze({
        branch: Object.freeze({
          id: Number(branch.id),
          name: branch.name,
          parentBranchId:
            branch.parent_branch_id === null
              ? null
              : Number(branch.parent_branch_id),
          headSnapshotId: null,
        }),
        errorDetail: snapshotRead.errorDetail,
        proofBreadth: snapshotRead.proofBreadth,
      });
    }
    return Object.freeze({
      branch: Object.freeze({
        id: Number(branch.id),
        name: branch.name,
        parentBranchId:
          branch.parent_branch_id === null
            ? null
            : Number(branch.parent_branch_id),
        headSnapshotId: snapshotRead.snapshotId,
      }),
      errorDetail: null,
      proofBreadth: snapshotRead.proofBreadth,
    });
  } catch (error) {
    return Object.freeze({
      branch: null,
      errorDetail: readHistoryRuntimeErrorDetail(
        "resource effect branch speculation is unavailable because current_branch(...) failed",
        error,
      ),
      proofBreadth: 1,
    });
  }
}

function readBranchSpeculationSnapshotId(history, branch) {
  if (branch.head_snapshot_id !== null) {
    return Object.freeze({
      snapshotId: Number(branch.head_snapshot_id),
      errorDetail: null,
      proofBreadth: 1,
    });
  }
  if (typeof history.branch_snapshot_id !== "function") {
    return Object.freeze({
      snapshotId: null,
      errorDetail: null,
      proofBreadth: 1,
    });
  }
  try {
    const snapshotId = history.branch_snapshot_id(branch.id);
    return Object.freeze({
      snapshotId: snapshotId === null ? null : Number(snapshotId),
      errorDetail: null,
      proofBreadth: 2,
    });
  } catch (error) {
    return Object.freeze({
      snapshotId: null,
      errorDetail: readHistoryRuntimeErrorDetail(
        "resource effect branch speculation is unavailable because branch_snapshot_id(...) rejected restore-target lookup",
        error,
      ),
      proofBreadth: 2,
    });
  }
}

function canRestoreExactBranch(history) {
  return typeof history.restore_branch_snapshot_by_id === "function";
}

function canUseCompactInverse(profile, inverseDescriptor) {
  return (
    profile.rollback === "branchRestoreOrInverse"
    && profile.preimage === "compactInverse"
    && inverseDescriptor !== null
  );
}

function createCommittedOnlyBranchPosture(profile, reason, detail) {
  return Object.freeze({
    kind: "committedOnly",
    profileName: profile?.name ?? null,
    optimism: profile?.optimism ?? "none",
    rollback: profile?.rollback ?? "unavailable",
    reason,
    detail,
    proofBreadth: 0,
  });
}

function createOptimisticUnavailableBranchPosture(
  profile,
  reason,
  detail,
  branchId,
  snapshotId,
  proofBreadth,
) {
  return Object.freeze({
    kind: "optimisticUnavailable",
    profileName: profile.name,
    optimism: profile.optimism,
    rollback: profile.rollback,
    reason,
    detail,
    branchId,
    snapshotId,
    inverseAvailable: false,
    proofBreadth,
  });
}

export { resolveResourceEffectBranchPosture };

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
  const requiredCommands = [
    "worker_branch_basis",
    "fork_branch",
    "apply_transaction_to_branch",
    "retire_branch",
  ];
  const unavailableCommand = requiredCommands.find(
    (command) => typeof history?.[command] !== "function",
  );
  if (unavailableCommand !== undefined) {
    return createOptimisticUnavailableBranchPosture(
      profile,
      "unsupportedByRuntime",
      `resource effect branch speculation requires history.${unavailableCommand}(...)`,
      null,
      null,
      0,
    );
  }
  return Object.freeze({
    kind: "effectOwnedBranchPlanned",
    profileName: profile.name,
    optimism: profile.optimism,
    rollback: profile.rollback,
    rollbackMode: "EffectBranchRetirement",
    branchId: null,
    snapshotId: null,
    restoreMode: null,
    inverse: inverseDescriptor,
    proofBreadth: requiredCommands.length,
  });
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

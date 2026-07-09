function requireSpeculativeHistory(history) {
  if (history === null || typeof history !== "object") {
    throw new TypeError(
      "speculativeBranchPlan.open(...) requires a signals.history()-compatible object",
    );
  }
  for (const method of [
    "current_branch",
    "create_branch",
    "switch_branch",
    "plan_merge_policy_preview_with_proof",
    "merge_branches_with_proof",
  ]) {
    if (typeof history[method] !== "function") {
      throw new TypeError(
        `speculativeBranchPlan.open(...) requires history.${method}(...)`,
      );
    }
  }
  return history;
}

function requireSpeculativeSpecialist(specialist) {
  if (
    specialist === null
    || typeof specialist !== "object"
    || typeof specialist.evaluateDirty !== "function"
  ) {
    throw new TypeError(
      "speculativeBranchSession.dirtyExit(...) requires a specialist.evaluateDirty()-compatible object",
    );
  }
  return specialist;
}

function requireActiveSpeculativeSession(terminalOperation, methodName) {
  if (terminalOperation !== null) {
    throw new TypeError(
      `speculativeBranchSession.${methodName}(...) cannot run after session ${terminalOperation}`,
    );
  }
}

function requireCommitPreviewForSession(
  plan,
  verification,
  openedBranch,
  originBranch,
  commitPosture,
  previewArtifact,
) {
  if (commitPosture === "direct-merge-commit") {
    return null;
  }
  if (previewArtifact === null || typeof previewArtifact !== "object") {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires a preview artifact from session.commitPreview(...) when commitPosture is merge-preview-before-commit",
    );
  }
  if (
    previewArtifact.kind !== "speculativeBranchCommitPreview"
    || typeof previewArtifact.verification !== "function"
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires a preview artifact returned by session.commitPreview(...)",
    );
  }
  if (
    previewArtifact.sourceBranchId !== openedBranch.id
    || previewArtifact.targetBranchId !== originBranch.id
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) preview artifact does not match this speculative session branch pairing",
    );
  }
  const previewVerification = previewArtifact.verification();
  if (
    previewVerification.speculativeBranchDigest
    !== plan.verification().speculativeBranchDigest
    || previewVerification.speculativeSessionDigest
    !== verification.speculativeSessionDigest
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) preview artifact proof does not match this speculative session",
    );
  }
  return previewArtifact;
}

function requireDirtyExitForSession(
  plan,
  verification,
  openedBranch,
  dirtyExitArtifact,
  dirtyExitConfirmation,
) {
  if (dirtyExitArtifact === null || typeof dirtyExitArtifact !== "object") {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires a dirty-exit artifact from session.dirtyExit(...)",
    );
  }
  if (
    dirtyExitArtifact.kind !== "speculativeBranchDirtyExit"
    || typeof dirtyExitArtifact.verification !== "function"
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires a dirty-exit artifact returned by session.dirtyExit(...)",
    );
  }
  if (dirtyExitArtifact.branchId !== openedBranch.id) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) dirty-exit artifact does not match this speculative session branch",
    );
  }
  const dirtyExitVerification = dirtyExitArtifact.verification();
  if (
    dirtyExitVerification.speculativeBranchDigest
    !== plan.verification().speculativeBranchDigest
    || dirtyExitVerification.speculativeSessionDigest
    !== verification.speculativeSessionDigest
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) dirty-exit artifact proof does not match this speculative session",
    );
  }
  if (dirtyExitArtifact.disposition !== "dirty-exit-requires-confirmation") {
    return dirtyExitArtifact;
  }
  if (dirtyExitConfirmation === null || typeof dirtyExitConfirmation !== "object") {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires an explicit dirty-exit confirmation witness when dirtyExit(...) reports confirmation is required",
    );
  }
  if (
    dirtyExitConfirmation.kind !== "speculativeBranchDirtyExitConfirmation"
    || typeof dirtyExitConfirmation.verification !== "function"
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) requires a confirmation witness returned by dirtyExitArtifact.confirm()",
    );
  }
  const confirmationVerification = dirtyExitConfirmation.verification();
  if (
    confirmationVerification.speculativeBranchDigest
    !== plan.verification().speculativeBranchDigest
    || confirmationVerification.speculativeSessionDigest
    !== verification.speculativeSessionDigest
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) dirty-exit confirmation proof does not match this speculative session",
    );
  }
  if (
    confirmationVerification.speculativeDirtyExitDigest
    !== dirtyExitVerification.speculativeDirtyExitDigest
  ) {
    throw new TypeError(
      "speculativeBranchSession.commit(...) dirty-exit confirmation proof does not match the supplied dirty-exit artifact",
    );
  }
  return dirtyExitArtifact;
}

function normalizePreviewOverrides(options) {
  if (options === undefined) {
    return {};
  }
  if (options === null || typeof options !== "object" || Array.isArray(options)) {
    throw new TypeError(
      "speculativeBranchSession.commitPreview(...) options must be an object when provided",
    );
  }
  const unknownKeys = Object.keys(options).filter(
    (key) => ![
      "conflict_policy_name",
      "conflict_isolation_policy_name",
      "identity_matcher_name",
      "deletion_policy_name",
    ].includes(key),
  );
  if (unknownKeys.length > 0) {
    throw new TypeError(
      `speculativeBranchSession.commitPreview(...) does not support: ${unknownKeys.join(", ")}`,
    );
  }
  return options;
}

async function normalizedMaybePromise(value) {
  return await value;
}

export {
  normalizePreviewOverrides,
  normalizedMaybePromise,
  requireActiveSpeculativeSession,
  requireCommitPreviewForSession,
  requireDirtyExitForSession,
  requireSpeculativeHistory,
  requireSpeculativeSpecialist,
};

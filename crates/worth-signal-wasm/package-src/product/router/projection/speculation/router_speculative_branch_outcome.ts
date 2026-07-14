import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";

function createSpeculativePendingOutcome(plan, routeOutcome) {
  return createSpeculativeOutcomeArtifact(
    plan,
    "pending",
    "preBranchAdmission",
    "candidate-route-remains-pending-until-commit",
    routeOutcome,
    {
      branchLifecycleResult: "notOpened",
      recoveryDisposition:
        routeOutcome.recovery() === null
          ? "no-recovery-before-pending"
          : "recovery-attached-before-pending",
    },
  );
}

function createSpeculativeActivePendingOutcome(plan, verification, openedBranch) {
  return createSpeculativeOutcomeArtifact(
    plan,
    "pending",
    "activeSpeculativeBranch",
    "candidate-route-bound-to-active-branch",
    null,
    {
      branchLifecycleResult: "activeBranchPending",
      branchId: openedBranch.id,
    },
    verification,
  );
}

function createSpeculativeRejectedOutcome(plan, routeOutcome) {
  return createSpeculativeOutcomeArtifact(
    plan,
    routeOutcome.kind,
    "preBranchAdmission",
    `${routeOutcome.kind}-before-branch-open`,
    routeOutcome,
    {
      branchLifecycleResult: "notOpened",
      recoveryDisposition:
        routeOutcome.recovery() === null
          ? "no-recovery-before-terminal-outcome"
          : "recovery-attached-before-terminal-outcome",
    },
  );
}

function createSpeculativeCommittedOutcome(
  plan,
  verification,
  openedBranch,
  originBranch,
  mergeResult,
  normalizedPreview,
) {
  return createSpeculativeOutcomeArtifact(
    plan,
    "committed",
    "historyCommit",
    "committed-through-history-merge",
    null,
    {
      branchLifecycleResult: "merged",
      sourceBranchId: openedBranch.id,
      targetBranchId: originBranch.id,
      previewDisposition:
        normalizedPreview === null
          ? "committed-without-preview-artifact"
          : "committed-from-preview-artifact",
      mergeResultDigest: mergeResult.proof.resultDigest,
      mergeLineageDigest: mergeResult.proof.lineageDigest,
    },
    verification,
  );
}

function createSpeculativeDiscardedOutcome(plan, verification, openedBranch) {
  const keepsBranchPending =
    plan.branching().discardPosture === "keep-branch-pending";
  return createSpeculativeOutcomeArtifact(
    plan,
    keepsBranchPending ? "pending" : "discarded",
    "historyDiscard",
    keepsBranchPending
      ? "candidate-route-remained-pending-after-discard"
      : "discarded-and-abandoned",
    null,
    {
      branchLifecycleResult: keepsBranchPending ? "remainedPending" : "discarded",
      branchId: openedBranch.id,
      discardDisposition: keepsBranchPending
        ? "keep-branch-pending-without-merge"
        : "abandon-branch-without-merge",
    },
    verification,
  );
}

function createSpeculativeOutcomeArtifact(
  plan,
  kind,
  outcomeSource,
  branchDisposition,
  routeOutcome,
  extraDiagnostics,
  sessionVerification = null,
) {
  const diagnostics = Object.freeze({
    status: kind,
    outcomeSource,
    branchDisposition,
    routeOutcomeKind: routeOutcome?.kind ?? null,
    ...extraDiagnostics,
  });
  const routeOutcomeDigest = routeOutcome?.verification().routeOutcomeDigest ?? null;
  const visibleProjection = createSpeculativeVisibleProjection(
    plan,
    kind,
    routeOutcome,
    diagnostics.branchLifecycleResult,
    sessionVerification,
  );
  return Object.freeze({
    kind,
    routeId: routeOutcome?.routeId ?? plan.routeId,
    href: routeOutcome?.href ?? plan.href,
    routeOutcome() {
      return routeOutcome;
    },
    visibleProjection() {
      return visibleProjection;
    },
    diagnostics() {
      return diagnostics;
    },
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: sessionVerification?.speculativeSessionDigest ?? null,
        routeOutcomeDigest,
        speculativeOutcomeDigest: createCanonicalDigest("speculative-branch-outcome", {
          routeId: routeOutcome?.routeId ?? plan.routeId,
          href: routeOutcome?.href ?? plan.href,
          kind,
          outcomeSource,
          branchDisposition,
          routeOutcomeKind: routeOutcome?.kind ?? null,
          routeOutcomeDigest,
          speculativeSessionDigest: sessionVerification?.speculativeSessionDigest ?? null,
          extraDiagnostics,
        }),
      });
    },
  });
}

function createSpeculativeVisibleProjection(
  plan,
  outcomeKind,
  routeOutcome,
  branchLifecycleResult,
  sessionVerification,
) {
  const posture = plan.branching().visiblePosture;
  const pendingVisible =
    outcomeKind === "pending"
    && posture === "allow-visible-flicker-before-commit";
  const state =
    outcomeKind === "committed"
      ? "candidateVisibleAfterCommit"
      : pendingVisible
        ? "candidateVisibleWhilePending"
        : outcomeKind === "pending"
          ? "candidateSuppressedUntilCommit"
          : "candidateNotVisible";
  const routeId =
    state === "candidateVisibleWhilePending" || state === "candidateVisibleAfterCommit"
      ? routeOutcome?.routeId ?? plan.routeId
      : null;
  const href =
    state === "candidateVisibleWhilePending" || state === "candidateVisibleAfterCommit"
      ? routeOutcome?.href ?? plan.href
      : null;
  return Object.freeze({
    kind: "speculativeVisibleProjection",
    posture,
    state,
    routeId,
    href,
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: sessionVerification?.speculativeSessionDigest ?? null,
        speculativeVisibleProjectionDigest: createCanonicalDigest("speculative-visible-projection", {
          routeId,
          href,
          posture,
          state,
          branchLifecycleResult,
          speculativeSessionDigest: sessionVerification?.speculativeSessionDigest ?? null,
        }),
      });
    },
  });
}

export {
  createSpeculativeActivePendingOutcome,
  createSpeculativeCommittedOutcome,
  createSpeculativeDiscardedOutcome,
  createSpeculativePendingOutcome,
  createSpeculativeRejectedOutcome,
};

import { createCanonicalDigest } from "../../url_authority/router_verification_packages.js";
import {
  createSpeculativeActivePendingOutcome,
  createSpeculativeCommittedOutcome,
  createSpeculativeDiscardedOutcome,
} from "./router_speculative_branch_outcome.js";

function createSpeculativeSessionVerification(plan, originBranch, openedBranch) {
  return Object.freeze({
    speculativeBranchDigest: plan.verification().speculativeBranchDigest,
    speculativeSessionDigest: createCanonicalDigest("speculative-branch-session", {
      routeId: plan.routeId,
      href: plan.href,
      originBranchId: originBranch.id,
      speculativeBranchId: openedBranch.id,
      branchName: openedBranch.name,
      commitPosture: plan.branching().commitPosture,
      discardPosture: plan.branching().discardPosture,
    }),
  });
}

function createSpeculativeSessionPendingOutcome(plan, verification, openedBranch) {
  return createSpeculativeActivePendingOutcome(plan, verification, openedBranch);
}

function createSpeculativeCommitPreviewArtifact(
  plan,
  verification,
  openedBranch,
  originBranch,
  preview,
) {
  return Object.freeze({
    kind: "speculativeBranchCommitPreview",
    sourceBranchId: openedBranch.id,
    targetBranchId: originBranch.id,
    preview,
    posture: plan.branching().commitPosture,
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: verification.speculativeSessionDigest,
        speculativeCommitPreviewDigest: createCanonicalDigest(
          "speculative-branch-commit-preview",
          {
            routeId: plan.routeId,
            sourceBranchId: openedBranch.id,
            targetBranchId: originBranch.id,
            posture: plan.branching().commitPosture,
            planDigest: preview.proof.planDigest,
          },
        ),
      });
    },
  });
}

function createSpeculativeCommitArtifact(
  plan,
  verification,
  openedBranch,
  originBranch,
  mergeResult,
  normalizedPreview,
) {
  return Object.freeze({
    kind: "speculativeBranchCommit",
    routeId: plan.routeId,
    href: plan.href,
    sourceBranchId: openedBranch.id,
    targetBranchId: originBranch.id,
    mergeResult,
    previewDisposition:
      normalizedPreview === null
        ? "committed-without-preview-artifact"
        : "committed-from-preview-artifact",
    outcome() {
      return createSpeculativeCommittedOutcome(
        plan,
        verification,
        openedBranch,
        originBranch,
        mergeResult,
        normalizedPreview,
      );
    },
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: verification.speculativeSessionDigest,
        speculativeCommitDigest: createCanonicalDigest("speculative-branch-commit", {
          routeId: plan.routeId,
          href: plan.href,
          sourceBranchId: openedBranch.id,
          targetBranchId: originBranch.id,
          previewDisposition:
            normalizedPreview === null
              ? "committed-without-preview-artifact"
              : "committed-from-preview-artifact",
          mergeResultDigest: mergeResult.proof.resultDigest,
          mergeLineageDigest: mergeResult.proof.lineageDigest,
        }),
      });
    },
  });
}

function createSpeculativeDirtyExitArtifact(plan, verification, openedBranch, runSummary) {
  const disposition = runSummary.touchedNodes === 0
    ? "clean-exit"
    : "dirty-exit-requires-confirmation";
  const speculativeDirtyExitDigest = createCanonicalDigest("speculative-branch-dirty-exit", {
    routeId: plan.routeId,
    href: plan.href,
    branchId: openedBranch.id,
    disposition,
    touchedNodes: runSummary.touchedNodes,
    nodesEvaluated: runSummary.nodesEvaluated,
  });
  return Object.freeze({
    kind: "speculativeBranchDirtyExit",
    routeId: plan.routeId,
    href: plan.href,
    branchId: openedBranch.id,
    runSummary,
    disposition,
    confirmationRequired: disposition === "dirty-exit-requires-confirmation",
    confirm() {
      if (disposition !== "dirty-exit-requires-confirmation") {
        return null;
      }
      return Object.freeze({
        kind: "speculativeBranchDirtyExitConfirmation",
        routeId: plan.routeId,
        href: plan.href,
        branchId: openedBranch.id,
        verification() {
          return Object.freeze({
            speculativeBranchDigest: plan.verification().speculativeBranchDigest,
            speculativeSessionDigest: verification.speculativeSessionDigest,
            speculativeDirtyExitDigest,
            speculativeDirtyExitConfirmationDigest: createCanonicalDigest(
              "speculative-branch-dirty-exit-confirmation",
              {
                routeId: plan.routeId,
                href: plan.href,
                branchId: openedBranch.id,
                speculativeSessionDigest: verification.speculativeSessionDigest,
                speculativeDirtyExitDigest,
                touchedNodes: runSummary.touchedNodes,
                nodesEvaluated: runSummary.nodesEvaluated,
              },
            ),
          });
        },
      });
    },
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: verification.speculativeSessionDigest,
        speculativeDirtyExitDigest,
      });
    },
  });
}

function createSpeculativePendingBranchArtifact(
  plan,
  verification,
  openedBranch,
  originBranch,
  resumePendingBranch,
) {
  return Object.freeze({
    kind: "speculativePendingBranch",
    routeId: plan.routeId,
    href: plan.href,
    originBranch() {
      return originBranch;
    },
    branch() {
      return openedBranch;
    },
    async resume(history) {
      return resumePendingBranch(history);
    },
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: verification.speculativeSessionDigest,
        speculativePendingBranchDigest: createCanonicalDigest(
          "speculative-pending-branch",
          {
            routeId: plan.routeId,
            href: plan.href,
            branchId: openedBranch.id,
            originBranchId: originBranch.id,
          },
        ),
      });
    },
  });
}

function createSpeculativeDiscardArtifact(
  plan,
  verification,
  openedBranch,
  originBranch,
  resumePendingBranch,
) {
  const keepsBranchPending =
    plan.branching().discardPosture === "keep-branch-pending";
  const pendingBranch = keepsBranchPending
    ? createSpeculativePendingBranchArtifact(
      plan,
      verification,
      openedBranch,
      originBranch,
      resumePendingBranch,
    )
    : null;
  return Object.freeze({
    kind: "speculativeBranchDiscard",
    routeId: plan.routeId,
    href: plan.href,
    branchId: openedBranch.id,
    disposition:
      plan.branching().discardPosture === "discard-speculative-branch"
        ? "abandon-branch-without-merge"
        : "keep-branch-pending-without-merge",
    outcome() {
      return createSpeculativeDiscardedOutcome(plan, verification, openedBranch);
    },
    pendingBranch() {
      return pendingBranch;
    },
    verification() {
      return Object.freeze({
        speculativeBranchDigest: plan.verification().speculativeBranchDigest,
        speculativeSessionDigest: verification.speculativeSessionDigest,
        speculativeDiscardDigest: createCanonicalDigest("speculative-branch-discard", {
          routeId: plan.routeId,
          href: plan.href,
          branchId: openedBranch.id,
          discardPosture: plan.branching().discardPosture,
        }),
      });
    },
  });
}

export {
  createSpeculativeCommitArtifact,
  createSpeculativeCommitPreviewArtifact,
  createSpeculativeDirtyExitArtifact,
  createSpeculativeDiscardArtifact,
  createSpeculativePendingBranchArtifact,
  createSpeculativeSessionPendingOutcome,
  createSpeculativeSessionVerification,
};

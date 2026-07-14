import {
  createSpeculativeCommitArtifact,
  createSpeculativeCommitPreviewArtifact,
  createSpeculativeDirtyExitArtifact,
  createSpeculativeDiscardArtifact,
  createSpeculativeSessionPendingOutcome,
  createSpeculativeSessionVerification,
} from "./router_speculative_branch_session_artifacts.js";
import {
  normalizePreviewOverrides,
  normalizedMaybePromise,
  requireActiveSpeculativeSession,
  requireCommitPreviewForSession,
  requireDirtyExitForSession,
  requireSpeculativeHistory,
  requireSpeculativeSpecialist,
} from "./router_speculative_branch_session_guards.js";

function openSpeculativeRouteBranchSession(plan, history) {
  const normalizedHistory = requireSpeculativeHistory(history);
  return createSpeculativeSessionPromise(plan, normalizedHistory);
}

async function createSpeculativeSessionPromise(plan, history) {
  const originBranch = await normalizedMaybePromise(history.current_branch());
  const openedBranch = await normalizedMaybePromise(
    history.create_branch(plan.branching().branchName),
  );
  await normalizedMaybePromise(history.switch_branch(openedBranch.id));
  const verification = createSpeculativeSessionVerification(plan, originBranch, openedBranch);
  return createSpeculativeSessionArtifact(
    plan,
    history,
    originBranch,
    openedBranch,
    verification,
  );
}

function createSpeculativeSessionArtifact(
  plan,
  history,
  originBranch,
  openedBranch,
  verification,
) {
  let terminalOperation = null;
  const lifecycle = Object.freeze({
    branchBinding: "candidate-route-bound-to-history-branch",
    originBranchId: originBranch.id,
    speculativeBranchId: openedBranch.id,
    branchState: "active-speculative-branch",
    commitPosture: plan.branching().commitPosture,
    discardPosture: plan.branching().discardPosture,
  });
  return Object.freeze({
    kind: "speculativeBranchSession",
    candidate() {
      return plan.candidate();
    },
    plan() {
      return plan;
    },
    originBranch() {
      return originBranch;
    },
    branch() {
      return openedBranch;
    },
    lifecycle() {
      return lifecycle;
    },
    outcome() {
      requireActiveSpeculativeSession(terminalOperation, "outcome");
      return createSpeculativeSessionPendingOutcome(plan, verification, openedBranch);
    },
    async commitPreview(options = {}) {
      requireActiveSpeculativeSession(terminalOperation, "commitPreview");
      const preview = await normalizedMaybePromise(
        history.plan_merge_policy_preview_with_proof({
          source_branch_id: openedBranch.id,
          target_branch_id: originBranch.id,
          ...normalizePreviewOverrides(options),
        }),
      );
      return createSpeculativeCommitPreviewArtifact(
        plan,
        verification,
        openedBranch,
        originBranch,
        preview,
      );
    },
    async commit(previewArtifact = null, dirtyExitArtifact = null, dirtyExitConfirmation = null) {
      requireActiveSpeculativeSession(terminalOperation, "commit");
      const normalizedPreview = requireCommitPreviewForSession(
        plan,
        verification,
        openedBranch,
        originBranch,
        plan.branching().commitPosture,
        previewArtifact,
      );
      requireDirtyExitForSession(
        plan,
        verification,
        openedBranch,
        dirtyExitArtifact,
        dirtyExitConfirmation,
      );
      const mergeResult = await normalizedMaybePromise(
        history.merge_branches_with_proof(openedBranch.id, originBranch.id),
      );
      await normalizedMaybePromise(history.switch_branch(originBranch.id));
      terminalOperation = "commit";
      return createSpeculativeCommitArtifact(
        plan,
        verification,
        openedBranch,
        originBranch,
        mergeResult,
        normalizedPreview,
      );
    },
    async dirtyExit(specialist) {
      requireActiveSpeculativeSession(terminalOperation, "dirtyExit");
      const normalizedSpecialist = requireSpeculativeSpecialist(specialist);
      const runSummary = await normalizedMaybePromise(normalizedSpecialist.evaluateDirty());
      return createSpeculativeDirtyExitArtifact(
        plan,
        verification,
        openedBranch,
        runSummary,
      );
    },
    async discard() {
      requireActiveSpeculativeSession(terminalOperation, "discard");
      await normalizedMaybePromise(history.switch_branch(originBranch.id));
      terminalOperation = "discard";
      return createSpeculativeDiscardArtifact(
        plan,
        verification,
        openedBranch,
        originBranch,
        async (resumeHistory) => {
          const normalizedResumeHistory = requireSpeculativeHistory(resumeHistory);
          await normalizedMaybePromise(normalizedResumeHistory.switch_branch(openedBranch.id));
          return createSpeculativeSessionArtifact(
            plan,
            normalizedResumeHistory,
            originBranch,
            openedBranch,
            verification,
          );
        },
      );
    },
    verification() {
      return verification;
    },
  });
}

export {
  openSpeculativeRouteBranchSession,
};

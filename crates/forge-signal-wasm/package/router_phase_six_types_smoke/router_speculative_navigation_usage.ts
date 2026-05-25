import type {
  CallableSignals,
  SpeculativeRouteBranchCommit,
  SpeculativeRouteBranchCommitPreview,
  SpeculativeRouteBranchDiagnostics,
  SpeculativeRouteBranchDirtyExit,
  SpeculativeRouteBranchDirtyExitConfirmation,
  SpeculativeRouteBranchHistory,
  SpeculativeRouteBranchLifecycle,
  SpeculativeRouteBranchOutcome,
  SpeculativeRouteBranchPlan,
  SpeculativeRoutePendingBranch,
  SpeculativeRouteBranchSession,
  SpeculativeRouteBranchSpecialist,
  SpeculativeRouteVisibleProjection,
} from "../index.js";

declare const signals: CallableSignals;
declare const history: SpeculativeRouteBranchHistory;
declare const specialist: SpeculativeRouteBranchSpecialist;

const routes = signals.router.define({
  detail: signals.router.route("/users/:userId"),
});

const candidate = routes.project("/users/u1");

if (candidate) {
  const speculativePlan: SpeculativeRouteBranchPlan = candidate.speculate({
    branchName: "router-speculation-u1",
    commitPosture: "merge-preview-before-commit",
    discardPosture: "discard-speculative-branch",
    visiblePosture: "preserve-visible-until-commit",
  });
  const speculativeLifecycle: SpeculativeRouteBranchLifecycle =
    speculativePlan.branching();
  const speculativeDiagnostics: SpeculativeRouteBranchDiagnostics =
    speculativePlan.diagnostics();
  const projectedDigest: string =
    speculativePlan.verification().projectedCandidateDigest;
  const speculativeSessionPromise:
    Promise<SpeculativeRouteBranchSession> = speculativePlan.open(history);
  const speculativeOutcomePromise:
    Promise<SpeculativeRouteBranchOutcome> = speculativePlan.evaluate();
  const pendingVisibilityPromise:
    Promise<SpeculativeRouteVisibleProjection> = speculativePlan.evaluate()
      .then((outcome) => outcome.visibleProjection());

  void speculativePlan;
  void speculativeLifecycle;
  void speculativeDiagnostics;
  void projectedDigest;
  void speculativeSessionPromise;
  void speculativeOutcomePromise;
  void pendingVisibilityPromise;
}

const rootSpeculation = routes.speculate("/users/u1");
const rootSessionPromise = rootSpeculation?.open(history);

async function verifySessionShape() {
  if (!rootSessionPromise) {
    return;
  }
  const session: SpeculativeRouteBranchSession = await rootSessionPromise;
  const speculativeSessionOutcome: SpeculativeRouteBranchOutcome = session.outcome();
  const cleanDirtyExit: SpeculativeRouteBranchDirtyExit = await session.dirtyExit(specialist);
  const cleanConfirmation: SpeculativeRouteBranchDirtyExitConfirmation | null =
    cleanDirtyExit.confirm();
  const preview: SpeculativeRouteBranchCommitPreview = await session.commitPreview();
  const commit: SpeculativeRouteBranchCommit = await session.commit(
    preview,
    cleanDirtyExit,
    cleanConfirmation,
  );
  const dirtyExit: SpeculativeRouteBranchDirtyExit = cleanDirtyExit;
  const discard = await session.discard();
  const speculativeCommitOutcome: SpeculativeRouteBranchOutcome = commit.outcome();
  const speculativeDiscardOutcome: SpeculativeRouteBranchOutcome = discard.outcome();
  const retainedPendingBranch: SpeculativeRoutePendingBranch | null = discard.pendingBranch();
  const visibleProjection: SpeculativeRouteVisibleProjection =
    speculativeSessionOutcome.visibleProjection();

  void session.lifecycle();
  void speculativeSessionOutcome.diagnostics();
  void visibleProjection.verification();
  void cleanConfirmation;
  void preview.verification();
  void commit.verification();
  void dirtyExit.verification();
  void discard.verification();
  void speculativeCommitOutcome.verification();
  void speculativeDiscardOutcome.diagnostics();
  void retainedPendingBranch?.verification();
}

void rootSpeculation;
void verifySessionShape;

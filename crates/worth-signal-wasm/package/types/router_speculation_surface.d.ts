declare const WorthSignalSpeculativeRouteBranchPlanBrand: unique symbol;
declare const WorthSignalSpeculativeRouteBranchVerificationPackageBrand: unique symbol;
declare const WorthSignalSpeculativeRouteBranchSessionBrand: unique symbol;
declare const WorthSignalSpeculativeRouteBranchSessionVerificationPackageBrand: unique symbol;

export interface SpeculativeRouteBranchVerificationPackage {
  readonly projectedCandidateDigest: string;
  readonly speculativeBranchDigest: string;
  readonly speculativeLifecycleDigest: string;
  readonly speculativeDiagnosticsDigest: string;
  readonly [WorthSignalSpeculativeRouteBranchVerificationPackageBrand]: "speculativeRouteBranchVerificationPackage";
}

export interface SpeculativeRouteBranchSessionVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly [WorthSignalSpeculativeRouteBranchSessionVerificationPackageBrand]: "speculativeRouteBranchSessionVerificationPackage";
}

export interface SpeculativeRouteBranchCommitPreviewVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativeCommitPreviewDigest: string;
}

export interface SpeculativeRouteBranchCommitVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativeCommitDigest: string;
}

export interface SpeculativeRouteBranchOutcomeVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string | null;
  readonly routeOutcomeDigest: string | null;
  readonly speculativeOutcomeDigest: string;
}

export interface SpeculativeRouteVisibleProjectionVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string | null;
  readonly speculativeVisibleProjectionDigest: string;
}

export interface SpeculativeRouteBranchDiscardVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativeDiscardDigest: string;
}

export interface SpeculativeRoutePendingBranchVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativePendingBranchDigest: string;
}

export interface SpeculativeRouteBranchDirtyExitVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativeDirtyExitDigest: string;
}

export interface SpeculativeRouteBranchDirtyExitConfirmationVerificationPackage {
  readonly speculativeBranchDigest: string;
  readonly speculativeSessionDigest: string;
  readonly speculativeDirtyExitDigest: string;
  readonly speculativeDirtyExitConfirmationDigest: string;
}

export interface SpeculativeRouteBranchLifecycle {
  readonly candidateTruth: "branch-native-candidate-route";
  readonly branchLifecycle: "create-branch-before-commit";
  readonly branchName: string;
  readonly commitPosture:
    | "merge-preview-before-commit"
    | "direct-merge-commit";
  readonly discardPosture:
    | "discard-speculative-branch"
    | "keep-branch-pending";
  readonly visiblePosture:
    | "preserve-visible-until-commit"
    | "allow-visible-flicker-before-commit";
  readonly dirtyExit: "evaluate-dirty-before-commit";
}

export interface SpeculativeRouteBranchDiagnostics {
  readonly flickerSuppression:
    | "suppresses-visible-flicker-until-commit"
    | "allows-visible-flicker-before-commit";
  readonly commitDisposition:
    | "requires-merge-preview-before-commit"
    | "allows-direct-merge-commit";
  readonly discardDisposition:
    | "discard-ends-speculation"
    | "discard-keeps-branch-pending";
  readonly pendingDisposition: "candidate-route-remains-pending-until-commit";
  readonly dirtyExitDisposition: "requires-dirty-evaluation-before-commit";
}

export interface SpeculativeRouteBranchOutcomeDiagnostics {
  readonly status:
    | "pending"
    | "committed"
    | "discarded"
    | "redirect"
    | "notFound"
    | "forbidden"
    | "unavailable"
    | "denied";
  readonly outcomeSource:
    | "preBranchAdmission"
    | "activeSpeculativeBranch"
    | "historyCommit"
    | "historyDiscard";
  readonly branchLifecycleResult:
    | "notOpened"
    | "activeBranchPending"
    | "merged"
    | "discarded"
    | "remainedPending";
  readonly branchDisposition: string;
  readonly routeOutcomeKind:
    | import("./router_admission_surface.js").RouteOutcome["kind"]
    | null;
  readonly recoveryDisposition?: string;
  readonly sourceBranchId?: number;
  readonly targetBranchId?: number;
  readonly previewDisposition?:
    | "committed-without-preview-artifact"
    | "committed-from-preview-artifact";
  readonly mergeResultDigest?: string;
  readonly mergeLineageDigest?: string;
  readonly branchId?: number;
  readonly discardDisposition?:
    | "abandon-branch-without-merge"
    | "keep-branch-pending-without-merge";
}

export interface SpeculativeRouteBranchOutcome<
  TRouteOutcome extends import("./router_admission_surface.js").RouteOutcome | null =
    import("./router_admission_surface.js").RouteOutcome | null,
> {
  readonly kind:
    | "pending"
    | "committed"
    | "discarded"
    | "redirect"
    | "notFound"
    | "forbidden"
    | "unavailable"
    | "denied";
  readonly routeId: string | null;
  readonly href: string | null;
  routeOutcome(): TRouteOutcome;
  visibleProjection(): SpeculativeRouteVisibleProjection;
  diagnostics(): SpeculativeRouteBranchOutcomeDiagnostics;
  verification(): SpeculativeRouteBranchOutcomeVerificationPackage;
}

export interface SpeculativeRouteVisibleProjection {
  readonly kind: "speculativeVisibleProjection";
  readonly posture:
    | "preserve-visible-until-commit"
    | "allow-visible-flicker-before-commit";
  readonly state:
    | "candidateSuppressedUntilCommit"
    | "candidateVisibleWhilePending"
    | "candidateVisibleAfterCommit"
    | "candidateNotVisible";
  readonly routeId: string | null;
  readonly href: string | null;
  verification(): SpeculativeRouteVisibleProjectionVerificationPackage;
}

export interface SpeculativeRouteBranchRuntimeHandle {
  id: number;
  name: string;
  parent_branch_id: number | null;
  head_snapshot_id: number | null;
}

export interface SpeculativeRouteBranchHistory {
  current_branch():
    | SpeculativeRouteBranchRuntimeHandle
    | Promise<SpeculativeRouteBranchRuntimeHandle>;
  create_branch(name: string):
    | SpeculativeRouteBranchRuntimeHandle
    | Promise<SpeculativeRouteBranchRuntimeHandle>;
  switch_branch(branchId: number): void | Promise<void>;
  plan_merge_policy_preview_with_proof(request: {
    source_branch_id: number;
    target_branch_id: number;
    conflict_policy_name?: string | null;
    conflict_isolation_policy_name?: string | null;
    identity_matcher_name?: string | null;
    deletion_policy_name?: string | null;
  }): unknown | Promise<unknown>;
  merge_branches_with_proof(
    sourceBranchId: number,
    targetBranchId: number,
  ): unknown | Promise<unknown>;
}

export interface SpeculativeRouteBranchOptions {
  branchName?: string;
  commitPosture?: "merge-preview-before-commit" | "direct-merge-commit";
  discardPosture?: "discard-speculative-branch" | "keep-branch-pending";
  visiblePosture?:
    | "preserve-visible-until-commit"
    | "allow-visible-flicker-before-commit";
}

export interface SpeculativeRouteBranchSessionLifecycle {
  readonly branchBinding: "candidate-route-bound-to-history-branch";
  readonly originBranchId: number;
  readonly speculativeBranchId: number;
  readonly branchState: "active-speculative-branch";
  readonly commitPosture:
    | "merge-preview-before-commit"
    | "direct-merge-commit";
  readonly discardPosture:
    | "discard-speculative-branch"
    | "keep-branch-pending";
}

export interface SpeculativeRouteBranchSpecialist {
  evaluateDirty(): {
    touchedNodes: number;
    nodesEvaluated: number;
    nodesRecomputed: number;
    nodesSuppressed: number;
    plansBuilt: number;
    stagesExecuted: number;
    totalNanos: string;
    evaluationNanos: string;
    commitNanos: string;
  } | Promise<{
    touchedNodes: number;
    nodesEvaluated: number;
    nodesRecomputed: number;
    nodesSuppressed: number;
    plansBuilt: number;
    stagesExecuted: number;
    totalNanos: string;
    evaluationNanos: string;
    commitNanos: string;
  }>;
}

export interface SpeculativeRouteBranchCommitPreview {
  readonly kind: "speculativeBranchCommitPreview";
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly preview: unknown;
  readonly posture:
    | "merge-preview-before-commit"
    | "direct-merge-commit";
  verification(): SpeculativeRouteBranchCommitPreviewVerificationPackage;
}

export interface SpeculativeRouteBranchCommit {
  readonly kind: "speculativeBranchCommit";
  readonly routeId: string;
  readonly href: string;
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly mergeResult: unknown;
  readonly previewDisposition:
    | "committed-without-preview-artifact"
    | "committed-from-preview-artifact";
  outcome(): SpeculativeRouteBranchOutcome<null>;
  verification(): SpeculativeRouteBranchCommitVerificationPackage;
}

export interface SpeculativeRouteBranchDiscard {
  readonly kind: "speculativeBranchDiscard";
  readonly routeId: string;
  readonly href: string;
  readonly branchId: number;
  readonly disposition:
    | "abandon-branch-without-merge"
    | "keep-branch-pending-without-merge";
  outcome(): SpeculativeRouteBranchOutcome<null>;
  pendingBranch(): SpeculativeRoutePendingBranch | null;
  verification(): SpeculativeRouteBranchDiscardVerificationPackage;
}

export interface SpeculativeRouteBranchDirtyExit {
  readonly kind: "speculativeBranchDirtyExit";
  readonly routeId: string;
  readonly href: string;
  readonly branchId: number;
  readonly runSummary: {
    touchedNodes: number;
    nodesEvaluated: number;
    nodesRecomputed: number;
    nodesSuppressed: number;
    plansBuilt: number;
    stagesExecuted: number;
    totalNanos: string;
    evaluationNanos: string;
    commitNanos: string;
  };
  readonly disposition:
    | "clean-exit"
    | "dirty-exit-requires-confirmation";
  readonly confirmationRequired: boolean;
  confirm(): SpeculativeRouteBranchDirtyExitConfirmation | null;
  verification(): SpeculativeRouteBranchDirtyExitVerificationPackage;
}

export interface SpeculativeRouteBranchDirtyExitConfirmation {
  readonly kind: "speculativeBranchDirtyExitConfirmation";
  readonly routeId: string;
  readonly href: string;
  readonly branchId: number;
  verification(): SpeculativeRouteBranchDirtyExitConfirmationVerificationPackage;
}

export interface SpeculativeRouteBranchCommitPreviewOptions {
  conflict_policy_name?: string | null;
  conflict_isolation_policy_name?: string | null;
  identity_matcher_name?: string | null;
  deletion_policy_name?: string | null;
}

export interface SpeculativeRoutePendingBranch {
  readonly kind: "speculativePendingBranch";
  readonly routeId: string;
  readonly href: string;
  originBranch(): SpeculativeRouteBranchRuntimeHandle;
  branch(): SpeculativeRouteBranchRuntimeHandle;
  resume(
    history: SpeculativeRouteBranchHistory,
  ): Promise<SpeculativeRouteBranchSession>;
  verification(): SpeculativeRoutePendingBranchVerificationPackage;
}

export interface SpeculativeRouteBranchSession<
  TProjectedCandidate = unknown,
> {
  readonly kind: "speculativeBranchSession";
  candidate(): TProjectedCandidate;
  plan(): SpeculativeRouteBranchPlan<TProjectedCandidate>;
  originBranch(): SpeculativeRouteBranchRuntimeHandle;
  branch(): SpeculativeRouteBranchRuntimeHandle;
  lifecycle(): SpeculativeRouteBranchSessionLifecycle;
  outcome(): SpeculativeRouteBranchOutcome<null>;
  commitPreview(
    options?: SpeculativeRouteBranchCommitPreviewOptions,
  ): Promise<SpeculativeRouteBranchCommitPreview>;
  commit(
    previewArtifact?: SpeculativeRouteBranchCommitPreview | null,
    dirtyExitArtifact?: SpeculativeRouteBranchDirtyExit | null,
    dirtyExitConfirmation?: SpeculativeRouteBranchDirtyExitConfirmation | null,
  ): Promise<SpeculativeRouteBranchCommit>;
  dirtyExit(
    specialist: SpeculativeRouteBranchSpecialist,
  ): Promise<SpeculativeRouteBranchDirtyExit>;
  discard(): Promise<SpeculativeRouteBranchDiscard>;
  verification(): SpeculativeRouteBranchSessionVerificationPackage;
  readonly [WorthSignalSpeculativeRouteBranchSessionBrand]: "speculativeRouteBranchSession";
}

export interface SpeculativeRouteBranchPlan<
  TProjectedCandidate = unknown,
> {
  readonly kind: "speculativeBranchPlan";
  readonly href: string;
  readonly routeId: string;
  candidate(): TProjectedCandidate;
  branching(): SpeculativeRouteBranchLifecycle;
  diagnostics(): SpeculativeRouteBranchDiagnostics;
  evaluate(
    facts?: import("./router_admission_surface.js").RouteAdmissionFacts,
  ): Promise<SpeculativeRouteBranchOutcome>;
  open(
    history: SpeculativeRouteBranchHistory,
  ): Promise<SpeculativeRouteBranchSession<TProjectedCandidate>>;
  verification(): SpeculativeRouteBranchVerificationPackage;
  readonly [WorthSignalSpeculativeRouteBranchPlanBrand]: "speculativeRouteBranchPlan";
}

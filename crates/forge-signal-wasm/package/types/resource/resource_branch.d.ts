import type { MergePolicyPreviewRequest } from "../diagnostics.js";

export interface ResourceBranchMergePlanSummary {
  readonly kind: "planned";
  readonly sourceBranchId: number;
  readonly targetBranchId: number;
  readonly mergeKind: string;
  readonly selectedSemantics: {
    readonly strategy: string;
    readonly mergeBase: string;
    readonly conflictPolicy: string;
    readonly conflictIsolation: string;
    readonly identityMatcher: string;
    readonly sourceOnlyPolicy: string;
    readonly deletionPolicy: string;
  };
  readonly breadth: {
    readonly nodeMapCount: number;
    readonly nodePlanCount: number;
    readonly adoptionPlanCount: number;
    readonly conflictRecordCount: number;
  };
  readonly proof: {
    readonly proofSchemaVersion: string;
    readonly planDigest: string;
    readonly semanticsDigest: string;
    readonly selectedStrategyDigest: string;
    readonly selectedMergeBaseDigest: string;
    readonly selectedConflictPolicyDigest: string;
    readonly selectedConflictIsolationDigest: string;
    readonly selectedIdentityMatcherDigest: string;
    readonly selectedSourceOnlyPolicyDigest: string;
    readonly selectedDeletionPolicyDigest: string;
  };
}

export interface ResourceBranchMergePlanDenial {
  readonly kind: "denied";
  readonly reason: "mergePlanUnavailable";
  readonly detail: string;
}

export type ResourceBranchMergePlanResult =
  | ResourceBranchMergePlanSummary
  | ResourceBranchMergePlanDenial;

export interface ResourceBranchNamespace {
  planMerge(request: MergePolicyPreviewRequest): ResourceBranchMergePlanResult;
}

import type {
  ResourceLineHistoryAvailability,
  ResourceLineBranchSummary,
} from "../resource/resource_line_history.js";
import type {
  ResourceLineFreshness,
  ResourceLineStatus,
} from "../resource/resource_lifecycle.js";
import type { ResourceEffectProfileDigest } from "../resource/resource_effect_envelope.js";
import type { ResourceRequestDescriptor } from "../resource/resource_postures.js";
import type {
  ResourceFamilyKind,
  ResourceLineDescriptor,
} from "../resource/resource_request_descriptor.js";
import type { ResourceLineSummary } from "../resource/resource_line_summary.js";
import type {
  ResourceMutationResponseConfirmationKind,
  ResourceMutationResponseTargetOutcomeSummary,
} from "../resource/resource_mutation_response.js";
import type { ResourceLineCompatibilityDigest } from "../resource/resource_verification.js";

export interface FormResourceRollbackDigest {
  readonly kind:
    | "exactBranchRestoreAvailable"
    | "compactInverseAvailable"
    | "unavailable"
    | "notApplicable";
  readonly mode: "SameRuntimeBranchExact" | "CompactInversePatch" | null;
  readonly branchId: string | number | null;
  readonly snapshotId: number | null;
  readonly reason: string | null;
  readonly detail: string;
}

export interface FormResourceMutationResponseReport {
  readonly confirmationKind: ResourceMutationResponseConfirmationKind;
  readonly confirmationDigest: string;
  readonly targetCount: number;
  readonly exactTargetCount: number;
  readonly fallbackTargetCount: number;
  readonly freshnessPostureDigest: string;
  readonly fallbackReasonDigest: string;
  readonly fallbackAffectedTargetDigest: string;
  readonly staleTargetReasonDigest: string;
  readonly staleTargetAffectedTargetDigest: string;
  readonly deliveryAwaitedDigest: string;
  readonly refetchRequiredDigest: string;
  readonly partialReconciliationDigest: string;
  readonly outOfContractTargetDigest: string;
  readonly noHiddenMutationDigest: string;
  readonly contract: {
    readonly deliveryAwaitedDigest: string;
    readonly refetchRequiredDigest: string;
    readonly partialReconciliationDigest: string;
    readonly outOfContractTargetDigest: string;
    readonly digest: string;
  };
  readonly targetOutcomeDigest: string;
  readonly targetOutcomes: readonly ResourceMutationResponseTargetOutcomeSummary[];
  readonly replayExactDigest: string;
  readonly restoreExactDigest: string;
  readonly rollbackDigest: string;
  readonly mergeRebaseDigest: string;
  readonly executionDigest: string;
  readonly diagnosticCount: number;
  readonly diagnosticDigest: string;
  readonly planCount: number;
  readonly completion: {
    readonly multiFamily: boolean;
    readonly familyKinds: readonly ("detail" | "collection" | "paged")[];
    readonly exactTargetCount: number;
    readonly fallbackTargetCount: number;
    readonly familyCounts: {
      readonly detail: number;
      readonly collection: number;
      readonly paged: number;
    };
    readonly placement: {
      readonly kind: "none" | "appendOnly" | "prependOnly" | "mixed";
      readonly count: number;
      readonly appendCount: number;
      readonly prependCount: number;
    };
    readonly deletion: {
      readonly kind: "none" | "deleteOnly" | "tombstoneOnly" | "mixed";
      readonly count: number;
      readonly deleteCount: number;
      readonly tombstoneCount: number;
    };
    readonly summaryTargetCount: number;
    readonly digest: string;
  };
  readonly identityMigration: {
    readonly digest: string;
    readonly needed: boolean;
    readonly partialAdmission: "notNeeded" | "admitted" | "denied";
    readonly targetCount: number;
    readonly exactTargetCount: number;
    readonly executionDigest: string;
    readonly fallbackDigest: string;
  } | null;
  readonly digest: string;
}

export interface FormResourceTransferReport {
  readonly upload: ResourceLineUpload;
  readonly processing: ResourceLineProcessing;
  readonly download: ResourceLineDownload;
  readonly summary: {
    readonly uploadActive: boolean;
    readonly processingActive: boolean;
    readonly downloadReadyCount: number;
    readonly downloadUnavailableCount: number;
    readonly downloadIncompatibleCount: number;
  };
  readonly digest: string;
}

export interface FormResourceShapeReport {
  readonly familyKind: ResourceFamilyKind;
  readonly familyId: string;
  readonly runtimeLineId: string;
  readonly scopeId: string;
  readonly canonicalKey: string;
  readonly patchLowering:
    | "detailFieldJsonPathRegion"
    | "collectionMembershipItemFieldJsonPathRegion"
    | "pagedWindowMembershipItemFieldJsonPathRegion";
  readonly digest: string;
}

export interface FormResourceLifecycleReport {
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly activity: ResourceLineSummary["diagnostics"]["activity"];
  readonly retry: {
    readonly kind: "notNeeded" | "recommended";
    readonly operation: import("../resource/resource_lifecycle.js").ResourceLineOperation | null;
    readonly attemptCount: number;
    readonly reason: string | null;
  };
  readonly supersession: {
    readonly kind: "none" | "observed";
    readonly count: number;
    readonly lastOperation: import("../resource/resource_lifecycle.js").ResourceLineOperation | null;
  };
  readonly deliveryBasis: {
    readonly kind: "stable" | "drifted";
    readonly currentBasisId: string | null;
    readonly deliveryKind: ResourceLineSummary["diagnostics"]["latest"]["deliveryKind"];
    readonly deliveryScope: ResourceLineSummary["diagnostics"]["latest"]["deliveryScope"];
    readonly deliveryBasisId: string | null;
    readonly invalidationCause: ResourceLineSummary["diagnostics"]["latest"]["invalidationCause"];
    readonly invalidationScope: ResourceLineSummary["diagnostics"]["latest"]["invalidationScope"];
  };
  readonly counts: {
    readonly refreshCount: number;
    readonly revalidateCount: number;
    readonly retryAttemptCount: number;
    readonly rejectionCount: number;
    readonly timeoutCount: number;
    readonly supersessionCount: number;
    readonly deliveryCount: number;
  };
  readonly digest: string;
}

export interface FormResourceSettlementReport {
  readonly kind: "none" | "pending" | "confirmed" | "failed";
  readonly operation: import("../resource/resource_lifecycle.js").ResourceLineOperation;
  readonly failureKind: "rejected" | "timedOut" | null;
  readonly continuity: import("../resource/resource_lifecycle.js").ResourceLineContinuity | null;
  readonly confirmationKind: ResourceMutationResponseConfirmationKind | null;
  readonly freshnessKind: ResourceLineFreshness["kind"];
  readonly freshnessReason: Exclude<ResourceLineFreshness, { readonly kind: "fresh" }>["reason"] | null;
  readonly visibleSelectionKind: FormResourceVisibleSelectionKind;
  readonly branchProof: FormResourceVisibleSelectionProof;
  readonly rebaseProof: FormResourceVisibleSelectionProof;
  readonly message: string | null;
  readonly retryRecommended: boolean;
  readonly retryOperation: import("../resource/resource_lifecycle.js").ResourceLineOperation | null;
  readonly detail: string;
  readonly digest: string;
}

export interface FormResourceExternalCompatibilityReport {
  readonly kind: ResourceLineCompatibilityDigest["kind"];
  readonly definitionId: string | null;
  readonly version: "forge-resource-external-v1" | null;
  readonly requestContract: "native-v1" | null;
  readonly reconciliationContract: "none" | "collection-v1" | "paged-v1" | null;
  readonly deliveryContract: "nativeInternalLine" | "basisCompatV1";
  readonly digest: string;
}

export type FormResourceVisibleSelectionKind =
  | "unavailable"
  | "committed"
  | "speculative"
  | "confirmed"
  | "restored"
  | "merged";

export interface FormResourceVisibleSelectionProof {
  readonly admitted: boolean;
  readonly reason: string | null;
}

export interface FormResourceVisibleSelectionReport {
  readonly kind: FormResourceVisibleSelectionKind;
  readonly source:
    | "initialLoad"
    | "refresh"
    | "revalidate"
    | "localPatch"
    | "optimismUnavailable"
    | "delivery"
    | "historyRestore"
    | "exactBranchRestore"
    | "compactInverse"
    | "branchMerge";
  readonly effectId: string | null;
  readonly branchId: string | number | null;
  readonly snapshotId: number | null;
  readonly basisId: string | null;
  readonly unavailableReason: string | null;
  readonly rollbackKind: string | null;
  readonly confirmationKind: string | null;
  readonly previousEffectId: string | null;
  readonly detail: string;
  readonly branchProof: FormResourceVisibleSelectionProof;
  readonly rebaseProof: FormResourceVisibleSelectionProof;
  readonly digest: string;
}

export interface FormResourceSourceReport {
  readonly sourceKind: "resourceLine";
  readonly descriptor: ResourceLineDescriptor<unknown>;
  readonly request: ResourceRequestDescriptor<unknown>;
  readonly summary: ResourceLineSummary<unknown>;
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly shape: FormResourceShapeReport;
  readonly externalCompatibility: FormResourceExternalCompatibilityReport;
  readonly transfer: FormResourceTransferReport;
  readonly lifecycle: FormResourceLifecycleReport;
  readonly settlement: FormResourceSettlementReport;
  readonly effectProfile: {
    readonly profile: ResourceEffectProfileDigest | null;
    readonly closeoutMatrixDigest: string | null;
  };
  readonly rollback: FormResourceRollbackDigest | null;
  readonly visibleSelection: FormResourceVisibleSelectionReport;
  readonly history: {
    readonly branch: ResourceLineBranchSummary | null;
    readonly availability: ResourceLineHistoryAvailability;
  };
  readonly verification: {
    readonly packageDigest: string;
    readonly externalCompatibility: ResourceLineCompatibilityDigest;
    readonly mutationResponseCloseoutMatrixDigest: string | null;
  };
  readonly mutationResponse: FormResourceMutationResponseReport | null;
  readonly counters: {
    readonly costBasis: "resourceLineProofRead";
    readonly incrementalStatus: "notIncremental";
    readonly descriptorReads: number;
    readonly requestReads: number;
    readonly summaryReads: number;
    readonly statusReads: number;
    readonly freshnessReads: number;
    readonly mutationResponseReads: number;
    readonly historyReads: number;
    readonly verificationPackageReads: number;
    readonly effectCloseoutMatrixReads: number;
    readonly mutationResponseCloseoutMatrixReads: number;
  };
  readonly digest: string;
}

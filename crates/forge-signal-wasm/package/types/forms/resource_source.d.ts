import type {
  ResourceLineHistoryAvailability,
  ResourceLineBranchSummary,
} from "../resource/resource_line_history.js";
import type {
  ResourceLineFreshness,
  ResourceLineStatus,
} from "../resource/resource_lifecycle.js";
import type { ResourceLineVisibleSelection } from "../resource/resource_line_diagnostics.js";
import type { ResourceEffectProfileDigest } from "../resource/resource_effect_envelope.js";
import type { ResourceRequestDescriptor } from "../resource/resource_postures.js";
import type { ResourceLineDescriptor } from "../resource/resource_request_descriptor.js";
import type { ResourceLineSummary } from "../resource/resource_line_summary.js";
import type {
  ResourceMutationResponseConfirmationKind,
  ResourceMutationResponseTargetOutcomeSummary,
} from "../resource/resource_mutation_response.js";

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
  readonly unsupportedTargetDigest: string;
  readonly noHiddenMutationDigest: string;
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

export interface FormResourceSourceReport {
  readonly sourceKind: "resourceLine";
  readonly descriptor: ResourceLineDescriptor<unknown>;
  readonly request: ResourceRequestDescriptor<unknown>;
  readonly summary: ResourceLineSummary<unknown>;
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly effectProfile: {
    readonly profile: ResourceEffectProfileDigest | null;
    readonly closeoutMatrixDigest: string | null;
  };
  readonly rollback: FormResourceRollbackDigest | null;
  readonly visibleSelection: ResourceLineVisibleSelection;
  readonly history: {
    readonly branch: ResourceLineBranchSummary | null;
    readonly availability: ResourceLineHistoryAvailability;
  };
  readonly verification: {
    readonly packageDigest: string;
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

import type {
  ResourceLineBranchSummary,
  ResourceLineDiagnosticsActivitySummary,
  ResourceLineDiagnosticsChangeCountSummary,
  ResourceLineDiagnosticsCurrentSummary,
  ResourceLineDiagnosticsLatestChangeSummary,
  ResourceLineFreshness,
  ResourceLineHistoryAvailability,
  ResourceLineStatus,
  ResourceLineBasisHistory,
} from "./resource_lifecycle.js";

export interface ResourceLineNativeCompatibilityDigest {
  readonly kind: "native";
}

export interface ResourceLineExternalDefinitionCompatibilityDigest {
  readonly kind: "externalDefinition";
  readonly version: "forge-resource-external-v1";
  readonly definitionId: string;
  readonly requestContract: "native-v1";
  readonly reconciliationContract: "none" | "collection-v1" | "paged-v1";
}

export type ResourceLineCompatibilityDigest =
  | ResourceLineNativeCompatibilityDigest
  | ResourceLineExternalDefinitionCompatibilityDigest;

export interface ResourceLineVerificationDeclarationDigest {
  readonly familyKind: "detail" | "collection" | "paged";
  readonly familyId: string;
  readonly canonicalKey: string;
  readonly runtimeLineId: string;
  readonly scopeId: string;
}

export interface ResourceLineVerificationRequestPostureDigest {
  readonly authKind: string;
  readonly headerNames: readonly string[];
  readonly correlationId: string | null;
  readonly branchId: string | number | null;
  readonly basisId: string | null;
  readonly continuationKind: string;
  readonly processingKind: string;
  readonly uploadKind: string;
}

export interface ResourceLineVerificationLifecycleDigest {
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly lastOperation: string;
  readonly lastOutcome: "fulfilled" | "rejected" | "pending" | "timedOut";
  readonly pendingOperation: string | null;
  readonly visibleValueVersion: number;
  readonly refreshCount: number;
  readonly revalidateCount: number;
  readonly retryAttemptCount: number;
  readonly rejectionCount: number;
  readonly timeoutCount: number;
  readonly supersessionCount: number;
  readonly invalidationCount: number;
  readonly patchCount: number;
  readonly deliveryCount: number;
  readonly basisAdvanceCount: number;
}

export interface ResourceLineVerificationProcessingDigest {
  readonly kind: "ready" | "accepted" | "processing";
  readonly completionKind: "none" | "poll" | "callback" | "webhook";
  readonly jobId: string | null;
  readonly message: string | null;
}

export interface ResourceLineVerificationUploadDigest {
  readonly kind: "ready" | "prepared" | "uploaded";
  readonly transportKind: "none" | "directMultipart" | "signed";
  readonly uploadId: string | null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: string | null;
  readonly hasDescriptor: boolean;
}

export interface ResourceLineVerificationContinuityDigest {
  readonly continuity: "preserveVisibleValue";
  readonly hasVisibleValue: boolean;
  readonly visibleValueVersion: number;
}

export interface ResourceLineVerificationReconciliationDigest {
  readonly broadReplace: boolean;
  readonly narrowItem: boolean;
  readonly narrowSummary: boolean;
  readonly aspectNames: readonly string[];
  readonly summaryNames: readonly string[];
  readonly lastPatchKind: "replace" | "item" | "itemAspect" | "summary" | null;
  readonly lastPatchScope: "line" | "item" | "aspect" | "summary" | null;
  readonly lastPatchedItemId: string | null;
  readonly lastPatchedAspect: string | null;
  readonly lastPatchedSummary: string | null;
}

export interface ResourceLineVerificationDiagnosticsDigest {
  readonly lastOperation: string;
  readonly lastOutcome: "fulfilled" | "rejected" | "pending" | "timedOut";
  readonly pendingOperation: string | null;
  readonly lastErrorMessage: string | null;
  readonly summary: {
    readonly current: ResourceLineDiagnosticsCurrentSummary;
    readonly activity: ResourceLineDiagnosticsActivitySummary;
    readonly counts: ResourceLineDiagnosticsChangeCountSummary;
    readonly latest: ResourceLineDiagnosticsLatestChangeSummary;
  };
}

export interface ResourceLineVerificationHistoryReplayRestoreDigest {
  readonly replay: unknown;
  readonly lineage: unknown;
  readonly branch: ResourceLineBranchSummary | null;
  readonly basis: ResourceLineBasisHistory;
  readonly availability: ResourceLineHistoryAvailability;
  readonly lifecycleLength: number;
  readonly lastLifecycleEvent: string | null;
}

export interface ResourceLineVerificationBinaryDownloadDigest {
  readonly count: number;
  readonly readyCount: number;
  readonly unavailableCount: number;
  readonly incompatibleCount: number;
  readonly descriptorKinds: readonly {
    readonly kind: "file" | "media" | "export";
    readonly downloadKind: "ready" | "unavailable" | "incompatible";
  }[];
}

export interface ResourceLineVerificationDeliveryProvenanceDigest {
  readonly deliveryCount: number;
  readonly lastDeliveryKind:
    | "replace"
    | "patch"
    | "invalidate"
    | "basisRefresh"
    | null;
  readonly lastDeliveryScope:
    | "line"
    | "item"
    | "aspect"
    | "summary"
    | "basis"
    | "invalidate"
    | null;
  readonly lastDeliveryPacketId: string | null;
  readonly lastDeliveryBasisId: string | null;
  readonly basisCurrentId: string | null;
  readonly basisAdvanceCount: number;
  readonly basisAdvanceFromId: string | null;
  readonly basisAdvanceToId: string | null;
}

export interface ResourceLineVerificationBoundaryPerformanceEnvelope {
  readonly lifecycleEntryCount: number;
  readonly downloadDescriptorCount: number;
  readonly summaryReadShape: "inspectionSummary";
}

export interface ResourceLineVerificationTypedDenialsDigest {
  readonly replay: ResourceLineHistoryAvailability["replay"] | null;
  readonly replayExact: ResourceLineHistoryAvailability["replayExact"] | null;
  readonly lineage: ResourceLineHistoryAvailability["lineage"] | null;
  readonly branch: ResourceLineHistoryAvailability["branch"] | null;
  readonly restoreExact: ResourceLineHistoryAvailability["restoreExact"] | null;
}

export interface ResourceLineVerificationPackage {
  readonly declaration: ResourceLineVerificationDeclarationDigest;
  readonly committedValue: unknown;
  readonly requestPosture: ResourceLineVerificationRequestPostureDigest;
  readonly processing: ResourceLineVerificationProcessingDigest;
  readonly upload: ResourceLineVerificationUploadDigest;
  readonly lifecycle: ResourceLineVerificationLifecycleDigest;
  readonly continuity: ResourceLineVerificationContinuityDigest;
  readonly reconciliation: ResourceLineVerificationReconciliationDigest;
  readonly diagnostics: ResourceLineVerificationDiagnosticsDigest;
  readonly historyReplayRestore: ResourceLineVerificationHistoryReplayRestoreDigest;
  readonly binaryDownload: ResourceLineVerificationBinaryDownloadDigest;
  readonly deliveryProvenance: ResourceLineVerificationDeliveryProvenanceDigest;
  readonly externalCompatibility: ResourceLineCompatibilityDigest;
  readonly boundaryPerformanceEnvelope:
    ResourceLineVerificationBoundaryPerformanceEnvelope;
  readonly typedDenials: ResourceLineVerificationTypedDenialsDigest;
}

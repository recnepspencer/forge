import type {
  ComputedSignalHandle,
  Signal,
} from "../callable_surface.js";
import type { SignalValue } from "../model.js";
import type {
  LineageSummary,
  ReplaySummary,
} from "../diagnostics.js";
import type {
  ResourceLineDescriptor,
  ResourcePolicyProfileName,
  ResourceProcessingCompletionKind,
  ResourceRequestDescriptor,
  ResourceRequestDiagnostics,
  ResourceUploadDescriptor,
  ResourceUploadTransportKind,
} from "./resource_postures.js";
import type {
  ResourceItemAspectMap,
  ResourceLineReconciliation,
} from "./resource_reconciliation.js";

export type ResourceLineOperation = "initialLoad" | "refresh" | "revalidate";
export type ResourceLineContinuity =
  | "preservedVisibleValue"
  | "noVisibleValueYet";

export interface ResourceLinePendingStatus {
  readonly kind: "pending";
  readonly operation: ResourceLineOperation;
  readonly continuity: ResourceLineContinuity;
}

export interface ResourceLineFulfilledStatus {
  readonly kind: "fulfilled";
  readonly operation: ResourceLineOperation;
}

export interface ResourceLineTimedOutStatus {
  readonly kind: "timedOut";
  readonly operation: ResourceLineOperation;
  readonly continuity: ResourceLineContinuity;
}

export interface ResourceLineRejectedStatus {
  readonly kind: "rejected";
  readonly operation: ResourceLineOperation;
  readonly message: string;
  readonly continuity: ResourceLineContinuity;
}

export type ResourceLineStatus =
  | ResourceLinePendingStatus
  | ResourceLineFulfilledStatus
  | ResourceLineTimedOutStatus
  | ResourceLineRejectedStatus;

export interface ResourceLineFresh {
  readonly kind: "fresh";
}

export interface ResourceLineStaleFreshness {
  readonly kind: "stale";
  readonly reason:
    | "policyProfile"
    | "initialLoadPending"
    | "initialLoadRejected"
    | "initialLoadTimedOut"
    | "refreshPending"
    | "refreshRejected"
    | "refreshTimedOut"
    | "revalidatePending"
    | "revalidateRejected"
    | "revalidateTimedOut"
    | "manualLineInvalidate"
    | "manualFamilyInvalidate"
    | "manualFamilyInvalidateAll";
}

export type ResourceLineFreshness =
  | ResourceLineFresh
  | ResourceLineStaleFreshness;

export interface ResourceLineReadyProcessing {
  readonly kind: "ready";
  readonly completionKind: ResourceProcessingCompletionKind;
  readonly jobId: null;
  readonly message: null;
}

export interface ResourceLineAcceptedProcessing {
  readonly kind: "accepted";
  readonly completionKind: Exclude<ResourceProcessingCompletionKind, "none">;
  readonly jobId: string;
  readonly message: string | null;
}

export interface ResourceLineInProgressProcessing {
  readonly kind: "processing";
  readonly completionKind: Exclude<ResourceProcessingCompletionKind, "none">;
  readonly jobId: string;
  readonly message: string | null;
}

export type ResourceLineProcessing =
  | ResourceLineReadyProcessing
  | ResourceLineAcceptedProcessing
  | ResourceLineInProgressProcessing;

export interface ResourceLineReadyUpload {
  readonly kind: "ready";
  readonly transportKind: ResourceUploadTransportKind;
  readonly uploadId: null;
  readonly descriptor: null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: null;
}

export interface ResourceLinePreparedUpload {
  readonly kind: "prepared";
  readonly transportKind: Exclude<ResourceUploadTransportKind, "none">;
  readonly uploadId: string;
  readonly descriptor: ResourceUploadDescriptor;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: false;
  readonly message: string | null;
}

export interface ResourceLineUploadedUpload {
  readonly kind: "uploaded";
  readonly transportKind: Exclude<ResourceUploadTransportKind, "none">;
  readonly uploadId: string;
  readonly descriptor: null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: string | null;
}

export type ResourceLineUpload =
  | ResourceLineReadyUpload
  | ResourceLinePreparedUpload
  | ResourceLineUploadedUpload;

export interface ResourceUploadDescriptorDiagnostics {
  readonly kind: "signed" | "directMultipart";
  readonly url: string;
  readonly method: "PUT" | "POST";
  readonly headerNames: readonly string[];
  readonly fieldNames: readonly string[];
  readonly objectKey: string | null;
  readonly expiresAt: string | null;
}

export interface ResourceLineReadyUploadDiagnostics {
  readonly kind: "ready";
  readonly transportKind: ResourceUploadTransportKind;
  readonly uploadId: null;
  readonly descriptor: null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: null;
}

export interface ResourceLinePreparedUploadDiagnostics {
  readonly kind: "prepared";
  readonly transportKind: Exclude<ResourceUploadTransportKind, "none">;
  readonly uploadId: string;
  readonly descriptor: ResourceUploadDescriptorDiagnostics;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: false;
  readonly message: string | null;
}

export interface ResourceLineUploadedUploadDiagnostics {
  readonly kind: "uploaded";
  readonly transportKind: Exclude<ResourceUploadTransportKind, "none">;
  readonly uploadId: string;
  readonly descriptor: null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: string | null;
}

export type ResourceLineUploadDiagnostics =
  | ResourceLineReadyUploadDiagnostics
  | ResourceLinePreparedUploadDiagnostics
  | ResourceLineUploadedUploadDiagnostics;

export interface ResourceLineDiagnostics {
  readonly policyProfileName: ResourcePolicyProfileName;
  readonly continuity: "preserveVisibleValue";
  readonly freshnessPolicy: ResourcePolicyProfileName;
  readonly request: ResourceRequestDiagnostics;
  readonly processing: ResourceLineProcessing;
  readonly upload: ResourceLineUploadDiagnostics;
  readonly lastOperation: ResourceLineOperation;
  readonly lastOutcome: "fulfilled" | "rejected" | "pending" | "timedOut";
  readonly pendingOperation: ResourceLineOperation | null;
  readonly refreshCount: number;
  readonly revalidateCount: number;
  readonly retryAttemptCount: number;
  readonly rejectionCount: number;
  readonly timeoutCount: number;
  readonly supersessionCount: number;
  readonly invalidationCount: number;
  readonly patchCount: number;
  readonly lastSupersededOperation: ResourceLineOperation | null;
  readonly lastInvalidationCause:
    | "manualLineInvalidate"
    | "manualFamilyInvalidate"
    | "manualFamilyInvalidateAll"
    | null;
  readonly lastInvalidationScope: "line" | "familyMember" | "familyAll" | null;
  readonly lastPatchKind: "replace" | "item" | "itemAspect" | "summary" | null;
  readonly lastPatchScope: "line" | "item" | "aspect" | "summary" | null;
  readonly lastPatchedItemId: string | null;
  readonly lastPatchedAspect: string | null;
  readonly lastPatchedSummary: string | null;
  readonly preservedVisibleValueOnLastRejection: boolean;
  readonly lastTimeoutOperation: ResourceLineOperation | null;
  readonly lastErrorMessage: string | null;
  readonly visibleValueVersion: number;
}

export interface ResourceLineDiagnosticsCurrentSummary {
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly hasVisibleValue: boolean;
  readonly visibleValueVersion: number;
}

export interface ResourceLineDiagnosticsActivitySummary {
  readonly lastOperation: ResourceLineOperation;
  readonly lastOutcome: "fulfilled" | "rejected" | "pending" | "timedOut";
  readonly pendingOperation: ResourceLineOperation | null;
  readonly continuity: "preserveVisibleValue";
  readonly freshnessPolicy: ResourcePolicyProfileName;
}

export interface ResourceLineDiagnosticsChangeCountSummary {
  readonly refreshCount: number;
  readonly revalidateCount: number;
  readonly retryAttemptCount: number;
  readonly rejectionCount: number;
  readonly timeoutCount: number;
  readonly supersessionCount: number;
  readonly invalidationCount: number;
  readonly patchCount: number;
}

export interface ResourceLineDiagnosticsLatestChangeSummary {
  readonly invalidationCause:
    | "manualLineInvalidate"
    | "manualFamilyInvalidate"
    | "manualFamilyInvalidateAll"
    | null;
  readonly invalidationScope: "line" | "familyMember" | "familyAll" | null;
  readonly patchKind: "replace" | "item" | "itemAspect" | "summary" | null;
  readonly patchScope: "line" | "item" | "aspect" | "summary" | null;
  readonly patchedItemId: string | null;
  readonly patchedAspect: string | null;
  readonly patchedSummary: string | null;
  readonly supersededOperation: ResourceLineOperation | null;
  readonly timeoutOperation: ResourceLineOperation | null;
  readonly errorMessage: string | null;
  readonly preservedVisibleValueOnLastRejection: boolean;
}

export interface ResourceLineDiagnosticsSummary {
  readonly current: ResourceLineDiagnosticsCurrentSummary;
  readonly activity: ResourceLineDiagnosticsActivitySummary;
  readonly counts: ResourceLineDiagnosticsChangeCountSummary;
  readonly latest: ResourceLineDiagnosticsLatestChangeSummary;
  readonly request: ResourceRequestDiagnostics;
  readonly processing: ResourceLineProcessing;
  readonly upload: ResourceLineUploadDiagnostics;
  readonly explainability: ResourceLineHistoryAvailability;
}

export type ResourceLineHistoryEvent =
  | "materialized"
  | "pending"
  | "superseded"
  | "patched"
  | "fulfilled"
  | "rejected"
  | "timedOut"
  | "invalidated";

export interface ResourceLineHistoryEntry {
  readonly sequence: number;
  readonly event: ResourceLineHistoryEvent;
  readonly status: ResourceLineStatus;
  readonly freshness: ResourceLineFreshness;
  readonly lastOperation: ResourceLineOperation;
  readonly lastOutcome: "fulfilled" | "rejected" | "pending" | "timedOut";
  readonly pendingOperation: ResourceLineOperation | null;
  readonly statusContinuity: ResourceLineContinuity | null;
  readonly retryAttemptCount: number;
  readonly rejectionCount: number;
  readonly timeoutCount: number;
  readonly supersessionCount: number;
  readonly supersededOperation: ResourceLineOperation | null;
  readonly invalidationCount: number;
  readonly patchCount: number;
  readonly lastSupersededOperation: ResourceLineOperation | null;
  readonly lastInvalidationCause:
    | "manualLineInvalidate"
    | "manualFamilyInvalidate"
    | "manualFamilyInvalidateAll"
    | null;
  readonly lastInvalidationScope: "line" | "familyMember" | "familyAll" | null;
  readonly lastPatchKind: "replace" | "item" | "itemAspect" | "summary" | null;
  readonly lastPatchScope: "line" | "item" | "aspect" | "summary" | null;
  readonly lastPatchedItemId: string | null;
  readonly lastPatchedAspect: string | null;
  readonly lastPatchedSummary: string | null;
  readonly preservedVisibleValueOnLastRejection: boolean;
  readonly lastTimeoutOperation: ResourceLineOperation | null;
  readonly lastErrorMessage: string | null;
  readonly visibleValueVersion: number;
}

export interface ResourceLineBranchSummary {
  readonly id: number;
  readonly name: string;
  readonly parentBranchId: number | null;
  readonly headSnapshotId: number | null;
}

export interface ResourceLineHistoryArtifactAvailable {
  readonly kind: "available";
}

export interface ResourceLineHistoryArtifactUnavailable {
  readonly kind: "unavailable";
  readonly reason: "unsupportedByRuntime";
  readonly detail: string;
}

export type ResourceLineHistoryArtifactAvailability =
  | ResourceLineHistoryArtifactAvailable
  | ResourceLineHistoryArtifactUnavailable;

export interface ResourceLineRestoreAvailable {
  readonly kind: "available";
  readonly mode: "SameRuntimeBranchExact";
  readonly branchId: number;
  readonly snapshotId: number;
}

export interface ResourceLineRestoreUnavailable {
  readonly kind: "unavailable";
  readonly reason: "unsupportedByRuntime" | "branchHeadUnavailable";
  readonly detail: string;
}

export type ResourceLineRestoreAvailability =
  | ResourceLineRestoreAvailable
  | ResourceLineRestoreUnavailable;

export interface ResourceLineHistoryAvailability {
  readonly replay: ResourceLineHistoryArtifactAvailability;
  readonly lineage: ResourceLineHistoryArtifactAvailability;
  readonly branch: ResourceLineHistoryArtifactAvailability;
  readonly restoreExact: ResourceLineRestoreAvailability;
}

export interface ResourceLineHistory {
  readonly replay: ReplaySummary | null;
  readonly lineage: LineageSummary | null;
  readonly branch: ResourceLineBranchSummary | null;
  readonly availability: ResourceLineHistoryAvailability;
  readonly lifecycle: readonly ResourceLineHistoryEntry[];
}

export interface ResourceLine<TParams = unknown, TValue = SignalValue> {
  value(): TValue;
  signal(): ComputedSignalHandle<TValue>;
  descriptor(): ResourceLineDescriptor<TParams>;
  request(): ResourceRequestDescriptor<TParams>;
  history(): ResourceLineHistory;
  processing(): ResourceLineProcessing;
  upload(): ResourceLineUpload;
  diagnostics(): ResourceLineDiagnostics;
  diagnosticsSummary(): ResourceLineDiagnosticsSummary;
  free(): void;
  invalidate(): ResourceLineFreshness;
  refresh(): ResourceLineStatus;
  revalidate(): ResourceLineStatus;
  [Symbol.dispose](): void;
  status(): ResourceLineStatus;
  freshness(): ResourceLineFreshness;
  view<TView = SignalValue>(
    project: (value: TValue) => TView,
  ): ResourceLineView<TView>;
}

export interface ResourceLineView<TValue = SignalValue> extends Signal<TValue> {}

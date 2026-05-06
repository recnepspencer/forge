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
  ResourceBinaryDescriptor,
  ResourceRequestDescriptor,
  ResourceRequestDiagnostics,
  ResourceUploadDescriptor,
  ResourceUploadTransportKind,
} from "./resource_postures.js";
import type {
  ResourceItemAspectMap,
  ResourceLineReconciliation,
} from "./resource_reconciliation.js";
import type { ResourceLineVerificationPackage } from "./resource_verification.js";
import type {
  ResourceLineHistory,
  ResourceLineHistoryAvailability,
} from "./resource_line_history.js";
import type {
  ResourceLineDiagnosticsSummary,
  ResourceLineSummary,
} from "./resource_line_summary.js";

export type ResourceLineOperation =
  | "initialLoad"
  | "refresh"
  | "revalidate"
  | "replay"
  | "restore"
  | "delivery";
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
    | "replayPending"
    | "replayRejected"
    | "replayTimedOut"
    | "deliveryInvalidate"
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

export interface ResourceLineDownloadReadyDescriptor {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "ready";
    readonly transportKind: "simple" | "directMultipart";
    readonly url: string;
    readonly method: "GET" | "POST";
    readonly headers: Readonly<Record<string, string>>;
    readonly fields: Readonly<Record<string, string>>;
    readonly objectKey: string | null;
    readonly expiresAt: string | null;
  };
}

export interface ResourceLineDownloadUnavailableDescriptor {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "unavailable";
    readonly reason: "notReady" | "unavailable";
    readonly detail: string;
  };
}

export interface ResourceLineDownloadIncompatibleDescriptor {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "incompatible";
    readonly reason: "staleDescriptor" | "transportBoundary";
    readonly detail: string;
  };
}

export type ResourceLineDownloadDescriptor =
  | ResourceLineDownloadReadyDescriptor
  | ResourceLineDownloadUnavailableDescriptor
  | ResourceLineDownloadIncompatibleDescriptor;

export interface ResourceLineDownload {
  readonly count: number;
  readonly readyCount: number;
  readonly unavailableCount: number;
  readonly incompatibleCount: number;
  readonly descriptors: readonly ResourceLineDownloadDescriptor[];
}

export interface ResourceLineDownloadReadyDescriptorDiagnostics {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "ready";
    readonly transportKind: "simple" | "directMultipart";
    readonly url: string;
    readonly method: "GET" | "POST";
    readonly headerNames: readonly string[];
    readonly fieldNames: readonly string[];
    readonly objectKey: string | null;
    readonly expiresAt: string | null;
  };
}

export interface ResourceLineDownloadUnavailableDescriptorDiagnostics {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "unavailable";
    readonly reason: "notReady" | "unavailable";
    readonly detail: string;
  };
}

export interface ResourceLineDownloadIncompatibleDescriptorDiagnostics {
  readonly kind: "file" | "media" | "export";
  readonly id: string;
  readonly label: string | null;
  readonly fileName: string | null;
  readonly mediaType: string | null;
  readonly byteLength: number | null;
  readonly download: {
    readonly kind: "incompatible";
    readonly reason: "staleDescriptor" | "transportBoundary";
    readonly detail: string;
  };
}

export type ResourceLineDownloadDescriptorDiagnostics =
  | ResourceLineDownloadReadyDescriptorDiagnostics
  | ResourceLineDownloadUnavailableDescriptorDiagnostics
  | ResourceLineDownloadIncompatibleDescriptorDiagnostics;

export interface ResourceLineDownloadDiagnostics {
  readonly count: number;
  readonly readyCount: number;
  readonly unavailableCount: number;
  readonly incompatibleCount: number;
  readonly descriptors: readonly ResourceLineDownloadDescriptorDiagnostics[];
}

export interface ResourceLineBasisDiagnostics {
  readonly currentBasisId: string | null;
  readonly advanceCount: number;
  readonly lastAdvanceFromBasisId: string | null;
  readonly lastAdvanceToBasisId: string | null;
}

export interface ResourceLineDiagnostics {
  readonly policyProfileName: ResourcePolicyProfileName;
  readonly continuity: "preserveVisibleValue";
  readonly freshnessPolicy: ResourcePolicyProfileName;
  readonly request: ResourceRequestDiagnostics;
  readonly basis: ResourceLineBasisDiagnostics;
  readonly processing: ResourceLineProcessing;
  readonly upload: ResourceLineUploadDiagnostics;
  readonly download: ResourceLineDownloadDiagnostics;
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
  readonly deliveryCount: number;
  readonly lastSupersededOperation: ResourceLineOperation | null;
  readonly lastInvalidationCause:
    | "deliveryInvalidate"
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
  readonly preservedVisibleValueOnLastRejection: boolean;
  readonly lastTimeoutOperation: ResourceLineOperation | null;
  readonly lastErrorMessage: string | null;
  readonly visibleValueVersion: number;
}

export interface ResourceLine<TParams = unknown, TValue = SignalValue> {
  value(): TValue;
  signal(): ComputedSignalHandle<TValue>;
  descriptor(): ResourceLineDescriptor<TParams>;
  request(): ResourceRequestDescriptor<TParams>;
  summary(): ResourceLineSummary<TParams>;
  download(): ResourceLineDownload;
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

export type {
  ResourceLineHistory,
  ResourceLineHistoryAvailability,
} from "./resource_line_history.js";
export type {
  ResourceLineDiagnosticsSummary,
  ResourceLineSummary,
} from "./resource_line_summary.js";

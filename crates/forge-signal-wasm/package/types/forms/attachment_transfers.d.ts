import type { SignalValue } from "../model.js";
import type {
  ResourceLineDownloadDescriptor,
  ResourceLineProcessing,
  ResourceLineUpload,
} from "../resource/resource_lifecycle.js";

export interface FormAttachmentTransferFieldReport {
  readonly field: string;
  readonly path: string;
  readonly fieldFamily: "attachment" | "evidence";
  readonly attachmentDigest: string | null;
  readonly attachmentPresent: boolean;
  readonly metadata: Readonly<Record<string, SignalValue>>;
  readonly bindingKind: "resourceTransfer" | "outsideTransferSurface" | "mappingUnavailable" | "noAttachment";
  readonly status: "ready" | "busy" | "unavailable";
  readonly reason: string;
  readonly transferActive: boolean;
  readonly upload: ResourceLineUpload | null;
  readonly processing: ResourceLineProcessing | null;
  readonly matchingDownloadDescriptors: readonly ResourceLineDownloadDescriptor[];
  readonly matchingDescriptorCount: number;
  readonly readyDescriptorCount: number;
  readonly unavailableDescriptorCount: number;
  readonly incompatibleDescriptorCount: number;
  readonly digest: string;
}

export interface FormAttachmentTransfersReport {
  readonly fields: readonly FormAttachmentTransferFieldReport[];
  readonly summary: {
    readonly totalFields: number;
    readonly binaryFields: number;
    readonly attachmentFields: number;
    readonly evidenceFields: number;
    readonly transferSurfaceFields: number;
    readonly busyFields: number;
    readonly unavailableFields: number;
    readonly mappingUnavailableFields: number;
  };
  readonly counters: {
    readonly costBasis: "attachmentTransferDerivedScan";
    readonly incrementalStatus: "notIncremental";
    readonly binaryFields: number;
    readonly attachmentFields: number;
    readonly evidenceFields: number;
    readonly transferSurfaceFields: number;
    readonly mappedFields: number;
    readonly busyFields: number;
    readonly unavailableFields: number;
  };
  readonly digest: string;
}

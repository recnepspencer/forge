declare const forgeSignalDeclaredResourceParamsBrand: unique symbol;
declare const forgeSignalResourceParamIdentityBrand: unique symbol;
declare const forgeSignalResourcePolicyProfileBrand: unique symbol;
declare const forgeSignalResourceAuthPostureBrand: unique symbol;
declare const forgeSignalResourceRequestContextBrand: unique symbol;
declare const forgeSignalResourceContinuationPostureBrand: unique symbol;
declare const forgeSignalResourceProcessingJobPostureBrand: unique symbol;
declare const forgeSignalResourceProcessingResultBrand: unique symbol;
declare const forgeSignalResourceUploadTransportPostureBrand: unique symbol;
declare const forgeSignalResourceUploadResultBrand: unique symbol;
declare const forgeSignalResourceDownloadBrand: unique symbol;
declare const forgeSignalResourceBinaryDescriptorBrand: unique symbol;
declare const forgeSignalResourceBinaryValueBrand: unique symbol;
export type { ResourceFamilyIdentity, ResourceLineDescriptor, ResourceRequestContextSummary, ResourceRequestDescriptor, ResourceRequestDiagnostics, ResourceRequestMethod, ResourceRequestTarget } from "./resource_request_descriptor.js";

export interface DeclaredResourceParams<TParams> {
  readonly [forgeSignalDeclaredResourceParamsBrand]: "declaredResourceParams";
  readonly __params?: TParams;
}

export interface ResourceParamIdentity<TParams> {
  readonly params: TParams;
  readonly canonicalKey: string;
  readonly [forgeSignalResourceParamIdentityBrand]: "resourceParamIdentity";
}

export type ResourcePolicyProfileName =
  | "stable"
  | "immediatelyStale"
  | "retryOnce"
  | "timeoutFast";

export interface ResourcePolicyProfile {
  readonly name: ResourcePolicyProfileName;
  readonly [forgeSignalResourcePolicyProfileBrand]: "resourcePolicyProfile";
}

export interface ResourcePolicyProfiles {
  stable(): ResourcePolicyProfile;
  immediatelyStale(): ResourcePolicyProfile;
  retryOnce(): ResourcePolicyProfile;
  timeoutFast(): ResourcePolicyProfile;
}

export type ResourceAuthKind = "anonymous" | "authenticated" | "workspace";

export interface ResourceAuthPosture {
  readonly kind: ResourceAuthKind;
  readonly [forgeSignalResourceAuthPostureBrand]: "resourceAuthPosture";
}

export interface ResourceAuth {
  anonymous(): ResourceAuthPosture;
  authenticated(): ResourceAuthPosture;
  workspace(): ResourceAuthPosture;
}

export interface ResourceRequestContext {
  readonly headers: Readonly<Record<string, string>>;
  readonly correlationId: string | null;
  readonly branchId: string | number | null;
  readonly basisId: string | null;
  readonly [forgeSignalResourceRequestContextBrand]: "resourceRequestContext";
}

export interface ResourceRequestContextOptions {
  headers?: Readonly<Record<string, string>>;
  correlationId?: string;
  branchId?: string | number;
  basisId?: string;
  basis?: string;
}

export type ResourceContinuationKind =
  | "none"
  | "redirect"
  | "callback"
  | "webhook";

export interface NoContinuationPosture {
  readonly kind: "none";
  readonly [forgeSignalResourceContinuationPostureBrand]:
    "resourceContinuationPosture";
}

export interface RedirectContinuationPosture {
  readonly kind: "redirect";
  readonly returnTo: string | null;
  readonly [forgeSignalResourceContinuationPostureBrand]:
    "resourceContinuationPosture";
}

export interface CallbackContinuationPosture {
  readonly kind: "callback";
  readonly callbackId: string;
  readonly returnTo: string | null;
  readonly [forgeSignalResourceContinuationPostureBrand]:
    "resourceContinuationPosture";
}

export interface WebhookContinuationPosture {
  readonly kind: "webhook";
  readonly correlationKey: string;
  readonly provider: string | null;
  readonly [forgeSignalResourceContinuationPostureBrand]:
    "resourceContinuationPosture";
}

export type ResourceContinuationPosture =
  | NoContinuationPosture
  | RedirectContinuationPosture
  | CallbackContinuationPosture
  | WebhookContinuationPosture;

export interface RedirectContinuationOptions {
  returnTo?: string;
}

export interface CallbackContinuationOptions {
  callbackId: string;
  returnTo?: string;
}

export interface WebhookContinuationOptions {
  correlationKey: string;
  provider?: string;
}

export interface ResourceContinuation {
  none(): NoContinuationPosture;
  redirect(options?: RedirectContinuationOptions): RedirectContinuationPosture;
  callback(options: CallbackContinuationOptions): CallbackContinuationPosture;
  webhook(options: WebhookContinuationOptions): WebhookContinuationPosture;
}

export type ResourceProcessingCompletionKind =
  | "none"
  | "poll"
  | "callback"
  | "webhook";

export interface NoProcessingJobPosture {
  readonly kind: "none";
  readonly [forgeSignalResourceProcessingJobPostureBrand]:
    "resourceProcessingJobPosture";
}

export interface PollProcessingJobPosture {
  readonly kind: "poll";
  readonly [forgeSignalResourceProcessingJobPostureBrand]:
    "resourceProcessingJobPosture";
}

export interface CallbackProcessingJobPosture {
  readonly kind: "callback";
  readonly callbackId: string;
  readonly [forgeSignalResourceProcessingJobPostureBrand]:
    "resourceProcessingJobPosture";
}

export interface WebhookProcessingJobPosture {
  readonly kind: "webhook";
  readonly correlationKey: string;
  readonly provider: string | null;
  readonly [forgeSignalResourceProcessingJobPostureBrand]:
    "resourceProcessingJobPosture";
}

export type ResourceProcessingJobPosture =
  | NoProcessingJobPosture
  | PollProcessingJobPosture
  | CallbackProcessingJobPosture
  | WebhookProcessingJobPosture;

export interface CallbackProcessingJobOptions {
  callbackId: string;
}

export interface WebhookProcessingJobOptions {
  correlationKey: string;
  provider?: string;
}

export interface ResourceProcessingJob {
  none(): NoProcessingJobPosture;
  poll(): PollProcessingJobPosture;
  callback(options: CallbackProcessingJobOptions): CallbackProcessingJobPosture;
  webhook(options: WebhookProcessingJobOptions): WebhookProcessingJobPosture;
}

export interface ResourceProcessingAcceptedResult {
  readonly kind: "accepted";
  readonly jobId: string;
  readonly message: string | null;
  readonly [forgeSignalResourceProcessingResultBrand]:
    "resourceProcessingResult";
}

export interface ResourceProcessingInProgressResult {
  readonly kind: "processing";
  readonly jobId: string;
  readonly message: string | null;
  readonly [forgeSignalResourceProcessingResultBrand]:
    "resourceProcessingResult";
}

export type ResourceProcessingResultValue =
  | ResourceProcessingAcceptedResult
  | ResourceProcessingInProgressResult;

export interface ResourceProcessingResult {
  accepted(options: {
    jobId: string;
    message?: string;
  }): ResourceProcessingAcceptedResult;
  processing(options: {
    jobId: string;
    message?: string;
  }): ResourceProcessingInProgressResult;
}

export type ResourceUploadTransportKind = "none" | "directMultipart" | "signed";

export interface NoUploadTransportPosture {
  readonly kind: "none";
  readonly [forgeSignalResourceUploadTransportPostureBrand]:
    "resourceUploadTransportPosture";
}

export interface DirectMultipartUploadTransportPosture {
  readonly kind: "directMultipart";
  readonly finalizeRequired: boolean;
  readonly [forgeSignalResourceUploadTransportPostureBrand]:
    "resourceUploadTransportPosture";
}

export interface SignedUploadTransportPosture {
  readonly kind: "signed";
  readonly method: "PUT" | "POST";
  readonly finalizeRequired: boolean;
  readonly [forgeSignalResourceUploadTransportPostureBrand]:
    "resourceUploadTransportPosture";
}

export type ResourceUploadTransportPosture =
  | NoUploadTransportPosture
  | DirectMultipartUploadTransportPosture
  | SignedUploadTransportPosture;

export interface DirectMultipartUploadTransportOptions {
  finalizeRequired?: boolean;
}

export interface SignedUploadTransportOptions {
  method?: "PUT" | "POST";
  finalizeRequired?: boolean;
}

export interface ResourceUploadTransport {
  none(): NoUploadTransportPosture;
  directMultipart(
    options?: DirectMultipartUploadTransportOptions,
  ): DirectMultipartUploadTransportPosture;
  signed(
    options?: SignedUploadTransportOptions,
  ): SignedUploadTransportPosture;
}

export interface ResourceUploadDescriptor {
  readonly kind: "signed" | "directMultipart";
  readonly url: string;
  readonly method: "PUT" | "POST";
  readonly headers: Readonly<Record<string, string>>;
  readonly fields: Readonly<Record<string, string>>;
  readonly objectKey: string | null;
  readonly expiresAt: string | null;
}

export interface ResourceUploadPreparedResult {
  readonly kind: "prepared";
  readonly uploadId: string;
  readonly descriptor: ResourceUploadDescriptor;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: false;
  readonly message: string | null;
  readonly [forgeSignalResourceUploadResultBrand]: "resourceUploadResult";
}

export interface ResourceUploadUploadedResult {
  readonly kind: "uploaded";
  readonly uploadId: string;
  readonly descriptor: null;
  readonly finalizeRequired: boolean;
  readonly awaitingProcessing: boolean;
  readonly message: string | null;
  readonly [forgeSignalResourceUploadResultBrand]: "resourceUploadResult";
}

export type ResourceUploadResultValue =
  | ResourceUploadPreparedResult
  | ResourceUploadUploadedResult;

export interface ResourceUploadResult {
  prepared(options: {
    uploadId: string;
    descriptor: ResourceUploadDescriptor;
    finalizeRequired: boolean;
    message?: string;
  }): ResourceUploadPreparedResult;
  uploaded(options: {
    uploadId: string;
    finalizeRequired: boolean;
    awaitingProcessing: boolean;
    message?: string;
  }): ResourceUploadUploadedResult;
}
export type ResourceDownloadTransportKind = "simple" | "directMultipart";
export interface ResourceDownloadReady {
  readonly kind: "ready";
  readonly transportKind: ResourceDownloadTransportKind;
  readonly url: string;
  readonly method: "GET" | "POST";
  readonly headers: Readonly<Record<string, string>>;
  readonly fields: Readonly<Record<string, string>>;
  readonly objectKey: string | null;
  readonly expiresAt: string | null;
  readonly [forgeSignalResourceDownloadBrand]: "resourceDownload";
}
export interface ResourceDownloadUnavailable {
  readonly kind: "unavailable"; readonly reason: "notReady" | "unavailable"; readonly detail: string;
  readonly [forgeSignalResourceDownloadBrand]: "resourceDownload";
}
export interface ResourceDownloadIncompatible {
  readonly kind: "incompatible"; readonly reason: "staleDescriptor" | "transportBoundary"; readonly detail: string;
  readonly [forgeSignalResourceDownloadBrand]: "resourceDownload";
}
export type ResourceDownloadDescriptorState = ResourceDownloadReady | ResourceDownloadUnavailable | ResourceDownloadIncompatible;
export interface ResourceBinaryDescriptor {
  readonly kind: "file" | "media" | "export"; readonly id: string; readonly label: string | null;
  readonly fileName: string | null; readonly mediaType: string | null; readonly byteLength: number | null;
  readonly download: ResourceDownloadDescriptorState;
  readonly [forgeSignalResourceBinaryDescriptorBrand]: "resourceBinaryDescriptor";
}
export interface ResourceBinaryValue<TValue> {
  readonly value: TValue; readonly descriptors: readonly ResourceBinaryDescriptor[];
  readonly [forgeSignalResourceBinaryValueBrand]: "resourceBinaryValue";
}

export interface ResourceDownload {
  ready(options: {
    url: string;
    method: "GET" | "POST";
    headers?: Readonly<Record<string, string>>;
    expiresAt?: string | null;
  }): ResourceDownloadReady;
  multipart(options: {
    url: string;
    headers?: Readonly<Record<string, string>>;
    fields?: Readonly<Record<string, string>>;
    objectKey?: string | null;
    expiresAt?: string | null;
  }): ResourceDownloadReady;
  unavailable(options: {
    reason: "notReady" | "unavailable";
    detail: string;
  }): ResourceDownloadUnavailable;
  incompatible(options: {
    reason: "staleDescriptor" | "transportBoundary";
    detail: string;
  }): ResourceDownloadIncompatible;
}

export interface ResourceBinaryDescriptorFactory {
  file(options: {
    id: string;
    label?: string | null;
    fileName?: string | null;
    mediaType?: string | null;
    byteLength?: number | null;
    download: ResourceDownloadDescriptorState;
  }): ResourceBinaryDescriptor;
  media(options: {
    id: string;
    label?: string | null;
    fileName?: string | null;
    mediaType?: string | null;
    byteLength?: number | null;
    download: ResourceDownloadDescriptorState;
  }): ResourceBinaryDescriptor;
  export(options: {
    id: string;
    label?: string | null;
    fileName?: string | null;
    mediaType?: string | null;
    byteLength?: number | null;
    download: ResourceDownloadDescriptorState;
  }): ResourceBinaryDescriptor;
}

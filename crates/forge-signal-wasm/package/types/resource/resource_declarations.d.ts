import type { SignalValue } from "../model.js";
import type {
  DeclaredResourceParams,
  ResourceAuthPosture,
  ResourceContinuationPosture,
  ResourceParamIdentity,
  ResourcePolicyProfile,
  ResourceProcessingJobPosture,
  ResourceProcessingResultValue,
  ResourceRequestContext,
  ResourceRequestDescriptor,
  ResourceUploadResultValue,
  ResourceUploadTransportPosture,
} from "./resource_postures.js";
import type { ResourceLine } from "./resource_lifecycle.js";
import type {
  ResourceItemAspectMap,
  ResourceLineReconciliation,
  ResourceCollectionShape,
  ResourcePatchForReconcile,
  ResourceReconcileAspectMap,
  ResourcePatchResult,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";

type ResourcePlainValue<TValue> =
  TValue extends ResourceProcessingResultValue | ResourceUploadResultValue
    ? never
    : TValue;

type ResourceProcessingCompatibleValue<TValue> =
  TValue extends ResourceUploadResultValue ? never : TValue;

type ResourceUploadCompatibleValue<TValue> =
  TValue extends ResourceProcessingResultValue ? never : TValue;

export interface DetailResourceDeclaration<TParams, TValue> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourcePlainValue<TValue>;
}

export interface ProcessingDetailResourceDeclaration<TParams, TValue> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceProcessingCompatibleValue<TValue> | ResourceProcessingResultValue;
}

export interface UploadDetailResourceDeclaration<TParams, TValue> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob?:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceUploadCompatibleValue<TValue> | ResourceUploadResultValue;
}

export interface ProcessingUploadDetailResourceDeclaration<TParams, TValue> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): TValue | ResourceProcessingResultValue | ResourceUploadResultValue;
}

export interface CollectionResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourcePlainValue<TValue>;
}

export interface ProcessingCollectionResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceProcessingCompatibleValue<TValue> | ResourceProcessingResultValue;
}

export interface UploadCollectionResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob?:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceUploadCompatibleValue<TValue> | ResourceUploadResultValue;
}

export interface ProcessingUploadCollectionResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): TValue | ResourceProcessingResultValue | ResourceUploadResultValue;
}

export interface PagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  accumulatePage(existing: TValue, next: TValue): TValue;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourcePlainValue<TValue>;
}

export interface ProcessingPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  accumulatePage(existing: TValue, next: TValue): TValue;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceProcessingCompatibleValue<TValue> | ResourceProcessingResultValue;
}

export interface UploadPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob?:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  accumulatePage(existing: TValue, next: TValue): TValue;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceUploadCompatibleValue<TValue> | ResourceUploadResultValue;
}

export interface ProcessingUploadPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  processingJob:
    | ResourceProcessingJobPosture
    | ((params: TParams) => ResourceProcessingJobPosture);
  uploadTransport:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  accumulatePage(existing: TValue, next: TValue): TValue;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): TValue | ResourceProcessingResultValue | ResourceUploadResultValue;
}

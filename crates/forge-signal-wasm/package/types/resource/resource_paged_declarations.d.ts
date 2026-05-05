import type { SignalValue } from "../model.js";
import type {
  DeclaredResourceParams,
  ResourceAuthPosture,
  ResourceContinuationPosture,
  ResourceParamIdentity,
  ResourcePolicyProfile,
  ResourceProcessingJobPosture,
  ResourceProcessingResultValue,
  ResourceBinaryValue,
  ResourceRequestContext,
  ResourceRequestDescriptor,
  ResourceUploadResultValue,
  ResourceUploadTransportPosture,
} from "./resource_postures.js";
import type {
  CollectionResourceDeclaration,
  ResourceExternalDefinitionVersion,
  ResourceExternalRequestContract,
} from "./resource_declarations.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";

type ResourcePlainValue<TValue> =
  TValue extends ResourceProcessingResultValue | ResourceUploadResultValue
    ? never
    : TValue;

type ResourceBinaryCompatibleValue<TValue> =
  | ResourcePlainValue<TValue>
  | ResourceBinaryValue<ResourcePlainValue<TValue>>;

type ResourceProcessingCompatibleValue<TValue> =
  TValue extends ResourceUploadResultValue ? never : TValue;

type ResourceUploadCompatibleValue<TValue> =
  TValue extends ResourceProcessingResultValue ? never : TValue;

type ResourceBinaryProcessingCompatibleValue<TValue> =
  | ResourceProcessingCompatibleValue<TValue>
  | ResourceBinaryValue<ResourcePlainValue<ResourceProcessingCompatibleValue<TValue>>>;

type ResourceBinaryUploadCompatibleValue<TValue> =
  | ResourceUploadCompatibleValue<TValue>
  | ResourceBinaryValue<ResourcePlainValue<ResourceUploadCompatibleValue<TValue>>>;

export interface PagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
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
  ): ResourceBinaryCompatibleValue<TValue>;
}

export interface ProcessingPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
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
  ): ResourceBinaryProcessingCompatibleValue<TValue> | ResourceProcessingResultValue;
}

export interface UploadPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
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
  ): ResourceBinaryUploadCompatibleValue<TValue> | ResourceUploadResultValue;
}

export interface ProcessingUploadPagedResourceDeclaration<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
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
  ):
    | ResourceBinaryCompatibleValue<TValue>
    | ResourceProcessingResultValue
    | ResourceUploadResultValue;
}

export interface ExternalPagedResourceDefinition<
  TParams,
  TValue,
  TItem = SignalValue,
  TReconcile extends ResourceCollectionShape<
    TValue,
    TItem,
    ResourceItemAspectMap<TItem>,
    ResourceValueSummaryMap<TValue>,
    any
  > | undefined = undefined,
> {
  readonly version: ResourceExternalDefinitionVersion;
  readonly family: "paged";
  readonly definitionId: string;
  readonly requestContract: ResourceExternalRequestContract;
  readonly reconciliationContract:
    TReconcile extends undefined ? "none" : "paged-v1";
  readonly declaration: PagedResourceDeclaration<TParams, TValue, TItem, TReconcile>;
}

export type {
  CollectionResourceDeclaration,
};

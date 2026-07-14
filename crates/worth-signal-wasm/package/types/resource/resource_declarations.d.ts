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
  ResourceRequestMethod,
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
  ResourceDetailReconcile,
  ResourceReconcileAspectMap,
  ResourcePatchResult,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type { ResourceEffectProfile } from "./resource_effect_profiles.js";

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

type ResourceMaybePromise<TValue> = TValue | PromiseLike<TValue>;
type ResourceEffectProfileInput<TParams> =
  | ResourceEffectProfile
  | ((params: TParams) => ResourceEffectProfile);

export interface DetailResourceDeclaration<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
  reconcile?: TReconcile;
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceMaybePromise<ResourceBinaryCompatibleValue<Awaited<TValue>>>;
}

export interface ProcessingDetailResourceDeclaration<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
  reconcile?: TReconcile;
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
  ): ResourceMaybePromise<
    | ResourceBinaryProcessingCompatibleValue<Awaited<TValue>>
    | ResourceProcessingResultValue
  >;
}

export interface UploadDetailResourceDeclaration<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
  reconcile?: TReconcile;
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
  ): ResourceMaybePromise<
    | ResourceBinaryUploadCompatibleValue<Awaited<TValue>>
    | ResourceUploadResultValue
  >;
}

export interface ProcessingUploadDetailResourceDeclaration<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  params: DeclaredResourceParams<TParams>;
  baseUrl?: string | ((params: TParams) => string);
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
  reconcile?: TReconcile;
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
  ): ResourceMaybePromise<
    | ResourceBinaryCompatibleValue<Awaited<TValue>>
    | ResourceProcessingResultValue
    | ResourceUploadResultValue
  >;
}

export interface CollectionResourceDeclaration<
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
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
  uploadTransport?:
    | ResourceUploadTransportPosture
    | ((params: TParams) => ResourceUploadTransportPosture);
  reconcile?: TReconcile;
  normalizeParams(params: TParams): ResourceParamIdentity<TParams>;
  itemIdentity(item: TItem): string;
  load(
    params: TParams,
    request: ResourceRequestDescriptor<TParams>,
  ): ResourceMaybePromise<ResourceBinaryCompatibleValue<Awaited<TValue>>>;
}

export interface ProcessingCollectionResourceDeclaration<
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
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
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
  ): ResourceMaybePromise<
    | ResourceBinaryProcessingCompatibleValue<Awaited<TValue>>
    | ResourceProcessingResultValue
  >;
}

export interface UploadCollectionResourceDeclaration<
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
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
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
  ): ResourceMaybePromise<
    | ResourceBinaryUploadCompatibleValue<Awaited<TValue>>
    | ResourceUploadResultValue
  >;
}

export interface ProcessingUploadCollectionResourceDeclaration<
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
  method?: ResourceRequestMethod;
  policy?: ResourcePolicyProfile;
  auth?: ResourceAuthPosture | ((params: TParams) => ResourceAuthPosture);
  requestContext?:
    | ResourceRequestContext
    | ((params: TParams) => ResourceRequestContext);
  continuation?:
    | ResourceContinuationPosture
    | ((params: TParams) => ResourceContinuationPosture);
  requestBody?: (params: TParams) => unknown;
  effects?: ResourceEffectProfileInput<TParams>;
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
  ): ResourceMaybePromise<
    | ResourceBinaryCompatibleValue<Awaited<TValue>>
    | ResourceProcessingResultValue
    | ResourceUploadResultValue
  >;
}

export type {
  ExternalPagedResourceDefinition,
  PagedResourceDeclaration,
  ProcessingPagedResourceDeclaration,
  ProcessingUploadPagedResourceDeclaration,
  UploadPagedResourceDeclaration,
} from "./resource_paged_declarations.js";

export type ResourceExternalDefinitionVersion =
  "worth-resource-external-v1";

export type ResourceExternalRequestContract = "native-v1";

export type ResourceExternalReconciliationContract =
  | "none"
  | "collection-v1"
  | "paged-v1";

export interface ExternalDetailResourceDefinition<
  TParams,
  TValue,
  TReconcile extends ResourceDetailReconcile<TValue> | undefined = undefined,
> {
  readonly version: ResourceExternalDefinitionVersion;
  readonly family: "detail";
  readonly definitionId: string;
  readonly requestContract: ResourceExternalRequestContract;
  readonly reconciliationContract: "none";
  readonly declaration: DetailResourceDeclaration<TParams, TValue, TReconcile>;
}

export interface ExternalCollectionResourceDefinition<
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
  readonly family: "collection";
  readonly definitionId: string;
  readonly requestContract: ResourceExternalRequestContract;
  readonly reconciliationContract:
    TReconcile extends undefined ? "none" : "collection-v1";
  readonly declaration: CollectionResourceDeclaration<
    TParams,
    TValue,
    TItem,
    TReconcile
  >;
}


import type { SignalValue } from "../model.js";
import type { CallableSignals, ScopedSignalNamespace } from "../callable_surface.js";
import type {
  DetailResourceFamily,
  CollectionResourceFamily,
  PagedResourceFamily,
} from "./resource_family_surfaces.js";
import type {
  DetailResourceDeclaration,
  ProcessingDetailResourceDeclaration,
  UploadDetailResourceDeclaration,
  ProcessingUploadDetailResourceDeclaration,
  CollectionResourceDeclaration,
  ProcessingCollectionResourceDeclaration,
  UploadCollectionResourceDeclaration,
  ProcessingUploadCollectionResourceDeclaration,
  PagedResourceDeclaration,
  ProcessingPagedResourceDeclaration,
  UploadPagedResourceDeclaration,
  ProcessingUploadPagedResourceDeclaration,
} from "./resource_declarations.js";
import type {
  DeclaredResourceParams,
  ResourceAuth,
  ResourceContinuation,
  ResourceParamIdentity,
  ResourcePolicyProfiles,
  ResourceProcessingJob,
  ResourceProcessingResult,
  ResourceRequestContext,
  ResourceRequestContextOptions,
  ResourceUploadResult,
  ResourceUploadTransport,
} from "./resource_postures.js";
import type {
  ResourceCollectionShape,
  ResourceItemAspectMap,
  ResourceItemAspects,
  ResourcePatchFactory,
  ResourceValueSummaryMap,
  ResourceValueSummaries,
} from "./resource_reconciliation.js";

export interface ResourceNamespace {
  detail<TParams, TValue>(
    declaration: ProcessingUploadDetailResourceDeclaration<TParams, TValue>,
  ): DetailResourceFamily<TParams, TValue | null>;
  detail<TParams, TValue>(
    declaration: ProcessingDetailResourceDeclaration<TParams, TValue>,
  ): DetailResourceFamily<TParams, TValue | null>;
  detail<TParams, TValue>(
    declaration: UploadDetailResourceDeclaration<TParams, TValue>,
  ): DetailResourceFamily<TParams, TValue | null>;
  detail<TParams, TValue>(
    declaration: DetailResourceDeclaration<TParams, TValue>,
  ): DetailResourceFamily<TParams, TValue>;
  collection<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: ProcessingUploadCollectionResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): CollectionResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  collection<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: ProcessingCollectionResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): CollectionResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  collection<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: UploadCollectionResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): CollectionResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  collection<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: CollectionResourceDeclaration<TParams, TValue, TItem, TReconcile>,
  ): CollectionResourceFamily<TParams, TValue, TItem, TReconcile>;
  paged<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: ProcessingUploadPagedResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): PagedResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  paged<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: ProcessingPagedResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): PagedResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  paged<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: UploadPagedResourceDeclaration<
      TParams,
      TValue,
      TItem,
      TReconcile
    >,
  ): PagedResourceFamily<TParams, TValue | null, TItem, TReconcile>;
  paged<
    TParams,
    TValue,
    TItem = SignalValue,
    TReconcile extends ResourceCollectionShape<
      TValue,
      TItem,
      ResourceItemAspectMap<TItem>,
      ResourceValueSummaryMap<TValue>
    > | undefined = undefined,
  >(
    declaration: PagedResourceDeclaration<TParams, TValue, TItem, TReconcile>,
  ): PagedResourceFamily<TParams, TValue, TItem, TReconcile>;
}

export function resourceParams<TParams>(): DeclaredResourceParams<TParams>;

export function resourceParamIdentity<TParams>(
  params: TParams,
  canonicalKey: string,
): ResourceParamIdentity<TParams>;

export const resourceAuth: ResourceAuth;
export const resourceContinuation: ResourceContinuation;
export const resourcePatch: ResourcePatchFactory;
export const resourcePolicyProfiles: ResourcePolicyProfiles;
export const resourceProcessingJob: ResourceProcessingJob;
export const resourceProcessingResult: ResourceProcessingResult;
export const resourceUploadTransport: ResourceUploadTransport;
export const resourceUploadResult: ResourceUploadResult;

export function resourceItemAspects<
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
>(definitions: TAspectMap): ResourceItemAspects<TItem, TAspectMap>;

export function resourceValueSummaries<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
>(definitions: TSummaryMap): ResourceValueSummaries<TValue, TSummaryMap>;

export function resourceValueSummaries<
  TValue,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
>(definitions: TSummaryMap): ResourceValueSummaries<TValue, TSummaryMap>;

export function resourceCollectionShape<
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
>(options: {
  items(value: TValue): readonly TItem[];
  replaceItems(value: TValue, nextItems: readonly TItem[]): TValue;
  aspects?: ResourceItemAspects<TItem, TAspectMap>;
  summaries?: ResourceValueSummaries<TValue, TSummaryMap>;
}): ResourceCollectionShape<TValue, TItem, TAspectMap, TSummaryMap>;

export function resourceRequestContext(
  options?: ResourceRequestContextOptions,
): ResourceRequestContext;

declare module "../callable_surface.js" {
  interface CallableSignals<TPersistence = SignalValue> {
    readonly resource: ResourceNamespace;
  }

  interface ScopedSignalNamespace<TPersistence = SignalValue> {
    readonly resource: ResourceNamespace;
  }
}

import type { SignalValue } from "../model.js";
import type {
  CollectionResourceDeclaration,
  PagedResourceDeclaration,
  ProcessingCollectionResourceDeclaration,
  ProcessingPagedResourceDeclaration,
  ProcessingUploadCollectionResourceDeclaration,
  ProcessingUploadPagedResourceDeclaration,
  UploadCollectionResourceDeclaration,
  UploadPagedResourceDeclaration,
} from "./resource_declarations.js";
import type {
  ResourceItemAspectMap,
  ResourceSummaryPatchScope,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiInlineReconcile,
  ApiRequestParamsShape,
  ApiRouteLineParams,
} from "./api_request_params.js";
import type { ApiRouteDownloadsDeclaration } from "./api_route_downloads.js";

type ApiRouteHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

type ApiRouteReconcileBoundDeclaration<TDeclaration, TParams> =
  Omit<TDeclaration, "itemIdentity" | "reconcile" | "params" | "normalizeParams" | "method" | "requestBody"> & {
    headers?: ApiRouteHeaders<TParams>;
    itemIdentity?: never;
    reconcile?: never;
    params?: never;
    normalizeParams?: never;
    baseUrl?: never;
    method?: never;
    requestBody?: never;
  };

type ApiRouteReconcileUploadOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteReconcileBoundDeclaration<Omit<TDeclaration, "uploadTransport">, TParams> & {
    uploadTransport?: never;
  };

type ApiRouteReconcileProcessingOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteReconcileBoundDeclaration<Omit<TDeclaration, "processingJob">, TParams> & {
    processingJob?: never;
  };

type ApiRouteReconcileProcessingUploadOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteReconcileBoundDeclaration<Omit<TDeclaration, "processingJob" | "uploadTransport">, TParams> & {
    processingJob?: never;
    uploadTransport?: never;
  };

type ApiRouteReconcileDownloadsField<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TBody,
  TValue,
  TDownloadsOwned extends boolean,
> = TDownloadsOwned extends true
  ? { downloads?: never }
  : {
      downloads?: ApiRouteDownloadsDeclaration<
        ApiRouteLineParams<TRoute, TRequestParams, TBody>,
        TValue
      >;
    };

export type ApiRouteReconcileCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileProcessingOwnedDeclaration<
  CollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingReconcileCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileUploadOwnedDeclaration<
  ProcessingCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteUploadReconcileCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileProcessingUploadOwnedDeclaration<
  UploadCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadReconcileCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileProcessingUploadOwnedDeclaration<
  ProcessingUploadCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteReconcilePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileBoundDeclaration<
  PagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingReconcilePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileProcessingOwnedDeclaration<
  ProcessingPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteUploadReconcilePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileUploadOwnedDeclaration<
  UploadPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadReconcilePagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TBody = undefined,
  TItem = SignalValue,
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteReconcileProcessingUploadOwnedDeclaration<
  ProcessingUploadPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteReconcileDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

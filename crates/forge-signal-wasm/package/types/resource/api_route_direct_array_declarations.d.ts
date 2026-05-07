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
  ApiImplicitArrayReconcile,
  ApiRequestParamsShape,
  ApiRouteLineParams,
} from "./api_request_params.js";
import type { ApiRouteDownloadsDeclaration } from "./api_route_downloads.js";

type ApiRouteDirectArrayHeaders<TParams> =
  | Record<string, string>
  | ((params: TParams) => Record<string, string>);

type ApiRouteDirectArrayBoundDeclaration<TDeclaration, TParams> =
  Omit<TDeclaration, "itemIdentity" | "reconcile" | "params" | "normalizeParams" | "method" | "requestBody"> & {
    headers?: ApiRouteDirectArrayHeaders<TParams>;
    itemIdentity?: never;
    reconcile?: never;
    params?: never;
    normalizeParams?: never;
    baseUrl?: never;
    method?: never;
    requestBody?: never;
  };

type ApiRouteDirectArrayUploadOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteDirectArrayBoundDeclaration<Omit<TDeclaration, "uploadTransport">, TParams> & {
    uploadTransport?: never;
  };

type ApiRouteDirectArrayProcessingOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteDirectArrayBoundDeclaration<Omit<TDeclaration, "processingJob">, TParams> & {
    processingJob?: never;
  };

type ApiRouteDirectArrayProcessingUploadOwnedDeclaration<TDeclaration, TParams> =
  ApiRouteDirectArrayBoundDeclaration<Omit<TDeclaration, "processingJob" | "uploadTransport">, TParams> & {
    processingJob?: never;
    uploadTransport?: never;
  };

type ApiRouteDirectArrayDownloadsField<
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

export type ApiRouteDirectArrayCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayProcessingOwnedDeclaration<
  CollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingDirectArrayCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayUploadOwnedDeclaration<
  ProcessingCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteUploadDirectArrayCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayProcessingUploadOwnedDeclaration<
  UploadCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadDirectArrayCollectionDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayProcessingUploadOwnedDeclaration<
  ProcessingUploadCollectionResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteDirectArrayPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayBoundDeclaration<
  PagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingDirectArrayPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayProcessingOwnedDeclaration<
  ProcessingPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteUploadDirectArrayPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayUploadOwnedDeclaration<
  UploadPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

export type ApiRouteProcessingUploadDirectArrayPagedDeclaration<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TBody = undefined,
  TItem = TValue[number],
  TAspectMap extends ResourceItemAspectMap<TItem> = {},
  TSummaryMap extends ResourceValueSummaryMap<TValue> = {},
  TSummaryPatchScope extends ResourceSummaryPatchScope = "line",
  TDownloadsOwned extends boolean = false,
> = ApiRouteDirectArrayProcessingUploadOwnedDeclaration<
  ProcessingUploadPagedResourceDeclaration<
    ApiRouteLineParams<TRoute, TRequestParams, TBody>,
    TValue,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      TSummaryPatchScope
    >
  >,
  ApiRouteLineParams<TRoute, TRequestParams, TBody>
> & ApiRouteDirectArrayDownloadsField<TRoute, TRequestParams, TBody, TValue, TDownloadsOwned>;

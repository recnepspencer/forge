import type {
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiCollectionResourceFamily,
  ApiImplicitArrayReconcile,
  ApiPagedResourceFamily,
  ApiRequestParamsShape,
} from "./api_request_params.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type { ApiRouteTransferValue } from "./api_route_collection_transfer_state.js";
import type {
  ApiRouteDirectArrayCollectionDeclaration,
  ApiRouteDirectArrayPagedDeclaration,
  ApiRouteProcessingDirectArrayCollectionDeclaration,
  ApiRouteProcessingDirectArrayPagedDeclaration,
  ApiRouteProcessingUploadDirectArrayCollectionDeclaration,
  ApiRouteProcessingUploadDirectArrayPagedDeclaration,
  ApiRouteUploadDirectArrayCollectionDeclaration,
  ApiRouteUploadDirectArrayPagedDeclaration,
} from "./api_route_direct_array_declarations.js";
import type {
  ApiRouteArrayItemsSummaryMode,
  ApiRouteArrayItemsSummaryPatchScope,
} from "./api_route_array_items_common.js";

interface ApiRouteArrayItemsBuilderBaseCore<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody,
  TDownloadsOwned extends boolean,
  THeadersOwned extends boolean,
  TCollectionDeclaration,
  TPagedDeclaration,
> {
  list(
    declaration: ApiRouteArrayItemsOwnedHeadersDeclaration<
      TCollectionDeclaration,
      THeadersOwned
    >,
  ): ApiCollectionResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
    >,
    TBody
  >;
  paged(
    declaration: ApiRouteArrayItemsOwnedHeadersDeclaration<
      TPagedDeclaration,
      THeadersOwned
    >,
  ): ApiPagedResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    ApiImplicitArrayReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
    >,
    TBody
  >;
}

type ApiRouteArrayItemsOwnedHeadersDeclaration<
  TDeclaration,
  THeadersOwned extends boolean,
> = THeadersOwned extends true
  ? Omit<TDeclaration, "headers"> & { headers?: never }
  : TDeclaration;

export type ApiRouteArrayItemsBuilderBaseNone<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteArrayItemsBuilderBaseCore<
  TRoute,
  TRequestParams,
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryMode,
  "none",
  "none",
  TBody,
  TDownloadsOwned,
  THeadersOwned,
  ApiRouteDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >,
  ApiRouteDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >
>;

export type ApiRouteArrayItemsBuilderBaseUpload<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TUploadKind extends Exclude<ApiRouteUploadKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteArrayItemsBuilderBaseCore<
  TRoute,
  TRequestParams,
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryMode,
  "none",
  TUploadKind,
  TBody,
  TDownloadsOwned,
  THeadersOwned,
  ApiRouteUploadDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >,
  ApiRouteUploadDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >
>;

export type ApiRouteArrayItemsBuilderBaseProcessing<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends Exclude<ApiRouteProcessingKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteArrayItemsBuilderBaseCore<
  TRoute,
  TRequestParams,
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryMode,
  TProcessingKind,
  "none",
  TBody,
  TDownloadsOwned,
  THeadersOwned,
  ApiRouteProcessingDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >,
  ApiRouteProcessingDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >
>;

export type ApiRouteArrayItemsBuilderBaseProcessingUpload<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends Exclude<ApiRouteProcessingKind, "none">,
  TUploadKind extends Exclude<ApiRouteUploadKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteArrayItemsBuilderBaseCore<
  TRoute,
  TRequestParams,
  TValue,
  TItem,
  TAspectMap,
  TSummaryMap,
  TSummaryMode,
  TProcessingKind,
  TUploadKind,
  TBody,
  TDownloadsOwned,
  THeadersOwned,
  ApiRouteProcessingUploadDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >,
  ApiRouteProcessingUploadDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TBody,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>,
    TDownloadsOwned
  >
>;

export type ApiRouteArrayItemsBuilderBase<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
  TBody = undefined,
  THeadersOwned extends boolean = false,
> = TProcessingKind extends "none"
  ? TUploadKind extends "none"
    ? ApiRouteArrayItemsBuilderBaseNone<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TAspectMap,
        TSummaryMap,
        TSummaryMode,
        TBody
        ,
        THeadersOwned
      >
    : ApiRouteArrayItemsBuilderBaseUpload<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TAspectMap,
        TSummaryMap,
        TSummaryMode,
        Exclude<TUploadKind, "none">,
        TBody,
        THeadersOwned
      >
  : TUploadKind extends "none"
    ? ApiRouteArrayItemsBuilderBaseProcessing<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TAspectMap,
        TSummaryMap,
        TSummaryMode,
        Exclude<TProcessingKind, "none">,
        TBody,
        THeadersOwned
      >
    : ApiRouteArrayItemsBuilderBaseProcessingUpload<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TAspectMap,
        TSummaryMap,
        TSummaryMode,
        Exclude<TProcessingKind, "none">,
        Exclude<TUploadKind, "none">,
        TBody,
        THeadersOwned
      >;

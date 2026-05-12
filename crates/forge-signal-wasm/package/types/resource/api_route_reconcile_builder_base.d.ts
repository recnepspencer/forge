import type {
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type {
  ApiCollectionResourceFamily,
  ApiInlineReconcile,
  ApiPagedResourceFamily,
  ApiRequestParamsShape,
} from "./api_request_params.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type { ApiRouteTransferValue } from "./api_route_collection_transfer_state.js";
import type {
  ApiRouteArrayItemsSummaryMode,
  ApiRouteArrayItemsSummaryPatchScope,
} from "./api_route_array_items_common.js";
import type {
  ApiRouteProcessingReconcileCollectionDeclaration,
  ApiRouteProcessingReconcilePagedDeclaration,
  ApiRouteProcessingUploadReconcileCollectionDeclaration,
  ApiRouteProcessingUploadReconcilePagedDeclaration,
  ApiRouteReconcileCollectionDeclaration,
  ApiRouteReconcilePagedDeclaration,
  ApiRouteUploadReconcileCollectionDeclaration,
  ApiRouteUploadReconcilePagedDeclaration,
} from "./api_route_reconcile_declarations.js";

interface ApiRouteReconcileBuilderBaseCore<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
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
    declaration: ApiRouteReconcileOwnedHeadersDeclaration<
      TCollectionDeclaration,
      THeadersOwned
    >,
  ): ApiCollectionResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
    >,
    TBody
  >;
  paged(
    declaration: ApiRouteReconcileOwnedHeadersDeclaration<
      TPagedDeclaration,
      THeadersOwned
    >,
  ): ApiPagedResourceFamily<
    TRoute,
    TRequestParams,
    ApiRouteTransferValue<TValue, TProcessingKind, TUploadKind>,
    TItem,
    ApiInlineReconcile<
      TValue,
      TItem,
      TAspectMap,
      TSummaryMap,
      ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
    >,
    TBody
  >;
}

type ApiRouteReconcileOwnedHeadersDeclaration<
  TDeclaration,
  THeadersOwned extends boolean,
> = THeadersOwned extends true
  ? Omit<TDeclaration, "headers"> & { headers?: never }
  : TDeclaration;

export type ApiRouteReconcileBuilderBaseNone<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteReconcileBuilderBaseCore<
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
  ApiRouteReconcileCollectionDeclaration<
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
  ApiRouteReconcilePagedDeclaration<
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

export type ApiRouteReconcileBuilderBaseUpload<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TUploadKind extends Exclude<ApiRouteUploadKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteReconcileBuilderBaseCore<
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
  ApiRouteUploadReconcileCollectionDeclaration<
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
  ApiRouteUploadReconcilePagedDeclaration<
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

export type ApiRouteReconcileBuilderBaseProcessing<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends Exclude<ApiRouteProcessingKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteReconcileBuilderBaseCore<
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
  ApiRouteProcessingReconcileCollectionDeclaration<
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
  ApiRouteProcessingReconcilePagedDeclaration<
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

export type ApiRouteReconcileBuilderBaseProcessingUpload<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends Exclude<ApiRouteProcessingKind, "none">,
  TUploadKind extends Exclude<ApiRouteUploadKind, "none">,
  TBody = undefined,
  TDownloadsOwned extends boolean = false,
  THeadersOwned extends boolean = false,
> = ApiRouteReconcileBuilderBaseCore<
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
  ApiRouteProcessingUploadReconcileCollectionDeclaration<
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
  ApiRouteProcessingUploadReconcilePagedDeclaration<
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

export type ApiRouteReconcileBuilderBase<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
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
    ? ApiRouteReconcileBuilderBaseNone<
        TRoute,
        TRequestParams,
        TValue,
        TItem,
        TAspectMap,
        TSummaryMap,
        TSummaryMode,
        TBody,
        THeadersOwned
      >
    : ApiRouteReconcileBuilderBaseUpload<
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
    ? ApiRouteReconcileBuilderBaseProcessing<
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
    : ApiRouteReconcileBuilderBaseProcessingUpload<
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

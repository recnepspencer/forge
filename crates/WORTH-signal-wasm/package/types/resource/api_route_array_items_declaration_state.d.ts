import type {
  ResourceItemAspectMap,
  ResourceValueSummaryMap,
} from "./resource_reconciliation.js";
import type { ApiRequestParamsShape } from "./api_request_params.js";
import type {
  ApiRouteProcessingKind,
  ApiRouteUploadKind,
} from "./api_route_transfer_kinds.js";
import type { ApiRouteDeclarationForTransferState } from "./api_route_collection_transfer_state.js";
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

export type ApiRouteArrayItemsCollectionDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteDeclarationForTransferState<
  ApiRouteDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteUploadDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingUploadDirectArrayCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  TProcessingKind,
  TUploadKind
>;

export type ApiRouteArrayItemsPagedDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue extends readonly TItem[],
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteDeclarationForTransferState<
  ApiRouteDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteUploadDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingUploadDirectArrayPagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  TProcessingKind,
  TUploadKind
>;

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

export type ApiRouteReconcileCollectionDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteDeclarationForTransferState<
  ApiRouteReconcileCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteUploadReconcileCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingReconcileCollectionDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingUploadReconcileCollectionDeclaration<
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

export type ApiRouteReconcilePagedDeclarationForState<
  TRoute extends string,
  TRequestParams extends ApiRequestParamsShape | undefined,
  TValue,
  TItem,
  TAspectMap extends ResourceItemAspectMap<TItem>,
  TSummaryMap extends ResourceValueSummaryMap<TValue>,
  TSummaryMode extends ApiRouteArrayItemsSummaryMode,
  TProcessingKind extends ApiRouteProcessingKind,
  TUploadKind extends ApiRouteUploadKind,
> = ApiRouteDeclarationForTransferState<
  ApiRouteReconcilePagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteUploadReconcilePagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingReconcilePagedDeclaration<
    TRoute,
    TRequestParams,
    TValue,
    TItem,
    TAspectMap,
    TSummaryMap,
    ApiRouteArrayItemsSummaryPatchScope<TSummaryMode>
  >,
  ApiRouteProcessingUploadReconcilePagedDeclaration<
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
